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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_primera_huella_se_acepta_y_un_cambio_no() {
        assert!(coincide("", "SHA256:aaa"));
        assert!(!es_distinta("", "SHA256:aaa"));
        assert!(coincide("SHA256:aaa", "SHA256:aaa"));
        assert!(es_distinta("SHA256:aaa", "SHA256:bbb"));
        assert!(!coincide("SHA256:aaa", "SHA256:bbb"));
    }
}
