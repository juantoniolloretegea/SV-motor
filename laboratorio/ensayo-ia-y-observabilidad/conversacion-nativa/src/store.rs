use super::{Context,Profile,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs::{self,File,OpenOptions},io::{Read,Write},path::{Path,PathBuf},os::unix::fs::{OpenOptionsExt,PermissionsExt},sync::atomic::{AtomicU64,Ordering},time::{SystemTime,UNIX_EPOCH}};
pub const MAX_STORAGE:u64=512*1024*1024;
pub const MAX_EVENT:u64=4*1024*1024;
// Dos exportadores de 10 MiB, ciclo de servicio de 16 MiB y 1 MiB de estados.
const LOG_RESERVE:u64=37*1024*1024;
static COUNT:AtomicU64=AtomicU64::new(0);
pub fn now()->u128{SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis()}
pub fn id(prefix:&str)->String{format!("{prefix}-{}-{}",now(),COUNT.fetch_add(1,Ordering::SeqCst))}
pub fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
pub fn file_hash(p:&Path)->Result<String>{let mut f=File::open(p)?;let mut sha=Sha256::new();let mut b=[0u8;65536];loop{let n=f.read(&mut b)?;if n==0{break}sha.update(&b[..n])}Ok(format!("{:x}",sha.finalize()))}
#[derive(Clone,Serialize,Deserialize)]pub struct Case{pub id:String,pub title:String,pub created_ms:u128}
#[derive(Clone,Serialize,Deserialize)]pub struct Chat{pub id:String,pub case_id:String,pub title:String,pub turns:Vec<Turn>}
#[derive(Clone,Serialize,Deserialize)]pub struct Turn{pub id:String,pub user:String,pub raw:String,pub answer:String,pub thinking:String,pub status:String,pub context:Context,pub profile:Profile,pub result:Value}
#[derive(Clone,Serialize,Deserialize)]#[serde(deny_unknown_fields)]pub struct Event{pub schema:String,pub case_id:String,pub seq:u64,pub utc_ms:u128,pub kind:String,pub data:Value,pub previous_sha256:String,pub sha256:String}
pub struct Database{pub cases:BTreeMap<String,Case>,pub chats:BTreeMap<String,Chat>,pub events:BTreeMap<String,Vec<Event>>,pub bytes:u64,pub limit:u64,pub recovered:usize,root:PathBuf}
impl Event{fn digest(&self)->Result<String>{Ok(hash(&serde_json::to_vec(&json!({"schema":self.schema,"case_id":self.case_id,"seq":self.seq,"utc_ms":self.utc_ms,"kind":self.kind,"data":self.data,"previous_sha256":self.previous_sha256}))?))}}
impl Database{
 pub fn open(root:PathBuf)->Result<Self>{
  fs::create_dir_all(&root)?;fs::set_permissions(&root,fs::Permissions::from_mode(0o700))?;
  let mut db=Self{cases:BTreeMap::new(),chats:BTreeMap::new(),events:BTreeMap::new(),bytes:0,limit:MAX_STORAGE,recovered:0,root:root.clone()};
  let mut paths:Vec<_>=fs::read_dir(&root)?.filter_map(|e|e.ok()).map(|e|e.path()).filter(|p|p.extension().is_some_and(|s|s=="jsonl")).collect();paths.sort();
  for p in paths{let bytes=fs::read(&p)?;db.bytes+=bytes.len() as u64;if bytes.last().is_some_and(|b|*b!=b'\n'){return Err(format!("Registro incompleto, conservado sin modificación: {}",p.display()).into())}
   for line in bytes.split(|b|*b==b'\n').filter(|s|!s.is_empty()){let e:Event=serde_json::from_slice(line)?;let prior=db.events.get(&e.case_id).and_then(|v|v.last());if e.seq!=prior.map_or(1,|x|x.seq+1)||e.previous_sha256!=prior.map_or(String::new(),|x|x.sha256.clone())||e.sha256!=e.digest()?||p.file_stem().and_then(|s|s.to_str())!=Some(e.case_id.as_str()){return Err(format!("Integridad no conforme en {}",p.display()).into())}db.apply(&e)?;db.events.entry(e.case_id.clone()).or_default().push(e);}
  }
  // Cuota de expedientes ajustada al almacenamiento auxiliar ya existente.
  // Se reserva además el máximo de trazas de esta instancia y del ciclo de servicio.
  let total=directory_bytes(&root)?;
  db.limit=MAX_STORAGE.checked_sub(total.saturating_sub(db.bytes)).and_then(|n|n.checked_sub(LOG_RESERVE)).ok_or("Almacenamiento agregado sin margen")?;
  let pending:Vec<_>=db.chats.values().flat_map(|c|c.turns.iter().filter(|t|t.status=="en_curso").map(|t|(c.case_id.clone(),c.id.clone(),t.id.clone(),t.raw.clone(),t.profile.thinking))).collect();
  db.recovered=pending.len();
  for(case,chat,id,raw,mode)in pending{let(thinking,answer)=super::model::split(&raw,mode);db.append(&case,"respuesta_finalizada",json!({"chat_id":chat,"request_id":id,"raw":raw,"thinking":thinking,"answer":answer,"finish":"interrumpida_por_reinicio","error":"La sesión terminó sin un cierre de generación registrado; se conserva la última salida registrada."}))?;}
  Ok(db)
 }
 pub fn append(&mut self,case:&str,kind:&str,data:Value)->Result<()>{
  let previous=self.events.get(case).and_then(|v|v.last());let mut e=Event{schema:"EIO-SUCESO-1".into(),case_id:case.into(),seq:previous.map_or(1,|e|e.seq+1),utc_ms:now(),kind:kind.into(),data,previous_sha256:previous.map_or(String::new(),|e|e.sha256.clone()),sha256:String::new()};e.sha256=e.digest()?;
  let mut bytes=serde_json::to_vec(&e)?;bytes.push(b'\n');if bytes.len() as u64>MAX_EVENT{return Err("Suceso excede la cota individual; original conservado".into())}
  let limit=if kind=="respuesta_finalizada"{self.limit}else{self.limit.saturating_sub(MAX_EVENT)};
  if self.bytes+bytes.len() as u64>limit{return Err("Se alcanzó el límite de conservación; reserva de cierre protegida".into())}
  let path=self.root.join(format!("{case}.jsonl"));let mut file=OpenOptions::new().create(true).append(true).mode(0o600).open(&path)?;file.write_all(&bytes)?;file.sync_all()?;File::open(&self.root)?.sync_all()?;
  self.bytes+=bytes.len() as u64;self.apply(&e)?;self.events.entry(case.into()).or_default().push(e);Ok(())
 }
 fn apply(&mut self,e:&Event)->Result<()>{
  let s=|k:&str|e.data[k].as_str().unwrap_or("").to_string();
  match e.kind.as_str(){
   "expediente_creado"=>{self.cases.insert(e.case_id.clone(),Case{id:e.case_id.clone(),title:s("title"),created_ms:e.utc_ms});},
   "conversacion_creada"=>{let id=s("id");self.chats.insert(id.clone(),Chat{id,case_id:e.case_id.clone(),title:s("title"),turns:vec![]});},
   "peticion_admitida"=>{let c=self.chats.get_mut(&s("chat_id")).ok_or("Conversación ausente del registro")?;c.turns.push(Turn{id:s("request_id"),user:s("user"),raw:String::new(),answer:String::new(),thinking:String::new(),status:"en_curso".into(),context:serde_json::from_value(e.data["context"].clone())?,profile:serde_json::from_value(e.data["profile"].clone())?,result:Value::Null});},
   "respuesta_parcial"|"respuesta_finalizada"=>{let c=self.chats.get_mut(&s("chat_id")).ok_or("Conversación ausente del registro")?;let t=c.turns.iter_mut().find(|t|t.id==s("request_id")).ok_or("Petición ausente del registro")?;t.raw=s("raw");if e.kind=="respuesta_finalizada"{t.answer=s("answer");t.thinking=s("thinking");t.status=s("finish");t.result=e.data.clone();}},
   "contexto_excedido"=>{},_=>return Err(format!("Suceso no reconocido: {}",e.kind).into())
  }Ok(())
 }
}
pub fn check_cli()->Result<()>{let p=PathBuf::from(std::env::args().nth(2).ok_or("Uso: --check DIRECTORIO; reconstruye el registro y cierra peticiones pendientes")?);let _guard=super::lifecycle::Lifecycle::open(&p)?;let db=Database::open(p)?;println!("{}",json!({"integridad":"conforme","expedientes":db.cases.len(),"conversaciones":db.chats.len(),"sucesos":db.events.values().map(Vec::len).sum::<usize>(),"bytes":db.bytes,"peticiones_recuperadas":db.recovered}));Ok(())}
#[cfg(test)]mod tests{use super::*;
 #[test]fn recuperacion_y_alteracion(){let p=std::env::temp_dir().join(id("eio-check"));let mut db=Database::open(p.clone()).unwrap();db.append("exp-1","expediente_creado",json!({"title":"Prueba"})).unwrap();db.append("exp-1","conversacion_creada",json!({"id":"chat-1","title":"Conversación"})).unwrap();drop(db);let db=Database::open(p.clone()).unwrap();assert_eq!(db.chats["chat-1"].title,"Conversación");drop(db);let file=p.join("exp-1.jsonl");let s=fs::read_to_string(&file).unwrap().replace("Prueba","Cambio");fs::write(&file,s).unwrap();assert!(Database::open(p.clone()).is_err());fs::remove_dir_all(p).unwrap();}
}

