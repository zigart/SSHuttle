use sshuttle_ssh::Espera;

#[test]
fn crece_hasta_30_y_se_queda() {
    let mut espera = Espera::nueva();
    let segundos: Vec<u64> = (0..7).map(|_| espera.siguiente().as_secs()).collect();
    assert_eq!(segundos, vec![1, 2, 4, 8, 16, 30, 30]);
    espera.reiniciar();
    assert_eq!(espera.siguiente().as_secs(), 1);
}
