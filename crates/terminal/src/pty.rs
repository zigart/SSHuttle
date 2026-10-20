use std::collections::VecDeque;
use std::time::Duration;

use russh::client::Handler;
use russh::ChannelMsg;

use crate::tmux;

#[derive(Debug, thiserror::Error)]
pub enum ErrorPty {
    #[error("no se pudo abrir la consola")]
    Canal,
}

pub enum Lectura {
    Datos(Vec<u8>),
    Ignorar,
    Fin,
}

/// Consola sobre un canal ya abierto. No abre sockets y no guarda lo que el operador escribió.
pub struct Consola {
    lectura: russh::ChannelReadHalf,
    escritura: russh::ChannelWriteHalf<russh::client::Msg>,
    pendiente: VecDeque<Vec<u8>>,
    con_tmux: bool,
    avisar_interrumpido: bool,
    aviso_entregado: bool,
}

impl Consola {
    pub async fn abrir<H: Handler>(
        handle: &russh::client::Handle<H>,
        columnas: u32,
        filas: u32,
        tras_corte: bool,
    ) -> Result<Self, ErrorPty> {
        let con_tmux = hay_tmux(handle).await?;
        let canal = handle.channel_open_session().await.map_err(|_| ErrorPty::Canal)?;
        canal
            .request_pty(false, "xterm-256color", columnas, filas, 0, 0, &[])
            .await
            .map_err(|_| ErrorPty::Canal)?;
        // Sin tmux se pide el shell del usuario. Un `exec` con sintaxis de bash lo corre
        // `sshd` mediante `$SHELL -c`, y fish cierra el canal enseguida.
        if con_tmux {
            canal.exec(false, tmux::SESION.as_bytes()).await.map_err(|_| ErrorPty::Canal)?;
        } else {
            canal.request_shell(false).await.map_err(|_| ErrorPty::Canal)?;
        }
        let (lectura, escritura) = canal.split();
        Ok(Self {
            lectura,
            escritura,
            pendiente: VecDeque::new(),
            con_tmux,
            avisar_interrumpido: trabajo_interrumpido(tras_corte, con_tmux),
            aviso_entregado: false,
        })
    }

    pub fn con_tmux(&self) -> bool {
        self.con_tmux
    }

    /// Una sola vez, y solo si esta consola es la vuelta de un corte sin tmux.
    pub fn tomar_aviso(&mut self) -> bool {
        if self.avisar_interrumpido && !self.aviso_entregado {
            self.aviso_entregado = true;
            true
        } else {
            false
        }
    }

    pub async fn leer(&mut self) -> Lectura {
        if let Some(datos) = self.pendiente.pop_front() {
            return Lectura::Datos(datos);
        }
        match self.lectura.wait().await {
            Some(ChannelMsg::Data { data }) => Lectura::Datos(data.to_vec()),
            Some(ChannelMsg::ExtendedData { data, .. }) => Lectura::Datos(data.to_vec()),
            Some(ChannelMsg::Eof)
            | Some(ChannelMsg::Close)
            | Some(ChannelMsg::ExitStatus { .. })
            | None => Lectura::Fin,
            Some(_) => Lectura::Ignorar,
        }
    }

    pub async fn escribir(&self, datos: &[u8]) -> Result<(), ErrorPty> {
        let actuales = bytes_actuales(datos);
        if actuales.is_empty() {
            return Ok(());
        }
        self.escritura.data_bytes(actuales.to_vec()).await.map_err(|_| ErrorPty::Canal)
    }

    pub async fn redimensionar(&self, columnas: u32, filas: u32) -> Result<(), ErrorPty> {
        self.escritura.window_change(columnas, filas, 0, 0).await.map_err(|_| ErrorPty::Canal)
    }
}

pub fn trabajo_interrumpido(tras_corte: bool, con_tmux: bool) -> bool {
    tras_corte && !con_tmux
}

/// Devuelve solo los bytes de esta escritura. No hay historial para reenviar.
pub fn bytes_actuales(datos: &[u8]) -> &[u8] {
    datos
}

async fn hay_tmux<H: Handler>(handle: &russh::client::Handle<H>) -> Result<bool, ErrorPty> {
    let mut canal = handle.channel_open_session().await.map_err(|_| ErrorPty::Canal)?;
    canal.exec(true, tmux::SONDA.as_bytes()).await.map_err(|_| ErrorPty::Canal)?;
    let sonda = async {
        loop {
            match canal.wait().await {
                Some(ChannelMsg::ExitStatus { exit_status }) => return exit_status == 0,
                Some(ChannelMsg::Close) | None => return false,
                Some(_) => {}
            }
        }
    };
    match tokio::time::timeout(Duration::from_secs(8), sonda).await {
        Ok(encontrado) => Ok(encontrado),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_entrada_no_se_acumula() {
        let primera = bytes_actuales(b"echo a\n");
        let segunda = bytes_actuales(b"echo b\n");
        assert_eq!(segunda, b"echo b\n");
        assert_ne!(primera, segunda);
        assert!(trabajo_interrumpido(true, false));
        assert!(!trabajo_interrumpido(true, true));
        assert!(!trabajo_interrumpido(false, false));
    }
}
