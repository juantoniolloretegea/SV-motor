//! Delimita la réplica autorizada a MD01 sin modificar sus fuentes históricas.
use crate::{catalogo::{path,put,save},suministro_pdf::*};
use serde_json::{json,Value};
use std::fs;
pub fn sembrar()->R<Value>{
    let dest=super::manual::base();need(!dest.exists(),"Réplica ya preparada")?;
    let source=path("ejecucion/astra-manual-20261008");
    let bank_raw=fs::read(source.join("fuentes/BANCO.json")).map_err(|e|e.to_string())?;
    let key_raw=fs::read(source.join("reservado/CLAVE.json")).map_err(|e|e.to_string())?;
    need(sha(&bank_raw)=="f7da23f424186437c16f74d53416c997df02f974c782f7f69728e8952e01d843"&&sha(&key_raw)=="e8f9e1cf7c65856106dab42b07ccc3348de5862ea877979d02dcdd1d7f3e81c7","Origen distinto")?;
    let mut bank=parse(&bank_raw)?;let mut key=parse(&key_raw)?;
    let question=bank["preguntas"][0].clone();let criterion=key["preguntas"][0].clone();
    need(question["id"]=="MD01"&&criterion["id"]=="MD01","Caso distinto")?;
    bank["preguntas"]=json!([question]);bank["version"]=json!("1.1.0-md01-diagnostica");
    key["preguntas"]=json!([criterion]);key["version"]=json!("1.1.0-md01-diagnostica");key["umbral_correctas"]=json!(1);
    key["requisitos_transversales_prefijados"]=json!({"contenido":"Distinción documental prefijada para MD01, sin modificarla.","estructura":"JSON, caso y etapa conformes; citas y localizadores cotejados.","revision":"Fiel a las respuestas y a las instrucciones históricas completas. Una imputación falsa de incumplimiento se registra como error de revisión, separado del contenido principal.","resultado":"Diagnóstico de MD01 por etapa, sin construir vector de nueve ni recalificar el banco original. Conformidad sólo si contenido, estructura y revisión resultan conformes. U requiere justificación y no equivale a conformidad."});
    for dir in ["fuentes/preparado","reservado"]{fs::create_dir_all(dest.join(dir)).map_err(|e|e.to_string())?;}
    for name in ["CATALOGO.json","FUENTES.json"]{let b=fs::read(source.join(format!("fuentes/preparado/{name}"))).map_err(|e|e.to_string())?;put(&dest.join(format!("fuentes/preparado/{name}")),&b)?;}
    save(&dest.join("fuentes/BANCO.json"),&bank)?;save(&dest.join("reservado/CLAVE.json"),&key)?;
    let admission=fs::read(path("seguimiento/revision-md01-20261008/ADMISION-REPLICA.md")).map_err(|e|e.to_string())?;
    put(&dest.join("ADMISION.md"),&admission)?;
    let out=json!({"banco_sha256":sha(&fs::read(dest.join("fuentes/BANCO.json")).map_err(|e|e.to_string())?),"clave_sha256":sha(&fs::read(dest.join("reservado/CLAVE.json")).map_err(|e|e.to_string())?),"casos":["MD01"],"etapas":3,"inferencia":false});
    save(&dest.join("ORIGEN.json"),&out)?;Ok(out)
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
