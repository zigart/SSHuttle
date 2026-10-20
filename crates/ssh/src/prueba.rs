use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use russh::keys::PrivateKey;
use russh::server::{Auth, Handler, Msg, Session};
use russh::ChannelId;
use tokio::net::TcpListener;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::JoinHandle;

use crate::{
    Boveda, ErrorBoveda, Evento, Indicador, MotivoInactiva, PerfilConexion, Perfiles, Supervisor,
};

struct Memoria {
    secretos: Mutex<HashMap<String, String>>,
    huellas: Mutex<HashMap<String, String>>,
    secretos_id: Mutex<HashMap<String, String>>,
}

impl Boveda for Memoria {
    fn leer(&self, secreto_id: &str) -> Result<String, ErrorBoveda> {
        self.secretos.lock().unwrap().get(secreto_id).cloned().ok_or(ErrorBoveda::Ausente)
    }

    fn guardar(&self, secreto_id: &str, contrasena: &str) -> Result<(), ErrorBoveda> {
        self.secretos.lock().unwrap().insert(secreto_id.to_string(), contrasena.to_string());
        Ok(())
    }
}

impl Perfiles for Memoria {
    fn guardar_secreto_id(&self, id: &str, secreto_id: &str) -> Result<(), String> {
        self.secretos_id.lock().unwrap().insert(id.to_string(), secreto_id.to_string());
        Ok(())
    }

    fn guardar_huella(&self, id: &str, huella: &str) -> Result<(), String> {
        self.huellas.lock().unwrap().insert(id.to_string(), huella.to_string());
        Ok(())
    }
}

struct Estado {
    autenticos: AtomicUsize,
    conexiones: AtomicUsize,
    recibidos: Mutex<Vec<u8>>,
}

#[derive(Clone)]
struct Demo {
    usuario: String,
    contrasena: String,
    tmux: bool,
    estado: Arc<Estado>,
}

impl Handler for Demo {
    type Error = russh::Error;

    async fn auth_password(&mut self, user: &str, password: &str) -> Result<Auth, Self::Error> {
        self.estado.autenticos.fetch_add(1, Ordering::SeqCst);
        if user == self.usuario && password == self.contrasena {
            Ok(Auth::Accept)
        } else {
            Ok(Auth::reject())
        }
    }

    async fn channel_open_session(
        &mut self,
        _channel: russh::Channel<Msg>,
        reply: russh::server::ChannelOpenHandle,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        reply.accept().await;
        Ok(())
    }

