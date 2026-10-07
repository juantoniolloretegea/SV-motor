use crate::E;
use serde_json::{Value,json};
fn text<'a>(v:&'a Value,k:&str)->Result<&'a str,E>{v[k].as_str().filter(|s|!s.trim().is_empty()).ok_or_else(||format!("Campo textual {k}").into())}

// Mismo contrato fijado antes de A01; no modifica la política del candidato.
pub fn check(v:&Value,doc:&str,section:&str,pages:&[String],case:&str,layer:u64)->Result<(),E>{
 let o=v.as_object().ok_or("Se exige objeto")?;
 let fields=["decision","reglas","evidencias","justificacion_breve","recepcion_documental","revision"];
 if o.len()!=fields.len()||fields.iter().any(|k|!o.contains_key(*k)){return Err("Campos finales no conformes".into())}
 if !["RESPALDADA","CONTRADICHA","EVIDENCIA_INSUFICIENTE"].contains(&text(v,"decision")?){return Err("Decisión".into())}
 let r=v["reglas"].as_array().ok_or("Reglas")?;
 if r.is_empty()||r.iter().any(|r|!matches!(r.as_str(),Some("D1"|"D2"|"D3"|"D4"|"D5"))){return Err("Reglas".into())}
 text(v,"justificacion_breve")?;
 let ev=v["evidencias"].as_array().ok_or("Evidencias")?;if ev.is_empty(){return Err("Sin evidencias".into())}
 for e in ev {
  if e.as_object().map(|o|o.len())!=Some(4)||e["documento"]!=doc||e["seccion"]!=section{return Err("Localizador".into())}
  let p=e["pagina"].as_u64().ok_or("Página")? as usize;
  if !pages.get(p).is_some_and(|t|t.contains(text(e,"fragmento").unwrap_or("\0"))){return Err("Cita no literal o página inexistente".into())}
 }
 let rec=v["recepcion_documental"].as_array().ok_or("Recepción")?;
 if rec.len()!=1||rec[0].as_object().map(|o|o.len())!=Some(3)||rec[0]["documento"]!=doc||rec[0]["seccion"]!=section||rec[0]["paginas"]!=json!((0..pages.len()).collect::<Vec<_>>()){return Err("Recepción incompleta o inexistente".into())}
 if layer==0 {if !v["revision"].is_null(){return Err("Revisión inicial".into())}}
 else {
  let rev=&v["revision"];
  if rev.as_object().map(|o|o.len())!=Some(3){return Err("Campos revisión".into())}
  let prior=rev["antecedentes"].as_array().ok_or("Antecedentes")?;
  if prior.len()!=layer as usize||prior.iter().enumerate().any(|(i,a)|a!=&json!({"caso":case,"capa":i})){return Err("Antecedentes".into())}
  let adv=rev["adversarial"].as_array().ok_or("Adversarial")?;
  if adv.is_empty(){return Err("Adversarial vacía".into())}
  for a in adv {if a.as_object().map(|o|o.len())!=Some(3){return Err("Campos adversarial".into())}text(a,"objecion")?;text(a,"contraste_documental")?;if !["mantener","corregir","retirar","indeterminar"].contains(&text(a,"conclusion")?){return Err("Conclusión".into())}}
  text(rev,"fundamento_del_cambio_o_mantenimiento")?;
 }
 Ok(())
}
