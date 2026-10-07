use std::{fs,io::Write,process::{Command,Stdio}};
fn main()->Result<(),Box<dyn std::error::Error>>{
 let base="/opt/sv-safeguard/retroalimentacion-20261003/diagnostico-A08-03";let out=format!("{base}/compilacion-01");fs::create_dir(&out)?;
 fs::copy("/opt/sv-safeguard/fuentes/mistral.rs/Cargo.lock",format!("{base}/conductor/Cargo.lock"))?;
 let mut reg=fs::OpenOptions::new().create_new(true).write(true).open(format!("{out}/ESTADO.tsv"))?;
 for (id,crate_,accion,extra) in [("instrumentacion-pruebas","instrumentacion","test",vec![]),("base-compilar","instrumentacion","build",vec!["--bins"]),("custodio-pruebas","custodio","test",vec![]),("custodio-compilar","custodio","build",vec![]),("conductor-pruebas","conductor","test",vec![]),("conductor-compilar","conductor","build",vec![]),("verificador-compilar","verificador","build",vec![])] {
  writeln!(reg,"{id}\tinicio")?;reg.sync_all()?;
  let mut c=Command::new("/opt/sv-safeguard/cargo/bin/cargo");c.current_dir(format!("{base}/{crate_}")).args(["+1.98.1",accion,"--offline","--release","-j2","--target-dir","/opt/sv-safeguard/fuentes/mistral.rs/target"]).args(extra);
  c.env("CARGO_HOME","/opt/sv-safeguard/cargo").env("RUSTUP_HOME","/opt/sv-safeguard/rustup").env("PATH","/opt/sv-safeguard/cargo/bin:/usr/bin:/bin").env("RUSTFLAGS","-Ctarget-cpu=native");
  c.stdout(Stdio::from(fs::OpenOptions::new().create_new(true).write(true).open(format!("{out}/{id}.stdout"))?)).stderr(Stdio::from(fs::OpenOptions::new().create_new(true).write(true).open(format!("{out}/{id}.stderr"))?));
  let status=c.status()?;writeln!(reg,"{id}\tfin\t{:?}",status.code())?;reg.sync_all()?;if !status.success(){return Err(format!("Fallo conservado: {id}").into())}
 }
 fs::write(format!("{out}/COMPLETO"),b"conforme\n")?;Ok(())}
