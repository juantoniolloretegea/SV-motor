//! Adaptación del ejemplo quantized-qwen3 de Candle en el commit fijado.
//! Candidato no compilado; CPU, sin herramientas ni descargas en la generación.
use candle_core::{Device,Tensor,quantized::gguf_file};
use candle_transformers::{models::quantized_qwen3::ModelWeights,generation::{LogitsProcessor,Sampling}};
use tokenizers::Tokenizer;
use std::{io::Cursor,sync::atomic::{AtomicBool,Ordering},time::{Instant,Duration}};
pub type Fallo=Box<dyn std::error::Error+Send+Sync>;
#[derive(serde::Serialize)]
pub struct Salida {pub texto:String,pub fin:String,pub entrada:usize,pub generados:usize,pub segundos:f64,pub tokens:Vec<u32>}
pub fn generar(pesos:&[u8],tokenizer_bytes:&[u8],peticion:&str,cancel:&AtomicBool)->Result<Salida,Fallo>{
 let inicio=Instant::now();
 let tokenizer=Tokenizer::from_bytes(tokenizer_bytes).map_err(|e|e.to_string())?;
 let eos=tokenizer.token_to_id("<|im_end|>").ok_or("EOS ausente")?;
 if eos!=151645 {return Err("EOS incompatible".into());}
 // Especialización de plantilla Qwen: un turno, sin tools, thinking=false.
 let texto=format!("<|im_start|>user\n{peticion}<|im_end|>\n<|im_start|>assistant\n<think>\n\n</think>\n\n");
 let entrada=tokenizer.encode(texto,false).map_err(|e|e.to_string())?;let ids=entrada.get_ids();
 if ids.len()+128>2048 {return Err("LIMITE_ENTRADA".into());}
 let dev=Device::Cpu;let mut reader=Cursor::new(pesos);
 let contenido=gguf_file::Content::read(&mut reader)?;
 crate::nativa::marca("modelo.antes")?;
 let mut modelo=ModelWeights::from_gguf(contenido,&mut reader,&dev)?;
 crate::nativa::marca("modelo.despues")?;
 let mut logits=LogitsProcessor::from_sampling(299792458,Sampling::ArgMax);
 let mut salida=Vec::new();let mut posicion=0;let mut actuales=ids.to_vec();
 let fin=loop {
  if cancel.load(Ordering::Relaxed){break "CANCELACION";}
  if inicio.elapsed()>=Duration::from_secs(120){break "TIEMPO";}
  let input=Tensor::new(actuales.as_slice(),&dev)?.unsqueeze(0)?;
  if salida.is_empty(){crate::nativa::marca("forward.antes")?;}
  let l=modelo.forward(&input,posicion)?.squeeze(0)?;
  if salida.is_empty(){crate::nativa::marca("forward.despues")?;}
  let token=logits.sample(&l)?;
  posicion+=actuales.len();salida.push(token);
  if let Some(fin)=crate::parada(token,eos,salida.len(),ids.len()+salida.len(),cancel.load(Ordering::Relaxed)){break fin;}
  actuales=vec![token];
 };
 let contenido=if salida.last()==Some(&eos){&salida[..salida.len()-1]}else{&salida[..]};
 let texto=tokenizer.decode(contenido,false).map_err(|e|e.to_string())?;
 Ok(Salida{texto,fin:fin.into(),entrada:ids.len(),generados:salida.len(),segundos:inicio.elapsed().as_secs_f64(),tokens:salida})
}
