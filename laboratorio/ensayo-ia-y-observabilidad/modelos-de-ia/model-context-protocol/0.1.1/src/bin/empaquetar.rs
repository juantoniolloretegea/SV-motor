
use std::{fs,path::{Path,PathBuf},io::Write};
use serde_json::json;
use sv_mcp_documental::sha256;
fn visit(root:&Path,at:&Path,out:&mut Vec<(String,Vec<u8>)>)->Result<(),Box<dyn std::error::Error>>{
 for e in fs::read_dir(at)?{let p=e?.path();if p.is_dir(){visit(root,&p,out)?}else if p.is_file(){out.push((p.strip_prefix(root)?.to_string_lossy().replace('\\',"/"),fs::read(&p)?));}}Ok(())
}
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a:Vec<String>=std::env::args().collect();let root=Path::new(&a[1]);let prefix=&a[2];let output=&a[3];let mut files=Vec::new();visit(root,root,&mut files)?;files.sort_by(|a,b|a.0.cmp(&b.0));
 let mut manifest=String::new();for (p,b) in &files{manifest.push_str(&format!("{}  {}\n",sha256(b),p));}
 fs::write(root.join("MANIFIESTO.sha256"),&manifest)?;files.push(("MANIFIESTO.sha256".into(),manifest.into_bytes()));
 let tree:Result<Vec<_>,std::string::FromUtf8Error>=files.into_iter().map(|(p,b)|Ok(json!({"path":format!("{prefix}/{p}"),"mode":"100644","type":"blob","content":String::from_utf8(b)?}))).collect();
 fs::write(output,serde_json::to_vec(&json!({"tree":tree?}))?)?;Ok(())
}
