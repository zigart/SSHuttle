use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use uuid::Uuid;
use zeroize::Zeroize;

use crate::indicador::{self, EstadoIndicador, Indicador, MotivoInactiva};
use crate::retry::Espera;
use crate::session::{ErrorSesion, Pedido, SessionHandle};

#[derive(Debug, Clone)]
pub struct PerfilConexion {
    pub id: String,
    pub host: String,
    pub puerto: u16,
    pub usuario: String,
    pub secreto_id: String,
    pub huella_host: String,
}

#[derive(Debug)]
pub enum ErrorBoveda {
    Ausente,
    Llavero,
}

pub trait Boveda: Send + Sync {
    fn leer(&self, secreto_id: &str) -> Result<String, ErrorBoveda>;
    fn guardar(&self, secreto_id: &str, contrasena: &str) -> Result<(), ErrorBoveda>;
}

pub trait Perfiles: Send + Sync {
    fn guardar_secreto_id(&self, id: &str, secreto_id: &str) -> Result<(), String>;
    fn guardar_huella(&self, id: &str, huella: &str) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub enum Evento {
    Indicador { id: String, valor: Indicador, motivo: Option<MotivoInactiva> },
    Salida { id: String, datos: Vec<u8> },
    TrabajoInterrumpido { id: String },
}

enum Orden {
    Escribir(Vec<u8>),
    Redimensionar { columnas: u32, filas: u32 },
    Cerrar,
    ConfirmarHuella,
}

struct Vivo {
    ordenes: UnboundedSender<Orden>,
    generacion: u64,
}

enum FinBombeo {
    Usuario,
    Corte,
}

pub struct Supervisor {
    sesiones: Arc<Mutex<HashMap<String, Vivo>>>,
    boveda: Arc<dyn Boveda>,
    perfiles: Arc<dyn Perfiles>,
    eventos: UnboundedSender<Evento>,
    generacion: Arc<Mutex<u64>>,
    /// El hilo de la ventana no entra en Tokio. El lazo de reconexión se lanza en este runtime.
    runtime: tokio::runtime::Handle,
}

impl Supervisor {
    pub fn nuevo(
        boveda: Arc<dyn Boveda>,
        perfiles: Arc<dyn Perfiles>,
        eventos: UnboundedSender<Evento>,
        runtime: tokio::runtime::Handle,
    ) -> Self {
        Self {
            sesiones: Arc::new(Mutex::new(HashMap::new())),
            boveda,
            perfiles,
            eventos,
            generacion: Arc::new(Mutex::new(0)),
            runtime,
        }
    }

    pub fn conectar(&self, perfil: PerfilConexion, contrasena: Option<String>) {
        let mut sesiones = self.sesiones.lock().unwrap_or_else(|e| e.into_inner());
        if sesiones.get(&perfil.id).is_some_and(|vivo| !vivo.ordenes.is_closed()) {
            return;
        }
        let (tx, rx) = mpsc::unbounded_channel();
        let generacion = {
            let mut n = self.generacion.lock().unwrap_or_else(|e| e.into_inner());
            *n += 1;
            *n
        };
        sesiones.insert(perfil.id.clone(), Vivo { ordenes: tx, generacion });
        drop(sesiones);

        let sesiones = Arc::clone(&self.sesiones);
        let boveda = Arc::clone(&self.boveda);
        let perfiles = Arc::clone(&self.perfiles);
        let eventos = self.eventos.clone();
        self.runtime.spawn(async move {
            bucle(perfil, contrasena, boveda, perfiles, eventos, rx, sesiones, generacion).await;
        });
    }

    pub fn cerrar(&self, id: &str) {
        self.enviar(id, Orden::Cerrar);
    }

    pub fn confirmar_huella(&self, id: &str) {
        self.enviar(id, Orden::ConfirmarHuella);
    }

    pub fn escribir(&self, id: &str, datos: Vec<u8>) {
        self.enviar(id, Orden::Escribir(datos));
    }

