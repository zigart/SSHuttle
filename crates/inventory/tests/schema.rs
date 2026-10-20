use sshuttle_inventory::{validar, Alta, ErrorPerfil};

#[test]
fn el_puerto_vacio_queda_en_22_y_el_nombre_copia_el_host() {
    let perfil = validar(Alta {
        id: None,
        host: " bastion ".into(),
        puerto: None,
        usuario: " ana ".into(),
    })
    .unwrap();
    assert_eq!(perfil.puerto, 22);
    assert_eq!(perfil.host, "bastion");
    assert_eq!(perfil.usuario, "ana");
    assert_eq!(perfil.nombre_visible, "bastion");
    assert!(!perfil.id.is_empty());
}

#[test]
fn rechaza_host_usuario_y_puerto_cero() {
    assert_eq!(
        validar(Alta { id: None, host: "  ".into(), puerto: None, usuario: "ana".into() }),
        Err(ErrorPerfil::HostVacio)
    );
    assert_eq!(
        validar(Alta { id: None, host: "bastion".into(), puerto: None, usuario: " ".into() }),
        Err(ErrorPerfil::UsuarioVacio)
    );
    assert_eq!(
        validar(Alta { id: None, host: "bastion".into(), puerto: Some(0), usuario: "ana".into() }),
        Err(ErrorPerfil::PuertoInvalido)
    );
}
