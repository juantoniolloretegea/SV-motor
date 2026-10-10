#![forbid(unsafe_code)]
use sv_claude_kaggle::{self as sv,need,parse,sha,R};
use serde_json::json;
use std::{fs,path::Path,io::Write};
fn run()->R<()>{
    let a:Vec<_>=std::env::args().collect();need(a.len()==6,"Uso: recibir-cyb09 CLAVE JUICIO ORIGINALES CAMPAÑA SALIDA")?;
    let kb=fs::read(&a[1]).map_err(|e|e.to_string())?;let k=parse(&kb)?;
    need(sha(&kb)=="e961d2a9c1a1cef0a816954b1ff3deb06b56afb74e71ed9b92b9367299680dda"&&k["b"]==3&&k["n"]==9&&k["umbral_auxiliar"]==7,"Clave distinta")?;
    let jb=fs::read(&a[2]).map_err(|e|e.to_string())?;let j=parse(&jb)?;
    need(j["clave_sha256"]==sha(&kb)&&j["autoridad"]=="dictamen asistido exterior al candidato"&&j["recepcion_independiente"]=="pendiente","Autoridad o clave discordantes")?;
    let rows=j["adjudicaciones"].as_array().ok_or("Juicios ausentes")?;need(rows.len()==27,"Cobertura incompleta")?;
    let mut stages=Vec::new();
    for stage in 0..3{
        let mut vector=Vec::new();let mut correct=0;let mut ids=std::collections::BTreeSet::new();
        for key in k["preguntas"].as_array().ok_or("Criterios")?{
            let id=key["id"].as_str().ok_or("Identidad")?;
            let found:Vec<_>=rows.iter().filter(|v|v["caso"]==id&&v["etapa"]==stage).collect();need(found.len()==1&&ids.insert(id),"Juicio ausente o duplicado")?;
            let r=found[0];need(r["critica"]==key["critica"]&&r["criterio"]==key["esperado"]&&r["motivo"].as_str().is_some_and(|s|!s.is_empty()),"Criterio alterado")?;
            let old=id=="C01"||(id=="C02"&&stage==0);
            let d=if old{Path::new(&a[3]).join(format!("campana/cyb16/{id}-R{stage}"))}else{Path::new(&a[4]).join(format!("cyb09/{id}-R{stage}"))};
            let text=fs::read(d.join("FINAL.txt")).map_err(|e|e.to_string())?;need(r["final_sha256"]==sha(&text),"Juicio de otra respuesta")?;
            let mark=r["valor"].as_str().ok_or("Marca")?;need(["0","1","U"].contains(&mark),"Marca fuera de escala")?;
            if mark=="0"{correct+=1;}vector.push(mark.to_owned());
        }
        let critical=vector.iter().all(|v|v=="0");
        stages.push(json!({"etapa":stage,"vector":vector,"correctas":correct,"total":9,"criticos_correctos":critical,"umbral_auxiliar":7,"admisible_segun_clave":correct>=7&&critical}));
    }
    let report=json!({"conforme":true,"b":3,"n":9,"alfabeto":["0","1","U"],"orden":k["preguntas"].as_array().unwrap().iter().map(|v|v["id"].clone()).collect::<Vec<_>>(),"juicio_sha256":sha(&jb),"clave_sha256":sha(&kb),"resultados":stages,"etapa_final":2,"mejor_etapa_seleccionada":false,"alcance":"Identidad, cobertura, correspondencia y cálculo Rust; el juicio semántico es exterior asistido","recepcion_independiente":"pendiente","licencia":sv::LICENCIA});
    let mut f=fs::OpenOptions::new().create_new(true).write(true).open(&a[5]).map_err(|e|e.to_string())?;f.write_all(&serde_json::to_vec_pretty(&report).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    println!("Adjudicación recibida: 27 juicios y tres vectores completos; recepción independiente pendiente");Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("NO CONFORME: {e}");std::process::exit(1)}}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
