//! Perfil experimental del Árbitro - Director para inferencia local.
//! Reutiliza LIG/0.1. No constituye autoridad R1 ni una célula SV.
use super::*;
use selector::{Clase, Decision, Politica, Recurso, Solicitud};
use sv_core::{bindings::*, compile_svp, Nat};
use std::collections::BTreeSet;

pub const NUCLEO: &str = "1c6b84238e62e867c4d5c5ab8069c9ae8f67dccc";
pub const ORDEN: [Option<Clase>;8] = [None,Some(Clase::Cpu),Some(Clase::Gpu),Some(Clase::Cpu),Some(Clase::Gpu),None,Some(Clase::Cpu),Some(Clase::Gpu)];
// Portador documental de ligaduras; declaraciones nominales, sin ejecución
// celular, ternarización ni construcción de autoridad productiva.
const PROGRAMA: &str = r#"
semantic_relation R { kind: DeclaredRelation; constraints: [Local]; }
graph G { nodes: []; edges: []; relation: R; regime: Simple; }
horizon H { architecture: G; events: [Recepcion]; }
capture_spec Cap { parameter_id: 1; observation_domain: Bytes; observation_space: Documento; failure_symbol: Bottom; mapping: Captura; }
admissibility_spec Adm { parameter_id: 1; states: {Ok,Degraded,NotAdmitted}; rule: Integridad; }
ternarizer TerDeclarado { observation_space: Documento; partition_zero: B0; partition_one: B1; partition_u: BU; mapping: NoEjecutado; }
domain D { parameters: [Entrada]; interface: Documental; horizon: H; capture_specs: [Cap]; admissibility_specs: [Adm]; ternarizers: [TerDeclarado]; exogeneity_mask: Exo; silent_u: SU; transduction_policy: TP; u_policy: UP; closure_criterion: Cierre; }
agent AG { architecture: G; domain: D; query_engine: Documental; }
"#;
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn art(id:&str,kind:ArtifactKind,bytes:&[u8])->BindingArtifact{BindingArtifact{reference:ExactReference{identifier:id.into(),version:"1".into(),sha256:sha(bytes)},kind,bytes:bytes.to_vec()}}

/// Validación pública del núcleo sobre los bytes reales y la política fijada.
/// La procedencia documental no se confunde con autenticación de titulares.
pub fn ligar(esperada:&[u8],recibida:&[u8],politica:&[u8])->R<Value>{
 let p=compile_svp(PROGRAMA,"inferencia-qwen.svp").map_err(|e|format!("Compilación documental: {e:?}"))?;
 let a=vec![art("Constitucion",ArtifactKind::Constitution,b"Perfil experimental de inferencia Qwen; sin autoridad R1 ni celula SV."),
 art("Declaracion",ArtifactKind::AuthorityDeclaration,b"Declaracion documental del ensayo autorizado; no autentica titulares ni crea permisos productivos."),
 art("Captura",ArtifactKind::CaptureDefinition,b"Entrada efectiva en tokens, pesos identificados y politica fijada fuera del modelo."),
 art("Integridad",ArtifactKind::AdmissionDefinition,b"Igualdad exacta de contenido y revision; rechazo antes del calculo."),
 art("Inferir",ArtifactKind::OperationDefinition,include_bytes!("gobierno.rs")),
 art("Entrada",ArtifactKind::Provenance,esperada),art("Politica",ArtifactKind::Provenance,politica)];
 let get=|id:&str|a.iter().find(|a|a.reference.identifier==id).unwrap().reference.clone();
 let instances=["Entrada","Politica"].iter().enumerate().map(|(i,id)|ParameterInstanceBinding{identifier:format!("I{i}"),owner:"D".into(),parameter:"Entrada".into(),parameter_id:Nat::from_u64(1),capture:RuleBinding{object:"Cap".into(),definition:get("Captura")},admission:RuleBinding{object:"Adm".into(),definition:get("Integridad")},ternarizer:None,provenance:vec![get(id)]}).collect();
 let mut c=BindingContract{schema:BINDING_SCHEMA.into(),identifier:"QWEN-ARBITRO-SELECTOR-20261014".into(),version:"1".into(),program:ProgramIdentity::of(&p),domain:"D".into(),agent:"AG".into(),constitution:get("Constitucion"),authority:get("Declaracion"),instances,operations:vec![OperationBindings{identifier:"Inferir".into(),version:"1".into(),definition:get("Inferir"),uses:(0..2).map(|i|BindingUse{identifier:format!("U{i}"),instance:format!("I{i}"),destination:None,alias_of:None}).collect(),requires_destination:false,sharing:vec![],input_scope:BindingInputScope::BindingsOnly,side_information:vec![]}],artifacts:a};
 let digest=binding_contract_sha256(&c);
 let q=BindingRequest{contract:ExactReference{identifier:c.identifier.clone(),version:c.version.clone(),sha256:digest.clone()},operation:"Inferir".into(),operation_version:"1".into()};
 c.artifacts.iter_mut().find(|a|a.reference.identifier=="Entrada").unwrap().bytes=recibida.to_vec();
 let v=validate_bindings(&p,c,&q).map_err(|e|format!("Ligadura rechazada: {e:?}"))?;
 if v.instances_in_use_order().count()!=2 || v.instances_in_use_order().any(|i|i.ternarizer.is_some()) {return Err("Ligaduras fuera del perfil".into());}
 Ok(json!({"nucleo_revision":NUCLEO,"contrato_sha256":digest,"entrada_sha256":sha(recibida),"politica_sha256":sha(politica),"instancias":2,"autoridad_r1":false,"ternarizacion":false}))
}

