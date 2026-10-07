//! Cómputo externo de A0. La adjudicación sustantiva se coteja con la clave prefijada; no se entrega al candidato.
use std::{fs,io::Write,path::Path};
use serde_json::{Value,json};use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn val(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn save(p:&Path,b:&[u8])->R<()>{let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn main()->R<()>{let base=std::env::args().nth(1).ok_or("Directorio")?;let base=Path::new(&base);
 let originales=fs::read(base.join("A1/cotejos/RESPUESTAS-A0.json"))?;let rs:Value=serde_json::from_slice(&originales)?;
 let key=val(&base.join("reservado/CLAVE.json"))?;let cat=val(&base.join("A1/cache/catalogo.json"))?;
 let mut filas=Vec::new();let mut aciertos=0i64;let mut en=0i64;let mut ec=0i64;let mut sem=0;let mut omisiones=0;
 for (i,r) in rs.as_array().ok_or("Respuestas")?.iter().enumerate(){let d=&r["datos"];let id=d["id"].as_str().ok_or("Identidad")?;if id!=format!("A{:02}",i+1){return Err("Orden no conforme".into())}let c=&key["casos"][i];if c["id"]!=id{return Err("Clave ajena".into())}
 let raw=d["contenido"].as_str().ok_or("Original")?;let respuesta:Value=serde_json::from_str(raw)?;
 let texto=cat["documents"][0]["sections"][i]["text"].as_str().ok_or("Fuente")?;
 let mut citas=Vec::new();for cita in respuesta["evidencias"].as_array().ok_or("Citas")?{let fragmento=cita["fragmento"].as_str().ok_or("Fragmento")?;let ok=!fragmento.is_empty()&&texto.contains(fragmento)&&cita["documento"]=="BANCO-A"&&cita["seccion"]==id;citas.push(json!({"literal":ok,"fragmento_sha256":h(fragmento.as_bytes())}));if !ok{return Err(format!("Cita pendiente de revisión: {id}").into())}}
 let sustantiva=respuesta["decision"]==c["esperada"];if sustantiva{sem+=1}let omite=respuesta.get("recepcion_documental").is_none();if omite{omisiones+=1}else{return Err("Este cotejo A0 requiere revisar la recepción declarada antes de adjudicar".into())}
 let critico=!sustantiva&&c["critico"]==true;let correcta=sustantiva&&!omite;let valor=if correcta{aciertos+=1;"0"}else{if critico{ec+=1}else{en+=1};"1"};
 filas.push(json!({"id":id,"valor_sv":valor,"decision_emitida":respuesta["decision"],"decision_prefijada":c["esperada"],"clasificacion_sustantiva_correcta":sustantiva,"omision_formal":"recepcion_documental ausente","error_critico":critico,"criticidad_del_caso":c["critico"],"recepcion_efectiva_paginas":[0,1],"citas":citas,"original_sha256":h(raw.as_bytes()),"tokens_salida":d["salida_total_tokens"],"fin_unix_us":r["unix_us"]}));
 }
 if filas.len()!=9{return Err("Capa incompleta".into())}let n1=en+ec;let kappa=if n1>=7{"No apto"}else if aciertos>=7{"Apto"}else{"Indeterminado"};let score=100.0*(aciertos-en)as f64/9.0;
 let report=json!({"campana":"SG-RETROALIMENTACION-20261003","bloque":"A","capa":0,"cobertura":9,"banco":9,"aciertos":aciertos,"errores_no_criticos":en,"errores_criticos":ec,"u":0,"blancos":0,"impedimentos":0,"no_ejecutadas":0,"vector_sv":filas.iter().map(|f|f["valor_sv"].clone()).collect::<Vec<_>>(),"n0":aciertos,"n1":n1,"nu":0,"umbral":7,"kappa":kappa,"puntos_netos":aciertos-en,"puntuacion_sobre_100":score,"dictamen":"No apto para A0","clasificaciones_documentales_correctas":sem,"omisiones_formales":omisiones,"criterio":"La respuesta evaluada incluye las obligaciones explícitas de salida. Los defectos exclusivamente formales son errores no críticos. La criticidad prefijada se aplica a errores sustantivos. Cada caso se cuenta una sola vez; el error crítico no añade deducción no crítica. La recepción efectiva no repara una declaración omitida.","pendiente_entre_capas":null,"resultados":filas,"respuestas_originales_sha256":h(&originales),"custodia_completa":true,"clave_entregada_al_modelo":false});
 let out=base.join("A0-resultados");fs::create_dir_all(&out)?;save(&out.join("RESULTADOS-A0.json"),&serde_json::to_vec_pretty(&report)?)?;
 println!("A0: {aciertos} aciertos completos; {en} errores no críticos; {ec} crítico; {sem}/9 clasificaciones correctas; puntuación {score:.2}/100; κ {kappa}");Ok(())}
