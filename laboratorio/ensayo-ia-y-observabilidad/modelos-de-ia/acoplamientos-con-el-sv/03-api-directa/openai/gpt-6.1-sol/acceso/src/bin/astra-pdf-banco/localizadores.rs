//! Cotejo formal común: referencias exactas a los fragmentos efectivamente suministrados.
use crate::suministro_pdf::{parse,need,num,sha,R,DOC};
use serde_json::{json,Value};
fn norm(s:&str)->String {s.split_whitespace().collect::<Vec<_>>().join(" ")}
pub fn cotejar(answer:&Value,request:&Value)->R<Value>{
    need(answer.is_object(),"La respuesta debe ser objeto JSON")?;
    for key in ["respuesta","fundamentos_verificables","evidencias","insuficiencias"] {need(answer.get(key).is_some(),&format!("Falta {key}"))?;}
    let input=parse(request["input"][0]["content"].as_str().ok_or("Contenido ausente")?.as_bytes())?;
    let allowed=input["paginas_fisicas"].as_array().ok_or("Páginas autorizadas ausentes")?;
    let fragments=input["fragmentos_documentales_completos"].as_array().ok_or("Fragmentos ausentes")?;
    let evidence=answer["evidencias"].as_array().filter(|a|!a.is_empty()).ok_or("Sin evidencias")?;
    let mut receipts=vec![];
    for q in evidence {
        let page=num(&q["pagina_fisica"])?;
        need(page>0&&allowed.contains(&json!(page))&&q["documento"]==DOC&&q["seccion"]==format!("PDF-P{:04}",page-1),"Localizador ajeno al suministro")?;
        let refs=q["fragmentos"].as_array().filter(|r|!r.is_empty()).ok_or("Sin fragmentos citados")?;
        let mut previous=None;let mut end=None;let mut text=String::new();let mut offsets=vec![];
        for r in refs {
            let matches:Vec<_>=fragments.iter().filter(|f|f["pagina_pdf_ordinal"]==page&&if r.is_u64(){f["pagina"]==*r}else{r.is_object()&&r["inicio_caracter"].is_u64()&&r["fin_caracter_exclusivo"].is_u64()&&r["inicio_caracter"]==f["inicio_caracter"]&&r["fin_caracter_exclusivo"]==f["fin_caracter_exclusivo"]}).collect();
            need(matches.len()==1,"Localizador no corresponde exactamente a un fragmento suministrado")?;
            let f=matches[0];let id=num(&f["pagina"])?;
            need(previous.is_none_or(|p|id==p+1),"Fragmentos repetidos, discontinuos o desordenados")?;
            if let Some(prev_end)=end {need(f["inicio_caracter"]==prev_end,"Intervalos discontinuos")?;}
            previous=Some(id);end=Some(f["fin_caracter_exclusivo"].clone());
            text.push_str(f["texto"].as_str().ok_or("Texto ausente")?);
            offsets.push(json!({"indice":id,"inicio_caracter":f["inicio_caracter"],"fin_caracter_exclusivo":f["fin_caracter_exclusivo"]}));
        }
        let quote=q["cita_literal_breve"].as_str().filter(|s|!s.trim().is_empty()).ok_or("Cita vacía")?;
        need(norm(&text).contains(&norm(quote)),"Cita no literal en fragmentos identificados")?;
        receipts.push(json!({"pagina_fisica":page,"seccion":q["seccion"],"fragmentos":offsets,"cita_sha256":sha(quote.as_bytes()),"literal_con_espacios_normalizados":true}));
    }
    Ok(json!({"conforme":true,"version_comprobador":"0.2.0","citas":receipts,"alcance":"estructura y pertenencia literal; no prueba suficiencia ni fidelidad sustantiva","adjudicacion_cientifica":false}))
}
#[cfg(test)]mod tests{
    use super::*;
    fn fixture()->(Value,Value){let i=json!({"paginas_fisicas":[1],"fragmentos_documentales_completos":[{"pagina_pdf_ordinal":1,"pagina":0,"inicio_caracter":0,"fin_caracter_exclusivo":5,"texto":"Texto"},{"pagina_pdf_ordinal":1,"pagina":1,"inicio_caracter":5,"fin_caracter_exclusivo":14,"texto":" íntegro."},{"pagina_pdf_ordinal":1,"pagina":2,"inicio_caracter":14,"fin_caracter_exclusivo":20,"texto":" Final"}]});(json!({"respuesta":"Texto","fundamentos_verificables":[],"insuficiencias":[],"evidencias":[{"documento":DOC,"pagina_fisica":1,"seccion":"PDF-P0000","fragmentos":[0,1],"cita_literal_breve":"Texto íntegro."}]}),json!({"input":[{"content":i.to_string()}]}))}
    #[test]fn indices_e_intervalos_exactos(){let(mut a,q)=fixture();cotejar(&a,&q).unwrap();a["evidencias"][0]["fragmentos"]=json!([{"inicio_caracter":0,"fin_caracter_exclusivo":5},{"inicio_caracter":5,"fin_caracter_exclusivo":14}]);cotejar(&a,&q).unwrap();}
    #[test]fn huecos_duplicados_y_desorden(){for ids in [json!([0,2]),json!([0,0]),json!([1,0])]{let(mut a,q)=fixture();a["evidencias"][0]["fragmentos"]=ids;assert!(cotejar(&a,&q).is_err());}}
    #[test]fn pagina_cero_ajena_y_cita_falsa(){for(k,v)in[("pagina_fisica",json!(0)),("pagina_fisica",json!(5)),("cita_literal_breve",json!("Texto falso")),("fragmentos",json!([{"inicio_caracter":0,"fin_caracter_exclusivo":4}]))]{let(mut a,q)=fixture();a["evidencias"][0][k]=v;assert!(cotejar(&a,&q).is_err());}}
    #[test]fn entrega_pdf06_original_aceptada(){let p=std::path::Path::new(crate::catalogo::ROOT).join("ejecucion/astra-pdf-transporte-20261007");let a=parse(&std::fs::read(p.join("FINAL.txt")).unwrap()).unwrap();let q=parse(&std::fs::read(p.join("SOLICITUD.json")).unwrap()).unwrap();let c=cotejar(&a,&q).unwrap();assert_eq!(c["citas"].as_array().unwrap().len(),3);}
}
