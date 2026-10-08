#![forbid(unsafe_code)]
//! Observaciones de MD01 sin clasificación e integridad de la edición pública.
//! No certifica la verdad de las fuentes ni la equivalencia experimental.

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, env, fs, io::Read, path::Path};

type R<T> = Result<T, String>;
const VERSION: &str = "1.2.0";
const FECHA: &str = "2026-10-08";
const LICENCIA: &str = "© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).";
const MANIFIESTO: &str = "MANIFIESTO.json";
const ALCANCE: &str = "Integridad de archivos y reproducción de las observaciones MD01-R0, sin clasificación comparativa. Las mediciones no están homologadas temporalmente. No certifica fuentes de terceros, equivalencia experimental, calidad semántica ni liquidación económica.";

fn exigir(condicion: bool, mensaje: impl Into<String>) -> R<()> {
    if condicion { Ok(()) } else { Err(mensaje.into()) }
}

fn texto<'a>(v: &'a Value, campo: &str) -> R<&'a str> {
    v.get(campo).and_then(Value::as_str).filter(|s| !s.trim().is_empty())
        .ok_or_else(|| format!("Campo textual ausente o vacío: {campo}"))
}

fn entero(v: &Value, campo: &str) -> R<u64> {
    v.get(campo).and_then(Value::as_u64)
        .ok_or_else(|| format!("Campo entero no negativo ausente: {campo}"))
}

fn leer_json(p: &Path) -> R<Value> {
    let b = fs::read(p).map_err(|e| format!("No se lee {}: {e}", p.display()))?;
    serde_json::from_slice(&b).map_err(|e| format!("JSON inválido en {}: {e}", p.display()))
}

fn bytes_json(v: &Value) -> R<Vec<u8>> {
    let mut b = serde_json::to_vec_pretty(v).map_err(|e| e.to_string())?;
    b.push(b'\n');
    Ok(b)
}

fn validar_datos(v: &Value) -> R<()> {
    exigir(texto(v, "version")? == VERSION, "Versión de datos distinta de la edición admitida")?;
    exigir(texto(v, "caso")? == "MD01", "Caso distinto de MD01")?;
    exigir(texto(v, "metrica")? == "primer_evento_texto_ms", "Métrica distinta de la admitida")?;
    let limites = v.get("limitaciones").and_then(Value::as_array)
        .ok_or("Faltan las limitaciones")?;
    exigir(!limites.is_empty() && limites.iter().all(|x| x.as_str().is_some_and(|s| !s.trim().is_empty())),
        "Limitaciones vacías o no textuales")?;
    let filas = v.get("filas").and_then(Value::as_array).ok_or("Faltan las filas")?;
    let mut pares = BTreeSet::new();
    let mut iniciales = BTreeSet::new();
    for f in filas {
        let modelo = texto(f, "modelo")?;
        let etapa = texto(f, "etapa")?;
        exigir(matches!(modelo, "gpt-6-astra" | "grok-4.7"), "Modelo ajeno al contraste MD01")?;
        exigir(matches!(etapa, "R0" | "R1" | "R2"), "Etapa no admitida")?;
        exigir(pares.insert((modelo, etapa)), format!("Par modelo-etapa repetido: {modelo}/{etapa}"))?;
        let entrada = entero(f, "input_tokens")?;
        let salida = entero(f, "output_tokens")?;
        let total = entero(f, "total_tokens")?;
        exigir(entrada.checked_add(salida) == Some(total), format!("Suma de tokens discordante: {modelo}/{etapa}"))?;
        entero(f, "primer_evento_texto_ms")?;
        let fuente = texto(f, "fuente")?;
        exigir(fuente.starts_with("https://github.com/") && !fuente.contains(['\r', '\n']),
            "La fuente debe ser una referencia pública HTTPS de GitHub")?;
        if etapa == "R0" { iniciales.insert(modelo); }
    }
    exigir(iniciales == BTreeSet::from(["gpt-6-astra", "grok-4.7"]),
        "R0 debe contener exactamente GPT-6 Astra y Grok 4.7")
}

