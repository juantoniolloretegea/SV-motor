use crate::{E,hash,read,save};
use serde_json::{Value,json};
use std::{fs,path::Path};
const KEY:&str="0cfefe66e3b27e314414d3e21f39de9c434f710813904e8764e9b76d798b1c8b";
fn nonempty(v:&Value,k:&str)->Result<(),E>{if v[k].as_str().is_some_and(|s|!s.trim().is_empty()){Ok(())}else{Err(format!("Falta fundamento: {k}").into())}}

fn value(substance:&str,formal:bool)->Result<&'static str,E>{match substance{"error"=>Ok("1"),"correcto"=>Ok(if formal{"0"}else{"1"}),"indeterminado"=>Ok(if formal{"U"}else{"1"}),_=>Err("Adjudicación sustantiva pendiente o desconocida".into())}}
fn score(rows:&[Value],custody:bool,incidents:bool)->Value{
 let count=|v:&str|rows.iter().filter(|r|r["valor"]==v).count();let n0=count("0");let n1=count("1");let nu=count("U");
 let critical_errors=rows.iter().filter(|r|r["error_critico"]==true).count();let noncritical=rows.iter().filter(|r|r["valor"]=="1"&&r["error_critico"]!=true).count();
 let complete=rows.len()==9;let k=if !complete{Value::Null}else{json!(if n1>=7{"No apto"}else if n0>=7{"Apto"}else{"Indeterminado"})};
 let all_critical=rows.iter().filter(|r|r["critico_prefijado"]==true).count()==6&&rows.iter().filter(|r|r["critico_prefijado"]==true).all(|r|r["valor"]=="0");
 json!({"N0":n0,"N1":n1,"NU":nu,"terna_completa":complete,"kappa":k,"errores_criticos":critical_errors,"errores_no_criticos":noncritical,"puntuacion":if complete{json!(100.0*(n0 as f64-noncritical as f64)/9.0)}else{Value::Null},"denominador":9,"criticos_todos_en_0":all_critical,"custodia_conforme":custody,"incidencias_pendientes":incidents,"conforme":complete&&k=="Apto"&&all_critical&&custody&&!incidents})
}
fn esc(s:&str)->String{s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace('"',"&quot;")}
fn frame(v:&Value)->Result<String,E>{
 let block=v["bloque"].as_str().ok_or("Bloque")?;let layer=v["capa"].as_u64().ok_or("Capa")?;let rows=v["casos"].as_array().ok_or("Casos")?;
 let mut cells=String::new();let mut table=String::new();
 for i in 0..9{let id=format!("{block}{:02}",i+1);let row=rows.iter().find(|r|r["caso"]==id);let val=row.map(|r|r["valor"].as_str().unwrap()).unwrap_or("NE");cells.push_str(&format!("<li data-pos=\"{}\" data-caso=\"{id}\" data-valor=\"{val}\" class=\"v{val}\"><span>{id}</span><strong>{val}</strong></li>",i+1));if let Some(r)=row{table.push_str(&format!("<tr><td>{id}</td><td>{val}</td><td>{}</td><td>{}</td><td>{}</td></tr>",r["critico_prefijado"],esc(r["sustantivo"].as_str().unwrap()),r["formal_conforme"]));}else{table.push_str(&format!("<tr><td>{id}</td><td>NE</td><td>{}</td><td>No ejecutado o pendiente; fuera de terna</td><td>—</td></tr>",i>=3));}}
 let embedded=serde_json::to_string(v)?.replace('<',"\\u003c").replace('>',"\\u003e").replace('&',"\\u0026");
 Ok(format!("<!doctype html><html lang=\"es\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>Catálogo Astra · {block}{layer}</title><style>body{{font:18px/1.5 system-ui;max-width:1200px;margin:2rem auto;padding:1rem;color:#17222c;background:#fff}}ol{{display:flex;flex-wrap:nowrap;gap:.4rem;list-style:none;padding:0;overflow:auto}}li{{min-width:5rem;flex:1;text-align:center;border:2px solid;padding:.5rem}}li span,li strong{{display:block}}li strong{{font-size:2rem}}.v0{{background:#dbf2de;border-color:#226b39}}.v1{{background:#f9dddd;border-color:#8d2828}}.vU{{background:#fff0c5;border-color:#87660b}}.vNE{{background:#eee;border-color:#666}}table{{border-collapse:collapse;width:100%}}th,td{{border:1px solid #aaa;padding:.4rem;text-align:left}}footer{{font-size:.8rem;margin-top:2rem}}</style><h1>GPT-6 Astra · {block}{layer}</h1><p>Vector canónico (9,3). Orden de lectura: de izquierda a derecha. Nueve posiciones; no representa una matriz 3 × 3.</p><ol id=\"frame\" data-bloque=\"{block}\" data-capa=\"{layer}\">{cells}</ol><p>Leyenda: <b>0</b>, correcto y completo; <b>1</b>, error demostrado o incumplimiento formal; <b>U</b>, indeterminación sustantiva evaluable. <b>NE</b> está fuera de la terna y señala ausencia de adjudicación, no una abstención del candidato.</p><p>κ: {}. Puntuación: {}. Conformidad conjunta: {}.</p><table><thead><tr><th>Caso</th><th>Valor</th><th>Criticidad prefijada</th><th>Contenido</th><th>Forma conforme</th></tr></thead><tbody>{table}</tbody></table><p>Datos y fundamentos completos en <a href=\"CAPA.json\">CAPA.json</a>. Un defecto exclusivamente formal se registra como error no crítico. La recepción independiente permanece separada de esta adjudicación.</p><script type=\"application/json\" id=\"datos\">{embedded}</script><footer>{}</footer></html>",esc(&v["medidas"]["kappa"].to_string()),v["medidas"]["puntuacion"],v["medidas"]["conforme"],crate::FOOTER))
}
fn verify_frame(html:&str,v:&Value)->Result<(),E>{
 let embedded=html.split_once("<script type=\"application/json\" id=\"datos\">").ok_or("Datos frame")?.1.split_once("</script>").ok_or("Cierre datos")?.0;
 if crate::estricto::parse(embedded.as_bytes())?!=*v{return Err("Datos incorporados discordantes".into())}
 let block=v["bloque"].as_str().ok_or("Bloque")?;let rows=v["casos"].as_array().ok_or("Casos")?;
 let frame=html.split_once("<ol id=\"frame\"").ok_or("Frame")?.1.split_once("</ol>").ok_or("Cierre frame")?.0;
 if frame.matches("<li ").count()!=9{return Err("Número de posiciones del frame".into())}
 let mut previous=0;
 for i in 0..9{let id=format!("{block}{:02}",i+1);let value=rows.iter().find(|r|r["caso"]==id).map(|r|r["valor"].as_str().unwrap()).unwrap_or("NE");let needle=format!("data-pos=\"{}\" data-caso=\"{id}\" data-valor=\"{value}\"",i+1);let pos=frame.find(&needle).ok_or("Posición, caso o valor discordante")?;if pos<previous||frame.matches(&needle).count()!=1{return Err("Orden o identidad del frame".into())}previous=pos;}
 for legend in ["correcto y completo","indeterminación sustantiva evaluable","fuera de la terna"]{if !html.contains(legend){return Err("Leyenda incompleta".into())}}
 Ok(())
}
pub fn capa(input:&Path,out:&Path)->Result<(),E>{
 let v=read(input)?;if v["casos"].as_array().map(Vec::len)!=Some(9){return Err("Vector incompleto: representación no admitida".into());}let block=v["bloque"].as_str().ok_or("Bloque")?;if !["A","B"].contains(&block){return Err("Bloque fuera de alcance".into())}let layer=v["capa"].as_u64().filter(|n|*n<=3).ok_or("Capa")?;
 let key_path=input.parent().ok_or("Carpeta")?.join(v["clave_local"].as_str().ok_or("Clave local")?);let key_bytes=fs::read(key_path)?;if key_bytes.len()!=8007||hash(&key_bytes)!=KEY{return Err("Clave reservada no coincidente".into())}let key=crate::estricto::parse(&key_bytes)?;
 let mut rows=Vec::new();
 for item in v["casos"].as_array().ok_or("Casos")?{let case=item["caso"].as_str().ok_or("Caso")?;let k=key["casos"].as_array().ok_or("Casos clave")?.iter().find(|k|k["id"]==case).ok_or("Caso ajeno a clave")?;if !case.starts_with(block)||rows.iter().any(|r:&Value|r["caso"]==case){return Err("Caso repetido o de otro bloque".into())}
  nonempty(item,"fundamento_sustantivo")?;if item["lectura_original_confirmada"]!=true{return Err("Original no leído".into())}
  let audit_path=input.parent().unwrap().join(item["auditoria"].as_str().ok_or("Auditoría")?);let audit=read(&audit_path)?;
  if audit["integridad_conforme"]!=true||audit["caso"]!=case||audit["capa"]!=layer||audit["final_sha256"]!=item["final_sha256"]{return Err("Auditoría no corresponde al original adjudicado".into())}
  let formal=audit["contrato_formal_conforme"].as_bool().ok_or("Forma")?;let substance=item["sustantivo"].as_str().ok_or("Adjudicación sustantiva")?;
  let category=if audit["decision_observada"].is_string(){json!(audit["decision_observada"]==k["esperada"])}else{Value::Null};
  if substance=="correcto"&&category==false{return Err("Contenido correcto incompatible con etiqueta observada".into())}
  let val=value(substance,formal)?;let critical=k["critico"].as_bool().ok_or("Criticidad")?;let critical_error=critical&&substance=="error";
  rows.push(json!({"caso":case,"valor":val,"sustantivo":substance,"formal_conforme":formal,"defecto_formal":audit["defecto_formal"],"critico_prefijado":critical,"dificultad":k["dificultad"],"error_critico":critical_error,"categoria_correcta":category,"fundamento_sustantivo":item["fundamento_sustantivo"],"pasajes_contrastados":item["pasajes_contrastados"],"final_sha256":item["final_sha256"],"auditoria_sha256":hash(&fs::read(audit_path)?)}));
 }
 rows.sort_by_key(|r|r["caso"].as_str().unwrap().to_owned());
 let custody=v["custodia_conforme"].as_bool().ok_or("Custodia")?;let incidents=v["incidencias_pendientes"].as_array().ok_or("Incidencias")?;
 let result=json!({"encargo":"ASTRA-CATALOGO-20261007/A0","bloque":block,"capa":layer,"criterio":"CRITERIO-PUNTUACION-MODELOS-20261001/r2 y apartado 9 del encargo","clave_sha256":KEY,"casos":rows,"medidas":score(&rows,custody,!incidents.is_empty()),"incidencias_pendientes":incidents,"no_adjudicados":9-rows.len(),"recepcion_independiente":"Pendiente","licencia":crate::FOOTER});
 if out.exists(){return Err("Destino existente: no sobrescribir capa".into())}fs::create_dir_all(out)?;save(&out.join("CAPA.json"),&result)?;
 let html=frame(&result)?;verify_frame(&html,&result)?;fs::write(out.join("FRAME.html"),html.as_bytes())?;verify_frame(&fs::read_to_string(out.join("FRAME.html"))?,&read(&out.join("CAPA.json"))?)?;
 save(&out.join("COTEJO-FRAME-RUST.json"),&json!({"conforme":true,"posiciones":9,"frame_sha256":hash(html.as_bytes()),"datos_sha256":hash(&fs::read(out.join("CAPA.json"))?)}))?;println!("{}",result["medidas"]);Ok(())
}
#[cfg(test)]mod tests{
 use super::*;
 fn rows(vals:&[&str;9])->Vec<Value>{vals.iter().enumerate().map(|(i,v)|json!({"caso":format!("A{:02}",i+1),"valor":v,"critico_prefijado":i>=3,"error_critico":i>=3&&*v=="1","sustantivo":if *v=="0"{"correcto"}else if *v=="U"{"indeterminado"}else{"error"},"formal_conforme":true})).collect()}
 #[test]fn negativo_sin_recorte(){let mut r=rows(&["1";9]);for x in &mut r{x["error_critico"]=json!(false);}let s=score(&r,true,false);assert_eq!(s["puntuacion"],-100.0);assert_eq!(s["kappa"],"No apto");}
 #[test]fn critico_no_doble_deduccion(){let r=rows(&["0","0","0","1","0","0","0","0","0"]);let s=score(&r,true,false);assert_eq!(s["errores_no_criticos"],0);assert_eq!(s["kappa"],"Apto");assert_eq!(s["conforme"],false);}
 #[test]fn u_critica_no_conforme(){let r=rows(&["0","0","0","U","0","0","0","0","0"]);assert_eq!(score(&r,true,false)["conforme"],false);}
 #[test]fn defecto_solo_formal(){assert_eq!(value("correcto",false).unwrap(),"1");assert_eq!(value("correcto",true).unwrap(),"0");assert!(value("pendiente",false).is_err());}
 #[test]fn incompleto_fuera_de_terna(){let r=rows(&["0";9]);let s=score(&r[..8],true,false);assert!(s["kappa"].is_null());assert!(s["puntuacion"].is_null());}
 #[test]fn frame_detecta_cambio_de_posicion(){let r=rows(&["0","1","U","0","0","0","0","0","0"]);let v=json!({"bloque":"A","capa":0,"casos":r,"medidas":score(&r,true,false)});let h=frame(&v).unwrap();assert!(verify_frame(&h,&v).is_ok());let bad=h.replace("data-caso=\"A02\" data-valor=\"1\"","data-caso=\"A02\" data-valor=\"0\"");assert!(verify_frame(&bad,&v).is_err());}
}
