//! Reconstrucción explícita de IDs añadidos sin alterar el vocabulario ordinario.
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::collections::{HashMap,HashSet};
type Error=Box<dyn std::error::Error+Send+Sync>;
pub fn load(bytes:&[u8])->Result<(tokenizers::Tokenizer,Value),Error>{
 let mut raw:Value=serde_json::from_slice(bytes)?;
 let added=raw["added_tokens"].as_array().ok_or("Faltan tokens añadidos")?.clone();
 let vocab=raw["model"]["vocab"].as_object_mut().ok_or("Falta vocabulario")?;
 let original=vocab.clone();let mut ids=HashMap::new();
 for (name,id) in vocab.iter(){let id=id.as_u64().ok_or("ID no entero")?;if ids.insert(id,name.clone()).is_some(){return Err("IDs ordinarios duplicados".into())}}
 let mut names=HashSet::new();let mut declared=HashSet::new();let mut inserted=vec![];
 for a in &added{
  let name=a["content"].as_str().ok_or("Token sin nombre")?;let id=a["id"].as_u64().ok_or("Token sin ID")?;
  if !names.insert(name.to_owned())||!declared.insert(id){return Err("Declaración duplicada".into())}
  if let Some(v)=vocab.get(name){if v.as_u64()!=Some(id){return Err("Nombre con ID contradictorio".into())}}
  else{if ids.contains_key(&id){return Err(format!("ID ocupado: {id}").into())}vocab.insert(name.into(),json!(id));ids.insert(id,name.into());inserted.push(json!({"token":name,"id":id}));}
 }
 for (name,id) in original{if vocab.get(&name)!=Some(&id){return Err("Vocabulario ordinario alterado".into())}}
 let derived=serde_json::to_vec(&raw)?;
 let tokenizer=tokenizers::Tokenizer::from_bytes(&derived).map_err(|e|e.to_string())?;
 for a in &added{let name=a["content"].as_str().unwrap();let id=u32::try_from(a["id"].as_u64().unwrap())?;
  if tokenizer.token_to_id(name)!=Some(id)||tokenizer.encode(name,false).map_err(|e|e.to_string())?.get_ids()!=[id]{return Err(format!("ID cargado no conforme: {name}").into())}
 }
 Ok((tokenizer,json!({"method":"Completar únicamente IDs ausentes con las declaraciones added_tokens; vocabulario original preservado","source_sha256":format!("{:x}",Sha256::digest(bytes)),"derived_sha256":format!("{:x}",Sha256::digest(&derived)),"inserted":inserted,"all_declared_ids_verified":true})))
}
#[cfg(test)]mod tests{
 use super::*;
 fn fixture()->Value{json!({"version":"1.0","truncation":null,"padding":null,"added_tokens":[{"id":1,"content":"<|end|>","single_word":false,"lstrip":false,"rstrip":false,"normalized":false,"special":true}],"normalizer":null,"pre_tokenizer":null,"post_processor":null,"decoder":null,"model":{"type":"BPE","dropout":null,"unk_token":null,"continuing_subword_prefix":null,"end_of_word_suffix":null,"fuse_unk":false,"byte_fallback":false,"ignore_merges":false,"vocab":{"a":0,"z":2},"merges":[]}})}
 #[test]fn conserva_ids_con_huecos(){let (t,e)=load(&serde_json::to_vec(&fixture()).unwrap()).unwrap();assert_eq!(t.encode("a<|end|>z",false).unwrap().get_ids(),[0,1,2]);assert_eq!(e["inserted"].as_array().unwrap().len(),1);}
 #[test]fn rechaza_colision_sin_sustituir_vocabulario(){let mut v=fixture();v["model"]["vocab"]["z"]=json!(1);assert!(load(&serde_json::to_vec(&v).unwrap()).is_err());}
}
