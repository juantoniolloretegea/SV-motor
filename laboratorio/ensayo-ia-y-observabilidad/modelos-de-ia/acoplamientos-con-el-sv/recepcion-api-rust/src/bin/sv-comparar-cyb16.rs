//! Contraste descriptivo del mismo banco; no clasifica capacidad general ni liquida gastos.
#![forbid(unsafe_code)]
use serde_json::{json,Value};
use std::{fs,path::Path};
use sv_cliente_api::{need,parse,save,sha,R};
fn load(p:&Path)->R<Value>{sv_cliente_api::guard(p)?;parse(&fs::read(p).map_err(|e|e.to_string())?)}
fn digest(p:&Path)->R<String>{sv_cliente_api::guard(p)?;Ok(sha(&fs::read(p).map_err(|e|e.to_string())?))}
fn summary(root:&Path)->R<Value>{
 let r=load(&root.join("RECEPCION-RUST.json"))?;
 need(r["conforme"]==true&&r["entregas_completas"]==48,"Recepción incompleta")?;
 let cmp=load(&root.join("COMPARACION-RUST.json"))?;
 need(cmp["conforme"]==true,"Adjudicación ausente")?;
 let rows=r["casos"].as_array().ok_or("Casos")?;
 let complete=rows.iter().filter(|x|x["completa"]==true).collect::<Vec<_>>();
 need(complete.len()==48,"Entregas completas distintas de 48")?;
 let all_times=rows.iter().map(|x|x["duracion_operacion_ms"].as_u64().ok_or("Duración desconocida".into())).collect::<R<Vec<u64>>>()?;
 let mut times=complete.iter().map(|x|x["duracion_operacion_ms"].as_u64().ok_or("Duración desconocida".into())).collect::<R<Vec<u64>>>()?;
 times.sort();let sum: u64=times.iter().sum();
 let(mut cache,mut reasoning)=(0u64,0u64);
 for x in &complete {let u=&x["uso"];let c=u["input_tokens_details"]["cached_tokens"].as_u64().or(u["prompt_tokens_details"]["cached_tokens"].as_u64()).or(u["cached_tokens"].as_u64()).ok_or("Caché no comunicada")?;let o=u["output_tokens_details"]["reasoning_tokens"].as_u64().or(u["completion_tokens_details"]["reasoning_tokens"].as_u64()).or(u["reasoning_tokens"].as_u64()).ok_or("Razonamiento no comunicado")?;cache+=c;reasoning+=o;}
 Ok(json!({"modelo":load(&root.join("CONTRATO-LIBRO.json"))?["modelo"],"solicitudes":rows.len(),"entregas_completas":complete.len(),"intentos_incompletos":rows.len()-complete.len(),"tokens_de_intentos_incompletos":if rows.len()>complete.len(){Value::Null}else{json!(0)},"etapas":cmp["capas"],"entrada":r["tokens_entrada"],"salida":r["tokens_salida"],"total":r["tokens_totales"],"cache_incluida":cache,"razonamiento_incluido":reasoning,"suma_duraciones_operaciones_ms":all_times.iter().sum::<u64>(),"suma_duraciones_entregas_completas_ms":sum,"mediana_operacion_ms":(times[23] as f64+times[24] as f64)/2.,"p95_operacion_ms":times[45],"maximo_operacion_ms":times[47],"muestras":r["muestras"],"intervalo_maximo_ms":r["intervalo_maximo_ms"],"recepcion_sha256":digest(&root.join("RECEPCION-RUST.json"))?,"comparacion_sha256":digest(&root.join("COMPARACION-RUST.json"))?,"importe_liquidado":null}))
}
fn main(){let result=(||->R<()>{let a=std::env::args().collect::<Vec<_>>();need(a.len()==3,"Raíces Astra y GLM")?;let x=Path::new(&a[1]);let y=Path::new(&a[2]);let mut identities=vec![];
 for f in ["fuentes/BANCO.json","reservado/CLAVE.json","CRITERIOS.md","fuentes/preparado/CATALOGO.json","fuentes/preparado/FUENTES.json"] {let h=digest(&x.join(f))?;need(h==digest(&y.join(f))?,"Contrato no comparable")?;identities.push(json!({"archivo":f,"sha256":h}));}
 let astra=summary(x)?;let glm=summary(y)?;need(astra["modelo"]=="gpt-6-astra"&&glm["modelo"]=="glm-5.3","Modelos inesperados")?;
 save(&x.join("CONTRASTE-GLM.json"),&json!({"conforme":true,"identidades_comunes":identities,"astra":astra,"glm":glm,"metodo_temporal":"Suma de todas las operaciones registradas, incluidos intentos interrumpidos y C01/R0 original una sola vez. Suma de entregas completas separada. Mediana y p95 se calculan sólo sobre las 48 entregas completas: posiciones 24/25 y rango más próximo ceil(0.95*48)=46. Excluye pausa diagnóstica, autenticación, preparación, revisión y archivo. No es un ensayo simultáneo ni controlado de infraestructura.","limites":"Mismo problema, corpus y reglas; transportes, segmentadores, configuración de razonamiento e infraestructura de proveedor distintos. Tokens no son una unidad semántica universal. Los contadores de intentos sin recepción completa son desconocidos, no cero. No se calcula velocidad de generación a partir del tiempo total. No se infiere precio liquidado del saldo de una cuenta compartida. Las etapas son revisiones del mismo candidato; estabilidad no prueba verdad general.","licencia":sv_cliente_api::LICENCIA}))?;Ok(())})();if let Err(e)=result{eprintln!("{e}");std::process::exit(1)}}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