    async fn pty_request(
        &mut self,
        channel: ChannelId,
        _term: &str,
        _col_width: u32,
        _row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _modes: &[(russh::Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let _ = session.channel_success(channel);
        Ok(())
    }

    async fn exec_request(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let comando = String::from_utf8_lossy(data);
        let _ = session.channel_success(channel);
        if comando.contains("command -v tmux") {
            let codigo = if self.tmux { 0 } else { 1 };
            let _ = session.exit_status_request(channel, codigo);
            let _ = session.close(channel);
        } else if self.tmux {
            let _ = session.data(channel, b"sesion-sshuttle\n".to_vec());
        }
        Ok(())
    }

    async fn shell_request(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let _ = session.channel_success(channel);
        let _ = session.data(channel, b"shell\n".to_vec());
        Ok(())
    }

    async fn data(
        &mut self,
        channel: ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.estado.recibidos.lock().unwrap().extend_from_slice(data);
        let _ = session.data(channel, data.to_vec());
        Ok(())
    }
}

struct ServidorPrueba {
    puerto: u16,
    vivo: Arc<AtomicBool>,
    lazo: JoinHandle<()>,
    sesiones: Arc<Mutex<Vec<JoinHandle<()>>>>,
    handles: Arc<Mutex<Vec<russh::server::Handle>>>,
    estado: Arc<Estado>,
}

impl Drop for ServidorPrueba {
    fn drop(&mut self) {
        self.vivo.store(false, Ordering::SeqCst);
        self.lazo.abort();
        for tarea in self.sesiones.lock().unwrap().drain(..) {
            tarea.abort();
        }
    }
}

impl ServidorPrueba {
    async fn arrancar(usuario: &str, contrasena: &str, tmux: bool) -> Self {
        let clave =
            PrivateKey::random(&mut rand::rng(), russh::keys::Algorithm::Ed25519).expect("clave");
        Self::con_clave(0, usuario, contrasena, tmux, clave).await
    }

    async fn con_clave(
        puerto: u16,
        usuario: &str,
        contrasena: &str,
        tmux: bool,
        clave: PrivateKey,
    ) -> Self {
        let mut config = russh::server::Config::default();
        config.keys = vec![clave];
        config.auth_rejection_time = Duration::from_millis(1);
        config.auth_rejection_time_initial = Some(Duration::ZERO);
        config.inactivity_timeout = None;
        let config = Arc::new(config);
        let listener = TcpListener::bind(("127.0.0.1", puerto)).await.expect("puerto de prueba");
        let puerto = listener.local_addr().unwrap().port();
        let estado = Arc::new(Estado {
            autenticos: AtomicUsize::new(0),
            conexiones: AtomicUsize::new(0),
            recibidos: Mutex::new(Vec::new()),
        });
        let vivo = Arc::new(AtomicBool::new(true));
        let sesiones = Arc::new(Mutex::new(Vec::new()));
        let handles = Arc::new(Mutex::new(Vec::new()));
        let demo = Demo {
            usuario: usuario.to_string(),
            contrasena: contrasena.to_string(),
            tmux,
            estado: Arc::clone(&estado),
        };
        let vivo_lazo = Arc::clone(&vivo);
        let sesiones_lazo = Arc::clone(&sesiones);
        let handles_lazo = Arc::clone(&handles);
        let estado_lazo = Arc::clone(&estado);
        let lazo = tokio::spawn(async move {
            loop {
                if !vivo_lazo.load(Ordering::SeqCst) {
                    break;
                }
                let aceptado =
                    tokio::time::timeout(Duration::from_millis(80), listener.accept()).await;
                let Ok(Ok((stream, _))) = aceptado else { continue };
                estado_lazo.conexiones.fetch_add(1, Ordering::SeqCst);
                let config = Arc::clone(&config);
                let demo = demo.clone();
                let handles = Arc::clone(&handles_lazo);
                let tarea = tokio::spawn(async move {
                    if let Ok(sesion) = russh::server::run_stream(config, stream, demo).await {
                        handles.lock().unwrap().push(sesion.handle());
                        let _ = sesion.await;
                    }
                });
                sesiones_lazo.lock().unwrap().push(tarea);
            }
        });
        Self { puerto, vivo, lazo, sesiones, handles, estado }
    }

    async fn apagar(&mut self) {
        self.vivo.store(false, Ordering::SeqCst);
        let handles: Vec<_> = self.handles.lock().unwrap().drain(..).collect();
        for handle in handles {
            let _ = handle
                .disconnect(russh::Disconnect::ByApplication, "corte".into(), String::new())
                .await;
        }
        if tokio::time::timeout(Duration::from_millis(800), &mut self.lazo).await.is_err() {
            self.lazo.abort();
        }
        for tarea in self.sesiones.lock().unwrap().drain(..) {
            tarea.abort();
        }
        tokio::time::sleep(Duration::from_millis(30)).await;
    }
}

fn memoria() -> Arc<Memoria> {
    Arc::new(Memoria {
        secretos: Mutex::new(HashMap::new()),
        huellas: Mutex::new(HashMap::new()),
        secretos_id: Mutex::new(HashMap::new()),
    })
}

fn perfil(puerto: u16, secreto_id: &str, huella: &str) -> PerfilConexion {
    PerfilConexion {
        id: "srv".into(),
        host: "127.0.0.1".into(),
        puerto,
        usuario: "ana".into(),
        secreto_id: secreto_id.to_string(),
        huella_host: huella.to_string(),
    }
}

fn contiene(datos: &[u8], aguja: &[u8]) -> bool {
    datos.windows(aguja.len()).any(|ventana| ventana == aguja)
}

async fn acumular(
    rx: &mut UnboundedReceiver<Evento>,
    listo: impl Fn(&[Evento]) -> bool,
    limite: Duration,
) -> Vec<Evento> {
    let mut eventos = Vec::new();
    let resultado = tokio::time::timeout(limite, async {
        loop {
            if listo(&eventos) {
                return;
            }
            eventos.push(rx.recv().await.expect("evento"));
        }
    })
    .await;
    if resultado.is_err() {
        panic!("no llegó el evento esperado: {eventos:?}");
    }
    eventos
}

fn es(evento: &Evento, valor: Indicador, motivo: Option<MotivoInactiva>) -> bool {
    matches!(evento, Evento::Indicador { valor: v, motivo: m, .. } if *v == valor && *m == motivo)
}

fn salida(evento: &Evento, aguja: &[u8]) -> bool {
    matches!(evento, Evento::Salida { datos, .. } if contiene(datos, aguja))
}

#[tokio::test]
async fn alta_consola_secreto_y_segunda_conexion_sin_clave() {
    let servidor = ServidorPrueba::arrancar("ana", "correcta", false).await;
    let memoria = memoria();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let supervisor = Supervisor::nuevo(
        Arc::clone(&memoria) as _,
        Arc::clone(&memoria) as _,
        tx,
        tokio::runtime::Handle::current(),
    );
    supervisor.conectar(perfil(servidor.puerto, "", ""), Some("correcta".into()));

    let _ = acumular(
        &mut rx,
        |ev| {
            ev.iter().any(|e| es(e, Indicador::Activa, None))
                && ev.iter().any(|e| salida(e, b"shell"))
        },
        Duration::from_secs(8),
    )
    .await;
    assert!(memoria.secretos.lock().unwrap().values().any(|valor| valor == "correcta"));
    assert!(memoria.secretos_id.lock().unwrap().get("srv").is_some_and(|id| !id.is_empty()));
    assert!(memoria
        .huellas
        .lock()
        .unwrap()
        .get("srv")
        .is_some_and(|huella| huella.starts_with("SHA256:")));

    supervisor.escribir("srv", b"hola\n".to_vec());
    let _ =
        acumular(&mut rx, |ev| ev.iter().any(|e| salida(e, b"hola")), Duration::from_secs(4)).await;

    let secreto_id = memoria.secretos_id.lock().unwrap().get("srv").unwrap().clone();
    let huella = memoria.huellas.lock().unwrap().get("srv").unwrap().clone();
    supervisor.cerrar("srv");
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Inactiva, Some(MotivoInactiva::Cerrada))),
        Duration::from_secs(4),
    )
    .await;

    supervisor.conectar(perfil(servidor.puerto, &secreto_id, &huella), None);
    let segunda = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Activa, None)),
        Duration::from_secs(8),
    )
    .await;
    assert!(!segunda.iter().any(|e| matches!(e, Evento::TrabajoInterrumpido { .. })));
}

