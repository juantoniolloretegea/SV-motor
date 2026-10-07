//! Revisión fuera de línea de localizadores y métricas; nunca realiza inferencia.
#![forbid(unsafe_code)]
#[path="../catalogo/estricto.rs"] mod estricto;
#[path="../catalogo/recepcion.rs"] mod recepcion;
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs::{self,OpenOptions},io::Write,path::Path,collections::BTreeSet};
type R<T>=Result<T,String>;
const ROOT:&str="C:/laboratorio/watson-local/lenguaje-computacion-sv";
const RUN:&str="ejecucion/astra-pdf-transporte-20261007";
fn e(x:impl std::fmt::Display)->String{x.to_string()}
fn check(b:bool,s:&str)->R<()>{if b{Ok(())}else{Err(s.into())}}
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn parse(b:&[u8])->R<Value>{estricto::parse(b).map_err(e)}
fn load(p:&Path)->R<Value>{parse(&fs::read(p).map_err(e)?)}
fn norm(s:&str)->String{s.split_whitespace().collect::<Vec<_>>().join(" ")}
fn number(v:&Value)->R<u64>{v.as_u64().ok_or("Entero no negativo requerido".into())}
fn citations(answer:&Value,request:&Value)->R<Value>{
    for k in ["respuesta","fundamentos_verificables","evidencias","insuficiencias"]{check(answer.get(k).is_some(),"Campo requerido ausente")?;}
    let input=parse(request["input"][0]["content"].as_str().ok_or("Contenido ausente")?.as_bytes())?;
    let fragments=input["fragmentos_documentales_completos"].as_array().ok_or("Fragmentos ausentes")?;
    let evidence=answer["evidencias"].as_array().ok_or("Evidencias ausentes")?;check(!evidence.is_empty(),"Sin citas")?;let mut receipts=vec![];
    for q in evidence{
        let page=number(&q["pagina_fisica"])?;check([5,8].contains(&page)&&q["documento"]=="LLS-HCL-2018"&&q["seccion"]==format!("PDF-P{:04}",page-1),"Página o sección ajena")?;
        let refs=q["fragmentos"].as_array().ok_or("Localizadores ausentes")?;check(!refs.is_empty(),"Sin localizadores")?;
        let mut ids=BTreeSet::new();let mut text=String::new();let mut offsets=vec![];let mut previous=None;
        for r in refs{
            let matched:Vec<_>=fragments.iter().filter(|f|f["pagina_pdf_ordinal"]==page&&if r.is_u64(){f["pagina"]==*r}else{r.is_object()&&r["inicio_caracter"].is_u64()&&r["fin_caracter_exclusivo"].is_u64()&&r["inicio_caracter"]==f["inicio_caracter"]&&r["fin_caracter_exclusivo"]==f["fin_caracter_exclusivo"]}).collect();
            check(matched.len()==1,"Intervalo o índice no identifica exactamente un fragmento suministrado")?;let f=matched[0];let id=number(&f["pagina"])?;
            check(ids.insert(id)&&previous.is_none_or(|n|id>n),"Localizadores repetidos o desordenados")?;previous=Some(id);
            if !text.is_empty(){text.push('\n');}text.push_str(f["texto"].as_str().ok_or("Texto ausente")?);
            offsets.push(json!({"indice":id,"inicio_caracter":f["inicio_caracter"],"fin_caracter_exclusivo":f["fin_caracter_exclusivo"]}));
        }
        let quote=q["cita_literal_breve"].as_str().filter(|s|!s.trim().is_empty()).ok_or("Cita vacía")?;
        check(norm(&text).contains(&norm(quote)),"Cita no literal en los fragmentos identificados")?;
        receipts.push(json!({"pagina_fisica":page,"seccion":q["seccion"],"localizadores_cotejados":offsets,"cita_sha256":sha(quote.as_bytes()),"literalidad_con_espacios_normalizados":true}));
    }
    Ok(json!({"conforme":true,"citas":receipts,"semantica_del_cotejo":"intervalo exacto o índice del fragmento realmente suministrado; no aceptación por semejanza","adjudicacion_cientifica":false}))
}
fn run()->R<Value>{
    let base=Path::new(ROOT).join(RUN);let result=load(&base.join("RESULTADO.json"))?;let source=load(&base.join("COTEJO-TRANSPORTE-RUST.json"))?;
    check(source["conforme"]==true&&source["resultado_sha256"]==sha(&fs::read(base.join("RESULTADO.json")).map_err(e)?),"Cotejo previo no corresponde")?;
    let raw=fs::read(base.join("SALIDA-SSE.txt")).map_err(e)?;check(source["sse_sha256"]==sha(&raw),"SSE alterado")?;
    let events=raw.split(|b|*b==b'\n').filter_map(|l|l.strip_prefix(b"data:")).filter(|l|l.iter().any(|b|!b.is_ascii_whitespace())&&!l.starts_with(b" [DONE]")).map(parse).collect::<R<Vec<_>>>()?;
    let received=recepcion::extract(&events)?;check(received==load(&base.join("ENTREGA-PROVEEDOR.json"))?,"Entrega distinta")?;
    let request_bytes=fs::read(base.join("SOLICITUD.json")).map_err(e)?;let request= parse(&request_bytes)?;
    check(load(&base.join("PREVIA.json"))?["solicitud_sha256"]==sha(&request_bytes),"Solicitud alterada")?;
    let review=citations(&parse(received["texto_original"].as_str().ok_or("Texto")?.as_bytes())?,&request)?;
    let original=load(&base.join("AUDITORIA-FORMAL.json"))?;check(original["conforme"]==false&&original["defecto"]=="Entero no negativo requerido","Defecto original diferente: revisar")?;
    let t=sv_instrumentacion::verify(&base.join("instrumentacion/telemetria.jsonl"))?;check(t==result["telemetria"],"Telemetría distinta")?;
    let rows=fs::read(base.join("instrumentacion/telemetria.jsonl")).map_err(e)?.split(|b|*b==b'\n').filter(|l|!l.is_empty()).map(|l|parse(l).and_then(|v|parse(v["cuerpo"].as_str().ok_or("Cuerpo")?.as_bytes()))).collect::<R<Vec<_>>>()?;
    let samples:Vec<_>=rows.iter().filter(|r|r["tipo"]=="muestra").collect();check(!samples.is_empty(),"Sin muestras")?;
    let a=samples[0];let z=samples.last().unwrap();let p0=&a["datos"]["proceso"];let p1=&z["datos"]["proceso"];
    let mut states=BTreeSet::new();let mut udp=0;let mut max_tcp=0;
    for s in &samples{let c=s["datos"]["conexiones"].as_array().ok_or("Conexiones")?;let mut tcp=0;for x in c{if x["protocolo"]=="TCP"{tcp+=1;states.insert(x["estado"].as_str().ok_or("Estado")?.to_string());}else if x["protocolo"]=="UDP"{udp+=1;}}max_tcp=max_tcp.max(tcp);}
    let mut metric=json!({"duracion_ms":result["duracion_ms"],"primer_texto_ms":result["primer_texto_ms"],"duracion_observada_ms":number(&z["transcurrido_ms"])?-number(&a["transcurrido_ms"])? ,"eventos_sse":events.len(),"bytes_sse":raw.len(),"muestras":samples.len(),"intervalo_maximo_ms":t["intervalo_maximo_ms"],"fallos_medicion":t["fallos_medicion"],"rss_maximo_bytes":samples.iter().filter_map(|s|s["datos"]["proceso"]["rss_bytes"].as_u64()).max(),"tcp_maximo_simultaneo":max_tcp,"tcp_estados":states,"observaciones_udp":udp,"sse_sha256":sha(&raw),"solicitud_sha256":sha(&request_bytes),"telemetria_sha256":t["sha256"],"final_sha256":sha(&fs::read(base.join("FINAL.txt")).map_err(e)?)});
    for(k,out)in[("cpu_acumulada_ms","cpu_delta_ms"),("io_lectura_acumulada_bytes","io_lectura_delta_bytes"),("io_escritura_acumulada_bytes","io_escritura_delta_bytes")]{metric[out]=json!(number(&p1[k])?.checked_sub(number(&p0[k])?).ok_or("Contador regresivo")?);}
    let u=&received["uso_proveedor"];check(number(&u["input_tokens"])?+number(&u["output_tokens"])?==number(&u["total_tokens"])?,"Uso discordante")?;
    let mut in_attribution=0;let mut out_attribution=0;
    for group in ["items","request_fields"]{for v in u["attribution"][group].as_object().ok_or("Atribución ausente")?.values(){in_attribution+=number(&v["input_tokens"])?;out_attribution+=number(&v["output_tokens"])?;}}
    check(in_attribution==number(&u["input_tokens"])?&&out_attribution==number(&u["output_tokens"])?,"Atribución de tokens discordante")?;
    let v=json!({"conforme":true,"utc_ms":sv_instrumentacion::utc_ms(),"caso":"PDF06","naturaleza":"revisión instrumental fuera de línea, sin inferencia ni recalificación científica","incidencia":"El comprobador inicial exigió índices enteros de fragmento no prescritos en el banco; la entrega contiene intervalos exactos y verificables","original_conservado":true,"cotejo_localizadores":review,"metricas":metric,"uso":{"entrada_tokens":u["input_tokens"],"entrada_cache_tokens":u["input_tokens_details"]["cached_tokens"],"salida_tokens":u["output_tokens"],"razonamiento_tokens":u["output_tokens_details"]["reasoning_tokens"],"total_tokens":u["total_tokens"]},"atribucion_tokens_cotejada":true,"resumen_proveedor_recibido":received["resumen_proveedor"].as_array().is_some_and(|a|!a.is_empty()),"inferencias_nuevas":0,"archivo_original_sha256":source["resultado_sha256"],"verificador_sha256":sha(&fs::read(std::env::current_exe().map_err(e)?).map_err(e)?)});
    let mut f=OpenOptions::new().write(true).create_new(true).open(base.join("REVISION-LOCALIZADORES-RUST.json")).map_err(e)?;f.write_all(&serde_json::to_vec_pretty(&v).map_err(e)?).and_then(|_|f.sync_all()).map_err(e)?;Ok(v)
}
fn main(){match run(){Ok(v)=>println!("{v}"),Err(e)=>{eprintln!("{e}");std::process::exit(1)}}}
#[cfg(test)]mod tests{
    use super::*;
    fn fixture()->(Value,Value){let i=json!({"fragmentos_documentales_completos":[{"pagina_pdf_ordinal":5,"pagina":1,"inicio_caracter":2000,"fin_caracter_exclusivo":4000,"texto":"Texto íntegro verificable"}]});(json!({"respuesta":{},"fundamentos_verificables":[],"insuficiencias":[],"evidencias":[{"documento":"LLS-HCL-2018","pagina_fisica":5,"seccion":"PDF-P0004","fragmentos":[{"inicio_caracter":2000,"fin_caracter_exclusivo":4000}],"cita_literal_breve":"Texto íntegro verificable"}]}),json!({"input":[{"content":i.to_string()}]}))}
    #[test]fn admite_intervalo_exacto_e_indice(){let(mut a,q)=fixture();citations(&a,&q).unwrap();a["evidencias"][0]["fragmentos"]=json!([1]);citations(&a,&q).unwrap();}
    #[test]fn rechaza_intervalo_ajeno_parcial_o_vacio(){for v in [json!({"inicio_caracter":1999,"fin_caracter_exclusivo":4000}),json!({"inicio_caracter":2000,"fin_caracter_exclusivo":2005}),json!({})]{let(mut a,q)=fixture();a["evidencias"][0]["fragmentos"]=json!([v]);assert!(citations(&a,&q).is_err());}}
    #[test]fn rechaza_cita_y_pagina_ajenas(){let(mut a,q)=fixture();a["evidencias"][0]["cita_literal_breve"]=json!("inventado");assert!(citations(&a,&q).is_err());let(mut a,q)=fixture();a["evidencias"][0]["pagina_fisica"]=json!(8);assert!(citations(&a,&q).is_err());}
}