pub struct Director { entrada:Value, destinos:Vec<selector::Destino>, usadas:BTreeSet<String>, numero:u64, anterior:String, dir:PathBuf, cerrado:bool }
impl Director{
 pub fn nuevo(entrada:Value,destinos:Vec<selector::Destino>,dir:&Path)->R<Self>{
  if entrada["modelo_sha256"]!=MODELO || entrada["texto"]!=PROMPT || entrada["max_salida"]!=16 || entrada["contexto"]!=64 || entrada["tokens"].as_array().is_none_or(|t|t.is_empty()||t.len()+16>64) {return Err("Perfil del Árbitro no admitido".into());}
  let mut s=Self{entrada,destinos,usadas:BTreeSet::new(),numero:0,anterior:"0".repeat(64),dir:dir.into(),cerrado:false};
  s.registrar("perfil",&json!({"entrada":s.entrada,"codigo_sha256":sha(include_bytes!("gobierno.rs")),"nucleo_revision":NUCLEO,"orden":ORDEN.iter().map(|c|c.map(conexion::nombre)).collect::<Vec<_>>()}))?;Ok(s)
 }
 fn registrar(&mut self,evento:&str,datos:&Value)->R<()> {
  if self.cerrado{return Err("Recorrido cerrado".into());}
  let base=json!({"numero":self.numero,"anterior":self.anterior,"evento":evento,"datos":datos}); let h=sha(&serde_json::to_vec(&base)?);
  if let Err(e)=guardar(&self.dir,&format!("ARBITRO-{:04}.json",self.numero),&json!({"base":base,"sha256":h})){self.cerrado=true;return Err(e);}
  self.numero+=1;self.anterior=h;Ok(())
 }
 pub fn peticion(&self,i:usize)->Value{json!({"id":format!("QAD-{:02}",i+1),"instancia":"Qwen3-8B-Q4_K_M/SV-Aprender","sesion":"QAD-20261014","entrada":self.entrada})}
 pub fn ejecutar<F,G>(&mut self,i:usize,raw:&[u8],rs:&[Recurso],ahora:u64,carga:&selector::Carga,fresco:G,calcular:F)->R<Value>
 where F:FnOnce(&Decision,&[u32])->R<Value>,G:FnOnce()->R<(u64,Vec<Recurso>)>{
  self.registrar("solicitud_original",&json!({"bytes":raw}))?;
  let r=self.recorrido(i,raw,rs,ahora,carga,fresco,calcular);
  match &r {Ok(v)=>self.registrar("devolucion",v)?,Err(e)=>self.registrar("rechazo",&json!({"id":format!("QAD-{:02}",i+1),"causa":e.to_string()}))?};r
 }
 fn recorrido<F,G>(&mut self,i:usize,raw:&[u8],rs:&[Recurso],ahora:u64,carga:&selector::Carga,fresco:G,calcular:F)->R<Value>
 where F:FnOnce(&Decision,&[u32])->R<Value>,G:FnOnce()->R<(u64,Vec<Recurso>)>{
  let clase=ORDEN.get(i).ok_or("Solicitud fuera de la serie autorizada")?;
  let esperada=serde_json::to_vec(&self.peticion(i))?;
  // La identidad sólo se consume después de cotejar los bytes. Una ejecución
  // admitida fallida no vuelve a habilitarse bajo el mismo identificador.
  if raw!=esperada {return Err("Solicitud o entrada distinta de la fijada".into());}
  let id=format!("QAD-{:02}",i+1);if !self.usadas.insert(id.clone()){return Err("Identidad ya consumida; no se repite".into());}
  let tokens=self.entrada["tokens"].as_array().unwrap().iter().map(|t|t.as_u64().and_then(|v|u32::try_from(v).ok()).ok_or("Token inválido")).collect::<Result<Vec<_>,_>>()?;
  if carga!=&generacion::carga(&tokens){return Err("Carga diferente de la entrada admitida".into());}
  let s=Solicitud{id:id.clone(),carga:carga.clone(),memoria_adicional_bytes:512*1024*1024,margen_bytes:512*1024*1024,residencia_obligatoria:None};
  let p=Politica{revision:format!("arbitro-qwen-generacion/v1/{}",clase.map(conexion::nombre).unwrap_or("automatico")),destinos_permitidos:self.destinos.clone(),preferencia:vec![Clase::Cpu,Clase::Gpu],clase_obligatoria:*clase,vigencia_ms:600_000,comparar_estimaciones:true};
  let pj=conexion::pjson(&p);let ligaduras=ligar(&esperada,raw,&serde_json::to_vec(&pj)?)?;
  self.registrar("admision",&json!({"id":id,"ligaduras":ligaduras,"politica":pj}))?;
  let d=selector::seleccionar(&s,&p,rs,ahora).map_err(|e|format!("Selector: {e:?}"))?;
  self.registrar("seleccion",&json!({"solicitud":conexion::solicitud_json(&s),"politica":pj,"recursos":conexion::rjson(rs),"decision":conexion::decision_json(&d)}))?;
  let (t,rr)=fresco()?;conexion::validar_envio(&d,&s,&p,&rr,t,&generacion::carga(&tokens))?;
  self.registrar("inicio_ejecucion",&json!({"id":id,"decision":conexion::decision_json(&d),"entrada_sha256":sha(raw),"recursos_revalidados":conexion::rjson(&rr),"ahora_ms":t}))?;
  let v=calcular(&d,&tokens)?;
  if v["id"]!=id || v["clase_ejecutada"]!=conexion::nombre(d.clase) || v["entrada_sha256"]!=generacion::tokens_sha(&tokens) || v["completo_instrumental"]!=true {return Err("Devolución incompatible con la solicitud admitida".into());}
  Ok(json!({"id":id,"decision":conexion::decision_json(&d),"ejecucion":v,"admision_instrumental":true,"evaluacion_semantica":null}))
 }
 pub fn cierre(&mut self)->R<()> {self.registrar("cierre",&json!({"identidades_consumidas":self.usadas,"finalizado":true}))?;self.cerrado=true;Ok(())}
}

