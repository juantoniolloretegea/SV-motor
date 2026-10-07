#![forbid(unsafe_code)]
use std::{fs,io::Write,path::Path};
use serde_json::{Value,json};
use sha2::{Digest,Sha256};
type R<T>=Result<T,Box<dyn std::error::Error>>;
const D:&str="publicaciones/astra-catalogo-20261006";
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn read(p:impl AsRef<Path>)->R<Value>{Ok(serde_json::from_slice(&fs::read(p)?)?)}
fn save(p:impl AsRef<Path>,b:&[u8])->R<()>{let p=p.as_ref();fs::create_dir_all(p.parent().unwrap())?;if p.exists(){assert_eq!(fs::read(p)?,b);}else{let mut f=fs::OpenOptions::new().write(true).create_new(true).open(p)?;f.write_all(b)?;f.sync_all()?;}Ok(())}
fn csv(s:&str)->Vec<Vec<String>>{let(mut rows,mut row,mut f,mut q)=(vec![],vec![],String::new(),false);let mut it=s.chars().peekable();while let Some(c)=it.next(){match c{'"'=>{if q&&it.peek()==Some(&'"'){f.push('"');it.next();}else{q=!q;}},',' if !q=>row.push(std::mem::take(&mut f)),'\n' if !q=>{row.push(std::mem::take(&mut f));rows.push(std::mem::take(&mut row));},'\r' if !q=>{},_=>f.push(c)}}assert!(!q);if !f.is_empty()||!row.is_empty(){row.push(f);rows.push(row)}rows}
fn main()->R<()>{
 let dir=Path::new(D);let root=std::env::current_dir()?.canonicalize()?;assert!(dir.canonicalize()?.starts_with(&root));
 let mode=std::env::args().nth(1).ok_or("Modo ausente")?;
 if mode=="motor"{let r=read(dir.join("RECUPERADOS-MOTOR.json"))?;let mut checked=vec![];for f in r["files"].as_array().ok_or("Sin archivos")?{let path=f["path"].as_str().unwrap();let name=path.rsplit('/').next().unwrap();assert!(matches!(name,"FICHA.md"|"CONTRATO-CATALOGO-20261006.md"|"PREPARACION-CATALOGO-20261006.md"));let b=fs::read(format!("publicaciones/astra-catalogo-20261006/preparados-motor/{name}"))?;assert_eq!(b,f["content"].as_str().unwrap().as_bytes(),"{name}");checked.push(json!({"path":path,"bytes":b.len(),"sha256":hash(&b)}));}save(dir.join("COTEJO-MOTOR.json"),&serde_json::to_vec_pretty(&json!({"estado":"conforme","commit":r["commit"],"archivos":checked,"custodia_operativa_integra_remota":false}))?)?;println!("MOTOR: {} archivos idénticos por bytes y SHA-256",checked.len());return Ok(());}
 let p=read(dir.join("PLAN-CALIDAD.json"))?;let files=p["files"].as_array().unwrap();let footer=p["autoria_licencia"].as_str().unwrap();assert_eq!(files.len(),9);
 let get=|suffix:&str|->&Value{files.iter().find(|f|f["path"].as_str().unwrap().ends_with(suffix)).unwrap()};
 if mode=="preparar"{
  let mut checked=vec![];
  for f in files{let path=f["path"].as_str().unwrap();assert!(path.starts_with("docs/calidad/")&&!path.contains("..")&&!path.to_lowercase().ends_with("readme.md"));let new=f["content"].as_str().unwrap();assert!(!new.contains('\u{fffd}'));if path.ends_with(".md"){assert!(new.trim_end().ends_with(footer));}if path.ends_with(".json"){let _:Value=serde_json::from_str(new)?;}
   if let Some(old)=f["original"].as_str(){
    if path.ends_with("HISTORIAL_SUCESOS_SV.csv")||path.ends_with("REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv")||path.ends_with("TIQUES_TECNICOS.csv"){assert!(new.starts_with(old));assert_eq!(csv(new).len(),csv(old).len()+1);}
    if path.ends_with("/SUCESOS_SV.csv"){let a=csv(old);let b=csv(new);assert_eq!(a.len(),b.len());for(x,y)in a.iter().zip(&b){if x[0]!="S39"{assert_eq!(x,y);}}}
    if path.ends_with("SUCESOS_SV.md"){let a=old.find("## S39 ").unwrap();let b=old.find("\n## S40 ").unwrap();assert!(new.starts_with(&old[..a])&&new.ends_with(&old[b..]));}
    if path.ends_with("ACTA_004_FINALIDAD_ALCANCE_Y_CONTINUIDAD_EIO_2026_09_22.md")||path.ends_with("REGISTRO_EVOLUCION_TECNICA_PROYECTO.md"){let k=old.rfind(footer).unwrap();assert!(new.starts_with(&old[..k])&&new.ends_with(&old[k..]));}
   }
   save(dir.join("preparados").join(path),new.as_bytes())?;checked.push(json!({"path":path,"bytes":new.len(),"sha256":hash(new.as_bytes()),"original_blob":f["original_blob"]}));
  }
  let sc=csv(get("/SUCESOS_SV.csv")["content"].as_str().unwrap());let sr=sc.iter().find(|r|r[0]=="S39").unwrap();let hc=csv(get("HISTORIAL_SUCESOS_SV.csv")["content"].as_str().unwrap());assert_eq!(hc.last().unwrap()[0],"45");assert_eq!(&hc.last().unwrap()[1..],sr.as_slice());assert_eq!(sr[1],"en ejecución");
  let tc=csv(get("TIQUES_TECNICOS.csv")["content"].as_str().unwrap());let tt=tc.last().unwrap();assert_eq!(tt[1],"TT-0021");assert_eq!(tt[3],"pendiente");let j:Value=serde_json::from_str(get("TT-0021_ASTRA_NODO03_2026-10-06.json")["content"].as_str().unwrap())?;assert_eq!(j["revision_suceso"],45);assert_eq!(j["resultado_actual"],tt[8]);assert_eq!(j["estado"],tt[3]);assert_eq!(j["catalogo_ejecutado"],false);assert_eq!(j["recepcion_finalizada"],false);
  let rc=csv(get("REGISTRO_EVOLUCION_TECNICA_PROYECTO.csv")["content"].as_str().unwrap());assert_eq!(rc.last().unwrap()[0],"RETP-2026-284");assert_eq!(rc.last().unwrap().len(),rc[0].len());assert!(get("ACTA_004_FINALIDAD_ALCANCE_Y_CONTINUIDAD_EIO_2026_09_22.md")["content"].as_str().unwrap().contains("## 39. Nodo 03:"));
  save(dir.join("CONTROL-CALIDAD.json"),&serde_json::to_vec_pretty(&json!({"estado":"conforme_documental","base":p["base"],"archivos":checked,"suceso":"S39","revision":45,"tique":"TT-0021","acta":"004, apartado 39","retp":"RETP-2026-284","comprobaciones":["Concordancia CSV, Markdown e historial","Antecedentes y filas ajenas intactos","Tique pendiente; catálogo y recepción integral no ejecutados","UTF-8, bytes y SHA-256"],"autoria_licencia":footer}))?)?;println!("CALIDAD: 9 archivos preparados y concordantes");
 }else if mode=="cotejar"{
  let r=read(dir.join("RECUPERADOS-CALIDAD.json"))?;let m=read(dir.join("CONTROL-CALIDAD.json"))?;assert_eq!(r["files"].as_array().unwrap().len(),files.len());for f in files{let g=r["files"].as_array().unwrap().iter().find(|g|g["path"]==f["path"]).unwrap();let b=f["content"].as_str().unwrap().as_bytes();let rb=g["content"].as_str().unwrap().as_bytes();assert_eq!(b,rb);let h=m["archivos"].as_array().unwrap().iter().find(|x|x["path"]==f["path"]).unwrap();assert_eq!(h["sha256"],hash(rb));assert_eq!(h["bytes"],rb.len());}save(dir.join("COTEJO-CALIDAD.json"),&serde_json::to_vec_pretty(&json!({"estado":"conforme","commit":r["commit"],"archivos":m["archivos"],"autoria_licencia":footer}))?)?;println!("CALIDAD: 9 archivos recuperados idénticos por bytes y SHA-256");
 }else{return Err("Modo desconocido".into());}Ok(())
}
