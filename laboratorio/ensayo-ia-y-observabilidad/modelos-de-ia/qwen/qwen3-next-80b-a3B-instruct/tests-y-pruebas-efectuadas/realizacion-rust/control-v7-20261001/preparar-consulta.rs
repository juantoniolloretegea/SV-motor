//! Copia independiente y transformación limitada de fuentes; no activa servicios.
use std::{fs::{self,OpenOptions},io::Write,path::Path,process::Command};
type R<T>=Result<T,Box<dyn std::error::Error>>;
const BASE:&str="/opt/sv-qwen80/examen-09";
const OLD:&str="/opt/sv-qwen80/examen-09/recuperacion-parametros";
const NEW:&str="/opt/sv-qwen80/examen-09/recuperacion-consulta";
fn save(p:&Path,b:&[u8])->R<()>{let mut f=OpenOptions::new().create_new(true).write(true).open(p)?;f.write_all(b)?;f.sync_all()?;fs::File::open(p.parent().ok_or("PADRE")?)?.sync_all()?;Ok(())}
fn tree(s:&Path,d:&Path)->R<()>{let m=fs::symlink_metadata(s)?;if m.file_type().is_symlink(){return Err("ENLACE".into())}if m.is_dir(){fs::create_dir(d)?;for e in fs::read_dir(s)?{let e=e?;if ["target",".git"].iter().any(|n|e.file_name()==*n){continue}tree(&e.path(),&d.join(e.file_name()))?}}else{if m.len()>20*1024*1024{return Err("FUENTE_EXCESIVA".into())}let b=fs::read(s)?;save(d,&b)?;if fs::read(d)?!=b{return Err("COPIA_DIVERGENTE".into())}}Ok(())}
fn patch(s:&mut String,old:&str,new:&str)->R<()>{if s.matches(old).count()!=1{return Err(format!("PARCHE_NO_UNIVOCO:{old}").into())}*s=s.replacen(old,new,1);Ok(())}
fn write(p:&Path,s:&str)->R<()>{let mut f=OpenOptions::new().write(true).truncate(true).open(p)?;f.write_all(s.as_bytes())?;f.sync_all()?;if fs::read_to_string(p)?!=s{return Err("ESCRITURA_DISCORDANTE".into())}Ok(())}
fn main()->R<()>{
 if fs::read_to_string("/etc/hostname")?.trim()!="modelos-ia-qwen3-next-80b-a3b-instruct"||fs::read_to_string("/proc/sys/kernel/random/boot_id")?.trim()!="9188529d-a078-4aed-aa91-6794806e1da6"{return Err("IDENTIDAD".into())}
 let o=Command::new("systemctl").args(["show","sv-qwen80.service","sv-qwen80-examen-09-parametros.service","-p","MainPID","-p","ActiveState"]).output()?;let state=String::from_utf8(o.stdout)?;if !o.status.success()||state.lines().filter(|l|*l=="MainPID=0").count()!=2||state.lines().any(|l|l=="ActiveState=active"||l=="ActiveState=activating"){return Err("PROCESOS_ACTIVOS".into())}
 for(n,h)in[("query.rs","eccf2a8071047f7ca531ad80ced4df45033a1f62601c5437169b41184365d05d"),("recovery.rs","8d86a100a6b283392387e299ddcf66e17e172f4be4c3182667342dfed83600e3"),("functional.rs","d1f3f9e0948c94fe82b81c3cfcbc08f77286514e811a07801e1e66b3e206b394"),("main.rs","329d7b2575f40eff79fe7f8c4e7059881311253a74328d4eeee3667911f21a02")]{let o=Command::new("sha256sum").arg(format!("{OLD}/supervisor/src/{n}")).output()?;if !o.status.success()||String::from_utf8(o.stdout)?.split_whitespace().next()!=Some(h){return Err(format!("FUENTE_DISCORDANTE:{n}").into())}}
 let root=Path::new(NEW);fs::create_dir(root)?;for n in ["bin","evidencias","configuracion","fuentes-correccion"]{fs::create_dir(root.join(n))?}save(&root.join("evidencias/ESTADO-PREVIO.txt"),state.as_bytes())?;
 for n in ["supervisor","mcp-0.1.2","preguntas.json"]{tree(&Path::new(OLD).join(n),&root.join(n))?}
 for n in ["consulta_recovery.rs","completion_policy.rs","consulta_query_tests.rs","preparar-consulta.rs","gestionar-parametros.rs"]{tree(&Path::new(BASE).join(n),&root.join("fuentes-correccion").join(n))?}
 let src=root.join("supervisor/src");
 let mut q=fs::read_to_string(src.join("query.rs"))?;
 patch(&mut q,"return Err(\"CONSULTA_FUERA_DE_COTA\".into())","return Err(crate::recovery::ItemCode::SearchQueryBounds.fault())")?;
 // Option expresa ausencia de vencimiento. No se inventa una fecha futura.
 q=q.replace("end:Instant)->Result<Value>{","end:impl Into<Option<Instant>>)->Result<Value>{\n let end=end.into();");
 patch(&mut q,"let remaining=end.checked_duration_since(Instant::now()).ok_or(\"PLAZO_CONSULTA\")?;\n  let response=client.post(format!(\"http://127.0.0.1:1234{path}\")).header(\"content-type\",\"application/json\").timeout(remaining).body(raw).send()?;","let response=crate::completion_policy::request(client.post(format!(\"http://127.0.0.1:1234{path}\")).header(\"content-type\",\"application/json\"),end)?.body(raw).send()?;")?;
 patch(&mut q,"client.post(\"http://127.0.0.1:1234/v1/chat/completions\").header(\"content-type\",\"application/json\").timeout(end.checked_duration_since(Instant::now()).ok_or(\"PLAZO\")?).body(raw).send()?","crate::completion_policy::request(client.post(\"http://127.0.0.1:1234/v1/chat/completions\").header(\"content-type\",\"application/json\"),end)?.body(raw).send()?")?;
 patch(&mut q,"let left=crate::continuation::remaining(&campaign,crate::continuation::QUERY_RESERVE_MS)?;\n let end=Instant::now()+Duration::from_millis(left);","let left=crate::completion_policy::window(&campaign,crate::continuation::QUERY_RESERVE_MS)?;\n let end=left.map(|ms|Instant::now()+Duration::from_millis(ms));")?;
 patch(&mut q,"end.checked_duration_since(Instant::now()).ok_or(\"PLAZO\")?.as_millis().min(30000)as u64","crate::completion_policy::mcp_ms(end)?")?;
 q.push_str(&fs::read_to_string(Path::new(BASE).join("consulta_query_tests.rs"))?);
 let mut r=fs::read_to_string(src.join("recovery.rs"))?;patch(&mut r,"CallLimit,MalformedParameters","CallLimit,MalformedParameters,SearchQueryBounds")?;
 let mut f=fs::read_to_string(src.join("functional.rs"))?;
 patch(&mut f,"if self.economic_deadline_utc_ms<=store::now()","if !crate::completion_policy::policy(self)&&self.economic_deadline_utc_ms<=store::now()")?;
 patch(&mut f,"let path=base.join(\"CAMPANA.json\");","if crate::completion_policy::policy(p){if run!=crate::consulta_recovery::RUN{return Err(\"SEGMENTO_SIN_AUTORIZACION\".into())}crate::consulta_recovery::verify_resume(base)?;}\n let path=base.join(\"CAMPANA.json\");")?;
 patch(&mut f,"crate::continuation::remaining(&v,crate::continuation::QUERY_RESERVE_MS)?;","if crate::completion_policy::policy(p){if !crate::completion_policy::unbounded(&v)?{return Err(\"AUTORIZACION_AUSENTE\".into())}durable(&base.join(format!(\"CARGA-{run}.json\")),&json!({\"run\":run,\"utc_ms\":store::now(),\"campana\":v}))?;return Ok(v)}\n crate::continuation::remaining(&v,crate::continuation::QUERY_RESERVE_MS)?;")?;
 let mut c=fs::read_to_string(src.join("control.rs"))?;
 patch(&mut c,"p.validate()?;if self.instrumental","p.validate()?;if crate::completion_policy::policy(p)&&(self.run_id!=crate::consulta_recovery::RUN||self.base!=Path::new(crate::consulta_recovery::NEW)){return Err(\"ALCANCE_CONTINUACION_DISTINTO\".into())}if self.instrumental")?;
 patch(&mut c,"let global=data[\"plazo_global_utc_ms\"]","if crate::completion_policy::unbounded(data)?{f.absolute_deadline_utc_ms=None;f.absolute_deadline_mono_ms=None;f.phase_deadline_ms=None;}else{\n          let global=data[\"plazo_global_utc_ms\"]")?;
 patch(&mut c,"f.phase_deadline_ms=f.absolute_deadline_mono_ms;\n         }","f.phase_deadline_ms=f.absolute_deadline_mono_ms;\n         }}")?;
 let mut e=fs::read_to_string(src.join("examen.rs"))?;
 patch(&mut e,"if begin.elapsed()>Duration::from_secs(43110)","if crate::completion_policy::driver_expired(c.functional.as_ref().is_some_and(crate::completion_policy::policy),begin.elapsed())")?;
 patch(&mut e,"crate::continuation::remaining(&read(&c.base.join(\"CAMPANA.json\"))?,90000)?;","crate::completion_policy::window(&read(&c.base.join(\"CAMPANA.json\"))?,90000)?;")?;
 let mut m=fs::read_to_string(src.join("main.rs"))?;patch(&mut m,"mod param_recovery;","mod param_recovery;\nmod consulta_recovery;\nmod completion_policy;")?;patch(&mut m,"Some(\"preparar-recuperacion-parametros\") if a.len()==2=>param_recovery::prepare(),","Some(\"preparar-recuperacion-parametros\") if a.len()==2=>param_recovery::prepare(),\n Some(\"preparar-recuperacion-consulta\") if a.len()==2=>consulta_recovery::prepare(),")?;
 for(n,t)in[("query.rs",q),("recovery.rs",r),("functional.rs",f),("control.rs",c),("examen.rs",e),("main.rs",m)]{write(&src.join(n),&t)?}
 let mut rr=fs::read_to_string(Path::new(BASE).join("consulta_recovery.rs"))?;
 patch(&mut rr,"pub fn verify_resume(base:&Path)->Result<()>{deadline(base)?;","pub fn verify_resume(base:&Path)->Result<()>{if !crate::completion_policy::unbounded(&read(&base.join(\"CAMPANA.json\"))?)?{return Err(\"AUTORIZACION_AUSENTE\".into())}deadline(Path::new(OLD))?;")?;
 patch(&mut rr,"let mut copies=vec![copy(&old.join(\"CAMPANA.json\"),&new.join(\"CAMPANA.json\"))?];","let mut copies=vec![copy(&old.join(\"CAMPANA.json\"),&new.join(\"CAMPANA-ANTERIOR.json\"))?];durable(&new.join(\"CAMPANA.json\"),&crate::completion_policy::campaign(&read(&old.join(\"CAMPANA.json\"))?)?)?;")?;
 patch(&mut rr,"\"plazo_ampliado\":false","\"plazo_ampliado\":true,\"autorizacion\":crate::completion_policy::AUTH,\"terminacion\":\"P25_O_FALLO_TECNICO\"")?;save(&src.join("consulta_recovery.rs"),rr.as_bytes())?;tree(&Path::new(BASE).join("completion_policy.rs"),&src.join("completion_policy.rs"))?;
 let mut g=fs::read_to_string(Path::new(BASE).join("gestionar-parametros.rs"))?;
 g=g.replace("recuperacion-parametros","recuperacion-consulta").replace("recuperacion-contexto","recuperacion-parametros").replace("examen09-20260930-04","examen09-20260930-05").replace("examen09-20260930-03","examen09-20260930-04").replace("examen-09-parametros.service","examen-09-consulta.service").replace("examen-09-contexto.service","examen-09-parametros.service").replace("cierre-parametros","cierre-consulta").replace("cerrada-contexto","cerrada-parametros").replace("SEGMENTO-03","SEGMENTO-04").replace("segmento03","segmento04").replace("SEGMENTO_03","SEGMENTO_04").replace("FUENTES_04","FUENTES_05").replace("2026-09-30 11:03:40 UTC","2026-09-30 12:23:00 UTC").replace("preparar-recuperacion-parametros","preparar-recuperacion-consulta").replace("zz-recuperacion-consulta.conf","zzz-recuperacion-consulta.conf").replace("P16","P23");
 let start=g.find("fn remaining()->R<u64>{").ok_or("GESTOR_INICIO")?;let end=g[start..].find("\nfn stopped").ok_or("GESTOR_FIN")?+start;
 g.replace_range(start..end,"fn remaining()->R<&'static str>{identity()?;let b=fs::read(format!(\"{BASE}/recepcion/CAMPANA.json\"))?;if hash(&b)!=\"fcc942d3249f852c03c240795243c47aec791cde6463a195f863c0fa96b82011\"{return Err(\"CAMPANA_ORIGINAL_MODIFICADA\".into())}Ok(\"infinity\")}");
 patch(&mut g,"c[\"auto_start\"]=json!(false);c[\"allow_model_loading\"]=json!(true);","c[\"auto_start\"]=json!(false);c[\"allow_model_loading\"]=json!(true);c[\"functional\"][\"economic_deadline_utc_ms\"]=json!(0);")?;
 g=g.replace("\"VENCIMIENTO_ORIGINAL\"","\"HASTA_P25\"").replace("SIGUIENTE_P23; VENCIMIENTO_ORIGINAL","SIGUIENTE_P23; HASTA_P25");
 patch(&mut g,"\"-p\",\"ExecStart\",","\"-p\",\"RuntimeMaxUSec\",\"-p\",\"ExecStart\",")?;patch(&mut g,"for t in [\"MemoryMax","for t in [\"RuntimeMaxUSec=infinity\",\"MemoryMax")?;
 save(&src.join("bin/gestionar_consulta.rs"),g.as_bytes())?;
 println!("FUENTES_05_PREPARADAS; SIN_ACTIVACION; MCP_MOTOR_BANCO_DEPENDENCIAS_INTACTOS");Ok(())
}
