use super::{Result,Work};
use std::{fs::File,io::{Read,Write},time::Instant};
use candle_core::{Device,Tensor,quantized::gguf_file};
use candle_transformers::{models::quantized_qwen3::ModelWeights,generation::{Sampling,LogitsProcessor}};
use serde_json::json;
fn emit(v:serde_json::Value)->Result<()>{let mut s=std::io::stdout().lock();serde_json::to_writer(&mut s,&v)?;s.write_all(b"\n")?;s.flush()?;Ok(())}
pub fn split(raw:&str,thinking:bool)->(String,String){
 if let Some((a,b))=raw.split_once("</think>"){return(a.trim().trim_start_matches("<think>").trim().to_string(),b.trim().to_string())}
 if thinking{return(raw.trim().trim_start_matches("<think>").trim().to_string(),String::new())}
 (String::new(),raw.trim().to_string())
}
pub fn worker()->Result<()>{
 let result=(||->Result<()>{
  // El proceso hijo termina si desaparece su supervisor; no modifica cgroups ni permisos.
  nix::sys::prctl::set_pdeathsig(nix::sys::signal::Signal::SIGTERM)?;
  let mut b=Vec::new();std::io::stdin().take(2*1024*1024).read_to_end(&mut b)?;let w:Work=serde_json::from_slice(&b)?;
  if nix::unistd::getppid().as_raw() as u32!=w.parent{return Err("El proceso supervisor ha terminado".into())}
  let clock=Instant::now();
  let tokenizer=tokenizers::Tokenizer::from_file(w.models.join("tokenizer.json")).map_err(|e|e.to_string())?;
  let tokenizer_seconds=clock.elapsed().as_secs_f64();
  emit(json!({"kind":"phase","text":"Cargando Qwen mediante Candle"}))?;
  let loading=Instant::now();
  let mut file=File::open(w.models.join("Qwen3-0.6B-Q4_K_M.gguf"))?;let gguf=gguf_file::Content::read(&mut file)?;let device=Device::Cpu;let mut model=ModelWeights::from_gguf(gguf,&mut file,&device)?;
  let mut timings=json!({"tokenizer_seconds":tokenizer_seconds,"model_load_seconds":loading.elapsed().as_secs_f64(),
   "prefill_seconds":null,"generation_seconds":null,"first_token_worker_seconds":null,"complete":false,
   "scope":"Intervalos monotónicos del proceso hijo. No incluyen cola, red ni preparación del contexto en el servidor. Las fases interrumpidas no se extrapolan."});
  emit(json!({"kind":"timings","timings":timings}))?;
  let (temperature,p)=if w.profile.thinking{(0.6,0.95)}else{(0.7,0.8)};
  let mut sampler=LogitsProcessor::from_sampling(w.profile.seed,Sampling::TopKThenTopP{k:20,p,temperature});
  let input=&w.context.token_ids;let mut pos=0usize;let mut logits=None;
  emit(json!({"kind":"phase","text":"Procesando los antecedentes"}))?;
  let prefill=Instant::now();
  // Fragmentar la lectura inicial reduce el tensor temporal de atención; conserva todos los tokens.
  for chunk in input.chunks(64){let t=Tensor::new(chunk,&device)?.unsqueeze(0)?;logits=Some(model.forward(&t,pos)?.squeeze(0)?);pos+=chunk.len();}
  timings["prefill_seconds"]=json!(prefill.elapsed().as_secs_f64());
  emit(json!({"kind":"timings","timings":timings}))?;
  let mut logits=logits.ok_or("Contexto vacío")?;let mut generated=Vec::new();let mut last=Instant::now();let mut raw=String::new();let mut finish="limite_generacion";
  emit(json!({"kind":"phase","text":if w.profile.thinking{"Generando con razonamiento"}else{"Generando respuesta"}}))?;
  let generation=Instant::now();
  for i in 0..w.profile.max_output{
   let token=sampler.sample(&logits)?;
   if token==151645{finish="fin_normal";break}
   generated.push(token);
   if generated.len()==1{timings["first_token_worker_seconds"]=json!(clock.elapsed().as_secs_f64());emit(json!({"kind":"timings","timings":timings}))?;}
   if generated.len()==1||last.elapsed().as_millis()>=200{
    raw=tokenizer.decode(&generated,false).map_err(|e|e.to_string())?;
    emit(json!({"kind":"progress","raw":raw,"tokens":generated.len()}))?;last=Instant::now();
   }
   if i+1<w.profile.max_output{let t=Tensor::new(&[token],&device)?.unsqueeze(0)?;logits=model.forward(&t,pos)?.squeeze(0)?;pos+=1;}
  }
  raw=tokenizer.decode(&generated,false).map_err(|e|e.to_string())?;
  timings["generation_seconds"]=json!(generation.elapsed().as_secs_f64());timings["complete"]=json!(true);
  emit(json!({"kind":"done","raw":raw,"tokens":generated.len(),"finish":finish,"timings":timings}))?;Ok(())
 })();
 if let Err(e)=result{let _=emit(json!({"kind":"error","error":e.to_string()}));return Err(e)}Ok(())
}
#[cfg(test)]mod tests{use super::*;#[test]fn texto_y_razonamiento(){assert_eq!(split("<think> Análisis </think> Respuesta",true),("Análisis".into(),"Respuesta".into()));assert_eq!(split("incompleto",true),("incompleto".into(),"".into()));assert_eq!(split("Respuesta",false),("".into(),"Respuesta".into()));}}