    pub fn redimensionar(&self, id: &str, columnas: u32, filas: u32) {
        self.enviar(id, Orden::Redimensionar { columnas, filas });
    }

    fn enviar(&self, id: &str, orden: Orden) {
        let sesiones = self.sesiones.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(vivo) = sesiones.get(id) {
            let _ = vivo.ordenes.send(orden);
        } else if matches!(orden, Orden::Cerrar) {
            emitir(&self.eventos, id, EstadoIndicador::inactiva(MotivoInactiva::Cerrada));
        }
    }
}

async fn bucle(
    mut perfil: PerfilConexion,
    mut contrasena_inicial: Option<String>,
    boveda: Arc<dyn Boveda>,
    perfiles: Arc<dyn Perfiles>,
    eventos: UnboundedSender<Evento>,
    mut ordenes: UnboundedReceiver<Orden>,
    sesiones: Arc<Mutex<HashMap<String, Vivo>>>,
    generacion: u64,
) {
    let id = perfil.id.clone();
    let mut espera = Espera::nueva();
    let mut tras_corte = false;
    let mut columnas = 80u32;
    let mut filas = 24u32;

    if perfil.secreto_id.is_empty() && contrasena_inicial.is_none() {
        emitir(&eventos, &id, EstadoIndicador::inactiva(MotivoInactiva::FaltaSecreto));
        retirar(&sesiones, &id, generacion);
        return;
    }
    if !perfil.secreto_id.is_empty() && contrasena_inicial.is_none() {
        emitir(&eventos, &id, indicador::tras_corte_de_red());
    }

    loop {
        if let Some((c, f)) = aplicar_ordenes_previas(&mut ordenes, &mut columnas, &mut filas) {
            let _ = (c, f);
        } else {
            emitir(&eventos, &id, EstadoIndicador::inactiva(MotivoInactiva::Cerrada));
            break;
        }

        let clave = match obtener_clave(&boveda, &perfil, contrasena_inicial.as_deref()).await {
            Some(clave) => clave,
            None => {
                emitir(&eventos, &id, EstadoIndicador::inactiva(MotivoInactiva::FaltaSecreto));
                break;
            }
        };

        let pedido = Pedido {
            host: perfil.host.clone(),
            puerto: perfil.puerto,
            usuario: perfil.usuario.clone(),
            contrasena: clave,
            huella_guardada: perfil.huella_host.clone(),
            columnas,
            filas,
            tras_corte,
        };
        let abierto = abrir_cancelable(pedido, &mut ordenes, &mut columnas, &mut filas).await;
        let sesion = match abierto {
            Apertura::Lista(sesion) => sesion,
            Apertura::Cerrada => {
                emitir(&eventos, &id, EstadoIndicador::inactiva(MotivoInactiva::Cerrada));
                break;
            }
            Apertura::Fallo(ErrorSesion::ClaveRechazada) => {
                if let Some(mut clave) = contrasena_inicial.take() {
                    clave.zeroize();
                }
                emitir(&eventos, &id, EstadoIndicador::inactiva(MotivoInactiva::ClaveRechazada));
                break;
            }
            Apertura::Fallo(ErrorSesion::HuellaDistinta { presentada }) => {
                emitir(&eventos, &id, EstadoIndicador::inactiva(MotivoInactiva::HuellaDistinta));
                match esperar_confirmacion(&mut ordenes, &mut columnas, &mut filas).await {
                    Confirmacion::Aceptada => {
                        if perfiles.guardar_huella(&id, &presentada).is_err() {
                            break;
                        }
                        perfil.huella_host = presentada;
                        continue;
                    }
                    Confirmacion::Cerrada => {
                        emitir(&eventos, &id, EstadoIndicador::inactiva(MotivoInactiva::Cerrada));
                        break;
                    }
                }
            }
            Apertura::Fallo(ErrorSesion::Red) => {
                emitir(&eventos, &id, indicador::tras_corte_de_red());
                if !esperar_reintento(&mut ordenes, espera.siguiente(), &mut columnas, &mut filas)
                    .await
                {
                    emitir(&eventos, &id, EstadoIndicador::inactiva(MotivoInactiva::Cerrada));
                    break;
                }
                continue;
            }
        };

        let mut sesion = sesion;
        espera.reiniciar();
        if let Some(clave) = contrasena_inicial.clone() {
            let secreto_id = if perfil.secreto_id.is_empty() {
                Uuid::new_v4().to_string()
            } else {
                perfil.secreto_id.clone()
            };
            let guardado = tokio::task::spawn_blocking({
                let boveda = Arc::clone(&boveda);
                let secreto_id = secreto_id.clone();
                move || {
                    let resultado = boveda.guardar(&secreto_id, &clave);
                    let mut clave = clave;
                    clave.zeroize();
                    resultado
                }
            })
            .await;
            if matches!(guardado, Ok(Ok(())))
                && perfiles.guardar_secreto_id(&id, &secreto_id).is_ok()
            {
                perfil.secreto_id = secreto_id;
                if let Some(mut clave) = contrasena_inicial.take() {
                    clave.zeroize();
                }
            } else {
                tracing::warn!("no se pudo guardar el secreto");
            }
        }
        if perfil.huella_host.is_empty() && !sesion.huella().is_empty() {
            let huella = sesion.huella().to_string();
            if perfiles.guardar_huella(&id, &huella).is_ok() {
                perfil.huella_host = huella;
            }
        }
        if sesion.tomar_aviso() {
            let _ = eventos.send(Evento::TrabajoInterrumpido { id: id.clone() });
        }
        emitir(&eventos, &id, indicador::tras_autenticacion());
        match bombear(&mut sesion, &mut ordenes, &eventos, &id, &mut columnas, &mut filas).await {
            FinBombeo::Usuario => {
                emitir(&eventos, &id, EstadoIndicador::inactiva(MotivoInactiva::Cerrada));
                break;
            }
            FinBombeo::Corte => {
                tras_corte = true;
                emitir(&eventos, &id, indicador::tras_corte_de_red());
                if !esperar_reintento(&mut ordenes, espera.siguiente(), &mut columnas, &mut filas)
                    .await
                {
                    emitir(&eventos, &id, EstadoIndicador::inactiva(MotivoInactiva::Cerrada));
                    break;
                }
            }
        }
    }

    retirar(&sesiones, &id, generacion);
}

enum Apertura {
    Lista(SessionHandle),
    Cerrada,
    Fallo(ErrorSesion),
}

async fn abrir_cancelable(
    pedido: Pedido,
    ordenes: &mut UnboundedReceiver<Orden>,
    columnas: &mut u32,
    filas: &mut u32,
) -> Apertura {
    let apertura = SessionHandle::abrir(pedido);
    tokio::pin!(apertura);
    loop {
        tokio::select! {
            resultado = &mut apertura => {
                return match resultado {
                    Ok(sesion) => Apertura::Lista(sesion),
                    Err(error) => Apertura::Fallo(error),
                };
            }
            orden = ordenes.recv() => {
                match orden {
                    Some(Orden::Cerrar) | None => return Apertura::Cerrada,
                    Some(Orden::Redimensionar { columnas: c, filas: f }) => {
                        *columnas = c;
                        *filas = f;
                    }
                    Some(Orden::Escribir(_)) | Some(Orden::ConfirmarHuella) => {}
                }
            }
        }
    }
}

async fn bombear(
    sesion: &mut SessionHandle,
    ordenes: &mut UnboundedReceiver<Orden>,
    eventos: &UnboundedSender<Evento>,
    id: &str,
    columnas: &mut u32,
    filas: &mut u32,
) -> FinBombeo {
    loop {
        tokio::select! {
            biased;
            orden = ordenes.recv() => {
                match orden {
                    Some(Orden::Cerrar) | None => {
                        sesion.cerrar().await;
                        return FinBombeo::Usuario;
                    }
                    Some(Orden::Escribir(datos)) => {
                        if sesion.escribir(&datos).await.is_err() {
                            return FinBombeo::Corte;
                        }
                    }
                    Some(Orden::Redimensionar { columnas: c, filas: f }) => {
                        *columnas = c;
                        *filas = f;
                        let _ = sesion.redimensionar(c, f).await;
                    }
                    Some(Orden::ConfirmarHuella) => {}
                }
            }
            lectura = sesion.leer() => {
                match lectura {
                    sshuttle_terminal::Lectura::Datos(datos) => {
                        let _ = eventos.send(Evento::Salida { id: id.to_string(), datos });
                    }
                    sshuttle_terminal::Lectura::Ignorar => {}
                    sshuttle_terminal::Lectura::Fin => {
                        tracing::info!("el canal de la consola se cerró");
                        return FinBombeo::Corte;
                    }
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(400)) => {
                if sesion.red_caida() {
                    return FinBombeo::Corte;
                }
            }
        }
    }
}

enum Confirmacion {
    Aceptada,
    Cerrada,
}

async fn esperar_confirmacion(
    ordenes: &mut UnboundedReceiver<Orden>,
    columnas: &mut u32,
    filas: &mut u32,
) -> Confirmacion {
    loop {
        match ordenes.recv().await {
            Some(Orden::ConfirmarHuella) => return Confirmacion::Aceptada,
            Some(Orden::Cerrar) | None => return Confirmacion::Cerrada,
            Some(Orden::Redimensionar { columnas: c, filas: f }) => {
                *columnas = c;
                *filas = f;
            }
            Some(Orden::Escribir(_)) => {}
        }
    }
}

async fn esperar_reintento(
    ordenes: &mut UnboundedReceiver<Orden>,
    pausa: Duration,
    columnas: &mut u32,
    filas: &mut u32,
) -> bool {
    let espera = tokio::time::sleep(pausa);
    tokio::pin!(espera);
    loop {
        tokio::select! {
            _ = &mut espera => return true,
            orden = ordenes.recv() => {
                match orden {
                    Some(Orden::Cerrar) | None => return false,
                    Some(Orden::Redimensionar { columnas: c, filas: f }) => {
                        *columnas = c;
                        *filas = f;
                    }
                    Some(Orden::Escribir(_)) | Some(Orden::ConfirmarHuella) => {}
                }
            }
        }
    }
}

fn aplicar_ordenes_previas(
    ordenes: &mut UnboundedReceiver<Orden>,
    columnas: &mut u32,
    filas: &mut u32,
) -> Option<(u32, u32)> {
    while let Ok(orden) = ordenes.try_recv() {
        match orden {
            Orden::Cerrar => return None,
            Orden::Redimensionar { columnas: c, filas: f } => {
                *columnas = c;
                *filas = f;
            }
            Orden::Escribir(_) | Orden::ConfirmarHuella => {}
        }
    }
    Some((*columnas, *filas))
}

async fn obtener_clave(
    boveda: &Arc<dyn Boveda>,
    perfil: &PerfilConexion,
    contrasena: Option<&str>,
) -> Option<String> {
    if let Some(contrasena) = contrasena {
        return Some(contrasena.to_string());
    }
    if perfil.secreto_id.is_empty() {
        return None;
    }
    let secreto_id = perfil.secreto_id.clone();
    let boveda = Arc::clone(boveda);
    let leido = tokio::task::spawn_blocking(move || boveda.leer(&secreto_id)).await.ok()?;
    leido.ok()
}

fn emitir(eventos: &UnboundedSender<Evento>, id: &str, estado: EstadoIndicador) {
    let _ = eventos.send(Evento::Indicador {
        id: id.to_string(),
        valor: estado.valor,
        motivo: estado.motivo,
    });
}

fn retirar(sesiones: &Mutex<HashMap<String, Vivo>>, id: &str, generacion: u64) {
    let mut sesiones = sesiones.lock().unwrap_or_else(|e| e.into_inner());
    if sesiones.get(id).is_some_and(|vivo| vivo.generacion == generacion) {
        sesiones.remove(id);
    }
}
