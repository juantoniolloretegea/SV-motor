//! Preparación de una copia de fuentes; no activa servicios ni carga pesos.
use std::{fs::{self,OpenOptions},io::Write,path::Path,process::Command};
type R<T>=Result<T,Box<dyn std::error::Error>>;
const BASE:&str="/opt/sv-qwen80/examen-09";
const OLD:&str="/opt/sv-qwen80/examen-09/recuperacion-contexto";
const NEW:&str="/opt/sv-qwen80/examen-09/recuperacion-parametros";
fn save(p:&Path,b:&[u8])->R<()>{let mut f=OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;fs::File::open(p.parent().ok_or("PADRE")?)?.sync_all()?;Ok(())}
fn tree(s:&Path,d:&Path)->R<()>{let m=fs::symlink_metadata(s)?;if m.file_type().is_symlink(){return Err("ENLACE".into())}if m.is_dir(){fs::create_dir(d)?;for e in fs::read_dir(s)?{let e=e?;if ["target",".git"].iter().any(|n|e.file_name()==*n){continue}tree(&e.path(),&d.join(e.file_name()))?}}else{if m.len()>20*1024*1024{return Err("FUENTE_EXCESIVA".into())}let b=fs::read(s)?;save(d,&b)?;if fs::read(d)?!=b{return Err("COPIA_DIVERGENTE".into())}}Ok(())}
fn patch(s:&mut String,old:&str,new:&str)->R<()>{if s.matches(old).count()!=1{return Err(format!("PARCHE_NO_UNIVOCO:{old}").into())}*s=s.replacen(old,new,1);Ok(())}
fn main()->R<()>{
 if fs::read_to_string("/etc/hostname")?.trim()!="modelos-ia-qwen3-next-80b-a3b-instruct"||fs::read_to_string("/proc/sys/kernel/random/boot_id")?.trim()!="9188529d-a078-4aed-aa91-6794806e1da6"{return Err("IDENTIDAD".into())}
 let state=Command::new("systemctl").args(["show","sv-qwen80.service","sv-qwen80-examen-09-contexto.service","-p","MainPID","-p","ActiveState"]).output()?;
 let s=String::from_utf8(state.stdout)?;if !state.status.success()||s.lines().filter(|l|*l=="MainPID=0").count()!=2||s.lines().any(|l|l=="ActiveState=active"||l=="ActiveState=activating"){return Err("PROCESOS_ACTIVOS".into())}
 for(n,h)in[("query.rs","040d3e70a5c354fa7de77665a18cd170816aa26abb45d491f965e48eb44c8df1"),("recovery.rs","6380d5d5ee4c98ae2214dd0958ed621ad9b00ee1ede90300d8c1795ba6d36020"),("functional.rs","69a2d306166fc6828c2e8700cd1fe22e56d77036cb8d7115be4b7a4247227a4d"),("main.rs","6b8bd693fe90a799635ae54bb62c50c0ad9230f28fb4df530d8369541d0489ff")]{let o=Command::new("sha256sum").arg(format!("{OLD}/supervisor/src/{n}")).output()?;if !o.status.success()||String::from_utf8(o.stdout)?.split_whitespace().next()!=Some(h){return Err(format!("FUENTE_DISCORDANTE:{n}").into())}}
 let root=Path::new(NEW);fs::create_dir(root)?;for n in ["bin","evidencias","configuracion","fuentes-correccion"]{fs::create_dir(root.join(n))?}save(&root.join("evidencias/ESTADO-PREVIO.txt"),s.as_bytes())?;
 for n in ["supervisor","mcp-0.1.2","preguntas.json"]{tree(&Path::new(OLD).join(n),&root.join(n))?}
 for n in ["param_recovery.rs","param_query_tests.rs","gestionar-parametros.rs","preparar-parametros.rs"]{tree(&Path::new(BASE).join(n),&root.join("fuentes-correccion").join(n))?}
 let src=root.join("supervisor/src");
 let mut q=fs::read_to_string(src.join("query.rs"))?;
 patch(&mut q,"return Err(\"PARAMETRO_AMBIGUO_O_DUPLICADO\".into())","return Err(crate::recovery::ItemCode::MalformedParameters.fault())")?;
 q.push_str(&fs::read_to_string(Path::new(BASE).join("param_query_tests.rs"))?);
 let mut r=fs::read_to_string(src.join("recovery.rs"))?;
 patch(&mut r,"MultipleDocumentCalls,ContextLimit,OutputLimit,GenerationLimit,CallLimit","MultipleDocumentCalls,ContextLimit,OutputLimit,GenerationLimit,CallLimit,MalformedParameters")?;
 patch(&mut r,"if d.codigo==ItemCode::MultipleDocumentCalls","if matches!(d.codigo,ItemCode::MultipleDocumentCalls|ItemCode::MalformedParameters)")?;
 let mut f=fs::read_to_string(src.join("functional.rs"))?;
 patch(&mut f,"let path=base.join(\"CAMPANA.json\");","if p.is_exam()&&run==crate::param_recovery::RUN{crate::param_recovery::verify_resume(base)?;}\n let path=base.join(\"CAMPANA.json\");")?;
 let mut m=fs::read_to_string(src.join("main.rs"))?;
 patch(&mut m,"mod context_recovery;","mod context_recovery;\nmod param_recovery;")?;
 patch(&mut m,"Some(\"preparar-recuperacion-contexto\") if a.len()==2=>context_recovery::prepare(),","Some(\"preparar-recuperacion-contexto\") if a.len()==2=>context_recovery::prepare(),\n Some(\"preparar-recuperacion-parametros\") if a.len()==2=>param_recovery::prepare(),")?;
 for(n,t)in[("query.rs",q),("recovery.rs",r),("functional.rs",f),("main.rs",m)]{let p=src.join(n);let mut file=OpenOptions::new().write(true).truncate(true).open(&p)?;file.write_all(t.as_bytes())?;file.sync_all()?;if fs::read_to_string(p)?!=t{return Err("ESCRITURA_DISCORDANTE".into())}}
 tree(&Path::new(BASE).join("param_recovery.rs"),&src.join("param_recovery.rs"))?;
 tree(&Path::new(BASE).join("gestionar-parametros.rs"),&src.join("bin/gestionar_parametros.rs"))?;
 println!("FUENTES_04_PREPARADAS; MODELO_DETENIDO; SIN_CAMBIOS_EN_MOTOR_MCP_BANCO_O_DEPENDENCIAS");Ok(())
}
