//! Fijación y contraste instrumentales. No se incorpora al recorrido del candidato.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{error::Error,fs,path::Path,time::{SystemTime,UNIX_EPOCH}};
fn hash(b:&[u8])->String {format!("{:x}",Sha256::digest(b))}
fn read(p:&Path)->Result<Value,Box<dyn Error>> {Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn write(p:&Path,v:&Value)->Result<(),Box<dyn Error>>{
    if p.exists(){return Err("Se preserva la salida anterior".into())} fs::write(p,serde_json::to_vec_pretty(v)?)?;Ok(())
}
fn content(v:&Value)->Value{
    let mut c=v.clone();
    for k in ["inicio_unix_ms","duracion_microsegundos"]{c.as_object_mut().unwrap().remove(k);}
    for r in c["casos"].as_array_mut().unwrap(){r.as_object_mut().unwrap().remove("duracion_microsegundos");} c
}
fn main()->Result<(),Box<dyn Error>>{
    let a:Vec<String>=std::env::args().collect();
    if a.len()!=3 {return Err("Uso: cotejar fijar|valorar DIRECTORIO".into())}
    let root=Path::new(&a[2]);
    if a[1]=="fijar" {
        let mut e=read(&root.join("ENTRADAS-PREVIAS.json"))?;
        let mut records=Vec::new();
        let refs=read(&root.join("REFERENCIAS.json"))?;
        let ids:std::collections::BTreeSet<_>=refs["casos"].as_array().unwrap().iter().map(|c|c["id"].as_str().unwrap()).collect();
        if ids.len()!=24 {return Err("Referencias incompletas o repetidas".into())}
        for case in e["casos"].as_array_mut().unwrap(){
            if !ids.contains(case["id"].as_str().unwrap()){return Err("Caso sin referencia".into())}
            for f in case["archivos"].as_array_mut().unwrap(){
                let name=f["ruta"].as_str().unwrap().to_string();let b=fs::read(root.join(&name))?;
                f["sha256"]=json!(hash(&b));f["bytes"]=json!(b.len());
                records.push(json!({"ruta":name,"sha256":hash(&b),"bytes":b.len()}));
            }
        }
        let local=read(&root.join("ORIGINALES-LOCALES.json"))?;let mut sources=Vec::new();
        for f in local["fuentes"].as_array().unwrap(){
            let b=fs::read(f["ruta"].as_str().unwrap())?;let actual=hash(&b);
            if f["sha256"].as_str()!=Some(&actual){return Err(format!("Original distinto: {}",f["id"]).into())}
            sources.push(json!({"id":f["id"],"sha256":actual,"bytes":b.len(),"conforme":true}));
        }
        write(&root.join("ENTRADAS.json"),&e)?;
        for p in ["ENTRADAS.json","REFERENCIAS.json","PROTOCOLO.md"]{
            let b=fs::read(root.join(p))?;records.push(json!({"ruta":p,"sha256":hash(&b),"bytes":b.len()}));
        }
        let manifest=json!({"elemento":"Buscador-Semántico - Diferencial","fijacion_unix_ms":SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),"archivos":records,"originales_cotejados":sources,"inferencia":false});
        write(&root.join("FIJACION-RUST.json"),&manifest)?;
        println!("Fijación anterior a ejecución: {} archivos y {} originales",records.len(),sources.len());
    } else if a[1]=="valorar" {
        let fix=read(&root.join("FIJACION-RUST.json"))?;
        for f in fix["archivos"].as_array().unwrap(){let b=fs::read(root.join(f["ruta"].as_str().unwrap()))?;
            if f["sha256"].as_str()!=Some(&hash(&b)){return Err("Material alterado después de fijación".into())}}
        let r1=read(&root.join("EJECUCION-01.json"))?;let r2=read(&root.join("EJECUCION-02.json"))?;
        let refs=read(&root.join("REFERENCIAS.json"))?;
        let mut rows=Vec::new(); let(mut tp,mut fn_,mut fp,mut tn,mut missing_alerts,mut unknown)=(0,0,0,0,0,0);
        for r in refs["casos"].as_array().unwrap(){
            let found=r1["casos"].as_array().unwrap().iter().find(|c|c["id"]==r["id"]).ok_or("Falta resultado")?;
            let alert=found["alerta"].as_bool().unwrap();
            match r["referencia"].as_str().unwrap(){
                "incompatibilidad"=>{if alert {tp+=1}else{fn_+=1}},
                "compatibilidad"=>{if alert {fp+=1}else{tn+=1}},
                "contexto_insuficiente"=>{unknown+=1;if alert{missing_alerts+=1}},
                _=>return Err("Referencia no reconocida".into())}
            rows.push(json!({"id":r["id"],"dominio":r["dominio"],"referencia":r["referencia"],"alerta":alert,
                "reglas":found["hallazgos"].as_array().unwrap().iter().map(|d|d["regla"].clone()).collect::<Vec<_>>(),
                "requisitos_reconocidos":found["archivos"].as_array().unwrap().iter().map(|f|f["requisitos_reconocidos"].as_u64().unwrap()).sum::<u64>()}));
        }
        let report=json!({"elemento":"Buscador-Semántico - Diferencial","metrica":"alerta por caso; no equivale a validacion del fundamento",
            "alertas_en_incompatibilidades":tp,"incompatibilidades_omitidas":fn_,"incompatibilidades_total":tp+fn_,
            "alertas_en_compatibles":fp,"compatibles_sin_alerta":tn,"compatibles_total":fp+tn,
            "contexto_insuficiente_total":unknown,"alertas_sin_contexto_suficiente":missing_alerts,
            "repeticion_sustantiva_identica":content(&r1)==content(&r2),"referencias_y_entradas_intactas":true,
            "casos":rows,"recepcion_cientifica_independiente":false});
        write(&root.join("COTEJO-RUST.json"),&report)?;println!("{}",serde_json::to_string(&report)?);
    } else{return Err("Operación desconocida".into())}Ok(())
}
