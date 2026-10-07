//! Adjudicación de las nueve emisiones conservadas; no convierte la interrupción técnica en cierre normal.
use std::{fs,io::Write,path::Path};
use serde_json::{json,Value};use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn val(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn main()->R<()>{
 let arg=std::env::args().nth(1).ok_or("Directorio")?;let b=Path::new(&arg);let out=b.join("A3-incidente-20261004");
 let raw=fs::read(out.join("FINALES-A3.json"))?;
 ck(h(&raw)=="5a1b4d1d375e2728b59bc6ac928b40b7f8da7ae535e1e3315bdc8a60bc008a12","Extracción distinta de la cotejada en el servidor")?;
 let rs:Value=serde_json::from_slice(&raw)?;let audit=val(&out.join("COTEJO-PUNTO-A09.json"))?;let entrada=val(&out.join("COTEJO-ENTRADAS-A09.json"))?;
 ck(audit["cotejo"]["conforme"]==true&&audit["cotejo"]["emisiones"]==9&&audit["cotejo"]["emision_parcial"]==0&&audit["cotejo"]["recorrido_completo"]==false&&audit["cierre_normal_acreditado"]==false,"Cotejo del punto incompleto")?;
 ck(audit["cotejo"]["sello_sha256"]=="c6ef379cfeb861e2d9caf8fba624410506de10e5443cfd864e28de4131430c72"&&entrada["conforme"]==true&&entrada["emisiones_decodificadas"]==9&&entrada["modelo_stdout_sha256"]=="b91cc65dc73da5740cc6ab84e3f30d1f41a6d22080f061fb6882571e77ef8259","Entradas sin fijación concordante")?;
 let key=val(&b.join("reservado/CLAVE.json"))?;let cat=val(&b.join("A3/cache/catalogo.json"))?;
 let prev=val(&b.join("A2-resultados/RESULTADOS-A2.json"))?;let prev_raw=val(&b.join("A2-resultados/RESPUESTAS-A2.json"))?;
 let mut filas=Vec::new();let(mut en,mut ec,mut sem,mut correcciones,mut regresiones,mut iguales)=(0i64,0i64,0i64,0,0,0);let mut matriz=[[0u64;3];3];let mut transiciones=Vec::new();
 for(i,r)in rs.as_array().ok_or("Respuestas")?.iter().enumerate(){
  let d=&r["datos"];let id=d["id"].as_str().ok_or("Id")?;ck(id==format!("A{:02}",i+1)&&key["casos"][i]["id"]==id&&prev["resultados"][i]["id"]==id,"Identidad discordante")?;
  let t=d["contenido"].as_str().ok_or("Final ausente")?;let v:Value=serde_json::from_str(t)?;let c=&key["casos"][i];let fuente=cat["documents"][0]["sections"][i]["text"].as_str().ok_or("Fuente")?;
  ck(v["reglas"].as_array().is_some_and(|a|!a.is_empty())&&v["justificacion_breve"].as_str().is_some_and(|s|!s.is_empty()),"Campos básicos pendientes")?;
  let mut citas=Vec::new();for cita in v["evidencias"].as_array().ok_or("Citas")?{
   let f=cita["fragmento"].as_str().ok_or("Fragmento")?;ck(!f.is_empty()&&fuente.contains(f)&&cita["documento"]=="BANCO-A"&&cita["seccion"]==id,"Cita discordante")?;
   if let Some(p)=cita.get("pagina"){let p=p.as_u64().ok_or("Página")?;ck(p<2,"Página ajena")?;let pagina=fuente.chars().skip(p as usize*2000).take(2000).collect::<String>();ck(pagina.contains(f),"Página discordante")?;}
   citas.push(json!({"literal":true,"localizadores_conformes":true,"pagina_opcional":cita.get("pagina"),"fragmento_sha256":h(f.as_bytes())}));
  }
  ck(!citas.is_empty(),"Sin evidencia")?;
  ck(v.get("recepcion_documental").is_none()&&v.get("revision").is_none(),"Declaración presente: requiere evaluación específica")?;
  let sust=v["decision"]==c["esperada"];let critico=!sust&&c["critico"]==true;if sust{sem+=1}if critico{ec+=1}else{en+=1}
  let antes=prev["resultados"][i]["clasificacion_sustantiva_correcta"].as_bool().ok_or("Previa")?;
  if !antes&&sust{correcciones+=1}if antes&&!sust{regresiones+=1}let identica=t==prev_raw[i]["datos"]["contenido"].as_str().ok_or("Original previo")?;if identica{iguales+=1}
  let previo=prev["vector_sv"][i].as_str().ok_or("Vector")?;let ix=match previo{"0"=>0,"1"=>1,"U"=>2,_=>return Err("Valor ajeno".into())};matriz[ix][1]+=1;
  transiciones.push(json!({"id":id,"desde":previo,"hasta":"1","correccion_sustantiva":!antes&&sust,"regresion_sustantiva":antes&&!sust,"recepcion_corregida":false,"revision_corregida":false,"respuesta_final_identica":identica}));
  filas.push(json!({"id":id,"valor_sv":"1","decision_emitida":v["decision"],"decision_prefijada":c["esperada"],"clasificacion_sustantiva_correcta":sust,"omisiones_formales":["recepcion_documental ausente","revision ausente"],"error_critico":critico,"citas":citas,"original_sha256":h(t.as_bytes()),"tokens_salida":d["salida_total_tokens"],"fin_unix_us":r["unix_us"]}));
 }
 ck(filas.len()==9,"Nueve respuestas requeridas")?;let score=-100.0*en as f64/9.0;let delta=score-prev["puntuacion_sobre_100"].as_f64().ok_or("Puntuación previa")?;
 let report=json!({"campana":"SG-RETROALIMENTACION-20261003","bloque":"A","capa":3,"cobertura":9,"aciertos":0,"errores_no_criticos":en,"errores_criticos":ec,"u":0,"vector_sv":["1","1","1","1","1","1","1","1","1"],"n0":0,"n1":en+ec,"nu":0,"umbral":7,"kappa":"No apto","puntuacion_sobre_100":score,"dictamen":"No apto en las respuestas producidas de A3; cierre de ejecución interrumpido por incidente instrumental","clasificaciones_documentales_correctas":sem,"casos_con_omisiones_formales":9,"criterio":prev["criterio"],"pendiente_entre_capas":delta,"correcciones_sustantivas":correcciones,"regresiones_sustantivas":regresiones,"correcciones_formales":0,"respuestas_finales_identicas":iguales,"matriz_transiciones":{"orden":["0","1","U"],"filas_origen_columnas_destino":matriz},"transiciones":transiciones,"resultados":filas,"respuestas_originales_sha256":h(&raw),"custodia_nueve_emisiones_cotejada":true,"cierre_normal_acreditado":false,"incidente":"OOM del custodio después de emitirse A09; no se atribuye al modelo ni se convierte en U","reservas":["Ejemplo JSON incompleto en la política","Distinción semántica del enunciado A08","Autocorrección contextual sin entrenamiento","Integridad no equivale a validación numérica independiente","A2 a A3 combina nueva revisión y cambio de medium a high"],"clave_entregada_al_modelo":false,"examen_habilitado":false,"conservacion_final_github":"Pospuesta hasta terminar las pruebas por instrucción humana"});
 let mut f=fs::OpenOptions::new().write(true).create_new(true).open(out.join("RESULTADOS-A3.json"))?;f.write_all(&serde_json::to_vec_pretty(&report)?)?;f.sync_all()?;
 println!("A3: {sem}/9 categorías correctas; {en} errores formales no críticos; {ec} crítico; score {score:.2}; pendiente {delta:.2}; correcciones {correcciones}; regresiones {regresiones}; finales idénticas {iguales}. Cierre técnico interrumpido.");Ok(())
}