#[cfg(test)]mod tests{
 use super::*;
 #[test]fn ligadura_conserva_entrada_y_politica(){let v=ligar(b"entrada",b"entrada",b"politica").unwrap();assert_eq!(v["instancias"],2);assert_eq!(v["autoridad_r1"],false);}
 #[test]fn nucleo_rechaza_entrada_alterada(){assert!(ligar(b"entrada",b"otra",b"politica").is_err());}
 #[test]fn politica_modifica_identidad_del_contrato(){assert_ne!(ligar(b"x",b"x",b"cpu").unwrap()["contrato_sha256"],ligar(b"x",b"x",b"gpu").unwrap()["contrato_sha256"]);}
 fn contexto()->(Director,Vec<Recurso>,selector::Carga){
  static N:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(0);
  let dir=PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/pruebas-gobierno").join(format!("{}-{}",std::process::id(),N.fetch_add(1,std::sync::atomic::Ordering::Relaxed)));fs::create_dir_all(&dir).unwrap();
  let ds=vec![conexion::destino("CPU-simulada","revision-prueba"),conexion::destino("GPU-simulada","revision-prueba")];let c=generacion::carga(&[1,2]);
  let rs=conexion::recursos(&c,&ds,&[20,10],"prueba-simulada",0,[2*1024*1024*1024;2]);
  let entrada=json!({"modelo_sha256":MODELO,"texto":PROMPT,"max_salida":16,"contexto":64,"tokens":[1,2]});
  (Director::nuevo(entrada,ds,&dir).unwrap(),rs,c)
 }
 fn simular(d:&Decision,t:&[u32])->R<Value>{Ok(json!({"id":d.solicitud,"clase_ejecutada":conexion::nombre(d.clase),"entrada_sha256":generacion::tokens_sha(t),"completo_instrumental":true}))}
 #[test]fn dirige_automatico_cpu_y_gpu_sin_admitir_politica_del_modelo(){for(i,cl)in[(0,"GPU"),(1,"CPU"),(2,"GPU")]{let(mut d,rs,c)=contexto();let q=serde_json::to_vec(&d.peticion(i)).unwrap();let v=d.ejecutar(i,&q,&rs,1,&c,||Ok((2,rs.clone())),simular).unwrap();assert_eq!(v["decision"]["clase"],cl);}}
 #[test]fn no_ejecuta_entrada_adulterada(){let(mut d,rs,c)=contexto();let mut q=d.peticion(0);q["entrada"]["tokens"]=json!([2,1]);assert!(d.ejecutar(0,&serde_json::to_vec(&q).unwrap(),&rs,1,&c,||panic!("No revalidar"),|_,_|panic!("No ejecutar")).is_err());}
 #[test]fn no_admite_clase_impuesta_por_el_candidato(){let(mut d,rs,c)=contexto();let mut q=d.peticion(0);q["politica"]=json!("GPU");assert!(d.ejecutar(0,&serde_json::to_vec(&q).unwrap(),&rs,1,&c,||panic!("No revalidar"),|_,_|panic!("No ejecutar")).is_err());}
 #[test]fn no_repite_identidad_consumida(){let(mut d,rs,c)=contexto();let q=serde_json::to_vec(&d.peticion(0)).unwrap();d.ejecutar(0,&q,&rs,1,&c,||Ok((2,rs.clone())),simular).unwrap();assert!(d.ejecutar(0,&q,&rs,3,&c,||panic!("No revalidar"),|_,_|panic!("No ejecutar")).is_err());}
 #[test]fn error_de_motor_no_reabre_la_identidad(){let(mut d,rs,c)=contexto();let q=serde_json::to_vec(&d.peticion(0)).unwrap();assert!(d.ejecutar(0,&q,&rs,1,&c,||Ok((2,rs.clone())),|_,_|Err("fallo simulado".into())).is_err());assert!(d.ejecutar(0,&q,&rs,3,&c,||panic!("No revalidar"),|_,_|panic!("No ejecutar")).is_err());}
 #[test]fn gpu_ausente_no_se_sustituye(){let(mut d,mut rs,c)=contexto();rs[1].disponible=false;let q=serde_json::to_vec(&d.peticion(2)).unwrap();assert!(d.ejecutar(2,&q,&rs,1,&c,||panic!("No revalidar"),|_,_|panic!("No ejecutar")).is_err());}
 #[test]fn gpu_desaparecida_despues_de_seleccion_no_se_sustituye(){let(mut d,rs,c)=contexto();let mut retirada=rs.clone();retirada[1].disponible=false;let q=serde_json::to_vec(&d.peticion(0)).unwrap();assert!(d.ejecutar(0,&q,&rs,1,&c,||Ok((2,retirada)),|_,_|panic!("No ejecutar")).is_err());}
 #[test]fn decision_caducada_impide_motor(){let(mut d,rs,c)=contexto();let q=serde_json::to_vec(&d.peticion(0)).unwrap();assert!(d.ejecutar(0,&q,&rs,1,&c,||Ok((5002,rs.clone())),|_,_|panic!("No ejecutar")).is_err());}
 #[test]fn devolucion_ajena_se_rechaza(){let(mut d,rs,c)=contexto();let q=serde_json::to_vec(&d.peticion(0)).unwrap();assert!(d.ejecutar(0,&q,&rs,1,&c,||Ok((2,rs.clone())),|de,t|{let mut v=simular(de,t)?;v["id"]=json!("otro");Ok(v)}).is_err());}
 #[test]fn cierre_no_permite_reapertura(){let(mut d,rs,c)=contexto();d.cierre().unwrap();let q=serde_json::to_vec(&d.peticion(0)).unwrap();assert!(d.ejecutar(0,&q,&rs,1,&c,||panic!("No revalidar"),|_,_|panic!("No ejecutar")).is_err());}
 #[test]fn fallo_de_custodia_impide_ejecucion(){let(mut d,rs,c)=contexto();fs::write(d.dir.join("ARBITRO-0001.json"),b"ocupado").unwrap();let q=serde_json::to_vec(&d.peticion(0)).unwrap();assert!(d.ejecutar(0,&q,&rs,1,&c,||panic!("No revalidar"),|_,_|panic!("No ejecutar")).is_err());assert!(d.cerrado);}
}
