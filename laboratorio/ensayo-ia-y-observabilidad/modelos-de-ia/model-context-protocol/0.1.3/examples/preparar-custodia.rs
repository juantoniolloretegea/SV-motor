use std::{fs,path::PathBuf,collections::BTreeMap};
use serde_json::{json,Value};
use sv_mcp_documental::{Catalog,sha256,auditoria::verify};
fn main(){
 let root=PathBuf::from(env!("CARGO_MANIFEST_DIR"));
 let mut files=BTreeMap::<String,String>::new();
 for name in ["Cargo.toml","Cargo.lock","rust-toolchain.toml","LEAME.md","PRUEBAS.txt",
 "src/lib.rs","src/main.rs","src/auditoria.rs","src/aislamiento.rs","src/custodia.rs","src/supervision.rs",
 "src/bin/cliente-minimo.rs","src/bin/observar-minimo.rs","src/bin/verificar-diario.rs",
 "tests/protocolo.rs","tests/auditoria.rs","examples/preparar-custodia.rs"]{
  files.insert(name.into(),fs::read_to_string(root.join(name)).unwrap());
 }
 let dirs=fs::read_dir(root.join("target/evidencias")).unwrap();
 let mut found=false;
 for entry in dirs {
  let dir=entry.unwrap().path();
  if !dir.join("cotejo.json").exists(){continue}
  let raw=fs::read(dir.join("catalogo.json")).unwrap();let hash=sha256(&raw);
  let c=Catalog::load(&dir.join("catalogo.json"),&hash,true).unwrap();
  let journal=fs::read(dir.join("diario.jsonl")).unwrap();
  let Ok(v)=verify(&c,&hash,&journal)else{continue};
  for name in ["catalogo.json","entrada.jsonl","salida.jsonl","diario.jsonl","error.txt"] {
   files.insert(format!("evidencias/{name}"),fs::read_to_string(dir.join(name)).unwrap());
  }
  files.insert("evidencias/COTEJO.json".into(),serde_json::to_string_pretty(&v).unwrap()+"\n");
  let stdout=fs::read_to_string(dir.join("salida.jsonl")).unwrap();
  for line in stdout.lines(){
   let v:Value=serde_json::from_str(line).unwrap();
   if v["id"]==2 {files.insert("HERRAMIENTAS.json".into(),serde_json::to_string_pretty(&v["result"]).unwrap()+"\n");}
  }
  found=true;break
 }
 assert!(found,"Falta recorrido real conforme con las fuentes actuales");
 let manifest:Vec<Value>=files.iter().map(|(p,c)|json!({"ruta":p,"bytes":c.len(),"sha256":sha256(c.as_bytes())})).collect();
 files.insert("MANIFIESTO.json".into(),serde_json::to_string_pretty(&json!({"version":"0.1.3","archivos":manifest,"alcance":"fuentes y recorrido sintético; no contiene inferencia Safeguard"})).unwrap()+"\n");
 for (p,c) in &files{let dest=root.join(p);fs::create_dir_all(dest.parent().unwrap()).unwrap();fs::write(dest,c).unwrap();}
 println!("{}",serde_json::to_string(&files).unwrap());
}
