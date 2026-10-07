use std::{fs,io::Write,path::Path};
use serde_json::{Value,json};
use sv_arbitro_comprobaciones::{huella,auditoria::auditar,retroalimentacion::validar_plan};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn val(p:&Path)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn save(p:&Path,b:&[u8])->R<()>{let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn main()->R<()>{
 let a=std::env::args().collect::<Vec<_>>();let base=Path::new(a.get(1).ok_or("Preparación requerida")?);let recorrido=Path::new(a.get(2).ok_or("Recorrido instrumental requerido")?);
 let audit=auditar(recorrido,false,false)?;ck(audit["conforme"]==true&&audit["emisiones"]==0&&audit["calculos"]==0,"Custodia instrumental no conforme")?;
 let nombre=a.get(3).map(String::as_str).unwrap_or("INSTRUMENTAL-03");ck(nombre.chars().all(|c|c.is_ascii_alphanumeric()||c=='-'),"Nombre de comprobación inválido")?;
 let raw=fs::read(recorrido.join("modelo.stdout"))?;let informe=val(&base.join(format!("cotejos/{nombre}.json")))?;
 ck(informe["conforme"]==true&&informe["entradas_prefijadas"]==1&&informe["emisiones_decodificadas"]==0&&informe["inferencia"]==false&&informe["modelo_stdout_sha256"]==huella(&raw),"Entradas sin cotejo externo")?;
 let plan=val(&base.join("config/plan.json"))?;validar_plan(&plan)?;ck(plan["capa"]==2&&plan["bloque"]=="A","Condición distinta")?;
 let contrato=val(&base.join("config/contrato.json"))?;
 for (campo,ruta) in [("plan_sha256","config/plan.json"),("catalogo_sha256","cache/catalogo.json"),("politica_sha256","config/politica.txt")]{ck(contrato[campo]==huella(&fs::read(base.join(ruta))?),"Perfil no fijado")?;}
 let e=std::str::from_utf8(&raw)?.lines().filter_map(|l|l.strip_prefix("SV_EVENT ")).map(serde_json::from_str::<Value>).collect::<Result<Vec<_>,_>>()?;
 ck(!e.iter().any(|v|matches!(v["datos"]["evento"].as_str(),Some("carga_inicio"|"carga_fin"|"contexto_conductor"|"entrada_motor"))),"Prueba instrumental cargó el modelo")?;
 ck(e.iter().any(|v|v["datos"]["evento"]=="pruebas_instrumentales_fin"&&v["datos"]["conforme"]==true),"Falta comprobación MCP")?;
 let entradas=e.iter().filter(|v|v["datos"]["evento"]=="contexto_previsto").map(|v|v["datos"].clone()).collect::<Vec<_>>();ck(entradas.len()==1,"Una entrada exigidas")?;
 for (i,d) in entradas.iter().enumerate(){let c=&plan["casos"][i];ck(d["id"]==c["id"]&&d["documento"]==c["documento"]&&d["seccion"]==c["seccion"]&&d["funciones"]==json!([]),"Caso o sección discordante")?;}
 let b=serde_json::to_vec_pretty(&entradas)?;
 let adm=base.join("config/ADMISION.json");let existente=adm.exists();if existente{ck(fs::read(&adm)?==b,"Recuperación cambia entradas fijadas")?;}
 save(&base.join(format!("cotejos/FIJACION-{nombre}.json")),&serde_json::to_vec_pretty(&json!({"conforme":true,"admision_sha256":huella(&b),"entradas":informe["detalle"],"inferencia":false,"cotejo_custodia":audit,"modelo_stdout_sha256":huella(&raw),"admision_previa_identica":existente}))?)?;
 if !existente{save(&adm,&b)?;}println!("Una entrada fijadas sin carga ni inferencia: {}",huella(&b));Ok(())
}