#[tokio::test]
async fn la_clave_rechazada_no_se_guarda_ni_reintenta() {
    let servidor = ServidorPrueba::arrancar("ana", "correcta", false).await;
    let memoria = memoria();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let supervisor = Supervisor::nuevo(
        Arc::clone(&memoria) as _,
        Arc::clone(&memoria) as _,
        tx,
        tokio::runtime::Handle::current(),
    );
    supervisor.conectar(perfil(servidor.puerto, "", ""), Some("mala".into()));
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Inactiva, Some(MotivoInactiva::ClaveRechazada))),
        Duration::from_secs(8),
    )
    .await;
    tokio::time::sleep(Duration::from_millis(1500)).await;
    assert_eq!(servidor.estado.autenticos.load(Ordering::SeqCst), 1);
    assert!(memoria.secretos.lock().unwrap().is_empty());
    assert!(memoria.huellas.lock().unwrap().is_empty());
    assert!(rx.try_recv().is_err());
}

#[tokio::test]
async fn sin_secreto_queda_inactiva_y_no_llama_al_servidor() {
    let servidor = ServidorPrueba::arrancar("ana", "correcta", false).await;
    let memoria = memoria();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let supervisor = Supervisor::nuevo(
        Arc::clone(&memoria) as _,
        Arc::clone(&memoria) as _,
        tx,
        tokio::runtime::Handle::current(),
    );
    supervisor.conectar(perfil(servidor.puerto, "id-ausente", ""), None);
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Inactiva, Some(MotivoInactiva::FaltaSecreto))),
        Duration::from_secs(3),
    )
    .await;
    tokio::time::sleep(Duration::from_millis(400)).await;
    assert_eq!(servidor.estado.conexiones.load(Ordering::SeqCst), 0);
    assert!(rx.try_recv().is_err());
}

