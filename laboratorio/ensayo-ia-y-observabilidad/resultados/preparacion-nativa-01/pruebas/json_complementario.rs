//! EIO-JSON-01: banco complementario; no cambia el receptor ni repara salidas.
use eio_candidato::{comprobar, Condiciones, Propuesta, Referencia};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Caso { id: String, propuesta: String, esperado: String, descripcion: String }

fn condiciones() -> Condiciones {
    Condiciones { id: "sintetica-01".into(), permiso: false, activa: false,
        veto: false, tokens_entrada: 120, exportacion_correcta: true }
}
fn referencias() -> Vec<Referencia> {
    vec![Referencia { id: "A".into(), version: 1 }, Referencia { id: "B".into(), version: 1 }]
}
fn informar(id: &str, esperado: Value, observado: Value, fallos: &mut usize) {
    let conforme = esperado == observado;
    if !conforme { *fallos += 1; }
    println!("{}", json!({"tipo":"EIO-JSON-01", "caso":id,
        "esperado":esperado,"observado":observado,"conforme":conforme}));
}
// La expectativa conserva texto UTF-8 y tokens en su orden; no tiempos ni IDs aleatorios.
#[derive(Clone, PartialEq, Eq)]
struct Contenido { texto: String, tokens: Vec<u32> }
fn contenido(v: &Value) -> Result<Contenido, String> {
    let s = v.get("salida").ok_or("salida ausente")?;
    let texto = s.get("texto").and_then(Value::as_str).ok_or("texto ausente")?.to_owned();
    let tokens = s.get("tokens").and_then(Value::as_array).ok_or("tokens ausentes")?
        .iter().map(|x| x.as_u64().and_then(|n| u32::try_from(n).ok())
            .ok_or_else(|| "token no u32".to_owned())).collect::<Result<Vec<_>, _>>()?;
    Ok(Contenido { texto, tokens })
}
fn comparar(v: &Value, esperado: &Contenido) -> bool {
    contenido(v).map(|c| c == *esperado).unwrap_or(false)
}

#[test]
fn complemento_json() -> Result<(), Box<dyn std::error::Error>> {
    let casos: Vec<Caso> = serde_json::from_str(include_str!("../resultados/revision-06/json-01/CASOS.json"))?;
    let mut fallos = 0;
    let mut controles = 0;
    let refs = referencias();
    for caso in &casos {
        let observado = comprobar(&condiciones(), &caso.propuesta, &refs, &refs).err().unwrap_or("OK");
        informar(&caso.id, json!(caso.esperado), json!(observado), &mut fallos);
        controles += 1;
        let _descripcion = &caso.descripcion;
    }
    // Dos formas de salida del serializador; no promesa de JSON canónico universal.
    for id in ["J11", "J17"] {
        let caso = casos.iter().find(|c| c.id == id).ok_or("caso de recorrido ausente")?;
        let original: Propuesta = serde_json::from_str(&caso.propuesta)?;
        for (nombre, bytes) in [("compacto", serde_json::to_vec(&original)?),
                               ("legible", serde_json::to_vec_pretty(&original)?)] {
            let vuelta: Propuesta = serde_json::from_slice(&bytes)?;
            let igual = vuelta.peticion == original.peticion && vuelta.accion == original.accion
                && vuelta.respuesta.as_bytes() == original.respuesta.as_bytes()
                && vuelta.referencias == original.referencias;
            informar(&format!("R-{id}-{nombre}"), json!(true), json!(igual), &mut fallos);
            controles += 1;
        }
    }
    // Entradas históricas completas, fijadas por SHA-256 antes del proceso Rust.
    let evidencia = include_str!("../resultados/continuacion-06/INFERENCIAS.jsonl");
    let lineas: Vec<&str> = evidencia.lines().collect();
    if lineas.len() != 2 { return Err("se requieren exactamente dos líneas históricas".into()); }
    for (i, linea) in lineas.iter().enumerate() {
        let original: Value = serde_json::from_str(linea)?;
        let esperado = contenido(&original)?;
        let variante = if i == 0 { "EIO-05" } else { "EIO-06" };
        let cantidad = if i == 0 { 74 } else { 59 };
        if original["variante_peticion"] != variante || esperado.tokens.len() != cantidad {
            return Err("identidad histórica no concordante".into());
        }
        let c = Condiciones { tokens_entrada: if i == 0 {120} else {212}, ..condiciones() };
        let observado = comprobar(&c, &esperado.texto, &refs, &refs).err().unwrap_or("OK");
        informar(&format!("H{i}-contrato"), json!("ESTRUCTURA"), json!(observado), &mut fallos);
        controles += 1;
        for (nombre, bytes) in [("compacto", serde_json::to_vec(&original)?),
                               ("legible", serde_json::to_vec_pretty(&original)?)] {
            let vuelta: Value = serde_json::from_slice(&bytes)?;
            informar(&format!("H{i}-{nombre}"), json!(true), json!(comparar(&vuelta, &esperado)), &mut fallos);
            controles += 1;
        }
        // Sensibilidad del comparador sobre copias: no modifica la evidencia histórica.
        let mut alterada = original.clone();
        alterada["salida"]["texto"] = json!(format!("{} ", esperado.texto));
        informar(&format!("M{i}-texto"), json!(false), json!(comparar(&alterada, &esperado)), &mut fallos);
        let mut alterada = original.clone();
        alterada["salida"]["tokens"].as_array_mut().ok_or("tokens")?.swap(0, 1);
        informar(&format!("M{i}-orden"), json!(false), json!(comparar(&alterada, &esperado)), &mut fallos);
        let mut alterada = original.clone();
        alterada["salida"]["tokens"].as_array_mut().ok_or("tokens")?.pop();
        informar(&format!("M{i}-omision"), json!(false), json!(comparar(&alterada, &esperado)), &mut fallos);
        controles += 3;
    }
    let texto = "\r\n\t\"\\ ñ é e\u{301} 😀 \0 final";
    let original = json!({"salida":{"texto":texto,"tokens":[0_u32,1_u32,151645_u32,u32::MAX]}});
    let esperado = Contenido { texto: texto.into(), tokens: vec![0, 1, 151645, u32::MAX] };
    let vuelta: Value = serde_json::from_slice(&serde_json::to_vec(&original)?)?;
    informar("R-texto-tokens-limites", json!(true), json!(comparar(&vuelta, &esperado)), &mut fallos);
    controles += 1;
    println!("{}", json!({"tipo":"EIO-JSON-01-resumen","controles":controles,
        "fallos":fallos,"conforme":fallos==0 && controles==35,"inferencias_nuevas":0}));
    assert_eq!(controles, 35, "inventario del banco");
    assert_eq!(fallos, 0, "discrepancias conservadas en las líneas anteriores");
    Ok(())
}
