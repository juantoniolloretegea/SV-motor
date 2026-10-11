//! Integración experimental de consulta. No modifica el registro administrativo.
#![forbid(unsafe_code)]
use serde::{Deserialize, Serialize};
use speccheck_core::{parser::{MarkdownParser, SpecParser}, rules::cross_file::CrossFileContradiction, ir::SourceSpan};
use std::{collections::BTreeSet, path::Path, time::Instant};
use sv_administracion_documental::{almacen::{Almacen, Carga, InformacionLectura, Uso}, huella};
pub type Resultado<T> = Result<T, String>;
const MAX_DOCUMENTOS: usize = 128;
const MAX_TEXTO: usize = 256 * 1024;
const MAX_TOTAL: usize = 4 * 1024 * 1024;

/// Sólo lo construye el adaptador autenticado; no admite deserialización HTTP.
pub struct Sesion { usuario: u64, hasta: u64, dominio: String }
impl Sesion {
    pub fn desde_adaptador(usuario: u64, hasta: u64, dominio: &str) -> Self {
        Self { usuario, hasta, dominio: dominio.into() }
    }
}
#[derive(Clone, Deserialize)]
#[serde(tag="modo", content="documentos", rename_all="snake_case", deny_unknown_fields)]
pub enum Seleccion { Omitir, Documento(String), Grupo(Vec<String>), Todos }
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Peticion { pub dominio: String, pub propuesto: String, pub seleccion: Seleccion }
/// El proveedor entrega bytes ya recibidos, nunca obtiene permisos de esta estructura.
pub struct Contenido { pub original: Vec<u8>, pub idioma: String, pub traduccion: Option<Traduccion> }
pub struct Traduccion { pub original_sha256: String, pub ingles: String, pub ingles_sha256: String, pub procedencia: String }
#[derive(Default, Serialize)]
pub struct Medicion { pub lecturas: usize, pub llamadas_comparador: usize, pub microsegundos: u128 }
#[derive(Serialize)]
pub struct Documento {
    pub id: String, pub dominio: String, pub titulo: String, pub original: String,
    pub ingles: String, pub original_sha256: String, pub ingles_sha256: String,
    pub revision_git: String, pub traduccion: Option<String>, pub derivacion: Option<serde_json::Value>,
}
#[derive(Serialize)]
pub struct Pasaje { pub documento: String, pub localizacion: SourceSpan, pub texto: String }
#[derive(Serialize)]
pub struct Sugerencia { pub propuesto: String, pub referencia: String, pub positivo: Pasaje, pub negativo: Pasaje }
#[derive(Serialize)]
pub struct Consulta { pub estado: String, pub revision_registro: Option<u64>, pub dominio: String, pub seleccionados: Vec<String>, pub documentos: Vec<Documento>, pub sugerencias: Vec<Sugerencia> }
fn exigir(ok: bool, causa: &str) -> Resultado<()> { if ok { Ok(()) } else { Err(causa.into()) } }
fn admitido(a: &Almacen, s: &Sesion, id: &str, ahora: u64) -> Resultado<InformacionLectura> {
    let c=a.contexto_lectura(s.usuario,s.hasta,id,ahora)?;
    exigir(c.dominio==s.dominio,"DOMINIO_DISCORDANTE")?;
    for u in [Uso::ConsultaPersonal,Uso::Extraccion,Uso::Fragmentos,Uso::Markdown] {
        exigir(c.usos_vigentes.get(&u)==Some(&true),"USO_COMPARACION_NO_ACREDITADO")?;
    }
    Ok(c)
}
fn omisible(e: &str) -> bool {
    ["USO_COMPARACION_NO_ACREDITADO","USO_NO_ACREDITADO","RECURSO_NO_VIGENTE","CARGA_NO_RECIBIDA","DOCUMENTO_ARCHIVADO_O_EN_PAPELERA"].contains(&e)
}
fn cargar(c: &InformacionLectura, contenido: Contenido) -> Resultado<Documento> {
    exigir(c.carga.ruta.ends_with(".md"),"REPRESENTACION_MARKDOWN_REQUERIDA")?;
    exigir(!contenido.original.is_empty() && contenido.original.len()<=MAX_TEXTO,"TEXTO_FUERA_DE_COTA")?;
    exigir(contenido.original.len() as u64==c.carga.bytes && huella(&contenido.original)==c.carga.sha256,"ORIGINAL_DISCORDANTE")?;
    let original=String::from_utf8(contenido.original).map_err(|_|"ORIGINAL_NO_UTF8")?;
    let (ingles,traduccion)=match (contenido.idioma.as_str(),contenido.traduccion) {
        ("en",None)=>(original.clone(),None),
        ("es",Some(t))=>{
            exigir(c.usos_vigentes.get(&Uso::Traduccion)==Some(&true),"TRADUCCION_NO_ACREDITADA")?;
            exigir(t.original_sha256==c.carga.sha256 && !t.ingles.is_empty() && t.ingles.len()<=MAX_TEXTO && huella(t.ingles.as_bytes())==t.ingles_sha256 && !t.procedencia.trim().is_empty(),"TRADUCCION_DISCORDANTE")?;
            (t.ingles,Some(t.procedencia))
        },
        _=>return Err("IDIOMA_O_TRADUCCION_NO_ADMITIDOS".into()),
    };
    Ok(Documento{id:c.carga.id.clone(),dominio:c.dominio.clone(),titulo:c.recurso.ficha.titulo.clone(),original,ingles_sha256:huella(ingles.as_bytes()),ingles,original_sha256:c.carga.sha256.clone(),revision_git:c.carga.revision_git.clone().ok_or("CARGA_NO_RECIBIDA")?,traduccion,derivacion:c.derivacion.as_ref().map(|d|serde_json::to_value(d).expect("estructura serializable"))})
}
fn pasaje(span: SourceSpan, docs: &[Documento]) -> Resultado<Pasaje> {
    let doc=docs.iter().find(|d|span.file==Path::new(&format!("{}.md",d.id))).ok_or("LOCALIZACION_AJENA")?;
    let texto=doc.ingles.get(span.byte_start..span.byte_end).filter(|s|!s.is_empty()).ok_or("LOCALIZACION_INVALIDA")?.to_owned();
    Ok(Pasaje{documento:doc.id.clone(),localizacion:span,texto})
}
/// Identidad, tiempo y proveedor deben proceder del servicio autenticado existente.
/// Revalidar la revisión y los permisos al volver a presentar un resultado conservado.
pub fn consultar(a: &Almacen, s: &Sesion, p: &Peticion, ahora: u64, mut leer: impl FnMut(&Carga)->Resultado<Contenido>, m: &mut Medicion) -> Resultado<Consulta> {
    let inicio=Instant::now();
    let resultado=(||{
        let mut out=Consulta{estado:"no_solicitada".into(),revision_registro:None,dominio:p.dominio.clone(),seleccionados:vec![],documentos:vec![],sugerencias:vec![]};
        if matches!(p.seleccion,Seleccion::Omitir){return Ok(out)}
        exigir(p.dominio==s.dominio,"DOMINIO_DISCORDANTE")?;
        a.admision(s.usuario,s.hasta,ahora)?;
        let vista=a.vista()?; exigir(vista["dominio"]==s.dominio,"DOMINIO_DISCORDANTE")?;
        let revision=vista["revision"].as_u64().ok_or("REVISION_AUSENTE")?;
        let propuesto=admitido(a,s,&p.propuesto,ahora)?;
        let ids=match &p.seleccion {
            Seleccion::Documento(id)=>vec![id.clone()],
            Seleccion::Grupo(ids)=>ids.clone(),
            Seleccion::Todos=>{
                let mut ids=vec![];
                for id in vista["cargas"].as_object().ok_or("CATALOGO_INVALIDO")?.keys() {
                    if id==&p.propuesto {continue}
                    match admitido(a,s,id,ahora) {Ok(_)=>ids.push(id.clone()),Err(e) if omisible(&e)=>{},Err(e)=>return Err(e)}
                }
                ids
            },
            Seleccion::Omitir=>unreachable!(),
        };
        exigir(ids.len()<=MAX_DOCUMENTOS,"SELECCION_SUPERA_COTA")?;
        let mut seen=BTreeSet::new();let mut contexts=vec![propuesto];
        for id in &ids {
            exigir(id!=&p.propuesto && seen.insert(id.clone()),"SELECCION_DUPLICADA_O_PROPIA")?;
            contexts.push(admitido(a,s,id,ahora)?);
        }
        exigir(a.revision()?==revision && contexts.iter().all(|c|c.revision==revision),"REGISTRO_CAMBIADO")?;
        out.revision_registro=Some(revision);out.seleccionados=ids;
        if out.seleccionados.is_empty(){out.estado="seleccion_vacia".into();return Ok(out)}
        // Completar admisión y cotas para todo el conjunto antes de leer o comparar.
        for c in &contexts {exigir(c.carga.ruta.ends_with(".md"),"REPRESENTACION_MARKDOWN_REQUERIDA")?;exigir(c.carga.bytes>0 && c.carga.bytes<=MAX_TEXTO as u64,"TEXTO_FUERA_DE_COTA")?;}
        exigir(contexts.iter().map(|c|c.carga.bytes).sum::<u64>()<=MAX_TOTAL as u64,"CONJUNTO_SUPERA_COTA")?;
        for c in &contexts {m.lecturas+=1;out.documentos.push(cargar(c,leer(&c.carga)?)?);}
        exigir(out.documentos.iter().map(|d|d.original.len()+d.ingles.len()).sum::<usize>()<=MAX_TOTAL,"CONJUNTO_SUPERA_COTA")?;
        exigir(a.revision()?==revision,"REGISTRO_CAMBIADO")?;
        let parser=MarkdownParser::new();
        let parsed=out.documentos.iter().map(|d|parser.parse(Path::new(&format!("{}.md",d.id)),&d.ingles).map_err(|_|"ANALISIS_NO_DISPONIBLE".to_string())).collect::<Resultado<Vec<_>>>()?;
        let rule=CrossFileContradiction::experimental_should(true);
        for i in 1..parsed.len() {
            m.llamadas_comparador+=1;
            for (diagnostic,positive) in rule.check_corpus_with_locations(&[&parsed[0],&parsed[i]]) {
                out.sugerencias.push(Sugerencia{propuesto:p.propuesto.clone(),referencia:out.documentos[i].id.clone(),positivo:pasaje(positive,&out.documentos)?,negativo:pasaje(diagnostic.span,&out.documentos)?});
            }
        }
        // Las consultas no escriben decisiones y un cambio concurrente invalida la entrega.
        for c in &contexts {admitido(a,s,&c.carga.id,ahora)?;}
        exigir(a.revision()?==revision,"REGISTRO_CAMBIADO")?;
        out.estado="consulta_experimental".into();Ok(out)
    })();
    m.microsegundos=inicio.elapsed().as_micros();resultado
}
