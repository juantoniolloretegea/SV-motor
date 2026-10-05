// Reproduce la ruta BPE qwen35 de mistral.rs 4400935451da5e2dc7379a3f92fbbada66557f6c.
// Referencia y licencia MIT conservadas junto al protocolo. No modifica el motor.
use crate::{E,hash,save};
use serde_json::{Value,json};
use std::{fs::{self,File},io::{Read,BufReader},path::Path,collections::BTreeMap};
use tokenizers::{Tokenizer,AddedToken,models::bpe::BpeBuilder,pre_tokenizers::{sequence::Sequence,split::{Split,SplitPattern},PreTokenizerWrapper},tokenizer::normalizer::SplitDelimiterBehavior};
fn bytes<const N:usize>(r:&mut impl Read)->Result<[u8;N],E>{let mut b=[0;N];r.read_exact(&mut b)?;Ok(b)}
fn u32r(r:&mut impl Read)->Result<u32,E>{Ok(u32::from_le_bytes(bytes(r)?))}
fn u64r(r:&mut impl Read)->Result<u64,E>{Ok(u64::from_le_bytes(bytes(r)?))}
fn text(r:&mut impl Read)->Result<String,E>{let n=u64r(r)?;if n>100_000_000{return Err("Cadena GGUF excesiva".into())}let mut b=vec![0;n as usize];r.read_exact(&mut b)?;Ok(String::from_utf8(b)?)}
fn value(r:&mut impl Read,t:u32)->Result<Value,E>{Ok(match t{
0=>json!(bytes::<1>(r)?[0]),1=>json!(bytes::<1>(r)?[0] as i8),2=>json!(u16::from_le_bytes(bytes(r)?)),3=>json!(i16::from_le_bytes(bytes(r)?)),4=>json!(u32r(r)?),5=>json!(i32::from_le_bytes(bytes(r)?)),6=>json!(f32::from_le_bytes(bytes(r)?)),7=>json!(bytes::<1>(r)?[0]!=0),8=>json!(text(r)?),9=>{let st=u32r(r)?;let n=u64r(r)?;if n>10_000_000{return Err("Matriz GGUF excesiva".into())}let mut a=Vec::with_capacity(n as usize);for _ in 0..n{a.push(value(r,st)?)}json!(a)},10=>json!(u64r(r)?),11=>json!(i64::from_le_bytes(bytes(r)?)),12=>json!(f64::from_le_bytes(bytes(r)?)),_=>return Err("Tipo GGUF desconocido".into())})}
pub fn native(gguf:&Path,out:&Path)->Result<Tokenizer,E>{
 let mut r=BufReader::new(File::open(gguf)?);if bytes::<4>(&mut r)?!=*b"GGUF"{return Err("GGUF".into())}let version=u32r(&mut r)?;if version!=3{return Err("Versión GGUF".into())}let _tensors=u64r(&mut r)?;let n=u64r(&mut r)?;let mut m=BTreeMap::new();for _ in 0..n{let k=text(&mut r)?;let t=u32r(&mut r)?;m.insert(k,value(&mut r,t)?);}
 if m["tokenizer.ggml.model"]!="gpt2"||m["tokenizer.ggml.pre"]!="qwen35"{return Err("Ruta tokenizadora no recibida".into())}
 let tokens=m["tokenizer.ggml.tokens"].as_array().ok_or("Tokens")?;let types=m["tokenizer.ggml.token_type"].as_array().ok_or("Tipos")?;if tokens.len()!=248320||types.len()!=tokens.len(){return Err("Vocabulario".into())}
 let mut vocab=ahash::AHashMap::new();for (i,t) in tokens.iter().enumerate(){vocab.insert(t.as_str().ok_or("Token")?.to_string(),i as u32);}
 let mut merges=vec![];for t in m["tokenizer.ggml.merges"].as_array().ok_or("Fusiones")?{let (l,r)=t.as_str().ok_or("Fusión")?.split_once(' ').ok_or("Par")?;if l.is_empty()||r.is_empty(){return Err("Fusión vacía".into())}merges.push((l.to_string(),r.to_string()));}
 let mut bpe=BpeBuilder::new().vocab_and_merges(vocab,merges).ignore_merges(false);
 if let Some(u)=m.get("tokenizer.ggml.unknown_token_id").and_then(Value::as_u64){bpe=bpe.unk_token(tokens[u as usize].as_str().ok_or("UNK")?.into());}
 let mut tok=Tokenizer::new(bpe.build()?);tok.with_normalizer(Some(tokenizers::normalizers::NFC));
 let regex="(?i:'s|'t|'re|'ve|'m|'ll|'d)|[^\\r\\n\\p{L}\\p{N}]?[\\p{L}\\p{M}]+|\\p{N}| ?[^\\s\\p{L}\\p{M}\\p{N}]+[\\r\\n]*|\\s*[\\r\\n]+|\\s+(?!\\S)|\\s+";
 let split=Split::new(SplitPattern::Regex(regex.into()),SplitDelimiterBehavior::Isolated,false)?;
 tok.with_pre_tokenizer(Some(Sequence::new(vec![PreTokenizerWrapper::Split(split),PreTokenizerWrapper::ByteLevel(tokenizers::pre_tokenizers::byte_level::ByteLevel::new(false,false,false))])));
 tok.with_decoder(Some(tokenizers::decoders::byte_level::ByteLevel::new(false,false,false)));
 tok.with_post_processor(Some(tokenizers::processors::byte_level::ByteLevel::new(false,false,false)));
 for name in ["bos_token_id","eos_token_id","unknown_token_id"]{if let Some(id)=m.get(&format!("tokenizer.ggml.{name}")).and_then(Value::as_u64){tok.add_special_tokens(&[AddedToken::from(tokens[id as usize].as_str().ok_or("Especial")?.to_string(),true)]);}}
 let mut special=vec![];for(i,t)in types.iter().enumerate(){let ty=t.as_i64().ok_or("Tipo")?;if ty!=1&&ty!=6{special.push(AddedToken::from(tokens[i].as_str().unwrap().to_string(),true));}}tok.add_special_tokens(&special);
 let serialized=tok.to_string(false)?;fs::write(out.join("TOKENIZADOR-EFECTIVO.json"),&serialized)?;
 save(&out.join("TOKENIZADOR-PROCEDENCIA.json"),&json!({"gguf":gguf.file_name().unwrap().to_string_lossy(),"entradas":tokens.len(),"especiales":special.len(),"sha256":hash(serialized.as_bytes()),"eos":m["tokenizer.ggml.eos_token_id"],"ruta":"Conversión BPE qwen35 idéntica a la fuente fijada; se coteja además con el contador nativo del servicio para cada petición"}))?;Ok(tok)
}
pub fn render(template:&str,messages:&Value,thinking:bool)->Result<String,E>{let mut env=minijinja::Environment::new();env.set_lstrip_blocks(true);env.set_trim_blocks(true);env.set_unknown_method_callback(minijinja_contrib::pycompat::unknown_method_callback);env.add_function("raise_exception",|s:String|->Result<String,minijinja::Error>{Err(minijinja::Error::new(minijinja::ErrorKind::InvalidOperation,s))});env.add_template("qwen",template)?;Ok(env.get_template("qwen")?.render(minijinja::context!{messages=>messages,add_generation_prompt=>true,enable_thinking=>thinking,bos_token=>"<|endoftext|>",eos_token=>"<|im_end|>",unk_token=>""})?)}
