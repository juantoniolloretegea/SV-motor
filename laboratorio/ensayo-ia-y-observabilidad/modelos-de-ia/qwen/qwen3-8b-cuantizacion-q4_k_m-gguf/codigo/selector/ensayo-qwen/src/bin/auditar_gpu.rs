//! Análisis posterior de trazas conservadas, sin inicialización de dispositivos.
use flate2::read::GzDecoder;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::{BTreeMap, BTreeSet}, error::Error, fs, io::Read, path::{Component, Path}};
type R<T> = Result<T, Box<dyn Error>>;
fn main() -> R<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 { return Err("Uso: auditar_gpu archivo sha256 informe".into()); }
    let raw = fs::read(&a[1])?;
    if format!("{:x}", Sha256::digest(&raw)) != a[2] { return Err("Identidad del archivo incorrecta".into()); }
    let mut tar = tar::Archive::new(GzDecoder::new(raw.as_slice()));
    let mut files = BTreeMap::new();
    for entry in tar.entries()? {
        let mut e = entry?; let p = e.path()?.into_owned();
        if !p.components().all(|c| matches!(c,Component::Normal(_))) || !p.starts_with("resultado") { return Err("Ruta no admitida".into()); }
        if e.header().entry_type().is_dir() { continue; }
        if !e.header().entry_type().is_file() || e.size() > 128*1024*1024 { return Err("Entrada no admitida".into()); }
        let n = p.strip_prefix("resultado")?.to_string_lossy().replace('\\',"/");
        let mut b=Vec::new(); e.read_to_end(&mut b)?;
        if files.insert(n,b).is_some() { return Err("Entrada duplicada".into()); }
    }
    let mut entries=BTreeMap::<String,BTreeSet<String>>::new();
    for (n,b) in &files { if n.ends_with(".ptx") {
        let tokens:Vec<_>=std::str::from_utf8(b)?.split_whitespace().collect();
        for w in tokens.windows(2) { if w[0]==".entry" { entries.entry(w[1].split('(').next().unwrap().into()).or_default().insert(n.clone()); } }
    }}
    let trace:Vec<Value>=std::str::from_utf8(files.get("TRAZA-GPU.jsonl").ok_or("Traza ausente")?)?.lines().map(serde_json::from_str).collect::<Result<_,_>>()?;
    let mut functions=BTreeMap::<u64,String>::new(); let mut launches=BTreeMap::<String,u64>::new();
    let mut phases=BTreeMap::<String,u64>::new(); let mut errors=Vec::new(); let mut denied=Vec::new();
    let mut transfers=BTreeMap::<String,Value>::new(); let mut count=0;
    for t in &trace {
        let d=&t["datos"];
        match t["tipo"].as_str().unwrap_or("") {
            "funcion"=>{ functions.insert(d["funcion"].as_u64().ok_or("Identidad de función ausente")?,d["nombre"].as_str().ok_or("Nombre de función ausente")?.into()); },
            "lanzamiento"=>{
                count+=1;
                let f=d["funcion"].as_u64().ok_or("Lanzamiento sin identidad")?;
                let name=functions.get(&f).cloned().unwrap_or_else(||"NO_IDENTIFICADA".into());
                if name=="NO_IDENTIFICADA" || !entries.contains_key(&name) { errors.push(json!({"motivo":"Función sin PTX identificable","registro":t})); }
                if d["resultado"]!=0 { errors.push(json!({"motivo":"Retorno CUDA con error","registro":t})); }
                *launches.entry(name).or_default()+=1;
                *phases.entry(t["fase"].as_str().unwrap_or("?").into()).or_default()+=1;
            },
            "recorrido_rechazado"=>denied.push(t.clone()),
            "transferencia"=>{
                let key=format!("{} / {}",t["fase"].as_str().unwrap_or("?"),d["operacion"].as_str().unwrap_or("?"));
                let v=transfers.entry(key).or_insert(json!({"llamadas":0,"bytes":0,"duracion_host_ns":0,"errores":0}));
                for (k,inc) in [("llamadas",1),("bytes",d["bytes"].as_u64().unwrap_or(0)),("duracion_host_ns",d["llamada_host_ns"].as_u64().unwrap_or(0)),("errores",u64::from(d["resultado"]!=0))] { v[k]=json!(v[k].as_u64().unwrap()+inc); }
            }, _=>{}
        }
    }
    let mut correspondence=Vec::new();
    for (n,c) in &launches { correspondence.push(json!({"funcion":n,"lanzamientos":c,"ptx":entries.get(n)})); }
    let parse=|n:&str| -> R<Value> { match files.get(n) { Some(b)=>Ok(serde_json::from_slice(b)?),None=>Ok(Value::Null) } };
    let out=json!({"alcance":"Auditoría posterior; correspondencia por nombre de función con PTX conservado, no inspección de código máquina del controlador","archivo_sha256":a[2],"lanzamientos":count,"lanzamientos_por_fase":phases,"funciones":correspondence,"incidencias_correspondencia_o_retorno":errors,"recorridos_rechazados":denied,"transferencias_por_fase":transfers,"resultado":parse("RESULTADO.json")?,"gpu_carga":parse("GPU-CARGA.json")?,"gpu":parse("GPU.json")?,"cpu":parse("CPU.json")?,"supervisor":parse("SUPERVISOR.json")?});
    if Path::new(&a[3]).exists(){return Err("Informe ya existente".into());}
    fs::write(&a[3],serde_json::to_vec_pretty(&out)?)?;
    println!("{} lanzamientos; {} funciones; {} incidencias de correspondencia o retorno; {} rechazos",count,launches.len(),errors.len(),denied.len());
    Ok(())
}
