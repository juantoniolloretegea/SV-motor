//! Vía de escalada sin E/S de custodia, sin locks compartidos ni espera por journal.
use std::{sync::mpsc::{SyncSender,sync_channel,Receiver},time::{Instant,Duration},thread};
use nix::{sys::signal::{killpg,Signal},unistd::Pid};
use serde_json::{Value,json};
pub struct Escalada{pub eventos:Receiver<Value>,pub terminada:std::sync::Arc<std::sync::atomic::AtomicBool>}
pub fn iniciar(pid:u32)->Escalada{
 let(tx,rx)=sync_channel(4);let terminada=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));let fin=terminada.clone();
 thread::spawn(move||{
  let inicio=Instant::now();
  let term=killpg(Pid::from_raw(pid as i32),Signal::SIGTERM);
  // Deadline se fija desde actuación TERM, no desde acabar una escritura.
  let deadline=inicio+Duration::from_millis(250);
  let _=tx.try_send(json!({"tipo":"SIGTERM","ns_desde_inicio":inicio.elapsed().as_nanos(),"resultado":format!("{term:?}")}));
  while let Some(d)=deadline.checked_duration_since(Instant::now()){thread::sleep(d);}
  let kill=killpg(Pid::from_raw(pid as i32),Signal::SIGKILL);
  let _=tx.try_send(json!({"tipo":"SIGKILL","ns_desde_inicio":inicio.elapsed().as_nanos(),"gracia_ns":250000000_u64,"resultado":format!("{kill:?}")}));
  fin.store(true,std::sync::atomic::Ordering::SeqCst);
 });
 Escalada{eventos:rx,terminada}
}
/// Heartbeat hacia guarda exterior; el hilo puede bloquear en pipe sin bloquear control.
pub fn latidos()->SyncSender<u8>{
 use std::io::Write;
 let(tx,rx)=sync_channel(4);
 thread::spawn(move||{let mut out=std::io::stdout().lock();while let Ok(b)=rx.recv(){if out.write_all(&[b]).and_then(|_|out.flush()).is_err(){break}}});
 tx
}
