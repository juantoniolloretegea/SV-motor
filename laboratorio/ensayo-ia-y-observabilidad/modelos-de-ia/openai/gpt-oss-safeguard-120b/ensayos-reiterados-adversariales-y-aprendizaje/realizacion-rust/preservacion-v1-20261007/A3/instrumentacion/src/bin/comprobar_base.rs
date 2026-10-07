use std::{fs,io::{Read,Write},path::Path};
use serde_json::{Value,json};use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
fn check(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn hash_file(p:&Path)->R<(u64,String)>{let mut f=fs::File::open(p)?;let mut buf=[0u8;1048576];let mut h=Sha256::new();let mut len=0;loop{let n=f.read(&mut buf)?;if n==0{break}h.update(&buf[..n]);len+=n as u64;}Ok((len,format!("{:x}",h.finalize())))}
fn main()->R<()>{
 let a=std::env::args().collect::<Vec<_>>();let base=Path::new(a.get(1).ok_or("Directorio requerido")?);
 check(fs::read_to_string("/sys/class/dmi/id/product_uuid")?.trim()=="00652369-8429-45a8-a287-3bdff932c66e","Identidad no autorizada")?;
 check(fs::read_to_string("/proc/swaps")?.lines().count()==1,"Intercambio habilitado")?;
 let mem=fs::read_to_string("/proc/meminfo")?;let free=mem.lines().find(|s|s.starts_with("MemAvailable:")).ok_or("Memoria")?.split_whitespace().nth(1).ok_or("Memoria")?.parse::<u64>()?;
 check(free>=120_000_000,"Memoria disponible insuficiente antes de la carga")?;
 for e in fs::read_dir("/proc")?{let e=e?;if !e.file_name().to_string_lossy().chars().all(|c|c.is_ascii_digit()){continue}if let Ok(exe)=fs::read_link(e.path().join("exe")){let n=exe.file_name().unwrap_or_default().to_string_lossy();check(!(n.contains("conductor")||n.contains("custodio"))||!exe.to_string_lossy().starts_with("/opt/sv-safeguard/"),"Proceso propio concurrente")?;}}
 let reference:Value=serde_json::from_slice(&fs::read(base.join("config/MODELO-REFERENCIA.json"))?)?;
 check(reference["modelo"]=="openai/gpt-oss-safeguard-120b"&&reference["revision"]=="3c7391182603991a904031244e7822488c67796d"&&reference["conforme"]==true,"Referencia de modelo distinta")?;
 let mut rows=Vec::new();for f in reference["archivos"].as_array().ok_or("Referencia incompleta")?{let n=f["ruta"].as_str().ok_or("Ruta")?;check(!n.contains('/')&&!n.contains('\\')&&n!="..","Ruta fuera del modelo")?;let p=Path::new("/opt/sv-safeguard/modelo").join(n);check(!fs::symlink_metadata(&p)?.file_type().is_symlink(),"Enlace en el modelo")?;let(len,sha)=hash_file(&p)?;check(f["bytes_declarados"]==len&&f["sha256_observado"]==sha,"Modelo distinto de la copia cotejada")?;rows.push(json!({"ruta":n,"bytes":len,"sha256":sha}));println!("Cotejado {n}");}
 check(rows.len()==27,"Modelo incompleto")?;
 let core:Value=serde_json::from_slice(&fs::read(base.join("config/NUCLEO-REFERENCIA.json"))?)?;for f in core.as_array().ok_or("Núcleo sin referencia")?{let p=Path::new("/opt/sv-safeguard/arbitro-v2/lenguaje/rust/sv_core").join(f["ruta"].as_str().ok_or("Ruta nuclear")?);let(len,sha)=hash_file(&p)?;check(f["bytes"]==len&&f["sha256"]==sha,"Núcleo modificado")?;}
 for(path,want)in [("/opt/sv-safeguard/aislado-v2-r1/bin/mcp","6e47cc556c8acd05d89f81b96f5eacb0e1a629aeaec0326ea3595b3c36941875"),("/opt/sv-safeguard/fuentes/mistral.rs/Cargo.lock","7ee0a0bfb402831a6a8649229672572637f83bbf635919ad5fd3dc744f6b61d8")]{check(hash_file(Path::new(path))?.1==want,"Componente conservado discordante")?;}
 let cg=Path::new("/sys/fs/cgroup/svsafeguard.slice");check(fs::read_to_string(cg.join("memory.max"))?.trim()=="122406567936"&&fs::read_to_string(cg.join("memory.swap.max"))?.trim()=="0"&&fs::read_to_string(cg.join("memory.swap.current"))?.trim()=="0","Guardas conjuntas discordantes")?;
 let ev=fs::read_to_string(cg.join("memory.events"))?;check(ev.lines().filter(|s|s.starts_with("oom ")||s.starts_with("oom_kill ")).all(|s|s.ends_with(" 0")),"Agotamiento de memoria registrado")?;
 let b=serde_json::to_vec_pretty(&json!({"conforme":true,"modelo":reference["modelo"],"revision":reference["revision"],"pesos_sha256_recalculados":true,"archivos":rows,"nucleo_archivos":core.as_array().unwrap().len(),"nucleo_sin_cambios":true,"swap":0,"memoria_disponible_kib":free,"fecha_unix_s":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs()}))?;
 let mut out=fs::OpenOptions::new().create_new(true).write(true).open(base.join("BASE-COTEJADA.json"))?;out.write_all(&b)?;out.sync_all()?;Ok(())
}
