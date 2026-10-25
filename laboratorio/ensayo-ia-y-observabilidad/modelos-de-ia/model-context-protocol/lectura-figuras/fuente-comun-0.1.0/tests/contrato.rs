use sv_figuras_fuente::{Texto, html, importar, leer_json, markdown, representar_svg, validar};

fn documento(interior: &str) -> String {
    format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 400">{interior}</svg>"#)
}

#[test]
fn una_modificacion_alimenta_las_tres_representaciones() {
    let mut fuente = importar(documento("<text>Inmunología</text>").as_bytes()).unwrap();
    fuente.textos[0].valor = "Célula β & <referencia>".into();
    let svg = representar_svg(&fuente).unwrap();
    let lectura = html(&fuente).unwrap();
    assert!(svg.contains("Célula β &amp; &lt;referencia&gt;"));
    assert!(lectura.contains("Célula β &amp; &lt;referencia&gt;"));
    assert!(markdown(&fuente).unwrap().contains("Célula β"));
    assert_eq!(
        importar(svg.as_bytes()).unwrap().textos[0].valor,
        fuente.textos[0].valor
    );
}

#[test]
fn tspan_preserva_texto_y_espacios_sin_inferir_relaciones() {
    let f = importar(documento("<text>A <tspan>β</tspan> C</text>").as_bytes()).unwrap();
    assert!(html(&f).unwrap().contains("<li>A β C</li>"));
    assert_eq!(f.relaciones, "no_inferidas");
}

#[test]
fn se_rechazan_codigo_recursos_externos_y_entidades() {
    for interior in [
        "<script>alert(1)</script>",
        "<foreignObject/>",
        "<image href=\"https://externo.invalid/a\"/>",
        "<text onclick=\"alert(1)\">Texto</text>",
        "<g style=\"background:url(https://externo.invalid)\"/>",
        "<text>&entidad;</text>",
        "<?xml-stylesheet href=\"https://externo.invalid\"?>",
    ] {
        assert!(
            importar(documento(interior).as_bytes()).is_err(),
            "{interior}"
        );
    }
    assert!(
        importar(
            format!(
                "<!DOCTYPE svg [<!ENTITY x SYSTEM 'file:///etc/passwd'>]>{}",
                documento("<text>&x;</text>")
            )
            .as_bytes()
        )
        .is_err()
    );
}

#[test]
fn se_rechazan_documentos_ambiguos_o_incompletos() {
    for xml in [
        format!("{}{}", documento(""), documento("")),
        documento("<text>"),
        documento("<?xml version=\"1.0\"?>"),
        format!("<![CDATA[ ]]>{}", documento("")),
        documento("<text><rect/></text>"),
        documento("<tspan>fuera</tspan>"),
        documento("<text id=\"a\" id=\"b\">x</text>"),
    ] {
        assert!(importar(xml.as_bytes()).is_err(), "{xml}");
    }
}

#[test]
fn las_referencias_no_pueden_ocultar_texto_ni_quedar_huerfanas() {
    let mut f = importar(documento("<text>Visible</text>").as_bytes()).unwrap();
    f.textos.push(Texto {
        id: "oculto".into(),
        valor: "Oculto".into(),
    });
    assert!(validar(&f).is_err());
    for interior in [
        "<text id=\"a\">x</text><text id=\"a\">y</text>",
        "<path d=\"M0 0 L1 1\" marker-end=\"url(#ausente)\"/>",
        "<text aria-labelledby=\"ausente\">x</text>",
    ] {
        assert!(importar(documento(interior).as_bytes()).is_err());
    }
}

#[test]
fn datos_json_estrictos_y_cotas() {
    let f = importar(documento("<text>Dato</text>").as_bytes()).unwrap();
    let mut j = serde_json::to_value(&f).unwrap();
    j["orden_de_ejecucion"] = "no_admitida".into();
    assert!(leer_json(&serde_json::to_vec(&j).unwrap()).is_err());
    assert!(importar(&vec![b' '; 2 * 1024 * 1024 + 1]).is_err());
    assert!(
        importar(documento(&format!("<text>{}</text>", "x".repeat(128 * 1024 + 1))).as_bytes())
            .is_err()
    );
    assert!(
        importar(documento(&format!("{}{}", "<g>".repeat(49), "</g>".repeat(49))).as_bytes())
            .is_err()
    );
    for valor in ["NaN", "inf", "100001"] {
        assert!(importar(documento(&format!("<circle cx=\"{valor}\"/>")).as_bytes()).is_err());
    }
}

#[test]
fn seis_figuras_conservan_etiquetas_y_composicion_explicita() {
    let originales = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/figuras");
    let nombres = [
        "4a20336b03d16a52cd12d8807132b71e5bf6910d43cdce93dff39d9d52e621a5",
        "3a2512c3d3726027742095326f7d661f9c2df0bddfc75d199428d74904ecd37f",
        "e8b4b4c8c92e13d0766c7f6616c85ff20a959f2e3520994d4c395b1e097c586e",
        "94717a53abc750a61a7609e2151fca9e055ba90b49f089dac8d68b29acaaee9a",
        "76b520283ce74f2414346f538c05a2de7314d7c141637c6e2a74fcef027acecb",
        "1a43aece810761485bc66c563cdd18a860f3baf5e2fbd7e5c5a861191ca86897",
    ];
    for nombre in nombres {
        let original = std::fs::read(originales.join(format!("{nombre}.svg"))).unwrap();
        let fuente = importar(&original).unwrap();
        let salida = representar_svg(&fuente).unwrap();
        let vuelta = importar(salida.as_bytes()).unwrap();
        assert_eq!(
            serde_json::to_value(&fuente.estructura_visual).unwrap(),
            serde_json::to_value(&vuelta.estructura_visual).unwrap()
        );
        assert_eq!(
            serde_json::to_value(&fuente.textos).unwrap(),
            serde_json::to_value(&vuelta.textos).unwrap()
        );
        assert!(html(&fuente).unwrap().contains("<li>"));
    }
}