fn derivar(v: &Value) -> R<Value> {
    validar_datos(v)?;
    let mut filas: Vec<Value> = v["filas"].as_array().unwrap().iter()
        .filter(|f| f["etapa"] == "R0").cloned().collect();
    filas.sort_by(|a, b| a["modelo"].as_str().cmp(&b["modelo"].as_str()));
    let mut salida = json!({
        "version": VERSION,
        "fecha": FECHA,
        "caso": "MD01",
        "etapa_presentada": "R0",
        "metrica": "primer_evento_texto_ms",
        "regla": "Presentación alfabética por identificador de modelo, sin clasificación comparativa ni puestos. Las demoras no determinan el orden de presentación. R1 y R2 no se agregan.",
        "naturaleza": "observaciones_no_homologadas_temporalmente",
        "alcance": ALCANCE,
        "datos_fuente": "DATOS-SV.json",
        "filas": filas,
        "limitaciones": v["limitaciones"]
    });
    for campo in ["criterio", "definicion", "contadores", "licencia_documento"] {
        if let Some(valor) = v.get(campo) { salida[campo] = valor.clone(); }
    }
    Ok(salida)
}

fn celda_csv(s: &str) -> String { format!("\"{}\"", s.replace('"', "\"\"")) }

fn csv(v: &Value) -> Vec<u8> {
    let mut s = String::from("version,caso,etapa,metrica,modelo,primer_evento_texto_ms,input_tokens,output_tokens,total_tokens,fuente\n");
    for f in v["filas"].as_array().unwrap() {
        s.push_str(&format!("{VERSION},MD01,R0,primer_evento_texto_ms,{},{},{},{},{},{}\n",
            celda_csv(f["modelo"].as_str().unwrap()),
            f["primer_evento_texto_ms"], f["input_tokens"], f["output_tokens"],
            f["total_tokens"], celda_csv(f["fuente"].as_str().unwrap())));
    }
    s.into_bytes()
}

fn excluir_directorio(nombre: &str) -> bool { matches!(nombre, "target" | "resultados" | ".git") }

fn huella_archivo(p: &Path) -> R<(u64, String)> {
    let mut f = fs::File::open(p).map_err(|e| format!("No se abre {}: {e}", p.display()))?;
    let mut h = Sha256::new();
    let mut bytes = 0u64;
    let mut buffer = [0u8; 65_536];
    loop {
        let n = f.read(&mut buffer).map_err(|e| format!("No se coteja {}: {e}", p.display()))?;
        if n == 0 { break; }
        bytes = bytes.checked_add(n as u64).ok_or("Desbordamiento de tamaño")?;
        h.update(&buffer[..n]);
    }
    Ok((bytes, format!("{:x}", h.finalize())))
}

