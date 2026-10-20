use super::*;

fn temporal() -> (std::path::PathBuf, Inventario) {
    let ruta = std::env::temp_dir().join(format!("sshuttle-perfil-{}.db", uuid::Uuid::new_v4()));
    let inventario = Inventario::abrir(&ruta).unwrap();
    (ruta, inventario)
}

#[test]
fn el_perfil_no_tiene_columna_de_contrasena_y_sobrevive() {
    let (ruta, inventario) = temporal();
    let guardado = inventario
        .guardar(Alta {
            id: None,
            host: "bastion".into(),
            puerto: None,
            usuario: "ana".into(),
            nombre_visible: None,
        })
        .unwrap();
    assert_eq!(guardado.puerto, 22);
    assert_eq!(guardado.nombre_visible, "bastion");
    assert!(guardado.secreto_id.is_empty());
    assert!(guardado.huella_host.is_empty());

    let columnas = {
        let mut consulta = inventario.conexion.prepare("PRAGMA table_info(servidores)").unwrap();
        let filas = consulta.query_map([], |fila| fila.get::<_, String>(1)).unwrap();
        filas.collect::<Result<Vec<_>, _>>().unwrap()
    };
    assert!(!columnas.iter().any(|c| c.eq_ignore_ascii_case("password")));
    assert!(!columnas.iter().any(|c| c.eq_ignore_ascii_case("contrasena")));
    assert!(columnas.iter().any(|c| c == "secreto_id"));

    inventario.asignar_secreto(&guardado.id, "sec-1").unwrap();
    inventario.asignar_huella(&guardado.id, "SHA256:abc").unwrap();
    drop(inventario);

    let reabierto = Inventario::abrir(&ruta).unwrap();
    let lista = reabierto.listar().unwrap();
    assert_eq!(lista.len(), 1);
    assert_eq!(lista[0].secreto_id, "sec-1");
    assert_eq!(lista[0].huella_host, "SHA256:abc");
    assert_eq!(lista[0].puerto, 22);
    let _ = std::fs::remove_file(ruta);
}

#[test]
fn renombrar_cambia_el_nombre_y_guardar_sin_nombre_lo_conserva() {
    let (ruta, inventario) = temporal();
    let guardado = inventario
        .guardar(Alta {
            id: None,
            host: "10.0.0.1".into(),
            puerto: None,
            usuario: "ana".into(),
            nombre_visible: None,
        })
        .unwrap();
    let renombrado = inventario.renombrar(&guardado.id, "  bastion prod  ").unwrap();
    assert_eq!(renombrado.nombre_visible, "bastion prod");

    let otra_vez = inventario
        .guardar(Alta {
            id: Some(guardado.id.clone()),
            host: "10.0.0.2".into(),
            puerto: Some(2222),
            usuario: "ana".into(),
            nombre_visible: None,
        })
        .unwrap();
    assert_eq!(otra_vez.nombre_visible, "bastion prod");
    assert_eq!(otra_vez.host, "10.0.0.2");
    assert_eq!(otra_vez.puerto, 2222);

    assert!(inventario.renombrar(&guardado.id, "   ").is_err());
    let _ = std::fs::remove_file(ruta);
}

#[test]
fn eliminar_quita_una_fila_y_deja_las_demas() {
    let (ruta, inventario) = temporal();
    let uno = inventario
        .guardar(Alta {
            id: None,
            host: "uno".into(),
            puerto: None,
            usuario: "ana".into(),
            nombre_visible: None,
        })
        .unwrap();
    let dos = inventario
        .guardar(Alta {
            id: None,
            host: "dos".into(),
            puerto: Some(2222),
            usuario: "bob".into(),
            nombre_visible: None,
        })
        .unwrap();
    inventario.asignar_secreto(&dos.id, "sec-dos").unwrap();
    inventario.asignar_huella(&dos.id, "SHA256:dos").unwrap();

    inventario.eliminar(&uno.id).unwrap();
    inventario.eliminar(&uno.id).unwrap();

    let lista = inventario.listar().unwrap();
    assert_eq!(lista.len(), 1);
    assert_eq!(lista[0].id, dos.id);
    assert_eq!(lista[0].nombre_visible, "dos");
    assert_eq!(lista[0].host, "dos");
    assert_eq!(lista[0].puerto, 2222);
    assert_eq!(lista[0].usuario, "bob");
    assert_eq!(lista[0].secreto_id, "sec-dos");
    assert_eq!(lista[0].huella_host, "SHA256:dos");
    drop(inventario);

    let reabierto = Inventario::abrir(&ruta).unwrap();
    assert!(reabierto.obtener(&uno.id).is_err());
    assert_eq!(reabierto.listar().unwrap().len(), 1);
    let _ = std::fs::remove_file(ruta);
}
