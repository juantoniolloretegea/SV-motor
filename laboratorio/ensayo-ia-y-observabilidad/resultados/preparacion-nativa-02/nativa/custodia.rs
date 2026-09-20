//! Único hilo escritor. La API de control sólo usa try_send/try_recv.
//! Un bloqueo write/sync no bloquea detección ni TERM/KILL; sí impide ACK de sellado.
use super::*;
use std::{fs::{self,File,OpenOptions},io::{Read,Write},path::{Path,PathBuf},os::unix::fs::PermissionsExt,sync::{Arc,atomic::{AtomicBool,Ordering},mpsc::{self,SyncSender,Receiver}},time::Instant};
use nix::sys::statvfs::statvfs;
use sha2::{Digest,Sha256};
const DISK_MARGIN:u64=2*1024*1024*1024;
const MAX_FILE:u64=8*1024*1024;
const MAX_TOTAL:u64=20*1024*1024;
#[derive(Debug)]
pub struct Snapshot{pub sello:String,pub originales:Vec<(String,Vec<u8>)>,pub completa:bool}
impl Snapshot{
 pub fn fragmento(&self,nombre:&str,offset:u64,sello:&str)->Value{
  if sello!=self.sello{return json!({"error":"SELLO_DISTINTO"})}
  let Some((_,b))=self.originales.iter().find(|(p,_)|p==nombre) else{return json!({"error":"ARCHIVO"})};
  let Ok(i)=usize::try_from(offset) else{return json!({"error":"OFFSET"})};
  if i>b.len(){return json!({"error":"OFFSET"})}
  let fin=(i+32768).min(b.len());
  json!({"archivo":nombre,"sello":self.sello,"offset":offset,"total":b.len(),"hex":b[i..fin].iter().map(|x|format!("{x:02x}")).collect::<String>(),"fin":fin==b.len(),"parcial":!self.completa})
 }
}
enum Trabajo{Datos(u64,usize,Vec<u8>),Entrada(u64,Vec<u8>),Evento(u64,String,Value),Sellar(u64,Admision,bool)}
pub struct Custodia{
 tx:SyncSender<Trabajo>,rx:Receiver<Result<Snapshot,String>>,pub fallo:Arc<AtomicBool>,
 pub fase:&'static str,pub corte:u64,pub tardios_bytes:u64,pub snapshot:Option<Snapshot>,
}
impl Custodia{
 pub fn nueva(dir:PathBuf,caso:String)->Result<Self,Error>{
  let(tx,work)=mpsc::sync_channel(64);let(done,rx)=mpsc::sync_channel(1);let fallo=Arc::new(AtomicBool::new(false));let ff=fallo.clone();
  std::thread::spawn(move||{
   let run=(||->Result<Snapshot,Error>{
    let mut d=Disco::nueva(&dir)?;d.fallo_inyectado=caso=="escritura";let mut ultimo=0;
    while let Ok(t)=work.recv(){
     match t{
      Trabajo::Datos(n,i,b)=>{if n!=ultimo+1{return Err("COLA_SECUENCIA".into())}ultimo=n;
       // FIFO de prueba: open/write realmente bloquean; no error inmediato sustitutorio.
       if caso=="bloqueo_antes" && i==0 && d.bytes[0]==0{bloquear_fifo(&dir)?;}
       if caso=="cola_pendiente"{std::thread::sleep(std::time::Duration::from_millis(50));}
       d.escribir(i,&b)?;},
      Trabajo::Entrada(n,b)=>{if n!=ultimo+1{return Err("COLA_SECUENCIA".into())}ultimo=n;let mut f=OpenOptions::new().write(true).create_new(true).open(dir.join("entrada.txt"))?;f.write_all(&b)?;f.sync_all()?;},
      Trabajo::Evento(n,s,v)=>{if n!=ultimo+1{return Err("COLA_SECUENCIA".into())}ultimo=n;
       if caso=="bloqueo_entre" && s=="senal" && v["tipo"]=="SIGTERM"{bloquear_fifo(&dir)?;}
       d.evento(&s,v)?;},
      Trabajo::Sellar(n,mut a,laguna)=>{
       if n!=ultimo{return Err("CORTE_COLA".into())}
       if caso=="fallo_sync"{return Err("SYNC_INYECTADO".into())}
       if caso=="bloqueo_sellado"{bloquear_fifo(&dir)?;}
       // Estado final se calcula con la MISMA función, condicional a éxito de cerrar.
       a.sellado_ok=true;let completa=a.admisible();
       return d.cerrar(a.estado(),completa,n,laguna);
      }
     }
    }Err("CANAL_SIN_SELLO".into())
   })();
   if run.is_err(){ff.store(true,Ordering::SeqCst);}
   let _=done.try_send(run.map_err(|e|e.to_string()));
   // Retorna; nunca vuelve a abrir archivos sellados.
  });
  Ok(Self{tx,rx,fallo,fase:"recepcion",corte:0,tardios_bytes:0,snapshot:None})
 }
 fn enviar(&mut self,t:Trabajo)->Result<(),Error>{
  if self.tx.try_send(t).is_err(){self.fallo.store(true,Ordering::SeqCst);return Err("COLA_CUSTODIA".into())}Ok(())
 }
 pub fn escribir(&mut self,i:usize,b:&[u8])->Result<(),Error>{
  if self.fase!="recepcion"&&self.fase!="drenaje"{self.tardios_bytes=self.tardios_bytes.saturating_add(b.len() as u64);return Ok(())}
  self.corte+=1;self.enviar(Trabajo::Datos(self.corte,i,b.to_vec()))
 }
 pub fn entrada(&mut self,b:&[u8])->Result<(),Error>{
  self.corte+=1;self.enviar(Trabajo::Entrada(self.corte,b.to_vec()))
 }
 pub fn evento(&mut self,s:&str,v:Value)->Result<(),Error>{
  if self.fase!="recepcion"&&self.fase!="drenaje"{return Ok(())}
  self.corte+=1;self.enviar(Trabajo::Evento(self.corte,s.into(),v))
 }
 pub fn cerrar(&mut self,a:Admision,laguna:bool)->Result<(),Error>{
  if self.fase!="recepcion"&&self.fase!="drenaje"{return Err("SELLO_REPETIDO".into())}
  self.fase="sellando";self.enviar(Trabajo::Sellar(self.corte,a,laguna))
 }
 pub fn consultar(&mut self){
  match self.rx.try_recv(){Ok(Ok(s))=>{self.snapshot=Some(s);self.fase="sellada";},Ok(Err(_))=>{self.fallo.store(true,Ordering::SeqCst);self.fase="fallida";},Err(_)=>{}}
 }
}
fn bloquear_fifo(dir:&Path)->Result<(),Error>{
 // Creado por supervisor sólo en modo testigo, antes de lanzar inferidor; es FIFO.
 // No existe lector hasta intervención del banco exterior; bloqueo E/S real.
 use std::os::unix::fs::FileTypeExt;
 let p=dir.parent().ok_or("PADRE")?.join("bloqueo.fifo");
 if !fs::symlink_metadata(&p)?.file_type().is_fifo(){return Err("FIFO_REQUERIDO".into())}
 let mut f=OpenOptions::new().write(true).open(p)?;f.write_all(&vec![0u8;131072])?;Ok(())
}
struct Disco{dir:PathBuf,total:u64,bytes:[u64;3],files:[File;3],write_ns:u128,writes:u64,seq:u64,inicio:Instant,fallo_inyectado:bool}
impl Disco{
 fn nueva(dir:&Path)->Result<Self,Error>{
  fs::create_dir(dir)?;fs::set_permissions(dir,fs::Permissions::from_mode(0o700))?;
  let abrir=|p:&str|OpenOptions::new().write(true).create_new(true).open(dir.join(p));
  Ok(Self{dir:dir.into(),total:0,bytes:[0;3],files:[abrir(FILES[0])?,abrir(FILES[1])?,abrir(FILES[2])?],write_ns:0,writes:0,seq:0,inicio:Instant::now(),fallo_inyectado:false})
 }
 fn escribir(&mut self,i:usize,b:&[u8])->Result<(),Error>{
  if self.fallo_inyectado && i==0 && self.bytes[0]>0{self.fallo_inyectado=false;return Err("ESCRITURA_INYECTADA".into())}
  let n=b.len() as u64;if self.total+n>MAX_TOTAL||self.bytes[i]+n>MAX_FILE{return Err("CUSTODIA_LIMITE".into())}
  let v=statvfs(self.dir.as_path())?;if v.blocks_available()*v.fragment_size()<DISK_MARGIN{return Err("DISCO_SIN_MARGEN".into())}
  let t=Instant::now();self.files[i].write_all(b)?;self.files[i].sync_data()?;
  self.write_ns+=t.elapsed().as_nanos();self.writes+=1;self.total+=n;self.bytes[i]+=n;Ok(())
 }
 fn evento(&mut self,tipo:&str,datos:Value)->Result<(),Error>{
  self.seq+=1;let mut b=serde_json::to_vec(&json!({"seq":self.seq,"tipo":tipo,"datos":datos,"civil_unix_ms":civil(),"mono_ns_custodio":self.inicio.elapsed().as_nanos(),"escrituras_previas":self.writes,"ns_escritura_previos":self.write_ns,"bytes_previos":self.total}))?;b.push(b'\n');self.escribir(2,&b)
 }
 fn cerrar(mut self,estado:&str,completa:bool,corte:u64,laguna:bool)->Result<Snapshot,Error>{
  self.evento("cierre",json!({"estado":estado,"evidencia_completa_segun_oraculo":completa,"corte_custodia":corte,"laguna_tardia_posible":laguna}))?;
  let mut items=Vec::new();let mut originales=Vec::new();
  for file in &self.files{file.sync_all()?;}
  for nombre in &FILES[..4]{let p=self.dir.join(nombre);if !p.exists(){continue}
   let mut f=File::open(p)?;let mut h=Sha256::new();let mut n=0;let mut b=[0;65536];
   loop{let z=f.read(&mut b)?;if z==0{break}n+=z;h.update(&b[..z]);}
   let bytes=fs::read(self.dir.join(nombre))?;
   let digest=format!("{:x}",h.finalize());if bytes.len()!=n || hash(&bytes)!=digest{return Err("SELLADO_IDENTIDAD".into())}
   items.push(json!({"archivo":nombre,"bytes":n,"sha256":digest}));originales.push((nombre.to_string(),bytes));
  }
  // El manifiesto no se auto-hashea. Incluye coste exacto del cierre del journal,
  // pero no el coste de su propia escritura/hash, que se declara excluido.
  let b=serde_json::to_vec_pretty(&json!({"contrato":"EIO-NAT/2","estado":estado,"completa":completa,"corte_custodia":corte,"laguna_tardia_posible":laguna,"archivos":items,"coste_escritura_ns":self.write_ns,"escrituras":self.writes,"excluido":"coste propio del manifiesto; recuperación independiente pendiente"}))?;
  let mut out=OpenOptions::new().write(true).create_new(true).open(self.dir.join(FILES[4]))?;out.write_all(&b)?;out.sync_all()?;drop(out);
  File::open(&self.dir)?.sync_all()?;
  let sello=hash(&b);originales.push((FILES[4].into(),b));
  // Consume self: cierra TODOS los descriptores de escritura antes de enviar ACK.
  drop(self);
  Ok(Snapshot{sello,originales,completa})
 }
}