fn recorrer(raiz: &Path, directorio: &Path, filas: &mut Vec<Value>) -> R<()> {
    let entries = fs::read_dir(directorio).map_err(|e| format!("No se enumera {}: {e}", directorio.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let p = entry.path();
        let meta = fs::symlink_metadata(&p).map_err(|e| e.to_string())?;
        exigir(!meta.file_type().is_symlink(), format!("Enlace simbólico excluido: {}", p.display()))?;
        let nombre = entry.file_name().into_string().map_err(|_| "Nombre no representable en UTF-8")?;
        if meta.is_dir() {
            if !excluir_directorio(&nombre) { recorrer(raiz, &p, filas)?; }
        } else {
            exigir(meta.is_file(), format!("Entrada no regular: {}", p.display()))?;
            let relativa = p.strip_prefix(raiz).map_err(|e| e.to_string())?.to_str()
                .ok_or("Ruta no representable en UTF-8")?.replace('\\', "/");
            if relativa == MANIFIESTO { continue; }
            let (bytes, sha256) = huella_archivo(&p)?;
            filas.push(json!({"ruta": relativa, "bytes": bytes, "sha256": sha256}));
        }
    }
    Ok(())
}

fn inventario(raiz: &Path) -> R<Vec<Value>> {
    let m = fs::symlink_metadata(raiz).map_err(|e| e.to_string())?;
    exigir(m.is_dir() && !m.file_type().is_symlink(), "La raíz debe ser un directorio real, sin enlace simbólico")?;
    let mut archivos = vec![];
    recorrer(raiz, raiz, &mut archivos)?;
    archivos.sort_by(|a, b| a["ruta"].as_str().cmp(&b["ruta"].as_str()));
    Ok(archivos)
}

fn manifiesto(archivos: &[Value]) -> Value {
    json!({"version": VERSION, "fecha": FECHA, "alcance": ALCANCE,
        "algoritmo": "SHA-256", "licencia_documento": LICENCIA, "archivos": archivos})
}

fn cotejar_manifiesto(v: &Value, actuales: &[Value]) -> R<()> {
    exigir(texto(v, "version")? == VERSION && texto(v, "fecha")? == FECHA,
        "Versión o fecha del manifiesto discordante")?;
    exigir(texto(v, "alcance")? == ALCANCE && texto(v, "algoritmo")? == "SHA-256",
        "Alcance o algoritmo discordante")?;
    let esperados = v["archivos"].as_array().ok_or("Manifiesto sin archivos")?;
    let mut rutas = BTreeSet::new();
    for f in esperados {
        let ruta = texto(f, "ruta")?;
        exigir(rutas.insert(ruta), format!("Ruta repetida en manifiesto: {ruta}"))?;
        entero(f, "bytes")?;
        let h = texto(f, "sha256")?;
        exigir(h.len() == 64 && h.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
            format!("SHA-256 inválida: {ruta}"))?;
    }
    exigir(esperados.len() == actuales.len(), "Cambió el inventario de archivos publicables")?;
    for (esperado, actual) in esperados.iter().zip(actuales) {
        exigir(esperado == actual, format!("Ruta, bytes o SHA-256 discordantes: {}", actual["ruta"]))?;
    }
    Ok(())
}

fn escribir(raiz: &Path, nombre: &str, b: &[u8]) -> R<()> {
    fs::write(raiz.join(nombre), b).map_err(|e| format!("No se escribe {nombre}: {e}"))
}

fn ejecutar(modo: &str, raiz: &Path) -> R<()> {
    // El recorrido previo impide leer o sobrescribir salidas mediante enlaces.
    inventario(raiz)?;
    let datos = leer_json(&raiz.join("DATOS-SV.json"))?;
    let observaciones = derivar(&datos)?;
    let observaciones_json = bytes_json(&observaciones)?;
    let observaciones_csv = csv(&observaciones);
    match modo {
        "generar" => {
            escribir(raiz, "OBSERVACIONES-MD01.json", &observaciones_json)?;
            escribir(raiz, "OBSERVACIONES-MD01.csv", &observaciones_csv)?;
            let archivos = inventario(raiz)?;
            escribir(raiz, MANIFIESTO, &bytes_json(&manifiesto(&archivos))?)?;
            println!("GENERADO: observaciones R0 sin clasificación; {} archivos con bytes y SHA-256.", archivos.len());
        }
        "verificar" => {
            let actual_json = fs::read(raiz.join("OBSERVACIONES-MD01.json")).map_err(|e| e.to_string())?;
            let actual_csv = fs::read(raiz.join("OBSERVACIONES-MD01.csv")).map_err(|e| e.to_string())?;
            exigir(actual_json == observaciones_json, "OBSERVACIONES-MD01.json no corresponde a DATOS-SV.json")?;
            exigir(actual_csv == observaciones_csv, "OBSERVACIONES-MD01.csv no corresponde a DATOS-SV.json")?;
            let archivos = inventario(raiz)?;
            cotejar_manifiesto(&leer_json(&raiz.join(MANIFIESTO))?, &archivos)?;
            println!("CONFORME: observaciones R0 y {} archivos cotejados; sin clasificación ni escritura.", archivos.len());
        }
        _ => return Err("Modo no admitido: utilice generar o verificar".into()),
    }
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args_os().collect();
    let resultado = if args.len() == 3 {
        args[1].to_str().ok_or_else(|| "Modo no representable en UTF-8".to_owned())
            .and_then(|modo| ejecutar(modo, Path::new(&args[2])))
    } else { Err("Uso: auditar_publicacion generar|verificar <raíz pública>".into()) };
    if let Err(e) = resultado { eprintln!("NO CONFORME: {e}"); std::process::exit(1); }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn datos() -> Value {
        let fila = |modelo, etapa, tiempo| json!({"modelo":modelo,"etapa":etapa,
            "input_tokens":10,"output_tokens":2,"total_tokens":12,
            "primer_evento_texto_ms":tiempo,"fuente":"https://github.com/ejemplo/datos"});
        json!({"version":VERSION,"caso":"MD01","metrica":"primer_evento_texto_ms",
            "limitaciones":["Una ejecución por candidato"],"filas":[
                fila("grok-4.7","R0",20), fila("gpt-6-astra","R0",10),
                fila("grok-4.7","R1",1)]})
    }

    #[test]
    fn preserva_r0_sin_puestos_ni_inferir_rango() {
        let mut d = datos();
        let r = derivar(&d).unwrap();
        assert_eq!(r["filas"].as_array().unwrap().len(), 2);
        assert_eq!(r["filas"][0]["modelo"], "gpt-6-astra");
        assert_eq!(r["naturaleza"], "observaciones_no_homologadas_temporalmente");
        assert!(r["filas"].as_array().unwrap().iter().all(|f| f.get("puesto_descriptivo").is_none()));
        // Invertir los tiempos no altera la presentación alfabética.
        d["filas"][0]["primer_evento_texto_ms"] = json!(1);
        d["filas"][1]["primer_evento_texto_ms"] = json!(1000);
        let r = derivar(&d).unwrap();
        assert_eq!(r["filas"][0]["modelo"], "gpt-6-astra");
        assert_eq!(r["filas"][0]["primer_evento_texto_ms"], 1000);
        assert_eq!(r["filas"][1]["modelo"], "grok-4.7");
        assert_eq!(r["filas"][1]["primer_evento_texto_ms"], 1);
        assert!(r["filas"].as_array().unwrap().iter().all(|f| f.get("puesto_descriptivo").is_none()));
        assert!(!String::from_utf8(csv(&r)).unwrap().contains("puesto"));
    }

    #[test]
    fn ausencia_no_se_convierte_en_cero_y_rechaza_duplicados() {
        let mut d = datos();
        d["filas"].as_array_mut().unwrap().remove(0);
        assert!(derivar(&d).is_err());
        let mut d = datos();
        let repetida = d["filas"][0].clone();
        d["filas"].as_array_mut().unwrap().push(repetida);
        assert!(derivar(&d).is_err());
        let mut d = datos();
        d["filas"][0].as_object_mut().unwrap().remove("primer_evento_texto_ms");
        assert!(derivar(&d).is_err());
    }

    #[test]
    fn comprueba_sumas_y_desbordamientos() {
        let mut d = datos();
        d["filas"][0]["total_tokens"] = json!(13);
        assert!(derivar(&d).is_err());
        d["filas"][0]["input_tokens"] = json!(u64::MAX);
        d["filas"][0]["output_tokens"] = json!(1);
        d["filas"][0]["total_tokens"] = json!(0);
        assert!(derivar(&d).is_err());
    }

    #[test]
    fn huella_conocida_y_alteracion_de_igual_tamano() {
        let h = format!("{:x}", Sha256::digest(b"abc"));
        assert_eq!(h, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        let archivo = json!({"ruta":"dato.txt","bytes":3,"sha256":h});
        let lista = vec![archivo];
        let m = manifiesto(&lista);
        assert!(cotejar_manifiesto(&m, &lista).is_ok());
        let alterado = vec![json!({"ruta":"dato.txt","bytes":3,
            "sha256":format!("{:x}",Sha256::digest(b"abd"))})];
        assert!(cotejar_manifiesto(&m, &alterado).is_err());
        assert!(cotejar_manifiesto(&m, &[]).is_err());
    }
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados.
// ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA)
// IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411
// Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
