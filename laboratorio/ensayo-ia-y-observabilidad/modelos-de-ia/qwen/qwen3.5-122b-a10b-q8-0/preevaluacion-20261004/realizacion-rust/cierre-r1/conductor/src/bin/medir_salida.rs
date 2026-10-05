// Medición externa de una solicitud cerrada. No usa la API ni genera texto.
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
use std::{fs,io::Write,path::Path};
type E=Box<dyn std::error::Error+Send+Sync>;
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn read(p:&Path)->Result<Value,E>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn main()->Result<(),E>{
 let a:Vec<_>=std::env::args().collect();
 let root=Path::new(a.get(1).ok_or("Original cerrado requerido")?);
 let dest=Path::new(a.get(2).ok_or("Informe nuevo requerido")?);
 let result=read(&root.join("RESULTADO.json"))?;
 if result["completa"]!=true||result["done"]!=true||result["terminacion"]!="stop"{return Err("La solicitud no tiene cierre completo".into())}
 let preparation=read(&root.join("PREPARACION.json"))?;
 let case=preparation["caso"].as_str().ok_or("Caso")?;
 if case.len()!=3||!matches!(case.as_bytes()[0],b'A'|b'B')||!(1..=9).contains(&case[1..].parse::<u32>()?)||preparation["capa"].as_u64().filter(|n|*n<=3).is_none(){return Err("Identidad fuera de banco".into())}
 let token_path=root.join("TOKENIZADOR-EFECTIVO.json");
 let token_hash=hash(&fs::read(&token_path)?);
 if read(&root.join("TOKENIZADOR-PROCEDENCIA.json"))?["sha256"]!=token_hash{return Err("Tokenizador discordante".into())}
 let tokenizer=tokenizers::Tokenizer::from_file(token_path)?;
 let rendered=fs::read_to_string(root.join("ENTRADA-RENDERIZADA.txt"))?;
 let input=tokenizer.encode_fast(rendered.clone(),true)?.get_ids().to_vec();
 if json!(input)!=read(&root.join("ENTRADA-TOKENS.json"))?||input.len()+4096+2048>32768||result["usage"]["prompt_tokens"]!=input.len()||preparation["entrada_sha256"]!=hash(rendered.as_bytes()){return Err("Entrada efectiva discordante".into())}
 let mut channels=serde_json::Map::new();
 for (name,file,field) in [("razonamiento","RAZONAMIENTO-EMITIDO.txt","razonamiento_sha256"),("final","FINAL.txt","final_sha256")]{
  let text=fs::read_to_string(root.join(file))?;
  if text.len()>16*1024*1024||result[field]!=hash(text.as_bytes()){return Err("Canal discordante".into())}
  let ids=tokenizer.encode_fast(text.clone(),false)?.get_ids().to_vec();
  channels.insert(name.into(),json!({"bytes":text.len(),"sha256":hash(text.as_bytes()),"tokens_recodificados":ids.len(),"identificadores_recodificados":ids}));
 }
 let mut reference_hash=Value::Null;
 if let Some(reference_path)=a.get(3){
  let bytes=fs::read(reference_path)?;let reference:Value=serde_json::from_slice(&bytes)?;
  if reference["caso"]!=case||reference["capa"]!=preparation["capa"]||reference["resultado_sha256"]!=hash(&fs::read(root.join("RESULTADO.json"))?)||reference["tokenizador_sha256"]!=token_hash||reference["tokens_entrada"]!=input.len()||reference["recuento_nativo_total_salida"]!=result["usage"]["completion_tokens"]{return Err("Identidad de referencia discordante".into())}
  for name in ["razonamiento","final"]{if reference[format!("identificadores_recodificados_{name}")]!=channels[name]["identificadores_recodificados"]||reference[format!("tokens_recodificados_{name}")]!=channels[name]["tokens_recodificados"]{return Err("Recodificación distinta de la referencia previa".into())}}
  reference_hash=json!(hash(&bytes));
 }
 let v=json!({"encargo":"QWEN35-PRE-20261004/r1","caso":case,"capa":preparation["capa"],"resultado_sha256":hash(&fs::read(root.join("RESULTADO.json"))?),"tokenizador_sha256":token_hash,"entrada_tokens":input.len(),"entrada_reproducida_identica":true,"salida_total_nativa":result["usage"]["completion_tokens"],"canales":channels,"referencia_previa_cotejada_sha256":reference_hash,"tiempos_nativos":result["usage"],"limite":"Los identificadores por canal son recodificación externa del texto, no identificadores originales generados. La API declara el total nativo de salida, sin desglose nativo por canal. No se asignan diferencias sin evidencia.","sin_inferencia":true,"licencia":"© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0)."});
 let mut f=fs::OpenOptions::new().create_new(true).write(true).open(dest)?;
 f.write_all(&serde_json::to_vec_pretty(&v)?)?;f.sync_all()?;
 println!("{}",json!({"caso":case,"capa":preparation["capa"],"entrada_reproducida_identica":true,"sin_inferencia":true}));
 Ok(())
}
