use std::{fs,io::Write,path::{Path,Component}};use serde_json::{Value,json};use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn ck(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn save(p:&Path,b:&[u8])->R<()>{fs::create_dir_all(p.parent().ok_or("Directorio")?)?;let mut f=fs::OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;Ok(())}
fn replace(s:&mut String,a:&str,b:&str)->R<()>{ck(s.contains(a),&format!("Patrón ausente: {a}"))?;*s=s.replace(a,b);Ok(())}
fn copy(src:&Path,dst:&Path)->R<()>{for e in fs::read_dir(src)?{let e=e?;let p=e.path();let meta=fs::symlink_metadata(&p)?;ck(!meta.file_type().is_symlink(),"Enlace rechazado")?;if meta.is_dir(){copy(&p,&dst.join(e.file_name()))?;continue}
 let mut s=fs::read_to_string(&p)?.replace("\r\n","\n").replace("diagnostico-A08-01","diagnostico-A08-02").replace("diag-A08-01","diag-A08-02").replace("diag-a08-01","diag-a08-02").replace("diag01","diag02");
 if p.ends_with("instrumentacion/src/retroalimentacion.rs"){replace(&mut s,"pub const SALIDA: usize = 2048;","pub const SALIDA: usize = 4096;")?;}
 if p.ends_with("conductor/src/main.rs"){
  replace(&mut s,"reasoning_effort=>\"medium\"","reasoning_effort=>\"high\"")?;
  replace(&mut s,"\"intento\":1,\"razonamiento\":\"medium\"","\"intento\":2,\"razonamiento\":\"high\"")?;
 }
 if p.ends_with("verificador/src/main.rs"){replace(&mut s,"reasoning_effort=>\"medium\"","reasoning_effort=>\"high\"")?;}
 if p.ends_with("instrumentacion/src/bin/recuperar.rs"){
  replace(&mut s,"sampling[\"max_len\"]!=2048","sampling[\"max_len\"]!=4096")?;
  replace(&mut s,"generaciones[0][\"datos\"][\"semilla\"]!=42","generaciones[0][\"datos\"][\"semilla\"]!=42||generaciones[0][\"datos\"][\"razonamiento\"]!=\"high\"||generaciones[0][\"datos\"][\"intento\"]!=2")?;
  replace(&mut s,"\"intento\":1,\"caso\":\"A08\"","\"intento\":2,\"caso\":\"A08\"")?;
 }
 if p.ends_with("instrumentacion/src/bin/comprobar_base.rs"){
  replace(&mut s,"let base=Path::new(a.get(1).ok_or(\"Directorio requerido\")?);",r#"let base=Path::new(a.get(1).ok_or("Directorio requerido")?);
 let manifest:Value=serde_json::from_slice(&fs::read(base.join("MANIFIESTO-PREPARACION.json"))?)?;
 for f in manifest["archivos"].as_array().ok_or("Manifiesto ausente")?{let n=f["ruta"].as_str().ok_or("Ruta")?;check(Path::new(n).components().all(|c|matches!(c,std::path::Component::Normal(_))),"Ruta fuera del perfil")?;let p=base.join(n);check(!fs::symlink_metadata(&p)?.file_type().is_symlink(),"Enlace rechazado")?;let(len,sha)=hash_file(&p)?;check(f["bytes"]==len&&f["sha256"]==sha,"Preparación recibida discordante")?;}
 "#)?;
 }
 save(&dst.join(e.file_name()),s.as_bytes())?;
 }Ok(())}
fn inventory(base:&Path,p:&Path,files:&mut Vec<Value>)->R<()>{for e in fs::read_dir(p)?{let e=e?;let p=e.path();ck(!fs::symlink_metadata(&p)?.file_type().is_symlink(),"Enlace rechazado")?;if p.is_dir(){inventory(base,&p,files)?}else{let rel=p.strip_prefix(base)?;ck(rel.components().all(|c|matches!(c,Component::Normal(_))),"Ruta")?;let bytes=fs::read(&p)?;files.push(json!({"ruta":rel.to_string_lossy().replace('\\',"/"),"bytes":bytes.len(),"sha256":h(&bytes)}));}}Ok(())}
fn main()->R<()>{let a=std::env::args().nth(1).ok_or("Directorio")?;let base=Path::new(&a);let prev=base.join("diagnostico-A08-01");let new=base.join("diagnostico-A08-02");ck(!new.exists(),"D02 ya preparado")?;
 let result:Value=serde_json::from_slice(&fs::read(prev.join("RESULTADO-D01.json"))?)?;ck(result["intento"]==1&&result["cierre_normal"]==true&&result["solventado"]==false,"D01 no está resuelto y cotejado para esta continuación")?;
 for name in ["conductor","custodio","instrumentacion","verificador"]{copy(&prev.join(name),&new.join(name))?;}
 for name in ["cache/catalogo.json","config/politica.txt","config/plan.json","config/MODELO-REFERENCIA.json","config/NUCLEO-REFERENCIA.json"]{save(&new.join(name),&fs::read(prev.join(name))?)?;}
 let mut contract:Value=serde_json::from_slice(&fs::read(prev.join("config/contrato.json"))?)?;contract["salida_maxima_tokens"]=json!(4096);save(&new.join("config/contrato.json"),&serde_json::to_vec_pretty(&contract)?)?;
 let prep=fs::read_to_string(prev.join("preparar.rs"))?.replace("diagnostico-A08-01","diagnostico-A08-02");save(&new.join("preparar.rs"),prep.as_bytes())?;
 copy(&prev.join("pensamiento-afinado"),&new.join("pensamiento-afinado"))?;
 for n in ["cotejos","autorizaciones"]{fs::create_dir_all(new.join(n))?;}
 let condition=json!({"intento":2,"maximo_total_intentos":3,"caso":"A08","referencia_controlada":"D01 y A2/A08","razonamiento":"high","cambio_principal":"Esfuerzo medium a high en la plantilla Harmony efectiva, con muestreo probabilístico idéntico a D01","fundamento":"D01 conservó el error sin truncamiento. A3 ensayó high con selección determinista y tres antecedentes; no contrastó high con el muestreo de D01 y sus mismos dos antecedentes. Se comprueba esta combinación sin entregar una señal de la respuesta correcta.","fuentes_politica_pregunta_antecedentes_identicos_a_d01":true,"antecedentes":2,"d01_no_se_entrega_como_antecedente":true,"temperatura":1.0,"top_p":1.0,"top_k":null,"semilla":42,"penalizaciones":false,"cota_salida_tokens":4096,"cota_salida_d01":2048,"justificacion_margen":"A3/A08 alcanzó 2566 tokens. Se concede margen a high sin obligar al modelo a agotarlo; el cierre nativo sigue siendo obligatorio.","limite_total_segundos":3600,"limite_sin_progreso_segundos":900,"maximo_generaciones":1,"custodio_gib":2,"modelo_gib":110,"conjunto_gib":114,"nuevos_recursos":false,"criterio":"Clasificación y justificación coherentes con la política y la clave externa, y requisitos formales evaluados por separado. Una respuesta aislada no concede acceso al examen.","limitaciones":"Si se alcanza una cota, se conserva la limitación instrumental; no se inventa U ni se repite con otra semilla. High y mayor margen de salida son cambios declarados; no prueba equivalencia numérica independiente ni se interpreta como entrenamiento de pesos.","nucleo_semantica_ir_pesos":"Sin cambios","github_final":"Pospuesto hasta terminar las pruebas"});
 save(&new.join("CONDICION.json"),&serde_json::to_vec_pretty(&condition)?)?;
 let mut files=Vec::new();inventory(&new,&new,&mut files)?;files.sort_by_key(|f|f["ruta"].as_str().unwrap().to_string());
 save(&new.join("MANIFIESTO-PREPARACION.json"),&serde_json::to_vec_pretty(&json!({"intento":2,"archivos":files,"sin_clave":true}))?)?;
 println!("D02 preparado: high, mismos documentos y antecedentes; margen 4096, máximo una generación. Sin carga ni inferencia.");Ok(())}
