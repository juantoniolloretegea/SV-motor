use std::{fs,env};
use serde_json::json;
fn main()->Result<(),Box<dyn std::error::Error+Send+Sync>>{
 let path=env::args().nth(1).ok_or("Falta archivo de tokenizador")?;
 let bytes=fs::read(&path)?;let raw:serde_json::Value=serde_json::from_slice(&bytes)?;
 let t=tokenizers::Tokenizer::from_bytes(&bytes).map_err(|e|e.to_string())?;
 let tokens=["<|endoftext|>","<|return|>","<|channel|>","<|start|>","<|end|>","<|message|>","<|fim_suffix|>"];
 let mut rows=vec![];
 for text in tokens{let ids=t.encode(text,false).map_err(|e|e.to_string())?.get_ids().to_vec();rows.push(json!({"text":text,"loaded_id":t.token_to_id(text),"encoded_ids":ids,"model_vocab_id":raw["model"]["vocab"][text],"declared_added":raw["added_tokens"].as_array().and_then(|a|a.iter().find(|v|v["content"]==text))}));}
 println!("{}",serde_json::to_string_pretty(&json!({"tokenizers":"0.22.2","model_vocab_size":raw["model"]["vocab"].as_object().map(|o|o.len()),"rows":rows}))?);Ok(())
}
