use sshuttle_vault::{borrar, Secreto, SERVICIO};

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

#[test]
fn borrar_un_id_vacio_no_toca_el_llavero() {
    assert!(borrar("").is_ok());
}
