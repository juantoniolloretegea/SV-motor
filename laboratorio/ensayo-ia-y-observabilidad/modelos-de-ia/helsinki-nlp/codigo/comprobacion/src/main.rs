//! Cotejo documental; no ejecuta inferencias ni altera archivos.
#![forbid(unsafe_code)]
use std::{collections::BTreeSet,error::Error,fs,path::{Component,Path}};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn ruta(base:&Path,rel:&str)->Result<std::path::PathBuf,Box<dyn Error>>{
    if rel.is_empty()||!Path::new(rel).components().all(|c|matches!(c,Component::Normal(_))){return Err("Ruta no admitida".into())}
    let p=base.join(rel).canonicalize()?;
    if !p.starts_with(base.canonicalize()?){return Err("Ruta fuera del expediente".into())}
    Ok(p)
}
fn lineas(p:&Path)->Result<Vec<Value>,Box<dyn Error>>{
    fs::read_to_string(p)?.lines().map(|l|Ok(serde_json::from_str(l)?)).collect()
}
fn main()->Result<(),Box<dyn Error>>{
    let a:Vec<String>=std::env::args().collect();
    if !(a.len()==2||a.len()==3){return Err("Uso: sv-cotejo-helsinki <expediente> [otra recuperacion]".into())}
    let b=Path::new(&a[1]);
    let raw=fs::read(b.join("MANIFIESTO.json"))?;
    let m:Value=serde_json::from_slice(&raw)?;
    let mut nombres=BTreeSet::new();let mut bytes=0u64;
    for f in m["archivos"].as_array().ok_or("Manifiesto incompleto")?{
        let n=f["ruta"].as_str().ok_or("Ruta ausente")?;
        if !nombres.insert(n){return Err("Archivo duplicado".into())}
        let datos=fs::read(ruta(b,n)?)?;
        if f["bytes"].as_u64()!=Some(datos.len() as u64)||f["sha256"].as_str()!=Some(hash(&datos).as_str()){return Err(format!("Identidad discordante: {n}").into())}
        if a.len()==3&&fs::read(ruta(Path::new(&a[2]),n)?)?!=datos{return Err(format!("Recuperacion discordante: {n}").into())}
        bytes+=datos.len() as u64;
    }
    if a.len()==3&&fs::read(Path::new(&a[2]).join("MANIFIESTO.json"))?!=raw{return Err("Manifiestos distintos".into())}
    let entradas=lineas(&b.join("evidencias/ENTRADAS-CPU-ANTERIORES.jsonl"))?;
    let salidas=lineas(&b.join("evidencias/SALIDAS-CPU-ANTERIORES.jsonl"))?;
    if entradas.len()!=13||salidas.len()!=entradas.len(){return Err("Cobertura documental discordante".into())}
    let mut ids=BTreeSet::new();let(mut te,mut ts,mut ms)=(0u64,0u64,0u64);
    for(e,s)in entradas.iter().zip(&salidas){
        let id=e["id"].as_str().ok_or("Identidad ausente")?;
        if !ids.insert(id)||e["id"]!=s["id"]{return Err("Correspondencia discordante".into())}
        for(clave,texto)in [("entrada_sha256",&e["texto"]),("salida_sha256",&s["texto"])]{
            if s[clave].as_str()!=Some(hash(texto.as_str().ok_or("Texto ausente")?.as_bytes()).as_str()){return Err(format!("Huella discordante: {id}").into())}
        }
        te+=s["tokens_entrada"].as_u64().ok_or("Tokens ausentes")?;
        ts+=s["tokens_salida"].as_u64().ok_or("Tokens ausentes")?;
        ms+=s["duracion_ms"].as_u64().ok_or("Duracion ausente")?;
    }
    if (te,ts,ms)!=(239,270,10127){return Err("Mediciones antecedentes discordantes".into())}
    println!("{}",serde_json::to_string_pretty(&json!({"conforme":true,"archivos":nombres.len(),"bytes":bytes,"manifiesto_sha256":hash(&raw),"pares_anteriores":13,"tokens_entrada":te,"tokens_salida":ts,"generacion_anterior_ms":ms,"recuperacion_comparada":a.len()==3,"inferencias_nuevas":0,"alcance":"integridad documental; no acredita ejecucion TPU"}))?);
    Ok(())
}

