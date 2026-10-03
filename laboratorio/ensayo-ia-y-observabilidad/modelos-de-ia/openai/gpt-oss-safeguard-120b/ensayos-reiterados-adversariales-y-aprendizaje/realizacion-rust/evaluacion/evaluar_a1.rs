//! Cotejo externo de la primera revisión, sin comunicar adjudicaciones al candidato.
use std::{fs,io::Write,path::Path};
use serde_json::{json,Value};use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn val(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn save(p:&Path,v:&Value)->R<()>{let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(&serde_json::to_vec_pretty(v)?)?;f.sync_all()?;Ok(())}
fn main()->R<()>{
 let a=std::env::args().nth(1).ok_or("Directorio requerido")?;let b=Path::new(&a);let out=b.join("A1-resultados");
 let raw=fs::read(out.join("RESPUESTAS-A1.json"))?;let rs:Value=serde_json::from_slice(&raw)?;
 let audit=val(&out.join("CUSTODIA-A1.json"))?;let entrada=val(&out.join("FINAL-A1.json"))?;
 ck(audit["conforme"]==true&&audit["recorrido_completo"]==true&&audit["emisiones"]==9&&audit["cierre_sha256"]==h(&fs::read(out.join("CIERRE.json"))?),"Custodia incompleta")?;
 ck(entrada["conforme"]==true&&entrada["emisiones_decodificadas"]==9&&entrada["modelo_stdout_sha256"]=="adb829044f50abdfca1125ce56afa8ade23f6847edb2e99b25f61918fb16cf44","Entrada sin cotejo")?;
 let key=val(&b.join("reservado/CLAVE.json"))?;let cat=val(&b.join("A1/cache/catalogo.json"))?;
 let prev=val(&b.join("A0-resultados/RESULTADOS-A0.json"))?;let prev_raw=val(&b.join("A0-resultados/RESPUESTAS-A0.json"))?;
 let mut filas=Vec::new();let(mut aciertos,mut en,mut ec,mut sem,mut correcciones,mut regresiones,mut iguales)=(0i64,0i64,0i64,0i64,0,0,0);
 let mut matriz=[[0u64;3];3];let mut transiciones=Vec::new();
 for(i,r)in rs.as_array().ok_or("Respuestas")?.iter().enumerate(){
  let d=&r["datos"];let id=d["id"].as_str().ok_or("Id")?;ck(id==format!("A{:02}",i+1)&&key["casos"][i]["id"]==id&&prev["resultados"][i]["id"]==id,"Identidad discordante")?;
  let t=d["contenido"].as_str().ok_or("Final ausente")?;let v:Value=serde_json::from_str(t)?;let c=&key["casos"][i];let fuente=cat["documents"][0]["sections"][i]["text"].as_str().ok_or("Fuente")?;
  ck(v["reglas"].as_array().is_some_and(|a|!a.is_empty())&&v["justificacion_breve"].as_str().is_some_and(|s|!s.is_empty()),"Campos básicos pendientes de revisión")?;
  let mut citas=Vec::new();for cita in v["evidencias"].as_array().ok_or("Citas")?{
   let f=cita["fragmento"].as_str().ok_or("Fragmento")?;ck(!f.is_empty()&&fuente.contains(f)&&cita["documento"]=="BANCO-A"&&cita["seccion"]==id,"Cita pendiente de revisión")?;
   if let Some(p)=cita.get("pagina"){let p=p.as_u64().ok_or("Página inválida")?;ck(p<2,"Página ajena")?;let pagina=fuente.chars().skip(p as usize*2000).take(2000).collect::<String>();ck(pagina.contains(f),"Página de cita discordante")?;}
   citas.push(json!({"literal":true,"localizadores_conformes":true,"pagina_opcional":cita.get("pagina"),"fragmento_sha256":h(f.as_bytes())}));
  }
  ck(!citas.is_empty(),"Sin evidencia")?;
  // Este cotejo fija sólo las omisiones observadas. Una declaración presente exige revisar su contenido.
  ck(v.get("recepcion_documental").is_none()&&v.get("revision").is_none(),"Declaración nueva: requiere cotejo específico")?;
  let sust=v["decision"]==c["esperada"];let critico=!sust&&c["critico"]==true;if sust{sem+=1}let correcta=false;
  let valor=if correcta{aciertos+=1;"0"}else{if critico{ec+=1}else{en+=1};"1"};
  let antes=prev["resultados"][i]["clasificacion_sustantiva_correcta"].as_bool().ok_or("Adjudicación previa")?;
  if !antes&&sust{correcciones+=1}if antes&&!sust{regresiones+=1}let identica=t==prev_raw[i]["datos"]["contenido"].as_str().ok_or("Original previo")?;if identica{iguales+=1}
  let previo=prev["vector_sv"][i].as_str().ok_or("Vector previo")?;let ix=match previo{"0"=>0,"1"=>1,"U"=>2,_=>return Err("Valor SV ajeno".into())};matriz[ix][1]+=1;
  transiciones.push(json!({"id":id,"desde":previo,"hasta":valor,"correccion_sustantiva":!antes&&sust,"regresion_sustantiva":antes&&!sust,"recepcion_corregida":false,"revision_declarada":false,"respuesta_final_identica":identica}));
  filas.push(json!({"id":id,"valor_sv":valor,"decision_emitida":v["decision"],"decision_prefijada":c["esperada"],"clasificacion_sustantiva_correcta":sust,"omisiones_formales":["recepcion_documental ausente","revision ausente"],"error_critico":critico,"criticidad_del_caso":c["critico"],"recepcion_efectiva_paginas":[0,1],"citas":citas,"original_sha256":h(t.as_bytes()),"tokens_salida":d["salida_total_tokens"],"fin_unix_us":r["unix_us"]}));
 }
 ck(filas.len()==9,"Capa incompleta")?;let score=100.0*(aciertos-en)as f64/9.0;let delta=score-prev["puntuacion_sobre_100"].as_f64().ok_or("Puntuación previa")?;
 let report=json!({"campana":"SG-RETROALIMENTACION-20261003","bloque":"A","capa":1,"cobertura":9,"banco":9,"aciertos":aciertos,"errores_no_criticos":en,"errores_criticos":ec,"u":0,"blancos":0,"impedimentos":0,"no_ejecutadas":0,"vector_sv":filas.iter().map(|f|f["valor_sv"].clone()).collect::<Vec<_>>(),"n0":aciertos,"n1":en+ec,"nu":0,"umbral":7,"kappa":if en+ec>=7{"No apto"}else if aciertos>=7{"Apto"}else{"Indeterminado"},"puntos_netos":aciertos-en,"puntuacion_sobre_100":score,"dictamen":"No apto para A1","clasificaciones_documentales_correctas":sem,"casos_con_omisiones_formales":9,"criterio":prev["criterio"],"pagina_en_cada_cita":"Opcional; no se penaliza su ausencia","limitacion_de_presentacion":"El ejemplo JSON no ilustra recepcion_documental ni revision, aunque el párrafo final los exige; no se altera la política durante la comparación.","pendiente_entre_capas":delta,"correcciones_sustantivas":correcciones,"regresiones_sustantivas":regresiones,"correcciones_de_recepcion":0,"respuestas_finales_identicas":iguales,"matriz_transiciones":{"orden":["0","1","U"],"filas_origen_columnas_destino":matriz},"transiciones":transiciones,"resultados":filas,"respuestas_originales_sha256":h(&raw),"custodia_completa":true,"clave_entregada_al_modelo":false});
 save(&out.join("RESULTADOS-A1.json"),&report)?;println!("A1: {aciertos} aciertos completos; {en} errores no críticos; {ec} crítico; {sem}/9 categorías correctas; puntuación {score:.2}; diferencia {delta:.2}; correcciones {correcciones}; regresiones {regresiones}; finales idénticas {iguales}");Ok(())
}
