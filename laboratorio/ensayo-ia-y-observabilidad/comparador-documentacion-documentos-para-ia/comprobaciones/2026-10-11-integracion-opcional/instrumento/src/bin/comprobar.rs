use serde_json::json;
use std::{collections::BTreeMap,fs::{self,OpenOptions},io::Write,path::Path};
use sv_administracion_documental::{almacen::*,huella};
use sv_bsd_integracion::*;
fn act(a:&Almacen,x:Accion){a.ejecutar(11,9000,a.revision().unwrap(),x,101).unwrap();}
fn alta(a:&Almacen,id:&str,raw:&[u8],permitido:bool,recibido:bool,ext:&str){
 let ficha=Ficha{id:id.into(),titulo:format!("Control {id}"),procedencia:"Documento sintético propio".into(),titular:"Titular del ensayo".into(),fundamento:"Comprobación local autorizada".into(),evidencia:"Protocolo del paso 7".into(),beneficiarios:"Ensayo instrumental".into(),condiciones:"Sin suministro a modelos".into(),hasta:8000,usos:Uso::todos().into_iter().map(|u|{let p=if u==Uso::Extraccion&&!permitido{Permision::NoPermitida}else{Permision::Permitida};(u,p)}).collect()};
 act(a,Accion::Declarar{ficha});act(a,Accion::RevisarLicencia{recurso:id.into(),favorable:true,informe:"Control de fuente propia".into()});act(a,Accion::AprobarLicencia{recurso:id.into()});
 act(a,Accion::PrepararCarga{id:id.into(),recurso:id.into(),sha256:huella(raw),bytes:raw.len() as u64,extension:ext.into()});
 if recibido{act(a,Accion::RecibirCarga{id:id.into(),revision_git:"a".repeat(40),sha256:huella(raw)});}
}
fn main()->Result<(),Box<dyn std::error::Error>>{
 let dest=std::env::args().nth(1).ok_or("Directorio de salida requerido")?;let dest=Path::new(&dest);
 if dest.exists(){return Err("Salida existente".into())}fs::create_dir_all(dest)?;
 let tests=["omitida","individual","grupo","todos","vacio","duplicado","propio","ausente","dominio","sin_permiso","sesion_vencida","sin_admision","original_alterado","traduccion_alterada","cambio_concurrente","formato","traduccion_valida","traduccion_sin_permiso","unicode","cota_grupo","sin_aviso"];
 let mut rows=Vec::new();let mut total_calls=0;let mut total_reads=0;
 for name in tests{
  let a=Almacen::abrir(&dest.join(format!("{name}.redb")),&Alta{dominio:"ciberseguridad".into(),repositorio:1,administrador:11,hasta:10000,instante:100})?;
  let mut texts:BTreeMap<String,Vec<u8>>=BTreeMap::from([
   ("propuesto".into(),b"# Current policy\n\nThe gateway should log all events.\n".to_vec()),
   ("r1".into(),b"# Reference one\n\nThe gateway should not log all events.\n".to_vec()),
   ("r2".into(),b"# Reference two\n\nThe gateway should log all events.\n".to_vec()),
   ("sin-permiso".into(),b"# Reserved\n\nThe gateway should log debug events.\n".to_vec()),
   ("pendiente".into(),b"# Pending\n\nThe gateway should log warnings.\n".to_vec()),
  ]);
  if name=="unicode"{texts.insert("propuesto".into(),format!("# Actual\n\nThe gateway should preserve logs about {}.\n","á".repeat(30)).into_bytes());texts.insert("r1".into(),format!("# Reference\n\nThe gateway should not preserve logs about {}.\n","á".repeat(30)).into_bytes());}
  if name.starts_with("traduccion"){texts.insert("propuesto".into(),"# Política actual\n\nLa pasarela debería registrar todos los eventos.\n".as_bytes().to_vec());}
  for (id,raw) in &texts{alta(&a,id,raw,id!="sin-permiso",id!="pendiente",if name=="formato"&&id=="r1"{"html"}else{"md"});}
  if name=="traduccion_sin_permiso"{let usos=Uso::todos().into_iter().map(|u|{let p=if u==Uso::Traduccion{Permision::NoPermitida}else{Permision::Permitida};(u,p)}).collect();act(&a,Accion::ActualizarAlcance{recurso:"propuesto".into(),fundamento:"Control sin traducción".into(),evidencia:"Protocolo".into(),condiciones:"Ensayo".into(),hasta:8000,usos});act(&a,Accion::RevisarLicencia{recurso:"propuesto".into(),favorable:true,informe:"Control".into()});act(&a,Accion::AprobarLicencia{recurso:"propuesto".into()});}
  let selection=match name{
   "omitida"=>Seleccion::Omitir,"grupo"=>Seleccion::Grupo(vec!["r2".into(),"r1".into()]),"todos"=>Seleccion::Todos,"vacio"=>Seleccion::Grupo(vec![]),"duplicado"=>Seleccion::Grupo(vec!["r1".into(),"r1".into()]),"propio"=>Seleccion::Documento("propuesto".into()),"ausente"=>Seleccion::Grupo(vec!["r1".into(),"inmunologia-ajena".into()]),"sin_permiso"=>Seleccion::Documento("sin-permiso".into()),"cota_grupo"=>Seleccion::Grupo((0..129).map(|i|format!("r-{i}")).collect()),"sin_aviso"=>Seleccion::Documento("r2".into()),_=>Seleccion::Documento("r1".into())};
  let p=Peticion{dominio:if name=="dominio"{"inmunologia"}else{"ciberseguridad"}.into(),propuesto:"propuesto".into(),seleccion:selection};
  let session=Sesion::desde_adaptador(if name=="sin_admision"{99}else{11},if name=="sesion_vencida"{100}else{9000},"ciberseguridad");
  let initial=serde_json::to_vec(&a.historial()?)?;
  let before=huella(&initial);let mut m=Medicion::default();let mut changed=false;
  let result=consultar(&a,&session,&p,101,|c|{
   if name=="cambio_concurrente"&&!changed{changed=true;act(&a,Accion::RetirarLicencia{recurso:"r1".into()});}
   let mut original=texts[&c.id].clone();if name=="original_alterado"{original.push(b'x');}
   let tr=if name.starts_with("traduccion")&&c.id=="propuesto"{let ingles="# Current policy\n\nThe gateway should log all events.\n".to_string();Some(Traduccion{original_sha256:if name=="traduccion_alterada"{"0".repeat(64)}else{huella(&original)},ingles_sha256:huella(ingles.as_bytes()),ingles,procedencia:"Traducción sintética de control; no nueva consulta a Google".into()})}else{None};
   Ok(Contenido{original,idioma:if tr.is_some(){"es"}else{"en"}.into(),traduccion:tr})
  },&mut m);
  let expected_error=match name{"duplicado"|"propio"=>Some("SELECCION_DUPLICADA_O_PROPIA"),"ausente"=>Some("CARGA_AUSENTE"),"dominio"=>Some("DOMINIO_DISCORDANTE"),"sin_permiso"=>Some("USO_COMPARACION_NO_ACREDITADO"),"sesion_vencida"|"sin_admision"=>Some("ADMISION_EXPRESA_REQUERIDA"),"original_alterado"=>Some("ORIGINAL_DISCORDANTE"),"traduccion_alterada"=>Some("TRADUCCION_DISCORDANTE"),"cambio_concurrente"=>Some("REGISTRO_CAMBIADO"),"formato"=>Some("REPRESENTACION_MARKDOWN_REQUERIDA"),"traduccion_sin_permiso"=>Some("TRADUCCION_NO_ACREDITADA"),"cota_grupo"=>Some("SELECCION_SUPERA_COTA"),_=>None};
  let final_state=serde_json::to_vec(&a.historial()?)?;
  let after=huella(&final_state);
  fs::write(dest.join(format!("{name}-registro.json")),&final_state)?;
  if changed {fs::write(dest.join(format!("{name}-registro-inicial.json")),initial)?;}
  let mut ok=match (&result,expected_error){(Err(e),Some(expected))=>e==expected,(Ok(_),None)=>true,_=>false};
  let mut span_ok=true;
  if let Ok(v)=&result{
   let expected_ids:Vec<String>=match name{"omitida"|"vacio"=>vec![],"grupo"=>vec!["r2".into(),"r1".into()],"todos"=>vec!["r1".into(),"r2".into()],"sin_aviso"=>vec!["r2".into()],_=>vec!["r1".into()]};
   ok &= v.seleccionados==expected_ids;
   let expected_alerts=if ["omitida","vacio","sin_aviso"].contains(&name){0}else{1};ok &= v.sugerencias.len()==expected_alerts;
   for sug in &v.sugerencias{span_ok &= sug.positivo.documento!=sug.negativo.documento;for psg in [&sug.positivo,&sug.negativo]{let d=v.documentos.iter().find(|d|d.id==psg.documento).unwrap();span_ok &= d.ingles.get(psg.localizacion.byte_start..psg.localizacion.byte_end)==Some(psg.texto.as_str());}}
  }
  let expected_calls=if expected_error.is_some()||["omitida","vacio"].contains(&name){0}else if ["grupo","todos"].contains(&name){2}else{1};
  ok &= m.llamadas_comparador==expected_calls && span_ok && (changed||before==after);
  if expected_error.is_some()&&!matches!(name,"original_alterado"|"traduccion_alterada"|"traduccion_sin_permiso"|"cambio_concurrente"){ok &= m.lecturas==0;}
  total_calls+=m.llamadas_comparador;total_reads+=m.lecturas;
  rows.push(json!({"id":name,"conforme":ok,"error_esperado":expected_error,"error_obtenido":result.as_ref().err(),"medicion":m,"registro_inicial_sha256":before,"registro_final_sha256":after,"cambio_externo_simulado":changed,"dos_localizaciones_cotejadas":span_ok,"resultado":result.as_ref().ok()}));
 }
 let ok=rows.iter().all(|v|v["conforme"]==true);
 let out=json!({"elemento":"Buscador-Semántico - Diferencial","paso":"7 / 8","ejecutable_sha256":huella(&fs::read(std::env::current_exe()?)?),"todos_conformes":ok,"controles":rows,"llamadas_comparador":total_calls,"lecturas":total_reads,"identidades":"sintéticas; no autenticación de navegador","inferencias":0,"solicitudes_traduccion":0,"desplegado":false});
 let mut f=OpenOptions::new().write(true).create_new(true).open(dest.join("RESULTADO.json"))?;f.write_all(&serde_json::to_vec_pretty(&out)?)?;f.sync_all()?;
 println!("{} controles; {} llamadas instrumentales; conformidad: {}",tests.len(),total_calls,ok);
 if !ok{return Err("Comprobaciones no conformes; conservar resultado".into())}Ok(())
}
