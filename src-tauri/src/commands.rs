use std::sync::{Arc, Mutex};

use serde::Serialize;
use sshuttle_inventory::{Alta, Inventario};
use sshuttle_ssh::{
    Boveda, ErrorBoveda, Evento, Indicador, MotivoInactiva, PerfilConexion, Perfiles, Supervisor,
};
use tauri::{AppHandle, Emitter, State};

pub struct AppState {
    pub inventario: Arc<Mutex<Inventario>>,
    pub supervisor: Supervisor,
}

pub struct BovedaSistema;

impl Boveda for BovedaSistema {
    fn leer(&self, secreto_id: &str) -> Result<String, ErrorBoveda> {
        match sshuttle_vault::leer(secreto_id) {
            Ok(secreto) => Ok(secreto.exponer().to_string()),
            Err(sshuttle_vault::ErrorLlave::Ausente) => Err(ErrorBoveda::Ausente),
            Err(sshuttle_vault::ErrorLlave::Llavero) => Err(ErrorBoveda::Llavero),
        }
    }

    fn guardar(&self, secreto_id: &str, contrasena: &str) -> Result<(), ErrorBoveda> {
        sshuttle_vault::guardar(secreto_id, contrasena).map_err(|_| ErrorBoveda::Llavero)
    }
}

pub struct Puente {
    pub inventario: Arc<Mutex<Inventario>>,
}

impl Perfiles for Puente {
    fn guardar_secreto_id(&self, id: &str, secreto_id: &str) -> Result<(), String> {
        self.inventario
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .asignar_secreto(id, secreto_id)
            .map_err(|error| error.to_string())
    }

    fn guardar_huella(&self, id: &str, huella: &str) -> Result<(), String> {
        self.inventario
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .asignar_huella(id, huella)
            .map_err(|error| error.to_string())
    }
}

#[derive(Serialize)]
pub struct ServidorDto {
    pub id: String,
    pub nombre_visible: String,
    pub host: String,
    pub puerto: u16,
    pub usuario: String,
    pub tiene_secreto: bool,
    pub huella_host: Option<String>,
}

fn dto(servidor: sshuttle_inventory::Servidor) -> ServidorDto {
    ServidorDto {
        tiene_secreto: !servidor.secreto_id.is_empty(),
        huella_host: if servidor.huella_host.is_empty() {
            None
        } else {
            Some(servidor.huella_host.clone())
        },
        id: servidor.id,
        nombre_visible: servidor.nombre_visible,
        host: servidor.host,
        puerto: servidor.puerto,
        usuario: servidor.usuario,
    }
}

fn perfil_de(servidor: &sshuttle_inventory::Servidor) -> PerfilConexion {
    PerfilConexion {
        id: servidor.id.clone(),
        host: servidor.host.clone(),
        puerto: servidor.puerto,
        usuario: servidor.usuario.clone(),
        secreto_id: servidor.secreto_id.clone(),
        huella_host: servidor.huella_host.clone(),
    }
}

#[tauri::command]
pub fn listar_servidores(state: State<'_, AppState>) -> Result<Vec<ServidorDto>, String> {
    let inventario = state.inventario.lock().unwrap_or_else(|error| error.into_inner());
    inventario
        .listar()
        .map(|lista| lista.into_iter().map(dto).collect())
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn guardar_servidor(
    state: State<'_, AppState>,
    id: Option<String>,
    host: String,
    puerto: Option<u16>,
    usuario: String,
) -> Result<ServidorDto, String> {
    let inventario = state.inventario.lock().unwrap_or_else(|error| error.into_inner());
    inventario
        .guardar(Alta { id, host, puerto, usuario })
        .map(dto)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn conectar(
    state: State<'_, AppState>,
    id: String,
    contrasena: Option<String>,
) -> Result<(), String> {
    let perfil = {
        let inventario = state.inventario.lock().unwrap_or_else(|error| error.into_inner());
        let servidor = inventario.obtener(&id).map_err(|error| error.to_string())?;
        perfil_de(&servidor)
    };
    state.supervisor.conectar(perfil, contrasena);
    Ok(())
}

#[tauri::command]
pub fn cerrar(state: State<'_, AppState>, id: String) {
    state.supervisor.cerrar(&id);
}

#[tauri::command]
pub async fn eliminar_servidor(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let secreto_id = {
        let inventario = state.inventario.lock().unwrap_or_else(|error| error.into_inner());
        match inventario.obtener(&id) {
            Ok(servidor) => servidor.secreto_id,
            Err(sshuttle_inventory::ErrorInventario::NoEncontrado) => return Ok(()),
            Err(error) => return Err(error.to_string()),
        }
    };

    state.supervisor.cerrar_y_esperar(&id).await;

    if !secreto_id.is_empty() {
        let cuenta = secreto_id;
        let borrado = tokio::task::spawn_blocking(move || sshuttle_vault::borrar(&cuenta))
            .await
            .map_err(|_| "No se pudo eliminar la conexión.".to_string())?;
        if borrado.is_err() {
            return Err("No se pudo eliminar la conexión.".to_string());
        }
    }

    let inventario = state.inventario.lock().unwrap_or_else(|error| error.into_inner());
    inventario.eliminar(&id).map_err(|_| "No se pudo eliminar la conexión.".to_string())
}

#[tauri::command]
pub fn confirmar_huella(state: State<'_, AppState>, id: String) {
    state.supervisor.confirmar_huella(&id);
}

#[tauri::command]
pub fn escribir_consola(state: State<'_, AppState>, id: String, datos: Vec<u8>) {
    state.supervisor.escribir(&id, datos);
}

#[tauri::command]
pub fn redimensionar_consola(state: State<'_, AppState>, id: String, columnas: u32, filas: u32) {
    state.supervisor.redimensionar(&id, columnas, filas);
}

#[derive(Clone, Serialize)]
struct IndicadorDto {
    id: String,
    valor: Indicador,
    #[serde(skip_serializing_if = "Option::is_none")]
    motivo: Option<MotivoInactiva>,
}

#[derive(Clone, Serialize)]
struct SalidaDto {
    id: String,
    datos: Vec<u8>,
}

#[derive(Clone, Serialize)]
struct InterrupcionDto {
    id: String,
}

pub fn publicar(app: &AppHandle, evento: Evento) {
    let resultado = match evento {
        Evento::Indicador { id, valor, motivo } => {
            app.emit("indicador", IndicadorDto { id, valor, motivo })
        }
        Evento::Salida { id, datos } => app.emit("salida_consola", SalidaDto { id, datos }),
        Evento::TrabajoInterrumpido { id } => {
            app.emit("trabajo_interrumpido", InterrupcionDto { id })
        }
    };
    if resultado.is_err() {
        tracing::warn!("no se pudo publicar un evento de la consola");
    }
}
