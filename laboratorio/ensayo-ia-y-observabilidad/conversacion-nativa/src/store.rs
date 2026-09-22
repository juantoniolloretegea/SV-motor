use super::{Context,Profile,Result};
use serde::{Serialize,Deserialize};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs::{self,File,OpenOptions},io::{Read,Write},path::{Path,PathBuf},os::unix::fs::{OpenOptionsExt,PermissionsExt},sync::atomic::{AtomicU64,Ordering},time::{SystemTime,UNIX_EPOCH}};
pub const MAX_STORAGE:u64=512*1024*1024;
static COUNT:AtomicU64=AtomicU64::new(0);
pub fn now()->u128{SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis()}
pub fn id(prefix:&str)->String{format!("{prefix}-{}-{}",now(),COUNT.fetch_add(1,Ordering::SeqCst))}
pub fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
pub fn file_hash(p:&Path)->Result<String>{let mut f=File::open(p)?;let mut sha=Sha256::new();let mut b=[0u8;65536];loop{let n=f.read(&mut b)?;if n==0{break}sha.update(&b[..n])}Ok(format!("{:x}",sha.finalize()))}
#[derive(Clone,Serialize,Deserialize)]pub struct Case{pub id:String,pub title:String,pub created_ms:u128}
#[derive(Clone,Serialize,Deserialize)]pub struct Chat{pub id:String,pub case_id:String,pub title:String,pub turns:Vec<Turn>}
#[derive(Clone,Serialize,Deserialize)]pub struct Turn{pub id:String,pub user:String,pub raw:String,pub answer:String,pub thinking:String,pub status:String,pub context:Context,pub profile:Profile,pub result:Value}
#[derive(Clone,Serialize,Deserialize)]#[serde(deny_unknown_fields)]pub struct Event{pub schema:String,pub case_id:String,pub seq:u64,pub utc_ms:u128,pub kind:String,pub data:Value,pub previous_sha256:String,pub sha256:String}
pub struct Database{pub cases:BTreeMap<String,Case>,pub chats:BTreeMap<String,Chat>,pub events:BTreeMap<String,Vec<Event>>,pub bytes:u64,pub recovered:usize,root:PathBuf}
impl Event{fn digest(&self)->Result<String>{Ok(hash(&serde_json::to_vec(&json!({"schema":self.schema,"case_id":self.case_id,"seq":self.seq,"utc_ms":self.utc_ms,"kind":self.kind,"data":self.data,"previous_sha256":self.previous_sha256}))?))}}
impl Database{
 pub fn open(root:PathBuf)->Result<Self>{
  fs::create_dir_all(&root)?;fs::set_permissions(&root,fs::Permissions::from_mode(0o700))?;
  let mut db=Self{cases:BTreeMap::new(),chats:BTreeMap::new(),events:BTreeMap::new(),bytes:0,recovered:0,root:root.clone()};
  let mut paths:Vec<_>=fs::read_dir(&root)?.filter_map(|e|e.ok()).map(|e|e.path()).filter(|p|p.extension().is_some_and(|s|s=="jsonl")).collect();paths.sort();
  for p in paths{let bytes=fs::read(&p)?;db.bytes+=bytes.len() as u64;if bytes.last().is_some_and(|b|*b!=b'\n'){return Err(format!("Registro incompleto, conservado sin modificación: {}",p.display()).into())}
   for line in bytes.split(|b|*b==b'\n').filter(|s|!s.is_empty()){let e:Event=serde_json::from_slice(line)?;let prior=db.events.get(&e.case_id).and_then(|v|v.last());if e.seq!=prior.map_or(1,|x|x.seq+1)||e.previous_sha256!=prior.map_or(String::new(),|x|x.sha256.clone())||e.sha256!=e.digest()?||p.file_stem().and_then(|s|s.to_str())!=Some(e.case_id.as_str()){return Err(format!("Integridad no conforme en {}",p.display()).into())}db.apply(&e)?;db.events.entry(e.case_id.clone()).or_default().push(e);}
  }
  let pending:Vec<_>=db.chats.values().flat_map(|c|c.turns.iter().filter(|t|t.status=="en_curso").map(|t|(c.case_id.clone(),c.id.clone(),t.id.clone(),t.raw.clone(),t.profile.thinking))).collect();
  db.recovered=pending.len();
  for(case,chat,id,raw,mode)in pending{let(thinking,answer)=super::model::split(&raw,mode);db.append(&case,"respuesta_finalizada",json!({"chat_id":chat,"request_id":id,"raw":raw,"thinking":thinking,"answer":answer,"finish":"interrumpida_por_reinicio","error":"La sesión terminó sin un cierre de generación registrado; se conserva la última salida registrada."}))?;}
  Ok(db)
 }
 pub fn append(&mut self,case:&str,kind:&str,data:Value)->Result<()>{
  let previous=self.events.get(case).and_then(|v|v.last());let mut e=Event{schema:"EIO-SUCESO-1".into(),case_id:case.into(),seq:previous.map_or(1,|e|e.seq+1),utc_ms:now(),kind:kind.into(),data,previous_sha256:previous.map_or(String::new(),|e|e.sha256.clone()),sha256:String::new()};e.sha256=e.digest()?;
  let mut bytes=serde_json::to_vec(&e)?;bytes.push(b'\n');if self.bytes+bytes.len() as u64>MAX_STORAGE{return Err("Se alcanzó el límite de conservación; no se ha borrado ningún registro".into())}
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
