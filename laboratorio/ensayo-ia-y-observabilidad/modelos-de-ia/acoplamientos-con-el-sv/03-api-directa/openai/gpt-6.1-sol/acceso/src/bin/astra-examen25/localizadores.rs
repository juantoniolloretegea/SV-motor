//! Cotejo formal común: referencias exactas a los fragmentos efectivamente suministrados.
use crate::suministro_pdf::{parse,need,num,sha,R};
use serde_json::{json,Value};
fn norm(s:&str)->String {s.split_whitespace().collect::<Vec<_>>().join(" ")}
pub fn cotejar(answer:&Value,request:&Value)->R<Value>{
    need(answer.is_object(),"La respuesta debe ser objeto JSON")?;
    for key in ["respuesta","fundamentos_verificables","evidencias","insuficiencias"] {need(answer.get(key).is_some(),&format!("Falta {key}"))?;}
    let input=parse(request["input"][0]["content"].as_str().ok_or("Contenido ausente")?.as_bytes())?;
    let allowed=input["secciones"].as_array().ok_or("Secciones autorizadas ausentes")?;
    let fragments=input["fragmentos_documentales_completos"].as_array().ok_or("Fragmentos ausentes")?;
    let evidence=answer["evidencias"].as_array().filter(|a|!a.is_empty()).ok_or("Sin evidencias")?;
    let mut receipts=vec![];
    for q in evidence {
        let section=q["seccion"].as_str().ok_or("Sección ausente")?;
        need(allowed.contains(&json!(section))&&q["documento"]=="pdq-nci-hcl-es","Localizador ajeno al suministro")?;
        let refs=q["fragmentos"].as_array().filter(|r|!r.is_empty()).ok_or("Sin fragmentos citados")?;
        let mut previous=None;let mut end=None;let mut text=String::new();let mut offsets=vec![];
        for r in refs {
            let matches:Vec<_>=fragments.iter().filter(|f|f["seccion"]==section&&if r.is_u64(){f["pagina"]==*r}else{r.is_object()&&r["inicio_caracter"].is_u64()&&r["fin_caracter_exclusivo"].is_u64()&&r["inicio_caracter"]==f["inicio_caracter"]&&r["fin_caracter_exclusivo"]==f["fin_caracter_exclusivo"]}).collect();
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
        receipts.push(json!({"seccion":q["seccion"],"fragmentos":offsets,"cita_sha256":sha(quote.as_bytes()),"literal_con_espacios_normalizados":true}));
    }
    Ok(json!({"conforme":true,"version_comprobador":"0.2.0","citas":receipts,"alcance":"estructura y pertenencia literal; no prueba suficiencia ni fidelidad sustantiva","adjudicacion_cientifica":false}))
}
#[cfg(test)]mod tests {
 use super::*;
 fn fixture()->(Value,Value){let i=json!({"secciones":["_1"],"fragmentos_documentales_completos":[{"seccion":"_1","pagina":0,"inicio_caracter":0,"fin_caracter_exclusivo":3,"texto":"abc"},{"seccion":"_1","pagina":1,"inicio_caracter":3,"fin_caracter_exclusivo":6,"texto":"def"}]});(json!({"respuesta":"r","fundamentos_verificables":[],"insuficiencias":[],"evidencias":[{"documento":"pdq-nci-hcl-es","seccion":"_1","fragmentos":[0,1],"cita_literal_breve":"abcdef"}]}),json!({"input":[{"content":i.to_string()}]}))}
 #[test]fn cita_no_exige_literalidad_de_conclusion(){let(a,q)=fixture();assert!(cotejar(&a,&q).is_ok());}
 #[test]fn impide_citas_fuera_de_seccion(){let(mut a,q)=fixture();a["evidencias"][0]["seccion"]=json!("_13");assert!(cotejar(&a,&q).is_err());}
 #[test]fn rechaza_fabricacion_duplicacion_y_salto(){let(a,q)=fixture();for(k,v)in[("cita_literal_breve",json!("abc def")),("fragmentos",json!([0,0])),("fragmentos",json!([1,0])),("fragmentos",json!([2]))]{let mut b=a.clone();b["evidencias"][0][k]=v;assert!(cotejar(&b,&q).is_err());}}
}
