use russh::keys::{HashAlg, PublicKey};

pub fn huella_sha256(clave: &PublicKey) -> String {
    clave.fingerprint(HashAlg::Sha256).to_string()
}

pub fn es_distinta(guardada: &str, presentada: &str) -> bool {
    !guardada.is_empty() && guardada != presentada
}

pub fn coincide(guardada: &str, presentada: &str) -> bool {
    !es_distinta(guardada, presentada)
}
