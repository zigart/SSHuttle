use sshuttle_ssh::{
    tras_autenticacion, tras_corte_de_red, EstadoIndicador, Indicador, MotivoInactiva,
};

#[test]
fn solo_hay_tres_valores_visibles() {
    let valores = [Indicador::Activa, Indicador::Inactiva, Indicador::Reintentando]
        .map(|valor| serde_json::to_string(&valor).unwrap());
    assert_eq!(valores, [r#""activa""#, r#""inactiva""#, r#""reintentando""#]);
}

#[test]
fn el_corte_reintenta_y_la_autenticacion_activa() {
    assert_eq!(tras_corte_de_red().valor, Indicador::Reintentando);
    assert!(tras_corte_de_red().motivo.is_none());
    assert_eq!(tras_autenticacion(), EstadoIndicador::activa());
    assert_eq!(EstadoIndicador::inactiva(MotivoInactiva::FaltaSecreto).valor, Indicador::Inactiva);
}
