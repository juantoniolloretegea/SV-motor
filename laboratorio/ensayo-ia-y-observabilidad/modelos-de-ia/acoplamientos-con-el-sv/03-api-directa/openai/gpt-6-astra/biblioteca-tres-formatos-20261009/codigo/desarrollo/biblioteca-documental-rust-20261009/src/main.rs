//! Verificación local sin proveedor de inferencia; aislamiento e instrumentación Rust.
use std::{fs, path::Path, sync::mpsc, time::{Duration, Instant}};
use serde_json::{json, Value};
use sv_biblioteca_documental::{leer_identidad, leer_json, preparar, ruta_segura, raiz_segura, Identidad, Politica, Recibo, MAX_MANIFEST};
use sv_mcp_documental::{auditoria::Diario, libro::guardar_nuevo, sha256};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 6 { return Err("Uso: sv-biblioteca-documental RAIZ POLITICA_RELATIVA SHA256_POLITICA BIBLIOTECA_RELATIVA SALIDA_NUEVA".into()); }
    sv_mcp_documental::aislamiento::no_network()?;
    let root = raiz_segura(Path::new(&a[1]))?;
    let raw = leer_identidad(&root, &Identidad{ruta:a[2].clone(),sha256:a[3].clone()}, 65536)?;
    let policy: Politica = leer_json(&raw)?;
    let manifest = leer_identidad(&root, &Identidad{ruta:a[4].clone(),sha256:policy.biblioteca_sha256.clone()}, MAX_MANIFEST)?;
    let out = Path::new(&a[5]);
    fs::create_dir(out)?;
    let mut log = Diario::create(&out.join("diario-preparacion.jsonl"))?;
    let isolation = sv_mcp_documental::aislamiento::probe(&ruta_segura(&root, &a[4])?);
    if isolation["red_externa_error"] != 1 || isolation["red_local_error"] != 1 { return Err("AISLAMIENTO_NO_ACREDITADO".into()); }
    log.append(json!({"evento":"inicio_preparacion","politica_sha256":a[3],"biblioteca_sha256":policy.biblioteca_sha256,
        "binario_sha256":sha256(&fs::read(std::env::current_exe()?)?),"aislamiento":isolation,
        "muestra":sv_mcp_documental::instrumentacion::muestra()}))?;
    // Plazo de proceso en Rust, sin recurrir al supervisor externo timeout.
    let (done, wait) = mpsc::channel();
    let timeout = out.join("PLAZO-AGOTADO.json");
    let supervisor = std::thread::spawn(move || {
        if matches!(wait.recv_timeout(Duration::from_secs(45)), Err(mpsc::RecvTimeoutError::Timeout)) {
            let _ = guardar_nuevo(&timeout, b"{\"conforme\":false,\"causa\":\"plazo_agotado\"}");
            std::process::exit(124);
        }
    });
    let start = Instant::now();
    let result = preparar(&root, &manifest, &policy);
    let ready = match result {
        Ok(r) => r,
        Err(e) => {
            log.append(json!({"evento":"rechazo_preparacion","causa":e}))?;
            done.send(())?; supervisor.join().map_err(|_|"SUPERVISOR_FALLIDO")?;
            return Err(e.into());
        }
    };
    let repeated = preparar(&root, &manifest, &policy)?;
    if serde_json::to_vec(&ready.catalogo)? != serde_json::to_vec(&repeated.catalogo)?
        || ready.mapa != repeated.mapa { return Err("PREPARACION_NO_REPRODUCIBLE".into()); }
    let mut files = Vec::new();
    for (name, value) in [("CATALOGO.json",serde_json::to_value(&ready.catalogo)?),
        ("FUENTES.json",serde_json::to_value(&ready.fuentes)?),("MAPA.json",ready.mapa)] {
        let bytes = serde_json::to_vec_pretty(&value)?;
        guardar_nuevo(&out.join(name), &bytes)?;
        if fs::read(out.join(name))? != bytes { return Err("ESCRITURA_DISCORDANTE".into()); }
        files.push(json!({"ruta":name,"bytes":bytes.len(),"sha256":sha256(&bytes)}));
    }
    guardar_nuevo(&out.join("POLITICA.json"), &raw)?;
    let identity = |name: &str| -> Result<Identidad, std::io::Error> {
        Ok(Identidad { ruta:name.into(), sha256:sha256(&fs::read(out.join(name))?) })
    };
    let receipt = Recibo { version:1, politica:identity("POLITICA.json")?, catalogo:identity("CATALOGO.json")?,
        fuentes:identity("FUENTES.json")?, mapa:identity("MAPA.json")?, documentos:ready.catalogo.documents.len(),
        secciones:ready.catalogo.documents.iter().map(|d|d.sections.len()).sum() };
    let receipt_raw = serde_json::to_vec_pretty(&receipt)?;
    guardar_nuevo(&out.join("RECIBO.json"), &receipt_raw)?;
    let report = json!({"conforme":true,"fase":"preparacion_local","documentos":ready.catalogo.documents.len(),
        "secciones":ready.catalogo.documents.iter().map(|d|d.sections.len()).sum::<usize>(),
        "documentos_sinteticos":ready.catalogo.documents.iter().filter(|d|d.synthetic).count(),
        "duracion_us_dos_preparaciones_y_escritura":start.elapsed().as_micros(),"reproducible":true,
        "biblioteca_sha256":policy.biblioteca_sha256,"politica_sha256":a[3],"archivos":files,"recibo_sha256":sha256(&receipt_raw),
        "inferencia":false,"consultas_proveedor":0,"tokens_entrada_proveedor":0,"tokens_salida_proveedor":0,
        "coste_api_nuevo":0,"coste_asistencia_no_atribuido":Value::Null,"arbitro_integral_ejecutado":false,
        "muestra":sv_mcp_documental::instrumentacion::muestra()});
    log.append(json!({"evento":"fin_preparacion","resultado":report}))?;
    guardar_nuevo(&out.join("PREPARACION.json"), &serde_json::to_vec_pretty(&report)?)?;
    done.send(())?; supervisor.join().map_err(|_|"SUPERVISOR_FALLIDO")?;
    println!("{report}");
    Ok(())
}
fn main() {
    if let Err(e) = run() { eprintln!("Preparación detenida: {e}"); std::process::exit(1); }
}
