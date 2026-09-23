use sha2::{Digest,Sha256};
use std::{fs::{self,OpenOptions},io::{Read,Write},path::Path,time::{Duration,Instant,SystemTime,UNIX_EPOCH}};
const URL:&str="https://huggingface.co/ggml-org/gpt-oss-20b-GGUF/resolve/b97cbb20d1995efd41dce8c4dd1ddf86e8db375b/gpt-oss-20b-MXFP4.gguf";
const SIZE:u64=12_109_566_624;
const SHA:&str="27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901";
fn run()->Result<(),Box<dyn std::error::Error>>{
 let origin=Instant::now();
 std::thread::spawn(||{std::thread::sleep(Duration::from_secs(450));eprintln!("plazo_exterior_descarga_450_s");std::process::exit(124);});
 let dir=Path::new("/tmp/sv-gguf-20260923");fs::create_dir_all(dir)?;
 let part=dir.join("gpt-oss-20b-MXFP4.gguf.part");let destination=dir.join("gpt-oss-20b-MXFP4.gguf");
 if destination.exists(){return Err("destino_preexistente; no se sobrescribe".into());}
 let mut file=OpenOptions::new().create_new(true).write(true).open(&part)?;
 println!("{}",serde_json::json!({"phase":"start","utc_ms":SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),"url":URL,"expected_bytes":SIZE,"expected_sha256":SHA,"timeout_s":420,"attempts":1}));
 let agent=ureq::AgentBuilder::new().timeout(Duration::from_secs(420)).timeout_connect(Duration::from_secs(15)).timeout_read(Duration::from_secs(30)).build();
 let response=agent.get(URL).call()?;
 if response.status()!=200{return Err("http_no_200".into());}
 if let Some(length)=response.header("Content-Length"){if length.parse::<u64>()?!=SIZE{return Err("tamano_anunciado_distinto".into());}}
 let mut reader=response.into_reader();let mut hash=Sha256::new();let mut buffer=vec![0u8;1024*1024];let mut total=0u64;let mut next=1024*1024*1024u64;
 loop{if origin.elapsed()>Duration::from_secs(420){return Err("plazo_descarga".into());}let n=reader.read(&mut buffer)?;if n==0{break;}total=total.checked_add(n as u64).ok_or("tamano_excedido")?;if total>SIZE{return Err("respuesta_excede_tamano".into());}file.write_all(&buffer[..n])?;hash.update(&buffer[..n]);if total>=next{println!("{}",serde_json::json!({"phase":"progress","bytes":total,"seconds":origin.elapsed().as_secs_f64()}));next+=1024*1024*1024;}}
 file.sync_all()?;let actual=format!("{:x}",hash.finalize());if total!=SIZE||actual!=SHA{return Err(format!("integridad_no_conforme bytes={total} sha256={actual}").into());}fs::rename(&part,&destination)?;
 println!("{}",serde_json::json!({"phase":"verified","bytes":total,"sha256":actual,"seconds":origin.elapsed().as_secs_f64(),"path":destination,"license":"Apache-2.0; conversión ggml-org del modelo OpenAI"}));Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("descarga_fallida: {e}");std::process::exit(1);}eprintln!("Sistema Vectorial SV · CC BY-NC-ND 4.0. Dependencias y pesos: licencias propias.");}
