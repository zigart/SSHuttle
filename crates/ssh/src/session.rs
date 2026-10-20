use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use russh::client::{self, Handler};
use russh::keys::PublicKeyOrCertificate;
use russh::{Disconnect, Error};
use sshuttle_terminal::Consola;
use zeroize::Zeroize;

use crate::hostkey;

/// Único lugar que abre un socket SSH.
pub struct SessionHandle {
    handle: client::Handle<Cliente>,
    consola: Consola,
    huella: String,
}

pub struct Pedido {
    pub host: String,
    pub puerto: u16,
    pub usuario: String,
    pub contrasena: String,
    pub huella_guardada: String,
    pub columnas: u32,
    pub filas: u32,
    pub tras_corte: bool,
}

impl Drop for Pedido {
    fn drop(&mut self) {
        self.contrasena.zeroize();
    }
}

impl std::fmt::Debug for Pedido {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pedido")
            .field("host", &self.host)
            .field("puerto", &self.puerto)
            .field("usuario", &self.usuario)
            .field("contrasena", &"[redactado]")
            .field("huella_guardada", &self.huella_guardada)
            .finish()
    }
}

#[derive(Debug)]
pub enum ErrorSesion {
    Red,
    ClaveRechazada,
    HuellaDistinta { presentada: String },
}

struct Cliente {
    huella_guardada: String,
    presentada: Arc<std::sync::Mutex<Option<String>>>,
    distinta: Arc<AtomicBool>,
}

impl Handler for Cliente {
    type Error = Error;

    async fn check_server_key(
        &mut self,
        clave: &PublicKeyOrCertificate,
    ) -> Result<bool, Self::Error> {
        let huella = hostkey::huella_sha256(&clave.public_key());
        if let Ok(mut guardada) = self.presentada.lock() {
            *guardada = Some(huella.clone());
        }
        if hostkey::es_distinta(&self.huella_guardada, &huella) {
            self.distinta.store(true, Ordering::SeqCst);
            tracing::warn!("la huella del host no coincide");
            return Ok(false);
        }
        Ok(true)
    }
}

impl SessionHandle {
    pub async fn abrir(pedido: Pedido) -> Result<Self, ErrorSesion> {
        let config = Arc::new(client::Config {
            keepalive_interval: Some(Duration::from_secs(15)),
            keepalive_max: 2,
            nodelay: true,
            ..client::Config::default()
        });
        let presentada = Arc::new(std::sync::Mutex::new(None));
        let distinta = Arc::new(AtomicBool::new(false));
        let cliente = Cliente {
            huella_guardada: pedido.huella_guardada.clone(),
            presentada: Arc::clone(&presentada),
            distinta: Arc::clone(&distinta),
        };
        let destino = (pedido.host.as_str(), pedido.puerto);
        tracing::info!(host = %pedido.host, puerto = pedido.puerto, usuario = %pedido.usuario, "abriendo sesion");
        let mut handle = match client::connect(config, destino, cliente).await {
            Ok(handle) => handle,
            Err(error) => {
                if distinta.load(Ordering::SeqCst) || matches!(error, Error::UnknownKey) {
                    let huella = presentada.lock().ok().and_then(|g| g.clone()).unwrap_or_default();
                    return Err(ErrorSesion::HuellaDistinta { presentada: huella });
                }
                tracing::warn!("no se pudo alcanzar el servidor");
                return Err(ErrorSesion::Red);
            }
        };

        let autenticado =
            handle.authenticate_password(pedido.usuario.clone(), pedido.contrasena.clone()).await;
        match autenticado {
            Ok(resultado) if resultado.success() => {
                tracing::info!("autenticacion aceptada");
            }
            Ok(_) => {
                tracing::info!("autenticacion rechazada");
                let _ = handle.disconnect(Disconnect::ByApplication, "", "").await;
                return Err(ErrorSesion::ClaveRechazada);
            }
            Err(_) => {
                tracing::warn!("la sesion se corto durante la autenticacion");
                return Err(ErrorSesion::Red);
            }
        }

        let huella = presentada.lock().ok().and_then(|g| g.clone()).unwrap_or_default();
        let consola = Consola::abrir(&handle, pedido.columnas, pedido.filas, pedido.tras_corte)
            .await
            .map_err(|_| {
                tracing::warn!("no se pudo abrir el pty");
                ErrorSesion::Red
            })?;
        Ok(Self { handle, consola, huella })
    }

    pub fn huella(&self) -> &str {
        &self.huella
    }

    pub fn tomar_aviso(&mut self) -> bool {
        self.consola.tomar_aviso()
    }

    pub async fn leer(&mut self) -> sshuttle_terminal::Lectura {
        self.consola.leer().await
    }

    pub async fn escribir(&self, datos: &[u8]) -> Result<(), ()> {
        self.consola.escribir(datos).await.map_err(|_| ())
    }

    pub async fn redimensionar(&self, columnas: u32, filas: u32) -> Result<(), ()> {
        self.consola.redimensionar(columnas, filas).await.map_err(|_| ())
    }

    pub fn red_caida(&self) -> bool {
        self.handle.is_closed()
    }

    pub async fn cerrar(&self) {
        let _ = self.handle.disconnect(Disconnect::ByApplication, "cerrada", "").await;
    }
}
