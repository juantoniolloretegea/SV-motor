//! Cierre por acotación humana: reconstruye exclusivamente hitos ya conservados.
#![forbid(unsafe_code)]
use std::{fs,path::{Path,PathBuf}};
use serde_json::{json,Value};
use sv_cliente_api::{guard,need,parse,save,sha,R,LICENCIA};
fn load(p:&Path)->R<Value>{guard(p)?;parse(&fs::read(p).map_err(|e|e.to_string())?)}
fn run()->R<()> {
 let a:Vec<_>=std::env::args().collect();need(a.len()==2,"Uso: sv-consolidar-acotacion DIRECTORIO")?;
 let root=PathBuf::from(&a[1]);guard(&root)?;let cut=load(&root.join("ACOTACION-EJECUTADA.json"))?;
 need(cut["proceso_detenido"]==true&&cut["entrega_final"]==48,"Corte no recibido")?;
 let contract=load(&root.join("CONTRATO-16.json"))?;need(contract["preguntas"]==16&&contract["base"]==4&&contract["etapas"]==3,"Contrato distinto")?;
 let dirs=fs::read_dir(root.join("intentos")).map_err(|e|e.to_string())?.collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())?;
 need(dirs.len()==48,"Hay intentos adicionales: recibirlos separadamente antes de consolidar")?;
 let initial=load(&root.join("INICIO-EXAMEN.json"))?;let technical=load(&root.join("RECEPCION-TRANSPORTE.json"))?;
 let mut total=technical["consumo_control"].as_u64().ok_or("Consumo técnico")?;let mut rows=vec![];
 for i in 1..=48 {let h=load(&root.join(format!("hitos/I{i:03}.json")))?;let n=(i-1)/3+1;let s=(i-1)%3;
  need(h["caso"]==format!("P{n:02}")&&h["etapa"]==s&&h["intento"]==i,"Orden de hitos distinto")?;
  let d=PathBuf::from(h["directorio"].as_str().ok_or("Directorio")?);guard(&d)?;
  let r=load(&d.join("RESULTADO.json"))?;need(r==h["resultado"]&&r["completa"]==true&&r["telemetria_conforme"]==true,"Entrega no completa o discordante")?;
  for f in h["originales"].as_array().ok_or("Originales")? {let p=PathBuf::from(f["ruta"].as_str().ok_or("Ruta")?);guard(&p)?;let b=fs::read(p).map_err(|e|e.to_string())?;need(sha(&b)==f["sha256"]&&b.len()==f["bytes"],"Original alterado")?;}
  total=total.checked_add(h["consumo_control"].as_u64().ok_or("Consumo ausente")?).ok_or("Desbordamiento")?;rows.push(h);
 }
 need(sha(&fs::read(root.join("hitos/I048.json")).map_err(|e|e.to_string())?)==cut["hito_final_sha256"],"Último hito distinto")?;
 let duration=cut["utc_ms"].as_u64().ok_or("Fin")?.checked_sub(initial["utc_ms"].as_u64().ok_or("Inicio")?).ok_or("Fechas regresivas")?;
 need(duration<=5_400_000,"Se superó el plazo inicial de 90 minutos")?;
 let bank=json!({"estado":"completo_por_acotacion_humana_a_16","proveedor":"Alibaba Cloud","modelo":"qwen3.8-max-0902","duracion_ms":duration,"consumo_o_reserva_acumulados":total,"unidad_control":"tokens_cuota_gratuita","coste_o_reserva_acumulados_ticks":null,"duracion_interrupcion_abierta_ms":null,"etapas_completas_por_pregunta":vec![3;16],"intentos":rows,"contrato_efectivo":contract,"contrato_sha256":sha(&fs::read(root.join("CONTRATO-16.json")).map_err(|e|e.to_string())?),"proveniencia":"Consolidación Rust de los 48 hitos originales tras corte autorizado en P16/R2. No se rehacen inferencias ni se alteran entregas.","adjudicacion_sustantiva":"pendiente","autoria_licencia":LICENCIA});
 save(&root.join("RESULTADO-BANCO.json"),&bank)?;println!("48 hitos recibidos; 16 preguntas completas; {total} tokens con comprobación técnica");Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(1)}}
