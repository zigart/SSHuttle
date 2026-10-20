use sshuttle_terminal::{bytes_actuales, trabajo_interrumpido};

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
