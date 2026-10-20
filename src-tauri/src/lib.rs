mod commands;

use std::sync::{Arc, Mutex};

use tauri::Manager;
use tracing_subscriber::EnvFilter;

use crate::commands::AppState;

/// Arranca la app. El registro no imprime contraseñas ni el contenido del llavero:
/// esos valores usan un tipo que se muestra como `[redactado]` y las llamadas al
/// llavero solo anotan la cuenta, nunca el secreto.
pub fn run() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(false)
        .try_init();

    tauri::Builder::default()
        .setup(|app| {
            let dir = app.path().app_data_dir().expect("directorio de datos");
            std::fs::create_dir_all(&dir).expect("crear directorio de datos");
            let inventario = sshuttle_inventory::Inventario::abrir(&dir.join("servidores.db"))
                .expect("abrir perfiles");
            let inventario = Arc::new(Mutex::new(inventario));
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            let supervisor = sshuttle_ssh::Supervisor::nuevo(
                Arc::new(commands::BovedaSistema),
                Arc::new(commands::Puente { inventario: Arc::clone(&inventario) }),
                tx,
                tauri::async_runtime::handle().inner().clone(),
            );
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                while let Some(evento) = rx.recv().await {
                    commands::publicar(&handle, evento);
                }
            });
            app.manage(AppState { inventario, supervisor });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::listar_servidores,
            commands::guardar_servidor,
            commands::conectar,
            commands::cerrar,
            commands::eliminar_servidor,
            commands::confirmar_huella,
            commands::escribir_consola,
            commands::redimensionar_consola,
        ])
        .run(tauri::generate_context!())
        .expect("error al ejecutar SSHuttle");
}