#[tokio::test]
async fn cerrar_y_esperar_detiene_la_sesion() {
    let servidor = ServidorPrueba::arrancar("ana", "correcta", false).await;
    let memoria = memoria();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let supervisor = Supervisor::nuevo(
        Arc::clone(&memoria) as _,
        Arc::clone(&memoria) as _,
        tx,
        tokio::runtime::Handle::current(),
    );
    supervisor.conectar(perfil(servidor.puerto, "", ""), Some("correcta".into()));
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Activa, None)),
        Duration::from_secs(8),
    )
    .await;
    let antes = servidor.estado.conexiones.load(Ordering::SeqCst);
    supervisor.cerrar_y_esperar("srv").await;
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Inactiva, Some(MotivoInactiva::Cerrada))),
        Duration::from_secs(4),
    )
    .await;
    tokio::time::sleep(Duration::from_millis(400)).await;
    assert_eq!(servidor.estado.conexiones.load(Ordering::SeqCst), antes);
    supervisor.cerrar_y_esperar("srv").await;
}

#[tokio::test]
async fn el_corte_reintenta_sin_reenviar_y_cerrar_detiene() {
    let clave =
        PrivateKey::random(&mut rand::rng(), russh::keys::Algorithm::Ed25519).expect("clave");
    let mut servidor = ServidorPrueba::con_clave(0, "ana", "correcta", false, clave.clone()).await;
    let puerto = servidor.puerto;
    let memoria = memoria();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let supervisor = Supervisor::nuevo(
        Arc::clone(&memoria) as _,
        Arc::clone(&memoria) as _,
        tx,
        tokio::runtime::Handle::current(),
    );
    supervisor.conectar(perfil(puerto, "", ""), Some("correcta".into()));
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Activa, None)),
        Duration::from_secs(8),
    )
    .await;
    supervisor.escribir("srv", b"hola\n".to_vec());
    let _ =
        acumular(&mut rx, |ev| ev.iter().any(|e| salida(e, b"hola")), Duration::from_secs(4)).await;

    servidor.apagar().await;
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Reintentando, None)),
        Duration::from_secs(5),
    )
    .await;

    let mut restaurado =
        ServidorPrueba::con_clave(puerto, "ana", "correcta", false, clave.clone()).await;
    let vuelta = acumular(
        &mut rx,
        |ev| {
            ev.iter().any(|e| es(e, Indicador::Activa, None))
                && ev.iter().any(|e| matches!(e, Evento::TrabajoInterrumpido { .. }))
        },
        Duration::from_secs(10),
    )
    .await;
    assert_eq!(
        vuelta.iter().filter(|e| matches!(e, Evento::TrabajoInterrumpido { .. })).count(),
        1
    );
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert!(restaurado.estado.recibidos.lock().unwrap().is_empty());

    restaurado.apagar().await;
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Reintentando, None)),
        Duration::from_secs(5),
    )
    .await;
    supervisor.cerrar("srv");
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Inactiva, Some(MotivoInactiva::Cerrada))),
        Duration::from_secs(4),
    )
    .await;
    let vigilante = ServidorPrueba::con_clave(puerto, "ana", "correcta", false, clave).await;
    tokio::time::sleep(Duration::from_millis(1600)).await;
    assert_eq!(vigilante.estado.conexiones.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn una_huella_distinta_espera_confirmacion() {
    let clave_a =
        PrivateKey::random(&mut rand::rng(), russh::keys::Algorithm::Ed25519).expect("clave");
    let clave_b =
        PrivateKey::random(&mut rand::rng(), russh::keys::Algorithm::Ed25519).expect("clave");
    let mut servidor = ServidorPrueba::con_clave(0, "ana", "correcta", false, clave_a).await;
    let puerto = servidor.puerto;
    let memoria = memoria();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let supervisor = Supervisor::nuevo(
        Arc::clone(&memoria) as _,
        Arc::clone(&memoria) as _,
        tx,
        tokio::runtime::Handle::current(),
    );
    supervisor.conectar(perfil(puerto, "", ""), Some("correcta".into()));
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Activa, None)),
        Duration::from_secs(8),
    )
    .await;
    let huella_vieja = memoria.huellas.lock().unwrap().get("srv").unwrap().clone();
    servidor.apagar().await;
    let _servidor_b = ServidorPrueba::con_clave(puerto, "ana", "correcta", false, clave_b).await;
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Inactiva, Some(MotivoInactiva::HuellaDistinta))),
        Duration::from_secs(8),
    )
    .await;
    tokio::time::sleep(Duration::from_millis(400)).await;
    assert_eq!(memoria.huellas.lock().unwrap().get("srv").unwrap(), &huella_vieja);

    supervisor.confirmar_huella("srv");
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Activa, None)),
        Duration::from_secs(8),
    )
    .await;
    let huella_nueva = memoria.huellas.lock().unwrap().get("srv").unwrap().clone();
    assert_ne!(huella_nueva, huella_vieja);
    assert!(huella_nueva.starts_with("SHA256:"));
}

