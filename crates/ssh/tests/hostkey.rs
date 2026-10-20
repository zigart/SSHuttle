use sshuttle_ssh::{coincide, es_distinta};

#[test]
fn la_primera_huella_se_acepta_y_un_cambio_no() {
    assert!(coincide("", "SHA256:aaa"));
    assert!(!es_distinta("", "SHA256:aaa"));
    assert!(coincide("SHA256:aaa", "SHA256:aaa"));
    assert!(es_distinta("SHA256:aaa", "SHA256:bbb"));
    assert!(!coincide("SHA256:aaa", "SHA256:bbb"));
}
