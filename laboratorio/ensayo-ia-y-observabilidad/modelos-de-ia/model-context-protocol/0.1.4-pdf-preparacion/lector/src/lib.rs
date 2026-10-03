//! Extracción documental local. No contiene cliente de red ni ejecuta contenido PDF.
use std::{collections::BTreeMap, panic::{catch_unwind, AssertUnwindSafe}};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use pdf_extract::{OutputDev, OutputError, Transform, MediaBox, PlainTextOutput};

pub const MAX_PDF:usize=20*1024*1024;
pub const MAX_STREAM:usize=64*1024*1024;
pub const MAX_TEXT:usize=1024*1024;
pub const MAX_PAGES:usize=64;
pub fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}

#[derive(Debug,Serialize,Deserialize)]
pub struct Page {
    pub indice:usize, pub pagina_impresa_ordinal:usize, pub texto:String,
    pub sha256:String, pub caracteres:usize,
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Extraction {
    pub esquema:String, pub original_sha256:String, pub original_bytes:usize,
    pub paginas:Vec<Page>, pub sustituciones_tipograficas:BTreeMap<String,usize>,
    pub metodo:String, pub advertencias:Vec<String>,
}

struct Sink<'a> {
    plain:PlainTextOutput<&'a mut String>,
    substitutions:BTreeMap<String,usize>,
    written:usize,
}
impl OutputDev for Sink<'_> {
    fn begin_page(&mut self,n:u32,m:&MediaBox,a:Option<(f64,f64,f64,f64)>)->Result<(),OutputError>{self.plain.begin_page(n,m,a)}
    fn end_page(&mut self)->Result<(),OutputError>{self.plain.end_page()}
    fn begin_word(&mut self)->Result<(),OutputError>{self.plain.begin_word()}
    fn end_word(&mut self)->Result<(),OutputError>{self.plain.end_word()}
    fn end_line(&mut self)->Result<(),OutputError>{self.plain.end_line()}
    fn output_character(&mut self,t:&Transform,w:f64,s:f64,f:f64,text:&str)->Result<(),OutputError>{
        if text.is_empty() || text.chars().any(|c|c=='\0'||c=='\u{fffd}'||(c.is_control()&&!c.is_whitespace())) {
            return Err(std::io::Error::other("CARACTER_NO_DESCODIFICADO").into());
        }
        self.written+=text.len();
        if self.written>MAX_TEXT {return Err(std::io::Error::other("TEXTO_SUPERA_COTA").into());}
        self.plain.output_character(t,w,s,f,text)
    }
    fn output_character_with_font(&mut self,t:&Transform,w:f64,s:f64,f:f64,text:&str,font:&str)->Result<(),OutputError>{
        let base=font.rsplit('+').next().unwrap_or(font);
        let mapped=if base=="Wingdings2" {
            match text {
                "y"=>{*self.substitutions.entry("Wingdings2:y -> U+2022".into()).or_default()+=1;"•"},
                "{"=>{*self.substitutions.entry("Wingdings2:{ -> U+25CB".into()).or_default()+=1;"○"},
                "•"|"○"|" "=>text,
                _=>return Err(std::io::Error::other("SIMBOLO_SIN_EQUIVALENCIA_DECLARADA").into()),
            }
        } else {text};
        self.output_character(t,w,s,f,mapped)
    }
}

/// Sólo se analiza el contenido cuya huella coincide con la fijada por quien prepara el catálogo.
pub fn extract(bytes:&[u8],expected:&str)->Result<Extraction,String>{
    if bytes.len()>MAX_PDF{return Err("PDF_SUPERA_COTA".into());}
    if expected.len()!=64||hash(bytes)!=expected{return Err("HUELLA_PDF_DISCORDANTE".into());}
    if !bytes.starts_with(b"%PDF-"){return Err("CABECERA_NO_PDF".into());}
    catch_unwind(AssertUnwindSafe(|| extract_inner(bytes,expected)))
        .map_err(|_|"FALLO_DEL_EXTRACTOR: no se admite resultado parcial".to_string())?
}
fn extract_inner(bytes:&[u8],expected:&str)->Result<Extraction,String>{
    let options=lopdf::LoadOptions{max_decompressed_size:Some(MAX_STREAM),..Default::default()};
    let doc=lopdf::Document::load_mem_with_options(bytes,options).map_err(|e|format!("PDF_INVALIDO: {e}"))?;
    if doc.was_encrypted(){return Err("PDF_CIFRADO_NO_ADMITIDO".into());}
    let pages=doc.get_pages();
    if pages.is_empty()||pages.len()>MAX_PAGES{return Err("PAGINAS_FUERA_DE_COTA".into());}
    let mut result=Vec::new();let mut substitutions=BTreeMap::new();let mut total=0;
    // Enumeración completa de páginas; un error nunca se trata como fin del documento.
    for (index,(number,_)) in pages.iter().enumerate(){
        let mut text=String::new();
        let mut sink=Sink{plain:PlainTextOutput::new(&mut text),substitutions:BTreeMap::new(),written:0};
        pdf_extract::output_doc_page(&doc,&mut sink,*number).map_err(|e|format!("PAGINA_{index}_NO_EXTRAIDA: {e}"))?;
        let changes=std::mem::take(&mut sink.substitutions);drop(sink);
        for(k,v)in changes{*substitutions.entry(k).or_default()+=v;}
        if text.trim().is_empty(){return Err(format!("PAGINA_{index}_SIN_TEXTO: requiere revisión visual u OCR; no se declara vacía"));}
        total+=text.len();if total>MAX_TEXT{return Err("TEXTO_SUPERA_COTA".into());}
        result.push(Page{indice:index,pagina_impresa_ordinal:index+1,sha256:hash(text.as_bytes()),caracteres:text.chars().count(),texto:text});
    }
    Ok(Extraction{esquema:"extraccion-pdf-sv/1".into(),original_sha256:expected.into(),original_bytes:bytes.len(),paginas:result,sustituciones_tipograficas:substitutions,
        metodo:"Capa de texto PDF, pdf-extract 0.12.1-sv.1 y lopdf 0.45.0; sin OCR, sin modelo, sin red; orden de operadores del PDF con separación espacial de palabras y líneas".into(),
        advertencias:vec!["Extracción de texto: no acredita lectura del modelo, integridad de figuras, orden visual universal ni validez clínica".into(),"Numeración física ordinal desde 1 e índice desde 0; no interpreta etiquetas editoriales distintas".into(),"El último fragmento no demuestra que se hayan consultado los anteriores".into()]})
}
