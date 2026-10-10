#![forbid(unsafe_code)]
use sv_claude_kaggle::{self as sv, need, parse, sha, R};
use serde_json::{json, Value};
use std::{fs, path::Path, io::{Read, Write}, time::{Instant, Duration}};
use zeroize::Zeroizing;
const BASE: &str = "https://mp-staging.kaggle.net/models/openapi";
const ROOT: &str = "/kaggle/working/claude-cyb09-20261010";
#[path="cyb09/recepcion.rs"] mod recepcion;
use recepcion::*;
#[path="cyb09/continuacion.rs"] mod continuacion;
use continuacion::*;

fn run() -> R<()> {
    let args: Vec<_> = std::env::args().collect();
    need(args.len() == 5, "Uso: ejecutar-cyb09 PAQUETE SHA256 ORIGINALES --comprobar|--ejecutar")?;
    need(["--comprobar", "--ejecutar"].contains(&args[4].as_str()), "Opción ajena")?;
    let bytes = fs::read(&args[1]).map_err(|e| e.to_string())?;
    need(sha(&bytes) == args[2], "Paquete distinto del recibido")?;
    let p = parse(&bytes)?;
    let original = Path::new(&args[3]);
    let report = comprobar_paquete(&p, original)?;
    need(std::env::var("MODEL_PROXY_URL").map_err(|_| "Destino no recibido")?.trim_end_matches('/') == BASE, "Destino ajeno al autorizado")?;
    let key = Zeroizing::new(std::env::var("MODEL_PROXY_API_KEY").map_err(|_| "Acceso de Kaggle no recibido")?);
    need(!key.is_empty(), "Credencial ausente")?;
    let binary_sha = sha(&fs::read(std::env::current_exe().map_err(|e| e.to_string())?).map_err(|e| e.to_string())?);
    let admission = Path::new("/kaggle/working/claude-cyb09-frontera-20261010");
    if args[4] == "--comprobar" {
        need(!admission.exists(), "Admisión ya realizada; conservarla")?;
        let monitor = sv::instrumentacion::Monitor::start_bounded(admission, 10)?;
        std::thread::sleep(Duration::from_millis(1100)); monitor.healthy()?;
        let telemetry = monitor.finish()?;
        need(telemetry["fallos_medicion"] == 0 && telemetry["intervalo_maximo_ms"].as_u64().is_some_and(|v| v <= 750), "Instrumentación remota no conforme")?;
        save(&admission.join("ADMISION-RUST.json"), &json!({"conforme":true,"modelo":sv::MODELO,"paquete_sha256":args[2],"binario_sha256":binary_sha,"sistema":std::env::consts::OS,"arquitectura":std::env::consts::ARCH,"destino_recibido_conforme":true,"acceso_disponible":true,"recuperacion":report,"telemetria":telemetry,"nuevas_inferencias":0,"licencia":sv::LICENCIA}))?;
        println!("Admisión remota Rust conforme: originales, composición, destino, acceso y medición; cero inferencias"); return Ok(());
    }
    let prior = leer(&admission.join("ADMISION-RUST.json"))?;
    need(prior["conforme"] == true && prior["paquete_sha256"] == args[2] && prior["binario_sha256"] == binary_sha && prior["modelo"] == sv::MODELO, "Admisión no corresponde al ejecutable recibido")?;
    let root = Path::new(ROOT); need(!root.exists(), "Campaña ya iniciada; no se repite automáticamente")?;
    fs::create_dir(root).map_err(|e| e.to_string())?;
    let limit = p["limite_nanodolares"].as_u64().ok_or("Límite ausente")?;
    save(&root.join("INICIO.json"), &json!({"utc_ms":sv::instrumentacion::utc_ms(),"modelo":sv::MODELO,"binario_sha256":binary_sha,"paquete_sha256":args[2],"limite_nanodolares":limit,"cuota_previa_diaria_disponible_usd":10,"historia_cyb16_coste_nanodolares":1_272_544_000u64,"reserva_historica_http400":"Pendiente de conciliación histórica; no equivale a cargo en esta ventana renovada","reintentos_automaticos":0,"licencia":sv::LICENCIA}))?;
    let client = reqwest::blocking::Client::builder().https_only(true).redirect(reqwest::redirect::Policy::none()).retry(reqwest::retry::never()).timeout(Duration::from_secs(300)).build().map_err(|_| "Cliente no recibido")?;
    let timer = Instant::now(); let mut spent = 0u64; let mut unknown = 0; let mut rows = Vec::new(); let mut state = "completo";
    'all: for case in p["casos"].as_array().ok_or("Casos")? {
        let id = case["id"].as_str().ok_or("Identidad")?; let mut history = Vec::new();
        for stage in 0..3 {
            let (q,w) = sv::compose(&case["base"], stage, &history)?;
            if CONSERVADAS.contains(&(id, stage)) {
                let row = cotejar_original(original, &p["originales"], id, stage, &q, &w)?;
                rows.push(json!({"caso":id,"etapa":stage,"resultado":row,"nueva_inferencia":false,"origen":"corte-01/campana/cyb16"}));
                save(&root.join(format!("HITO-{:03}.json",rows.len())),rows.last().unwrap())?;
                history.push(fs::read_to_string(original.join(format!("campana/cyb16/{id}-R{stage}/FINAL.txt"))).map_err(|e| e.to_string())?);
                continue;
            }
            if timer.elapsed().as_secs() >= 5400 { state="plazo_banco_agotado"; break 'all; }
            let reserved = reserva(&w)?;
            if !cuota_permite(spent, reserved, limit) { state="cuota_reservada_insuficiente"; break 'all; }
            let d = root.join(format!("cyb09/{id}-R{stage}"));
            save(&d.join("RESERVA.json"), &json!({"coste_anterior_nanodolares":spent,"reserva_nanodolares":reserved,"limite_nanodolares":limit,"costes_historicos_excluidos_de_esta_ventana":true}))?;
            let row = match single(&client, &key, &q, &w, &d) { Ok(v)=>v, Err(e)=>{save(&d.join("INCIDENCIA-INSTRUMENTAL.json"),&json!({"error":e,"repetir":false}))?;state="incidencia_instrumental";break 'all;} };
            println!("{id} R{stage}: completa={}; coste_nano={}",row["completa"],row["coste_nanodolares"]); std::io::stdout().flush().map_err(|e| e.to_string())?;
            rows.push(json!({"caso":id,"etapa":stage,"resultado":row,"nueva_inferencia":true}));
            save(&root.join(format!("HITO-{:03}.json",rows.len())),rows.last().unwrap())?;
            let row=&rows.last().unwrap()["resultado"]; let cost=row["coste_nanodolares"].as_u64();
            if let Some(c)=cost {spent=spent.checked_add(c).ok_or("Coste desbordado")?;} else {unknown+=1;}
            if row["completa"]!=true || cost.is_none() {state="incidencia_de_recepcion";break 'all;}
            if cost.unwrap()>reserved || spent>limit {state="desviacion_de_reserva";break 'all;}
            history.push(fs::read_to_string(d.join("FINAL.txt")).map_err(|e| e.to_string())?);
        }
    }
    save(&root.join("RESULTADO-CAMPANA.json"),&json!({"estado":state,"entregas":rows,"coste_nuevo_conocido_nanodolares":spent,"costes_no_recibidos":unknown,"dictamen_sustantivo":"pendiente exterior al candidato","recepcion_independiente":"pendiente","licencia":sv::LICENCIA}))?;
    println!("Cierre: {state}; coste nuevo {spent} nanodólares"); Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("IMPEDIMENTO: {e}");std::process::exit(1)}}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
