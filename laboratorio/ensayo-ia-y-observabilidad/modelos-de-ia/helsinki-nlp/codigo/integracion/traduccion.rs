//! Traducción auxiliar de lectura: sin representación recibida ni suministro a IA.
use crate::{Resultado,exigir,huella};
use serde::{Deserialize,Serialize};
use pulldown_cmark::{Parser,Options,Event,Tag,TagEnd};
use std::{ops::Range,path::PathBuf,process::Stdio};
use tokio::io::{AsyncReadExt,AsyncWriteExt};
pub const MODELO:&str="Helsinki-NLP/opus-mt-en-es@fdaddf76f50fcc1583ba42f95965862a7ab30f97";
pub const METODO:&str="lectura-estructurada-2";
static TURNO:tokio::sync::Semaphore=tokio::sync::Semaphore::const_new(1);
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeticionLectura {pub solicitud:String,pub carga:String,pub original_sha256:String,pub seleccion_sha256:String,pub desde:usize,pub hasta:usize,pub primer_pasaje:usize,pub ultimo_pasaje:usize,pub revision_git:String,pub motor_sha256:String,pub modelo:String,pub metodo:String,
    #[serde(default,skip_serializing_if="Option::is_none")] pub composicion:Option<ReferenciaComposicion>}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenciaComposicion {pub pagina:usize,pub plan_sha256:String,pub composicion_sha256:String,pub revision:u64}
#[derive(Clone,Debug,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultadoLectura {pub estado:String,pub salida_sha256:Option<String>,pub fragmentos:usize,pub conservados:usize,pub tokens_entrada:Option<usize>,pub tokens_salida:Option<usize>,pub duracion_ms:u64}
#[derive(Clone,Debug)]
pub struct Fragmento {pub rango:Range<usize>,pub texto:String}
#[derive(Debug)]
pub struct Preparacion {pub markdown:String,pub fragmentos:Vec<Fragmento>,pub conservados:usize}
// Primera versión conservadora: los pasajes con cantidades, símbolos, unidades,
// código o identificadores no se transforman. No se presentan como traducidos.
fn prosa(s:&str)->bool {
    s.trim().chars().count()>1 && s.chars().count()<=360 && s.chars().any(char::is_alphabetic) && s.chars().all(|c|c.is_alphabetic()||c.is_whitespace()||".,;:!?\"'’“”()–—-".contains(c))
        && !s.split_whitespace().any(|p|p.chars().filter(|c|c.is_alphabetic()).count()>1 && p.chars().filter(|c|c.is_alphabetic()).all(|c|c.is_uppercase()))
}
// Segmentación conservadora: conserva espacios y localizadores del original.
// Las abreviaturas ambiguas no se fuerzan a terminar una oración.
pub fn oraciones(s:&str)->Vec<Range<usize>> {
    let mut r=vec![];let mut inicio=0;
    for (i,c) in s.char_indices() {
        if !".!?".contains(c){continue}
        let mut fin=i+c.len_utf8();
        for c in s[fin..].chars(){if "\"'’”)]".contains(c){fin+=c.len_utf8()}else{break}}
        let resto=&s[fin..];
        if !resto.starts_with(char::is_whitespace)||!resto.trim_start().trim_start_matches(['\"','\'','“','‘','(']).starts_with(char::is_uppercase){continue}
        let palabra=s[..i].split_whitespace().last().unwrap_or("").to_ascii_lowercase();
        if c=='.'&&(palabra.len()<=1||palabra.contains('.')||["dr","mr","mrs","ms","prof","fig","eq","ref","et","al","vs","e.g","i.e"].contains(&palabra.as_str())){continue}
        r.push(inicio..fin);inicio=fin;
    }
    if inicio<s.len(){r.push(inicio..s.len())}
    r.into_iter().filter_map(|r|{let t=&s[r.clone()];let a=t.len()-t.trim_start().len();let b=t.trim_end().len();(a<b).then_some(r.start+a..r.start+b)}).collect()
}
fn negacion(s:&str,ingles:bool)->bool {
    let t=s.to_lowercase();
    t.split(|c:char|!c.is_alphabetic()&&c!='\''&&c!='’').any(|w|if ingles{
        ["no","not","never","neither","nor","without","cannot","none","nobody","nothing","nowhere"].contains(&w)||w.ends_with("n't")||w.ends_with("n’t")
    }else{["no","nunca","jamás","ni","sin","tampoco","ninguno","ningún","ninguna","nadie","nada"].contains(&w)})
}
pub fn preparar(md:&str)->Resultado<Preparacion>{
    exigir(!md.trim().is_empty() && md.len()<=16000,"TRADUCCION_SELECCION_EXCESIVA")?;
    // El contenedor completo es la unidad de protección. No se traduce por
    // separado una negación o condición interrumpida por marcado de Markdown.
    struct Unidad {texto:Option<Range<usize>>,protegida:bool}
    let mut pila:Vec<Unidad>=vec![];let mut fs=vec![];let mut protegidos=0;
    for(e,r)in Parser::new_ext(md,Options::ENABLE_TABLES|Options::ENABLE_STRIKETHROUGH|Options::ENABLE_MATH).into_offset_iter(){
        match e {
            Event::Start(Tag::Paragraph|Tag::Heading{..}|Tag::TableCell)=>pila.push(Unidad{texto:None,protegida:false}),
            Event::End(TagEnd::Paragraph|TagEnd::Heading(_)|TagEnd::TableCell)=>{
                if let Some(u)=pila.pop(){if let Some(r)=u.texto{let t=&md[r.clone()];
                    if u.protegida||t.chars().any(|c|c.is_numeric()||"<>=°%$`^*+±≥≤≠÷×/\\".contains(c)){protegidos+=1;continue}
                    for parte in oraciones(t){let texto=&t[parte.clone()];if prosa(texto){fs.push(Fragmento{rango:r.start+parte.start..r.start+parte.end,texto:texto.into()});}else{protegidos+=1;}}
                }else if u.protegida{protegidos+=1;}}
            },
            Event::Text(t) if !t.trim().is_empty()=>{
                if let Some(u)=pila.last_mut(){
                    if md.get(r.clone())!=Some(t.as_ref()){u.protegida=true}
                    if let Some(anterior)=u.texto.as_mut(){if r.start<anterior.end||!md[anterior.end..r.start].chars().all(char::is_whitespace){u.protegida=true}anterior.end=r.end;}else{u.texto=Some(r)}
                }else{protegidos+=1}
            },
            Event::SoftBreak=>{},
            Event::Start(_)=>{if let Some(u)=pila.last_mut(){u.protegida=true}},
            Event::Code(_)|Event::InlineMath(_)|Event::DisplayMath(_)|Event::Html(_)|Event::InlineHtml(_)|Event::HardBreak|Event::FootnoteReference(_)|Event::TaskListMarker(_)=>{
                if let Some(u)=pila.last_mut(){u.protegida=true}else{protegidos+=1}
            },
            _=>{}
        }
    }
    exigir(fs.len()<=24,"TRADUCCION_DEMASIADOS_FRAGMENTOS")?;
    Ok(Preparacion{markdown:md.into(),fragmentos:fs,conservados:protegidos})
}
#[derive(Deserialize)]
struct Salida {id:String,origen:String,destino:String,texto:String,entrada_sha256:String,salida_sha256:String,tokens_entrada:usize,tokens_salida:usize}
pub struct Traduccion {pub markdown:String,pub traducidos:usize,pub conservados:usize,pub tokens_entrada:usize,pub tokens_salida:usize}
fn literal_md(s:&str)->String {let mut out=String::new();for c in s.chars(){if "\\`*_{}[]()#+-.!|>~$<".contains(c){out.push('\\');}if c=='&'{out.push_str("&amp;")}else{out.push(c)}}out}
pub fn recomponer(p:&Preparacion,raw:&str)->Resultado<Traduccion>{
    let salidas=raw.lines().map(|l|serde_json::from_str::<Salida>(l).map_err(|_|"TRADUCCION_SALIDA_INVALIDA".to_string())).collect::<Resultado<Vec<_>>>()?;
    exigir(salidas.len()==p.fragmentos.len(),"TRADUCCION_INCOMPLETA")?;
    let mut out=String::new();let mut fin=0;let mut conservados=p.conservados;let mut traducidos=0;let mut te=0;let mut ts=0;
    for (i,(f,s)) in p.fragmentos.iter().zip(salidas).enumerate(){
        exigir(s.id==i.to_string() && s.origen=="en" && s.destino=="es" && s.entrada_sha256==huella(f.texto.as_bytes()) && s.salida_sha256==huella(s.texto.as_bytes()) && !s.texto.trim().is_empty() && s.texto.len()<=4000,"TRADUCCION_CORRESPONDENCIA_INVALIDA")?;
        te+=s.tokens_entrada;ts+=s.tokens_salida;out.push_str(&p.markdown[fin..f.rango.start]);
        // Las nuevas cantidades o símbolos inesperados tampoco se aceptan.
        if prosa(&s.texto) && !s.texto.contains(['\n','\r']) && negacion(&f.texto,true)==negacion(&s.texto,false) {out.push_str(&literal_md(&s.texto));traducidos+=1;}else{out.push_str(&p.markdown[f.rango.clone()]);conservados+=1;}
        fin=f.rango.end;
    }
    out.push_str(&p.markdown[fin..]);
    Ok(Traduccion{markdown:out,traducidos,conservados,tokens_entrada:te,tokens_salida:ts})
}
pub fn rutas()->Resultado<(PathBuf,PathBuf)> {
    let b=std::env::var_os("SV_TRADUCTOR_LOCAL").ok_or("TRADUCCION_NO_DISPONIBLE")?;let m=std::env::var_os("SV_MODELO_TRADUCCION").ok_or("TRADUCCION_NO_DISPONIBLE")?;
    let(b,m)=(PathBuf::from(b),PathBuf::from(m));exigir(b.is_absolute() && m.is_absolute() && b.is_file() && m.is_dir(),"TRADUCCION_NO_DISPONIBLE")?;Ok((b,m))
}
pub async fn ejecutar(p:&Preparacion)->Resultado<Traduccion>{
    let _turno=TURNO.try_acquire().map_err(|_|"TRADUCCION_OCUPADA")?;
    if p.fragmentos.is_empty(){return recomponer(p,"");}
    let (bin,modelo)=rutas()?;
    let mut hijo=tokio::process::Command::new(bin).arg(modelo).env_clear().stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).kill_on_drop(true).spawn().map_err(|_|"TRADUCCION_NO_INICIADA")?;
    let mut entrada=hijo.stdin.take().ok_or("TRADUCCION_NO_INICIADA")?;
    let mut salida=hijo.stdout.take().ok_or("TRADUCCION_NO_INICIADA")?;
    let mut datos=Vec::new();for (i,f)in p.fragmentos.iter().enumerate(){serde_json::to_writer(&mut datos,&serde_json::json!({"id":i.to_string(),"texto":f.texto})).map_err(|_|"TRADUCCION_ENTRADA_INVALIDA")?;datos.push(b'\n');}
    let ejecucion=async{
        let escritura=async{entrada.write_all(&datos).await.map_err(|_|"TRADUCCION_INTERRUMPIDA")?;entrada.shutdown().await.map_err(|_|"TRADUCCION_INTERRUMPIDA")?;drop(entrada);Ok::<_,String>(())};
        let lectura=async{let mut raw=vec![];(&mut salida).take(128*1024+1).read_to_end(&mut raw).await.map_err(|_|"TRADUCCION_INTERRUMPIDA")?;exigir(raw.len()<=128*1024,"TRADUCCION_SALIDA_EXCESIVA")?;Ok::<_,String>(raw)};
        let(_,raw)=tokio::try_join!(escritura,lectura)?;
        exigir(hijo.wait().await.map_err(|_|"TRADUCCION_INTERRUMPIDA")?.success(),"TRADUCCION_NO_COMPLETADA")?;
        recomponer(p,std::str::from_utf8(&raw).map_err(|_|"TRADUCCION_SALIDA_INVALIDA")?)
    };
    match tokio::time::timeout(std::time::Duration::from_secs(45),ejecucion).await{Ok(r)=>r,Err(_)=>{let _=hijo.kill().await;Err("TRADUCCION_TIEMPO_EXCEDIDO".into())}}
}
#[cfg(test)]mod tests{
use super::*;
#[test]fn formulas_cantidades_figuras_y_tablas_no_se_alteran(){
 let md="# Reading\n\nKeep x < 3 and y ≥ 9.\n\nThe temperature is 37.5 °C.\n\n| Name | Value |\n|---|---|\n| A | 17 |\n| B | 29 |\n\n![Diagram](images/figure.png)\n\n`unsafe()` and $x^2$";
 let p=preparar(md).unwrap();assert!(!p.fragmentos.iter().any(|f|f.texto.contains(['3','9','7','°'])));let raw=p.fragmentos.iter().enumerate().map(|(i,f)|serde_json::json!({"id":i.to_string(),"origen":"en","destino":"es","texto":"Texto","entrada_sha256":huella(f.texto.as_bytes()),"salida_sha256":huella(b"Texto"),"tokens_entrada":1,"tokens_salida":1}).to_string()).collect::<Vec<_>>().join("\n");let t=recomponer(&p,&raw).unwrap();for s in ["x < 3 and y ≥ 9","37.5 °C","| 17 |","| 29 |","![Diagram](images/figure.png)","`unsafe()`","$x^2$"]{assert!(t.markdown.contains(s),"{s}");}
 assert!(recomponer(&p,"").is_err());assert!(recomponer(&p,&raw.replace("\"en\"","\"fr\"")).is_err());
}
#[test]fn no_se_inventa_una_traduccion_de_codigo_o_identificadores(){let p=preparar("CVE-2026-1234\n\n37.5 °C\n\n$x<3$\n\n```rust\nlet n=9;\n```").unwrap();assert!(p.fragmentos.is_empty());assert_eq!(recomponer(&p,"").unwrap().markdown,p.markdown);}
#[test]fn oraciones_conservan_espacios_y_no_partieren_abreviaturas(){let s="  Dr. Smith reads.  The result is stable.\n";let r=oraciones(s);assert_eq!(r.iter().map(|r|&s[r.clone()]).collect::<Vec<_>>(),vec!["Dr. Smith reads.","The result is stable."]);let p=preparar(s).unwrap();assert_eq!(p.fragmentos.len(),2);for f in p.fragmentos{assert_eq!(&s[f.rango],f.texto)}}
#[test]fn cifra_en_parrafo_no_se_elude_segmentando(){let p=preparar("There are 9 cases. The treatment is available.").unwrap();assert!(p.fragmentos.is_empty());}
#[test]fn una_oracion_perdida_no_se_acepta(){let p=preparar("A result is available. Another result is pending.").unwrap();assert_eq!(p.fragmentos.len(),2);assert!(recomponer(&p,"").is_err());}
#[test]fn perdida_de_negacion_conserva_original(){let p=preparar("These cells do not become healthy cells.").unwrap();let s="Estas células se convierten en células sanas.";let raw=serde_json::json!({"id":"0","origen":"en","destino":"es","texto":s,"entrada_sha256":huella(p.fragmentos[0].texto.as_bytes()),"salida_sha256":huella(s.as_bytes()),"tokens_entrada":9,"tokens_salida":9}).to_string();let t=recomponer(&p,&raw).unwrap();assert_eq!(t.markdown,p.markdown);assert_eq!(t.traducidos,0);assert_eq!(t.conservados,1);}
#[test]fn marcado_no_separa_condicion_o_negacion(){for s in ["This does _not_ improve the result.","The [result](local.html) is not available.","This is ~~not~~ safe."]{let p=preparar(s).unwrap();assert!(p.fragmentos.is_empty(),"{s}");assert_eq!(recomponer(&p,"").unwrap().markdown,s)}}
#[test]fn salto_tipografico_no_separa_una_oracion(){let s="These cells do not\nbecome healthy cells.";let p=preparar(s).unwrap();assert_eq!(p.fragmentos.len(),1);assert_eq!(p.fragmentos[0].texto,s);}
#[test]fn comillas_no_ocultan_otra_oracion(){let s="A result is available. \"Another result is pending.\" The final result is unknown.";assert_eq!(oraciones(s).len(),3);assert_eq!(preparar(s).unwrap().fragmentos.len(),3);}
}
