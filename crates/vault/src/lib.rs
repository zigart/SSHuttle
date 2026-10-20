use std::fmt;

use zeroize::Zeroize;

pub const SERVICIO: &str = "sshuttle";

#[derive(thiserror::Error, Debug)]
pub enum ErrorLlave {
    #[error("el secreto no esta en el llavero")]
    Ausente,
    #[error("el llavero no respondio")]
    Llavero,
}

/// Contraseña que no se puede imprimir. El texto real solo sale hacia el llavero o hacia SSH.
pub struct Secreto(String);

impl Secreto {
    pub fn nuevo(valor: String) -> Self {
        Self(valor)
    }

    pub fn exponer(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secreto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[redactado]")
    }
}

impl fmt::Display for Secreto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[redactado]")
    }
}

impl Drop for Secreto {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

pub fn guardar(secreto_id: &str, contrasena: &str) -> Result<(), ErrorLlave> {
    let entrada = keyring::Entry::new(SERVICIO, secreto_id).map_err(|_| ErrorLlave::Llavero)?;
    entrada.set_password(contrasena).map_err(|_| ErrorLlave::Llavero)?;
    tracing::info!(cuenta = secreto_id, "secreto guardado en el llavero");
    Ok(())
}

pub fn leer(secreto_id: &str) -> Result<Secreto, ErrorLlave> {
    let entrada = keyring::Entry::new(SERVICIO, secreto_id).map_err(|_| ErrorLlave::Llavero)?;
    match entrada.get_password() {
        Ok(valor) => Ok(Secreto::nuevo(valor)),
        Err(error) => {
            if es_ausente(&error) {
                tracing::info!(cuenta = secreto_id, "el llavero no tiene el secreto");
                Err(ErrorLlave::Ausente)
            } else {
                tracing::warn!(cuenta = secreto_id, "el llavero no respondio");
                Err(ErrorLlave::Llavero)
            }
        }
    }
}

fn es_ausente(error: &keyring::Error) -> bool {
    matches!(error, keyring::Error::NoEntry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn el_secreto_no_aparece_en_logs() {
        let secreto = Secreto::nuevo("clave-que-no-se-imprime".into());
        let depurado = format!("{secreto:?} {secreto}");
        assert!(!depurado.contains("clave-que-no-se-imprime"));
        assert!(depurado.contains("[redactado]"));
        assert_eq!(secreto.exponer(), "clave-que-no-se-imprime");
    }

    #[test]
    fn el_servicio_del_llavero_es_sshuttle() {
        assert_eq!(SERVICIO, "sshuttle");
    }
}
