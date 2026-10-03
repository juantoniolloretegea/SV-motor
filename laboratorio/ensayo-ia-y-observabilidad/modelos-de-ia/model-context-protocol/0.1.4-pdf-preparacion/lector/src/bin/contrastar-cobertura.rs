use std::collections::BTreeMap;
use serde_json::{json, Value};
use unicode_normalization::UnicodeNormalization;
fn counts(s:&str)->BTreeMap<char,i64>{
    let mut map=BTreeMap::new();
    for c in s.nfkc().filter(|c|!c.is_whitespace()) {*map.entry(c).or_default()+=1;}
    map
}
fn main()->Result<(),Box<dyn std::error::Error>>{
    let a:Vec<_>=std::env::args().collect();
    let extra:Value=serde_json::from_slice(&std::fs::read(&a[1])?)?;
    let reference:Value=serde_json::from_slice(&std::fs::read(&a[2])?)?;
    let mut rows=Vec::new();
    for (i,page) in reference["segmentos"].as_array().unwrap().iter().enumerate(){
        let reference=page["texto"].as_str().unwrap().replace('•',"y").replace('○',"{");
        for name in ["lopdf_045","pdf_extract_0121"]{
            let value=&extra[name][i];
            let text=value.as_str().or(value["Ok"].as_str()).unwrap_or("");
            let expected=counts(&reference);let actual=counts(text);let mut diff=actual.clone();
            for (c,n) in &expected {*diff.entry(*c).or_default()-=*n;}
            diff.retain(|_,n|*n!=0);
            rows.push(json!({"pagina_indice":i,"extractor":name,"caracteres_referencia":expected.values().sum::<i64>(),"caracteres_extraidos":actual.values().sum::<i64>(),"diferencias":diff}));
        }
    }
    let result=json!({"alcance":"Comparación de multiconjuntos de caracteres por página; NFKC y equivalencia declarada de dos viñetas; no prueba orden ni significado.","resultados":rows});
    std::fs::write(&a[3],serde_json::to_vec_pretty(&result)?)?;
    println!("{}",result);
    Ok(())
}
