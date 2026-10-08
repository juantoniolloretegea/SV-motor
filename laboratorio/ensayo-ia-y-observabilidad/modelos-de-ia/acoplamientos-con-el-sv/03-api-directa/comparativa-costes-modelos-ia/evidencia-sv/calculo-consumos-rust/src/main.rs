use std::{collections::{BTreeMap,BTreeSet},env,fs,path::Path};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};

type Row=BTreeMap<String,String>;
fn table(p:&Path)->Vec<Row>{
 let text=fs::read_to_string(p).expect("lectura CSV");
 let mut lines=text.lines();
 let keys:Vec<_>=lines.next().expect("cabecera").split(',').collect();
 assert_eq!(keys.iter().collect::<BTreeSet<_>>().len(),keys.len(),"cabecera duplicada");
 lines.filter(|x|!x.is_empty()).map(|line|{
  let parts:Vec<_>=line.split(',').collect();
  assert_eq!(keys.len(),parts.len(),"numero de columnas");
  keys.iter().zip(parts).map(|(k,v)|(k.to_string(),v.to_string())).collect()
 }).collect()
}
fn num(r:&Row,k:&str)->Option<u64>{let s=&r[k];if s.is_empty(){None}else{Some(s.parse().expect("entero invalido"))}}
fn required(r:&Row,k:&str)->u64{num(r,k).expect("entero requerido")}
fn decimal(s:&str)->u64{
 let mut it=s.split('.');let a:u64=it.next().unwrap().parse().expect("decimal invalido");
 let b=it.next().unwrap_or("");assert!(it.next().is_none()&&b.len()<=10);
 let fraction:u64=if b.is_empty(){0}else{b.parse().expect("fraccion invalida")};
 a.checked_mul(10_000_000_000).unwrap()+fraction*10u64.pow(10-b.len() as u32)
}
fn money(t:u64)->String{format!("{}.{:010}",t/10_000_000_000,t%10_000_000_000)}
fn sha(p:&Path)->String{format!("{:x}",Sha256::digest(fs::read(p).expect("hash")))}
fn write(p:&Path,v:&Value){fs::write(p,format!("{}\n",serde_json::to_string_pretty(v).unwrap())).unwrap();}
fn main(){
 let args:Vec<_>=env::args().collect();
 let root=Path::new(args.get(1).map(String::as_str).unwrap_or("."));
 let rows=table(&root.join("CONSUMOS-POR-REGISTRO.csv"));
 assert_eq!(rows.len(),412);
 let mut ids=BTreeSet::new();
 let mut models:BTreeMap<String,Vec<&Row>>=BTreeMap::new();
 for r in &rows{
  assert!(ids.insert(r["id_publico"].clone()),"ID duplicado");
  assert!(r["id_publico"].starts_with("SV-ECO-20261008-"));
  assert!(["gpt-6-astra","grok-4.7","qwen3.8-max-0902","glm-5.3"].contains(&r["modelo"].as_str()));
  assert!(["pendiente","parcial"].contains(&r["estado_conciliacion"].as_str()));
  if let (Some(i),Some(o),Some(t))=(num(r,"entrada_tokens"),num(r,"salida_tokens"),num(r,"total_tokens")){assert_eq!(i+o,t);}
  assert!(required(r,"intentos")>0);
  if !r["cargo_comunicado"].is_empty(){
   assert_eq!(&r["moneda"],"USD");assert_eq!(&r["naturaleza_importe"],"importe_comunicado_no_factura");decimal(&r["cargo_comunicado"]);
  }else{assert_eq!(&r["naturaleza_importe"],"no_comunicado");}
  models.entry(r["modelo"].clone()).or_default().push(r);
 }
 let mut totals=Vec::new();
 for (model,rs) in &models{
  let sum=|key:&str|rs.iter().filter_map(|r|num(r,key)).sum::<u64>();
  let complete=rs.iter().filter(|r|["entrada_tokens","salida_tokens","total_tokens"].iter().all(|k|num(r,k).is_some())).count();
  let charges:Vec<_>=rs.iter().filter(|r|!r["cargo_comunicado"].is_empty()).collect();
  let money_sum=if charges.is_empty(){Value::Null}else{json!(money(charges.iter().map(|r|decimal(&r["cargo_comunicado"])).sum()))};
  totals.push(json!({"modelo":model,"registros":rs.len(),"intentos_declarados":sum("intentos"),
   "registros_con_tokens_completos":complete,"registros_sin_desglose_completo":rs.len()-complete,
   "entrada_conocida":sum("entrada_tokens"),"salida_conocida":sum("salida_tokens"),"total_conocido":sum("total_tokens"),
   "registros_con_importe_comunicado":charges.len(),"suma_importes_comunicados_usd":money_sum,
   "alcance_importes":"Suma de importes comunicados; no equivale a factura."}));
 }
 write(&root.join("CONSUMOS-POR-MODELO.json"),&json!({"edicion":"2026-10-08","naturaleza":"Agregación de registros de ensayos heterogéneos; no clasificación de eficiencia.","registros":rows.len(),"modelos":totals}));
 let cyb=table(&root.join("CYB16-POR-SOLICITUD.csv"));
 assert_eq!(cyb.len(),48);let by_id:BTreeMap<_,_>=rows.iter().map(|r|(r["id_publico"].as_str(),r)).collect();
 let mut cases=BTreeSet::new();let mut stages:BTreeMap<String,Vec<&Row>>=BTreeMap::new();
 for r in &cyb{
  let id=&r["id_publico"];let source=by_id.get(id.as_str()).expect("ID CYB16 no encontrado");
  assert_eq!(&source["modelo"],"glm-5.3");
  let(i,c,u,o,q,a,t)=(required(r,"entrada_total"),required(r,"entrada_cache"),required(r,"entrada_sin_cache"),
   required(r,"salida_total"),required(r,"razonamiento_incluido"),required(r,"salida_menos_razonamiento"),required(r,"total"));
  assert_eq!(c+u,i);assert_eq!(q+a,o);assert_eq!(i+o,t);
  assert_eq!(Some(i),num(source,"entrada_tokens"));assert_eq!(Some(o),num(source,"salida_tokens"));assert_eq!(Some(t),num(source,"total_tokens"));
  assert_eq!(decimal(&r["coste_entrada_sin_cache_usd"]),u*14_000);
  assert_eq!(decimal(&r["coste_lectura_cache_usd"]),c*2_600);
  assert_eq!(decimal(&r["coste_salida_usd"]),o*44_000);
  assert_eq!(decimal(&r["coste_estimado_usd"]),u*14_000+c*2_600+o*44_000);
  assert!(r["cargo_liquidado_usd"].is_empty(),"liquidacion no esperada");
  assert!(cases.insert((r["caso"].clone(),r["etapa"].clone())),"pregunta-etapa duplicada");
  assert!(required(r,"primer_evento_ms")<=required(r,"duracion_operacion_ms"));
  assert!(required(r,"primer_texto_ms")<=required(r,"duracion_operacion_ms"));
  stages.entry(r["etapa"].clone()).or_default().push(r);
 }
 for case in 1..=16{for stage in ["R0","R1","R2"]{assert!(cases.contains(&(format!("C{:02}",case),stage.to_owned())));}}
 let sum=|key:&str|cyb.iter().map(|r|required(r,key)).sum::<u64>();
 let cost:u64=cyb.iter().map(|r|decimal(&r["coste_estimado_usd"])).sum();
 assert_eq!(sum("entrada_total"),1_226_311);assert_eq!(sum("salida_total"),157_944);
 assert_eq!(sum("entrada_cache"),52_928);assert_eq!(sum("total"),1_384_255);assert_eq!(cost,23_514_510_800);
 let per_stage:Vec<_>=stages.iter().map(|(s,rs)|json!({"etapa":s,"solicitudes":rs.len(),
 "entrada":rs.iter().map(|r|required(r,"entrada_total")).sum::<u64>(),
 "salida":rs.iter().map(|r|required(r,"salida_total")).sum::<u64>(),
 "total":rs.iter().map(|r|required(r,"total")).sum::<u64>(),
 "duracion_operaciones_ms":rs.iter().map(|r|required(r,"duracion_operacion_ms")).sum::<u64>(),
 "coste_estimado_usd":money(rs.iter().map(|r|decimal(&r["coste_estimado_usd"])).sum())})).collect();
 write(&root.join("CYB16-RESUMEN.json"),&json!({"modelo":"glm-5.3","preguntas":16,"solicitudes":48,
 "entrada":sum("entrada_total"),"entrada_cache":sum("entrada_cache"),"salida":sum("salida_total"),
 "razonamiento_incluido_en_salida":sum("razonamiento_incluido"),"total":sum("total"),
 "coste_estimado_usd":money(cost),"cargo_liquidado_usd":Value::Null,
 "tarifas_usd_por_millon":{"entrada_sin_cache":"1.40","lectura_cache":"0.26","salida":"4.40"},
 "fecha_tarifas":"2026-10-08","fuente_tarifas":"https://docs.z.ai/guides/overview/pricing","etapas":per_stage,
 "alcance":"Coste estimado de las 48 solicitudes, no coste por tarea de una evaluación externa ni factura."}));
 write(&root.join("VERIFICACION.json"),&json!({"resultado":"CONFORME","registros_publicos":412,
 "detalle_cyb16":48,"pares_pregunta_etapa":48,"sumas_tokens":"CONFORME","aritmetica_monetaria":"enteros en unidades de 10^-10 USD",
 "desconocidos":"conservados; vacios CSV y null JSON","alcance":"Reproducción de las tablas públicas; no audita los sistemas del proveedor, facturas ni resultados de calidad.",
 "entradas":[{"archivo":"CONSUMOS-POR-REGISTRO.csv","sha256":sha(&root.join("CONSUMOS-POR-REGISTRO.csv"))},
 {"archivo":"CYB16-POR-SOLICITUD.csv","sha256":sha(&root.join("CYB16-POR-SOLICITUD.csv"))}]}));
 println!("CONFORME: 412 registros; 48 solicitudes CYB16; estimacion {} USD.",money(cost));
}

