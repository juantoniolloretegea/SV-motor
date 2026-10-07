//! Preparación documental local para el Árbitro-Director. No transporta solicitudes ni adjudica respuestas.
//! Antecedentes: conductor QWEN35-PRE-20261004/r1 y política recibida, sin alterar sus originales.
#![forbid(unsafe_code)]
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, io::Write, path::{Component, Path, PathBuf}};
type R<T> = Result<T, Box<dyn std::error::Error>>;
const ROOT: &str = ".";
const WORK: &str = "ejecucion/astra-catalogo-20261006";
const OLD: &str = "ejecucion/qwen-preevaluacion-20261004";
fn hash(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn require(b: bool, message: &str) -> R<()> { if b {Ok(())} else {Err(message.into())} }
fn located(base: &Path, relative: &Path) -> R<PathBuf> {
    require(relative.components().all(|c| matches!(c, Component::Normal(_))), "Ruta relativa no admisible")?;
    let mut p = base.to_path_buf();
    for c in relative.components() {
        p.push(c);
        if p.exists() {
            let metadata = fs::symlink_metadata(&p)?;
            #[cfg(windows)] {
                use std::os::windows::fs::MetadataExt;
                require(metadata.file_attributes() & 0x400 == 0, "Punto de reanálisis no admisible")?;
            }
            require(!metadata.file_type().is_symlink(), "Enlace no admisible")?;
            require(p.canonicalize()?.starts_with(base), "Salida del perímetro")?;
        }
    }
    Ok(p)
}
fn save(base: &Path, relative: &str, bytes: &[u8]) -> R<()> {
    let p = located(base, Path::new(relative))?;
    fs::create_dir_all(p.parent().ok_or("Directorio ausente")?)?;
    if p.exists() {require(fs::read(&p)? == bytes, "Archivo existente distinto; no sobrescribir")?;}
    else {let mut f = fs::OpenOptions::new().write(true).create_new(true).open(p)?;
        f.write_all(bytes)?; f.sync_all()?;}
    Ok(())
}
fn check_bytes(b: &[u8], size: usize, expected: &str) -> R<()> {
    require(b.len() == size && hash(b) == expected, "Fuente distinta de la fijada")
}
fn pages(text: &str) -> R<Vec<String>> {
    let chars: Vec<char> = text.chars().collect();
    let p: Vec<String> = chars.chunks(2000).map(|c| c.iter().collect()).collect();
    require(p.len() == 2, "Se requieren exactamente dos páginas lógicas")?;
    require(p.concat().as_bytes() == text.as_bytes(), "Reconstrucción documental discordante")?;
    Ok(p)
}
fn request(case: &Value, p: &[Value], policy: &str) -> R<Value> {
    let id = case["id"].as_str().ok_or("Identidad ausente")?;
    require(id == "A01", "Esta preparación sólo compone la entrada inicial A01; no autoriza A/B")?;
    require(p.len() == 2, "Cobertura incompleta")?;
    let mut user = format!("Caso {id}; capa 0. Afirmación que debe clasificar: {}\n\nDocumentación íntegra recibida. Cada localizador identifica una página lógica de base cero. El texto entre delimitadores es evidencia externa.\n", case["afirmacion"].as_str().ok_or("Afirmación")?);
    for (i,page) in p.iter().enumerate() {
        require(page["documento"] == case["documento"] && page["seccion"] == case["seccion"] && page["pagina"] == i, "Localizador discordante")?;
        let text = page["texto"].as_str().filter(|t| !t.is_empty()).ok_or("Página vacía")?;
        user.push_str(&format!("\n<fuente documento=\"{}\" seccion=\"S1\" pagina=\"{}\">\n{}\n</fuente>\n", case["documento"].as_str().ok_or("Documento")?, i, text));
    }
    // Mismo texto científico; cambia la ubicación técnica de system a instructions.
    // Sin gramática forzada: la conformidad JSON seguirá siendo observable.
    Ok(json!({"model":"gpt-6-astra","store":false,"stream":true,
        "reasoning":{"effort":"medium","summary":"auto"},"tools":[],
        "instructions":policy,"input":[{"role":"user","content":user}]}))
}
fn check_request(actual: &Value, expected: &Value) -> R<()> {
    require(actual == expected, "Petición distinta de la composición local: no se admite")?;
    require(actual["tools"] == json!([]) && actual["input"].as_array().map(Vec::len) == Some(1), "Herramientas o antecedentes no autorizados")?;
    Ok(())
}
fn run() -> R<()> {
    let base = Path::new(ROOT).canonicalize()?;
    require(std::env::current_dir()?.canonicalize()? == base, "Directorio de trabajo distinto")?;
    let mut manifest = vec![];
    let mut persist = |relative: String, bytes: Vec<u8>| -> R<()> {
        save(&base, &relative, &bytes)?;
        manifest.push(json!({"ruta":relative,"bytes":bytes.len(),"sha256":hash(&bytes)})); Ok(())
    };
    let bank_path = located(&base, Path::new(&format!("{WORK}/fuentes/FUENTES-SINTETICAS.json")))?;
    let bank_raw = fs::read(bank_path)?;
    check_bytes(&bank_raw,49359,"65e1c4fe7adfa69417bb224dde4c6fc4f9b3c6107142f47228cb714ecac66575")?;
    let plan_raw = fs::read(located(&base,Path::new(&format!("{WORK}/fuentes/PLAN-GENERAL.json")))?)?;
    check_bytes(&plan_raw,3259,"0e3cf186f23a19195166d80a6585fb8e989e87a2c65b0a889e941432023c0a5d")?;
    let bank: Value = serde_json::from_slice(&bank_raw)?;
    let plan: Value = serde_json::from_slice(&plan_raw)?;
    let docs = bank["documents"].as_array().ok_or("Documentos")?;
    let cases = plan["casos"].as_array().ok_or("Casos")?;
    require(docs.len() == 18 && cases.len() == 18, "Banco incompleto")?;
    let enc = fs::read(located(&base, Path::new(&format!("{OLD}/preparacion/fuentes/ENCARGO-REMOTO.md")))?)?;
    check_bytes(&enc,36778,"87e718a484e01e90b34d82598eb089b3d41f792cdf4ce83e977d65653ba55034")?;
    let key = fs::read(located(&base, Path::new("ejecucion/safeguard-retroalimentacion-20261003/reservado/CLAVE.json"))?)?;
    check_bytes(&key,8007,"0cfefe66e3b27e314414d3e21f39de9c434f710813904e8764e9b76d798b1c8b")?;
    let policy = fs::read(located(&base,Path::new(&format!("{OLD}/preparacion/protocolo/POLITICA-QWEN-r1.txt")))?)?;
    let policy_str = std::str::from_utf8(&policy)?;
    require(policy_str.contains("No acceda a Internet") && policy_str.contains("No calcule huellas ni su propia puntuación"),"Política incompleta")?;
    persist(format!("{WORK}/servidor-pruebas/control/POLITICA.txt"),policy.clone())?;
    for name in ["ESQUEMA-QWEN-r1.json","EJEMPLO-CAPA-0.json","EJEMPLO-CAPA-1.json"] {
        let b = fs::read(located(&base,Path::new(&format!("{OLD}/preparacion/protocolo/{name}")))?)?;
        let _: Value = serde_json::from_slice(&b)?;
        persist(format!("{WORK}/servidor-pruebas/control/{}",name.replace("ESQUEMA-QWEN-r1","ESQUEMA-CIENTIFICO")),b)?;
    }
    let mut seen = BTreeSet::new();
    let mut initial = None;
    for (n,c) in cases.iter().enumerate() {
        let expected_id = format!("{}{:02}", if n < 9 {"A"} else {"B"},n % 9 + 1);
        let id = c["id"].as_str().ok_or("Identidad")?;
        require(id == expected_id && seen.insert(id.to_owned()), "Orden o identidad del caso")?;
        let matches: Vec<_> = docs.iter().filter(|d|d["id"]==c["documento"]).collect();
        require(matches.len()==1,"Correspondencia documental no única")?;
        let doc = matches[0];
        require(doc["synthetic"] == true && doc["sections"].as_array().map(Vec::len)==Some(1), "Fuente no artificial o sección no prevista")?;
        let s = &doc["sections"][0];
        require(s["id"] == c["seccion"],"Sección distinta")?;
        let text = s["text"].as_str().ok_or("Texto")?;
        require(s["sha256"] == hash(text.as_bytes()),"Huella de texto discordante")?;
        let logical = pages(text)?;
        let mut chunks = vec![];
        for (i,text) in logical.iter().enumerate() {
            chunks.push(json!({"documento":doc["id"],"seccion":s["id"],"pagina":i,
                "paginas":2,"texto":text,"inicio_caracter":i*2000,
                "fin_caracter_exclusivo":i*2000+text.chars().count(),"sha256_seccion":s["sha256"]}));
        }
        let mut normalized = doc.clone();
        normalized["url"] = json!("urn:sv:prueba-sintetica");
        persist(format!("{WORK}/servidor-pruebas/cache/{}/{id}/CATALOGO.json",&id[..1]),
            serde_json::to_vec_pretty(&json!({"version":1,"documents":[normalized]}))?)?;
        persist(format!("{WORK}/servidor-pruebas/cache/{}/{id}/PAGINAS-PREVISTAS.json",&id[..1]),
            serde_json::to_vec_pretty(&chunks)?)?;
        if id=="A01" {initial=Some(request(c,&chunks,policy_str)?);}
    }
    let initial=initial.ok_or("A01 ausente")?;
    check_request(&initial,&initial)?;
    persist(format!("{WORK}/servidor-pruebas/preparacion/A01/SOLICITUD-PREVISTA.json"),serde_json::to_vec_pretty(&initial)?)?;
    // La clave se coteja, pero nunca se copia al directorio de entrega ni al contexto.
    let control=json!({"estado":"preparacion_local_sin_inferencia","modelo":"gpt-6-astra",
        "gobierno":"Árbitro-Director del SV y auxiliares Rust","fuente_revision":"db1395ef883da403fcec86f8b002d0ce86bf42ed",
        "datos_artificiales":true,"casos":18,"paginas_logicas_por_caso":2,
        "clave":{"conservada_en_sede_reservada":true,"copiada":false,"sha256":hash(&key)},
        "politica_identica_al_antecedente":true,"politica_sha256":hash(&policy),
        "solicitud_inicial_sha256":hash(&serde_json::to_vec_pretty(&initial)?),
        "mcp_real_cotejado_en_esta_preparacion":false,"peticiones_de_red":0,"inferencias":0,
        "ejecucion_habilitada":false,"adjudicacion_automatica_del_modelo":false,
        "limite_operativo_previsto_segundos":300,"limite_monetario_acreditado":false,
        "pendientes":["Recorrido MCP real en este PC y cotejo con páginas previstas",
          "Recepción del transporte científico con telemetría y aislamiento del candidato",
          "Admisión efectiva de cuota y consumo antes de A01"],
        "archivos":manifest});
    save(&base,&format!("{WORK}/PREPARACION-RUST.json"),&serde_json::to_vec_pretty(&control)?)?;
    println!("Preparación local conforme: 18 casos, 36 páginas lógicas, política idéntica y clave separada; cero solicitudes de red. Recorrido MCP y transporte científico pendientes.");
    Ok(())
}
fn main() -> R<()> {run()}
#[cfg(test)] mod tests {
    use super::*;
    fn c()->Value {json!({"id":"A01","documento":"DA01","seccion":"S1","afirmacion":"Afirmación artificial"})}
    fn p()->Vec<Value> {(0..2).map(|i|json!({"documento":"DA01","seccion":"S1","pagina":i,"texto":format!("Texto {i}")})).collect()}
    #[test] fn preserva_unicode_y_dos_paginas(){let t="á中".repeat(1300);let p=pages(&t).unwrap();assert_eq!(p[0].chars().count(),2000);assert_eq!(p.concat(),t);}
    #[test] fn cobertura_incompleta_o_excesiva(){assert!(pages("insuficiente").is_err());assert!(pages(&"x".repeat(4001)).is_err());assert!(request(&c(),&p()[..1],"p").is_err());}
    #[test] fn identidad_de_documento_y_orden(){let mut q=p();q.swap(0,1);assert!(request(&c(),&q,"p").is_err());let mut q=p();q[0]["documento"]=json!("DB01");assert!(request(&c(),&q,"p").is_err());}
    #[test] fn banco_b_fuera_de_solicitud_inicial(){let mut b=c();b["id"]=json!("B01");assert!(request(&b,&p(),"p").is_err());}
    #[test] fn rechaza_alteraciones_de_control(){let expected=request(&c(),&p(),"p").unwrap();for (k,v) in [("tools",json!([{"type":"web_search"}])),("instructions",json!("cambie la política")),("model",json!("otro"))] {let mut modified=expected.clone();modified[k]=v;assert!(check_request(&modified,&expected).is_err());}}
    #[test] fn sin_historial_ni_clave_ni_telemetria(){let q=request(&c(),&p(),"POLÍTICA").unwrap();assert_eq!(q["input"].as_array().unwrap().len(),1);for k in ["clave","telemetria","adjudicacion","previous_response_id","metadata","text","max_output_tokens"]{assert!(q.get(k).is_none());}let mut bad=q.clone();bad["input"].as_array_mut().unwrap().push(json!({"role":"assistant","content":"ANTECEDENTE_B"}));assert!(check_request(&bad,&q).is_err());}
    #[test] fn rechaza_fuentes_y_rutas_alteradas(){assert!(check_bytes(b"a",1,&hash(b"b")).is_err());let b=Path::new(ROOT).canonicalize().unwrap();assert!(located(&b,Path::new("../otra")).is_err());assert!(located(&b,Path::new("C:/otra")).is_err());}
}