#[tokio::test]
async fn con_tmux_no_avisa_trabajo_interrumpido() {
    let clave =
        PrivateKey::random(&mut rand::rng(), russh::keys::Algorithm::Ed25519).expect("clave");
    let mut servidor = ServidorPrueba::con_clave(0, "ana", "correcta", true, clave.clone()).await;
    let puerto = servidor.puerto;
    let memoria = memoria();
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let supervisor = Supervisor::nuevo(
        Arc::clone(&memoria) as _,
        Arc::clone(&memoria) as _,
        tx,
        tokio::runtime::Handle::current(),
    );
    supervisor.conectar(perfil(puerto, "", ""), Some("correcta".into()));
    let primera = acumular(
        &mut rx,
        |ev| {
            ev.iter().any(|e| es(e, Indicador::Activa, None))
                && ev.iter().any(|e| salida(e, b"sesion-sshuttle"))
        },
        Duration::from_secs(8),
    )
    .await;
    assert!(!primera.iter().any(|e| matches!(e, Evento::TrabajoInterrumpido { .. })));

    servidor.apagar().await;
    let _ = acumular(
        &mut rx,
        |ev| ev.iter().any(|e| es(e, Indicador::Reintentando, None)),
        Duration::from_secs(5),
    )
    .await;
    let _restaurado = ServidorPrueba::con_clave(puerto, "ana", "correcta", true, clave).await;
    let vuelta = acumular(
        &mut rx,
        |ev| {
            ev.iter().any(|e| es(e, Indicador::Activa, None))
                && ev.iter().any(|e| salida(e, b"sesion-sshuttle"))
        },
        Duration::from_secs(10),
    )
    .await;
    assert!(!vuelta.iter().any(|e| matches!(e, Evento::TrabajoInterrumpido { .. })));
}
