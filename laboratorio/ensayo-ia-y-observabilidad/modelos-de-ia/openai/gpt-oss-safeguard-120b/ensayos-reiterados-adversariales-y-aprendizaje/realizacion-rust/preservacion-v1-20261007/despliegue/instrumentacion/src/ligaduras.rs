//! Integridad documental LIG/0.1. No constituye autoridad R1 ni valores SV.
use sv_core::{bindings::*, compile_svp, Nat, IrProgram};
use crate::{Pagina, Ficha, huella};
pub const PROGRAMA: &str = r#"
semantic_relation R { kind: DeclaredRelation; constraints: [Local]; }
graph G { nodes: []; edges: []; relation: R; regime: Simple; }
horizon H { architecture: G; events: [Recepcion]; }
capture_spec Cap { parameter_id: 1; observation_domain: Bytes; observation_space: Documento; failure_symbol: Bottom; mapping: Captura; }
admissibility_spec Adm { parameter_id: 1; states: {Ok,Degraded,NotAdmitted}; rule: Integridad; }
ternarizer TerDeclarado { observation_space: Documento; partition_zero: B0; partition_one: B1; partition_u: BU; mapping: NoEjecutado; }
domain D { parameters: [Pagina]; interface: Documental; horizon: H; capture_specs: [Cap]; admissibility_specs: [Adm]; ternarizers: [TerDeclarado]; exogeneity_mask: Exo; silent_u: SU; transduction_policy: TP; u_policy: UP; closure_criterion: Cierre; }
agent AG { architecture: G; domain: D; query_engine: Documental; }
"#;
fn artefacto(id:&str,tipo:ArtifactKind,bytes:&[u8])->BindingArtifact {
    BindingArtifact{reference:ExactReference{identifier:id.into(),version:"1".into(),sha256:huella(bytes)},kind:tipo,bytes:bytes.to_vec()}
}
pub fn contrato(f:&Ficha)->Result<(IrProgram,BindingContract,BindingRequest),String> {
    let p=compile_svp(PROGRAMA,"suministro-documental.svp").map_err(|e|format!("{e:?}"))?;
    let mut a=vec![
        artefacto("Constitucion",ArtifactKind::Constitution,b"Portador documental experimental; no arquitectura productiva ni celula SV."),
        artefacto("Declaracion",ArtifactKind::AuthorityDeclaration,b"Declaracion documental sintetica; no autentica titulares ni concede autoridad R1."),
        artefacto("Captura",ArtifactKind::CaptureDefinition,b"Recibir bytes del MCP real con originales y correspondencia de solicitud."),
        artefacto("Integridad",ArtifactKind::AdmissionDefinition,b"Cotejar identidad, revision y conjunto exacto previamente fijados; rechazar discrepancias."),
        artefacto("Suministrar",ArtifactKind::OperationDefinition,include_bytes!("ligaduras.rs")),
        artefacto("Instrumentacion",ArtifactKind::OperationDefinition,include_bytes!("lib.rs")),
    ];
    for pagina in f.paginas(){a.push(artefacto(&format!("Pagina{}",pagina.indice),ArtifactKind::Provenance,pagina.texto.as_bytes()));}
    let get=|s:&str|a.iter().find(|a|a.reference.identifier==s).unwrap().reference.clone();
    let inst=f.paginas().iter().map(|pagina|ParameterInstanceBinding{
        identifier:format!("I{}",pagina.indice),owner:"D".into(),parameter:"Pagina".into(),parameter_id:Nat::from_u64(1),
        capture:RuleBinding{object:"Cap".into(),definition:get("Captura")},
        admission:RuleBinding{object:"Adm".into(),definition:get("Integridad")},ternarizer:None,
        provenance:vec![get(&format!("Pagina{}",pagina.indice))],
    }).collect();
    let c=BindingContract{schema:BINDING_SCHEMA.into(),identifier:"SG-RETROALIMENTACION-20261003-r1".into(),version:"1".into(),
        program:ProgramIdentity::of(&p),domain:"D".into(),agent:"AG".into(),constitution:get("Constitucion"),authority:get("Declaracion"),
        instances:inst,operations:vec![OperationBindings{identifier:"Suministrar".into(),version:"1".into(),definition:get("Suministrar"),
            uses:(0..2).map(|i|BindingUse{identifier:format!("U{i}"),instance:format!("I{i}"),destination:None,alias_of:None}).collect(),
            requires_destination:false,sharing:vec![],input_scope:BindingInputScope::BindingsOnly,side_information:vec![]}],artifacts:a};
    let q=BindingRequest{contract:ExactReference{identifier:c.identifier.clone(),version:c.version.clone(),sha256:binding_contract_sha256(&c)},
        operation:"Suministrar".into(),operation_version:"1".into()};
    Ok((p,c,q))
}
pub struct DocumentosLigados { ligaduras:ValidatedBindings, paginas:Vec<Pagina> }
impl DocumentosLigados {
    pub fn paginas(&self)->&[Pagina]{&self.paginas}
    pub fn ligaduras(&self)->&ValidatedBindings{&self.ligaduras}
}
pub fn recibir(f:&Ficha,recibidas:&[Pagina])->Result<DocumentosLigados,String> {
    f.cobertura(recibidas).map_err(|e|format!("{e:?}"))?;
    let (p,mut c,q)=contrato(f)?;
    // Las referencias siguen fijadas a la fuente; sólo cambian los bytes recibidos.
    for pagina in recibidas {
        c.artifacts.iter_mut().find(|a|a.reference.identifier==format!("Pagina{}",pagina.indice)).unwrap().bytes=pagina.texto.as_bytes().to_vec();
    }
    let v=validate_bindings(&p,c,&q).map_err(|e|format!("{e:?}"))?;
    let mut paginas=Vec::new();
    for (i,inst) in v.instances_in_use_order().enumerate(){
        let a=v.contract().artifacts.iter().find(|a|a.reference==inst.provenance[0]).ok_or("Procedencia ausente")?;
        let mut pagina=f.paginas()[i].clone(); pagina.texto=String::from_utf8(a.bytes.clone()).map_err(|e|e.to_string())?; paginas.push(pagina);
    }
    Ok(DocumentosLigados{ligaduras:v,paginas})
}
#[cfg(test)]mod tests {
 use super::*;
 fn ficha()->Ficha {Ficha::nueva((0..2).map(|i|Pagina{documento:"S".into(),seccion:"S".into(),revision:"r1".into(),indice:i,total:2,texto:format!("Pagina {i}")}).collect()).unwrap()}
 #[test]fn uso_publico_real_preserva_bytes_y_orden(){let f=ficha();let v=recibir(&f,f.paginas()).unwrap();assert_eq!(v.paginas(),f.paginas());assert!(v.ligaduras().instances_in_use_order().all(|i|i.ternarizer.is_none()));assert!(v.ligaduras().operation().uses.iter().all(|u|u.destination.is_none()));}
 #[test]fn nucleo_rechaza_artefacto_alterado(){let f=ficha();let(p,mut c,mut q)=contrato(&f).unwrap();c.artifacts.last_mut().unwrap().bytes.push(1);q.contract.sha256=binding_contract_sha256(&c);let e=validate_bindings(&p,c,&q).unwrap_err();assert_eq!(e.kind,BindingErrorKind::ArtifactIntegrity);}
 #[test]fn nucleo_rechaza_version_de_procedencia_distinta(){let f=ficha();let(p,mut c,mut q)=contrato(&f).unwrap();c.instances[0].provenance[0].version="2".into();q.contract.sha256=binding_contract_sha256(&c);assert_eq!(validate_bindings(&p,c,&q).unwrap_err().kind,BindingErrorKind::ExactReference);}
 #[test]fn nucleo_rechaza_operacion_no_fijada(){let f=ficha();let(p,c,mut q)=contrato(&f).unwrap();q.operation="Otra".into();assert_eq!(validate_bindings(&p,c,&q).unwrap_err().kind,BindingErrorKind::OperationIdentity);}
}
