use std::{fs,io::Write,path::Path};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn save(p:&Path,b:&[u8])->R<()>{fs::create_dir_all(p.parent().unwrap())?;let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn rep(s:&mut String,a:&str,b:&str)->R<()>{if !s.contains(a){return Err(format!("Patrón ausente: {a}").into())}*s=s.replace(a,b);Ok(())}
fn main()->R<()>{
 let root=std::env::args().nth(1).ok_or("Directorio")?;let base=Path::new(&root);let old=base.join("diagnostico-A08-02");let new=base.join("diagnostico-A08-02-incidente");if new.exists(){return Err("Incidente ya preparado".into())}
 let mut cargo=fs::read_to_string(old.join("verificador/Cargo.toml"))?;
 rep(&mut cargo,"sv-safeguard-verificador-diag-a08-02","sv-safeguard-cotejo-incidente-d02")?;
 rep(&mut cargo,"path=\"../instrumentacion\"","path=\"../diagnostico-A08-02/instrumentacion\"")?;
 save(&new.join("Cargo.toml"),cargo.as_bytes())?;
 let mut audit=fs::read_to_string(old.join("instrumentacion/src/auditoria.rs"))?;
 rep(&mut audit,"closure[\"error\"].is_null()","interrupted && closure[\"error\"]==\"Cota total de una hora alcanzada; conservar sin reintento\" && closure[\"codigo_modelo\"]==1 && closure[\"fin_conductor\"]==false")?;
 rep(&mut audit,"let sha=h(&serde_json::to_vec(&bare)?);","bare.sort_all_objects();let sha=h(&serde_json::to_vec(&bare)?);")?;
 save(&new.join("src/auditoria_incidente.rs"),audit.as_bytes())?;
 let mut main=fs::read_to_string(old.join("verificador/src/main.rs"))?;
 main.insert_str(0,"mod auditoria_incidente;\n");
 rep(&mut main,"ck(emisiones.len()==reales.len()&&finales.len()==emisiones.len()&&cierres.len()==emisiones.len(),\"Original incompleto\")?;","ck(reales.len()==1&&emisiones.is_empty()&&finales.is_empty()&&cierres.is_empty(),\"El incidente exige una entrada efectiva sin respuesta final\")?;")?;
 rep(&mut main,"ck(ev.iter().any(|v|v[\"datos\"][\"evento\"]==\"fin_conductor\"),\"Sin cierre\")?;","ck(!ev.iter().any(|v|v[\"datos\"][\"evento\"]==\"fin_conductor\"),\"El incidente no puede fingir cierre normal\")?;")?;
 rep(&mut main,"let informe=json!({","let mut informe=json!({")?;
 rep(&mut main,"let dest=std::path::PathBuf::from",r#"
 let audit=auditoria_incidente::auditar(p,true,false).map_err(|e|e.to_string())?;
 ck(audit["conforme"]==true&&audit["recorrido_completo"]==false&&audit["emisiones"]==0&&audit["emision_parcial"]==1982,"Cadena del incidente discordante")?;
 let closure:Value=serde_json::from_slice(&fs::read(p.join("CIERRE.json"))?)?;
 let gens=ev.iter().filter(|v|v["datos"]["evento"]=="configuracion_generacion_solicitada").collect::<Vec<_>>();
 ck(gens.len()==1&&gens[0]["datos"]["intento"]==2&&gens[0]["datos"]["razonamiento"]=="high","Configuración discordante")?;
 let sample=&gens[0]["datos"]["sampling"];
 ck(sample["temperature"]==1.0&&sample["top_p"]==1.0&&sample["max_len"]==4096&&sample["top_k"].is_null()&&gens[0]["datos"]["semilla"]==42,"Muestreo distinto")?;
 let tokens=ev.iter().filter(|v|v["datos"]["evento"]=="token").collect::<Vec<_>>();
 ck(tokens.len()==1982&&tokens.last().unwrap()["datos"]["posicion"]==1981,"Tokens discordantes")?;
 let first=tokens.first().unwrap()["unix_us"].as_u64().ok_or("Fecha")?;let last=tokens.last().unwrap()["unix_us"].as_u64().ok_or("Fecha")?;
 informe["intento"]=json!(2);informe["tipo_resultado"]=json!("Interrupción instrumental por cota temporal; sin respuesta final adjudicable");
 informe["custodia"]=audit;informe["cierre_original"]=closure;informe["configuracion_solicitada"]=gens[0].clone();
 informe["tokens_generados"]=json!(tokens.len());informe["primer_token_unix_us"]=json!(first);informe["ultimo_token_unix_us"]=json!(last);
 informe["tokens_por_segundo_entre_extremos"]=json!((tokens.len()-1)as f64/((last-first)as f64/1_000_000.0));
 informe["clasificacion_correcta"]=Value::Null;informe["valor_sv"]=Value::Null;informe["respuesta_final"]=Value::Null;
 informe["examen_habilitado"]=json!(false);informe["cierre_normal"]=json!(false);informe["originales_modificados"]=json!(false);
 let dest=std::path::PathBuf::from"#)?;
 save(&new.join("src/main.rs"),main.as_bytes())?;
 println!("Cotejador separado preparado: no modifica originales, no genera ni adjudica respuesta inexistente.");Ok(())
}
