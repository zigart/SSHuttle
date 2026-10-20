use sshuttle_inventory::{validar, validar_nombre, Alta, ErrorPerfil};

#[test]
fn el_puerto_vacio_queda_en_22_y_el_nombre_copia_el_host() {
    let perfil = validar(Alta {
        id: None,
        host: " bastion ".into(),
        puerto: None,
        usuario: " ana ".into(),
        nombre_visible: None,
    })
    .unwrap();
    assert_eq!(perfil.puerto, 22);
    assert_eq!(perfil.host, "bastion");
    assert_eq!(perfil.usuario, "ana");
    assert_eq!(perfil.nombre_visible, "bastion");
    assert!(!perfil.id.is_empty());
}

#[test]
fn el_nombre_explicito_queda_trimado() {
    let perfil = validar(Alta {
        id: None,
        host: "10.0.0.1".into(),
        puerto: None,
        usuario: "ana".into(),
        nombre_visible: Some("  bastion prod  ".into()),
    })
    .unwrap();
    assert_eq!(perfil.nombre_visible, "bastion prod");
    assert_eq!(perfil.host, "10.0.0.1");
}

#[test]
fn rechaza_host_usuario_y_puerto_cero() {
    assert_eq!(
        validar(Alta {
            id: None,
            host: "  ".into(),
            puerto: None,
            usuario: "ana".into(),
            nombre_visible: None,
        }),
        Err(ErrorPerfil::HostVacio)
    );
    assert_eq!(
        validar(Alta {
            id: None,
            host: "bastion".into(),
            puerto: None,
            usuario: " ".into(),
            nombre_visible: None,
        }),
        Err(ErrorPerfil::UsuarioVacio)
    );
    assert_eq!(
        validar(Alta {
            id: None,
            host: "bastion".into(),
            puerto: Some(0),
            usuario: "ana".into(),
            nombre_visible: None,
        }),
        Err(ErrorPerfil::PuertoInvalido)
    );
    assert_eq!(validar_nombre("   "), Err(ErrorPerfil::NombreVacio));
}