fn directory_bytes(root:&Path)->Result<u64>{
 let mut pending=vec![root.to_path_buf()];let mut total=0u64;let mut entries=0;
 while let Some(path)=pending.pop(){
  entries+=1;if entries>10000{return Err("Inventario de almacenamiento excedido".into())}
  let m=fs::symlink_metadata(&path)?;
  if m.file_type().is_symlink(){return Err("Enlace simbólico en la conservación".into())}
  if m.is_dir(){for entry in fs::read_dir(path)?{pending.push(entry?.path());}}
  else if m.is_file(){total=total.checked_add(m.len()).ok_or("Tamaño excedido")?;}
  else{return Err("Tipo de archivo no admitido en la conservación".into())}
 }Ok(total)
}
// Recuperación expresa en una sede nueva. El original y su copia completa se conservan.
pub fn recover(source:&Path,destination:&Path)->Result<Value>{
 let metadata=fs::symlink_metadata(source)?;if !metadata.is_file()||metadata.len()>MAX_STORAGE{return Err("Origen de recuperación no admitido".into())}
 let original=fs::read(source)?;if original.len() as u64>MAX_STORAGE{return Err("Origen excedido".into())}
 let name=source.file_name().ok_or("Nombre de expediente ausente")?;
 if source.extension().and_then(|s|s.to_str())!=Some("jsonl"){return Err("Se requiere un expediente JSONL".into())}
 fs::create_dir(destination)?;
 fs::set_permissions(destination,fs::Permissions::from_mode(0o700))?;
 let originals=destination.join("originales");fs::create_dir(&originals)?;
 let full=originals.join(name);let mut copy=OpenOptions::new().write(true).create_new(true).mode(0o600).open(&full)?;
 copy.write_all(&original)?;copy.sync_all()?;
 let prefix=original.iter().rposition(|b|*b==b'\n').map_or(0,|n|n+1);
 if prefix==0{return Err("Sin suceso completo recuperable; copia íntegra conservada".into())}
 let recovered=destination.join("recuperado");fs::create_dir(&recovered)?;
 let mut derived=OpenOptions::new().write(true).create_new(true).mode(0o600).open(recovered.join(name))?;
 derived.write_all(&original[..prefix])?;derived.sync_all()?;
 let validation=Database::open(recovered.clone());
 let report=json!({"schema":"EIO-RECUPERACION-1","original_sha256":hash(&original),"original_bytes":original.len(),
  "prefix_sha256":hash(&original[..prefix]),"prefix_bytes":prefix,"incomplete_tail_bytes":original.len()-prefix,
  "incomplete_tail_sha256":hash(&original[prefix..]),"original_preserved":fs::read(source)?==original&&fs::read(full)?==original,
  "derived_valid":validation.is_ok(),"recovered_requests":validation.as_ref().ok().map(|db|db.recovered),
  "error":validation.as_ref().err().map(|e|e.to_string()),
  "scope":"Se valida el prefijo y se cierran peticiones pendientes en una copia nueva. No reconstruye sucesos perdidos ni acredita ausencia de supresión de sufijos completos."});
 let mut manifest=OpenOptions::new().write(true).create_new(true).mode(0o600).open(destination.join("RECUPERACION.json"))?;
 manifest.write_all(&serde_json::to_vec_pretty(&report)?)?;manifest.sync_all()?;
 File::open(&originals)?.sync_all()?;File::open(&recovered)?.sync_all()?;File::open(destination)?.sync_all()?;
 validation?;if report["original_preserved"]!=true{return Err("El original cambió durante la recuperación; consulte el manifiesto".into())}Ok(report)
}
pub fn recover_cli()->Result<()>{
 let a:Vec<_>=std::env::args().collect();
 if a.len()!=4{return Err("Uso: --recover EXPEDIENTE_JSONL DIRECTORIO_NUEVO".into())}
 println!("{}",recover(Path::new(&a[2]),Path::new(&a[3]))?);Ok(())
}
#[cfg(test)]mod recovery_tests{
 use super::*;
 #[test]fn prefijo_derivado_y_original_intacto(){
  let p=std::env::temp_dir().join(id("recuperacion"));let mut db=Database::open(p.clone()).unwrap();
  db.append("exp-1","expediente_creado",json!({"title":"Dato sintético"})).unwrap();drop(db);
  let f=p.join("exp-1.jsonl");OpenOptions::new().append(true).open(&f).unwrap().write_all(b"{incompleto").unwrap();
  let original=fs::read(&f).unwrap();assert!(Database::open(p.clone()).is_err());
  let out=p.with_extension("recuperacion");let r=recover(&f,&out).unwrap();
  assert_eq!(r["original_preserved"],true);assert_eq!(r["incomplete_tail_bytes"],11);
  assert_eq!(fs::read(f).unwrap(),original);assert!(Database::open(out.join("recuperado")).unwrap().cases.contains_key("exp-1"));
  fs::remove_dir_all(p).unwrap();fs::remove_dir_all(out).unwrap();
 }
 #[test]fn reserva_cierre_y_cuota_agregada(){
  let p=std::env::temp_dir().join(id("reserva"));let mut db=Database::open(p.clone()).unwrap();
  db.append("exp-1","expediente_creado",json!({"title":"Dato"})).unwrap();
  let base=db.bytes;db.limit=base+MAX_EVENT+200;
  assert!(db.append("exp-2","expediente_creado",json!({"title":"x".repeat(1000)})).is_err());
  // Un cierre no agota su reserva en comprobaciones ni modifica la cadena si el chat no existe.
  assert!(db.limit-db.bytes>=MAX_EVENT);
  drop(db);
  fs::create_dir(p.join("servicio")).unwrap();
  fs::write(p.join("servicio/auxiliar.bin"),vec![0u8;4096]).unwrap();
  let db=Database::open(p.clone()).unwrap();assert_eq!(db.limit,MAX_STORAGE-4096-LOG_RESERVE);
  drop(db);fs::remove_dir_all(p).unwrap();
 }
}
