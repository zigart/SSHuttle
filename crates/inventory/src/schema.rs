use serde::Serialize;

/// Perfil guardado. No tiene campo de contraseña: el secreto vive en el llavero.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Servidor {
    pub id: String,
    pub nombre_visible: String,
    pub host: String,
    pub puerto: u16,
    pub usuario: String,
    pub secreto_id: String,
    pub huella_host: String,
}

/// Datos que el operador escribe al dar de alta o editar. El puerto vacío queda en 22.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alta {
    pub id: Option<String>,
    pub host: String,
    pub puerto: Option<u16>,
    pub usuario: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerfilValidado {
    pub id: String,
    pub nombre_visible: String,
    pub host: String,
    pub puerto: u16,
    pub usuario: String,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ErrorPerfil {
    #[error("el host es obligatorio")]
    HostVacio,
    #[error("el usuario es obligatorio")]
    UsuarioVacio,
    #[error("el puerto tiene que estar entre 1 y 65535")]
    PuertoInvalido,
}

pub fn validar(alta: Alta) -> Result<PerfilValidado, ErrorPerfil> {
    let host = alta.host.trim().to_string();
    if host.is_empty() {
        return Err(ErrorPerfil::HostVacio);
    }
    let usuario = alta.usuario.trim().to_string();
    if usuario.is_empty() {
        return Err(ErrorPerfil::UsuarioVacio);
    }
    let puerto = match alta.puerto {
        None => 22,
        Some(0) => return Err(ErrorPerfil::PuertoInvalido),
        Some(puerto) => puerto,
    };
    let id = match alta.id {
        Some(id) if !id.trim().is_empty() => id,
        _ => uuid::Uuid::new_v4().to_string(),
    };
    Ok(PerfilValidado { nombre_visible: host.clone(), host, puerto, usuario, id })
}
