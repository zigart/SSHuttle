mod schema;

use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension};
pub use schema::{validar, Alta, ErrorPerfil, PerfilValidado, Servidor};

#[derive(Debug, thiserror::Error)]
pub enum ErrorInventario {
    #[error(transparent)]
    Perfil(#[from] ErrorPerfil),
    #[error("no se pudo guardar el perfil")]
    Base,
    #[error("el servidor no existe")]
    NoEncontrado,
}

pub struct Inventario {
    conexion: Connection,
}

impl Inventario {
    pub fn abrir(ruta: &Path) -> Result<Self, ErrorInventario> {
        if let Some(dir) = ruta.parent() {
            std::fs::create_dir_all(dir).map_err(|_| ErrorInventario::Base)?;
        }
        let conexion = Connection::open(ruta).map_err(|_| ErrorInventario::Base)?;
        conexion
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS servidores (
                    id TEXT PRIMARY KEY,
                    nombre_visible TEXT NOT NULL,
                    host TEXT NOT NULL,
                    puerto INTEGER NOT NULL,
                    usuario TEXT NOT NULL,
                    secreto_id TEXT NOT NULL DEFAULT '',
                    huella_host TEXT NOT NULL DEFAULT ''
                );",
            )
            .map_err(|_| ErrorInventario::Base)?;
        Ok(Self { conexion })
    }

    pub fn guardar(&self, alta: Alta) -> Result<Servidor, ErrorInventario> {
        let perfil = validar(alta)?;
        self.conexion
            .execute(
                "INSERT INTO servidores (id, nombre_visible, host, puerto, usuario, secreto_id, huella_host)
                 VALUES (?1, ?2, ?3, ?4, ?5, '', '')
                 ON CONFLICT(id) DO UPDATE SET
                    nombre_visible = excluded.nombre_visible,
                    host = excluded.host,
                    puerto = excluded.puerto,
                    usuario = excluded.usuario",
                params![perfil.id, perfil.nombre_visible, perfil.host, perfil.puerto, perfil.usuario],
            )
            .map_err(|_| ErrorInventario::Base)?;
        self.obtener(&perfil.id)
    }

    pub fn listar(&self) -> Result<Vec<Servidor>, ErrorInventario> {
        let mut consulta = self
            .conexion
            .prepare(
                "SELECT id, nombre_visible, host, puerto, usuario, secreto_id, huella_host
                 FROM servidores
                 ORDER BY nombre_visible",
            )
            .map_err(|_| ErrorInventario::Base)?;
        let filas = consulta.query_map([], fila_servidor).map_err(|_| ErrorInventario::Base)?;
        filas.collect::<Result<Vec<_>, _>>().map_err(|_| ErrorInventario::Base)
    }

    pub fn obtener(&self, id: &str) -> Result<Servidor, ErrorInventario> {
        self.conexion
            .query_row(
                "SELECT id, nombre_visible, host, puerto, usuario, secreto_id, huella_host
                 FROM servidores WHERE id = ?1",
                [id],
                fila_servidor,
            )
            .optional()
            .map_err(|_| ErrorInventario::Base)?
            .ok_or(ErrorInventario::NoEncontrado)
    }

    pub fn asignar_secreto(&self, id: &str, secreto_id: &str) -> Result<(), ErrorInventario> {
        let cambiadas = self
            .conexion
            .execute("UPDATE servidores SET secreto_id = ?1 WHERE id = ?2", params![secreto_id, id])
            .map_err(|_| ErrorInventario::Base)?;
        if cambiadas == 0 {
            return Err(ErrorInventario::NoEncontrado);
        }
        Ok(())
    }

    pub fn eliminar(&self, id: &str) -> Result<(), ErrorInventario> {
        self.conexion
            .execute("DELETE FROM servidores WHERE id = ?1", [id])
            .map_err(|_| ErrorInventario::Base)?;
        Ok(())
    }

    pub fn asignar_huella(&self, id: &str, huella: &str) -> Result<(), ErrorInventario> {
        let cambiadas = self
            .conexion
            .execute("UPDATE servidores SET huella_host = ?1 WHERE id = ?2", params![huella, id])
            .map_err(|_| ErrorInventario::Base)?;
        if cambiadas == 0 {
            return Err(ErrorInventario::NoEncontrado);
        }
        Ok(())
    }
}

fn fila_servidor(fila: &rusqlite::Row<'_>) -> rusqlite::Result<Servidor> {
    Ok(Servidor {
        id: fila.get(0)?,
        nombre_visible: fila.get(1)?,
        host: fila.get(2)?,
        puerto: fila.get(3)?,
        usuario: fila.get(4)?,
        secreto_id: fila.get(5)?,
        huella_host: fila.get(6)?,
    })
}

#[cfg(test)]
mod lib_pruebas;
