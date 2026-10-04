use crate::{E,read,save,hash};
use serde_json::{Value,json};
use std::{fs,path::Path};
fn index(v:&Value)->Result<usize,E>{match v.as_str(){Some("0")=>Ok(0),Some("1")=>Ok(1),Some("U")=>Ok(2),_=>Err("Valor ajeno a la terna".into())}}
pub fn calculate(a:&Value,b:&Value)->Result<Value,E>{
 if a["bloque"]!=b["bloque"]||b["capa"].as_u64()!=a["capa"].as_u64().map(|n|n+1)||a["medidas"]["terna_completa"]!=true||b["medidas"]["terna_completa"]!=true{return Err("Se exigen capas completas, consecutivas y del mismo bloque".into())}
 let old=a["casos"].as_array().ok_or("Casos anteriores")?;let new=b["casos"].as_array().ok_or("Casos nuevos")?;if old.len()!=9||new.len()!=9{return Err("Vector incompleto".into())}
 let mut matrix=[[0u64;3];3];let mut rows=Vec::new();let mut cs=0;let mut rs=0;let mut cf=0;let mut rf=0;let mut resolved=0;let mut lost=0;
 for (x,y) in old.iter().zip(new){if x["caso"]!=y["caso"]||x["critico_prefijado"]!=y["critico_prefijado"]{return Err("Orden o criticidad alterados".into())}matrix[index(&x["valor"])?][index(&y["valor"])?]+=1;
  let correction=x["sustantivo"]=="error"&&y["sustantivo"]=="correcto";let regression=x["sustantivo"]=="correcto"&&y["sustantivo"]=="error";let fc=x["formal_conforme"]==false&&y["formal_conforme"]==true;let fr=x["formal_conforme"]==true&&y["formal_conforme"]==false;cs+=u64::from(correction);rs+=u64::from(regression);cf+=u64::from(fc);rf+=u64::from(fr);resolved+=u64::from(x["sustantivo"]=="indeterminado"&&y["sustantivo"]=="correcto");lost+=u64::from(x["sustantivo"]=="correcto"&&y["sustantivo"]=="indeterminado");
  rows.push(json!({"caso":x["caso"],"anterior":x["valor"],"posterior":y["valor"],"contenido_anterior":x["sustantivo"],"contenido_posterior":y["sustantivo"],"correccion_sustantiva":correction,"regresion_sustantiva":regression,"correccion_formal":fc,"regresion_formal":fr,"final_anterior_sha256":x["final_sha256"],"final_posterior_sha256":y["final_sha256"]}));
 }
 let category=|r:&Vec<Value>|json!({"correctas_observadas":r.iter().filter(|x|x["categoria_correcta"]==true).count(),"categorias_observables":r.iter().filter(|x|x["categoria_correcta"].is_boolean()).count(),"denominador_del_banco":9});
 let delta=b["medidas"]["puntuacion"].as_f64().ok_or("Puntuación posterior")?-a["medidas"]["puntuacion"].as_f64().ok_or("Puntuación anterior")?;
 Ok(json!({"bloque":a["bloque"],"capa_anterior":a["capa"],"capa_posterior":b["capa"],"orden_categorias":["0","1","U"],"matriz_de_transicion":matrix,"significado_matriz":"Recuentos entre categorías; no es una representación matricial de la célula canónica","correcciones_sustantivas_error_a_correcto":cs,"regresiones_sustantivas_correcto_a_error":rs,"correcciones_formales":cf,"regresiones_formales":rf,"indeterminaciones_resueltas_correctamente":resolved,"conclusiones_correctas_que_pasan_a_indeterminadas":lost,"delta_S_puntos_sobre_100":delta,"categorias_antes":category(old),"categorias_despues":category(new),"cumplimiento_completo_antes":a["medidas"]["N0"],"cumplimiento_completo_despues":b["medidas"]["N0"],"transiciones":rows,"interpretacion":"Comparación descriptiva. No prueba entrenamiento, convergencia o causalidad y no selecciona retrospectivamente otra capa.","licencia":crate::FOOTER}))
}
pub fn run(previous:&Path,current:&Path,out:&Path)->Result<(),E>{let a=read(previous)?;let b=read(current)?;let mut v=calculate(&a,&b)?;v["capa_anterior_sha256"]=json!(hash(&fs::read(previous)?));v["capa_posterior_sha256"]=json!(hash(&fs::read(current)?));save(out,&v)?;Ok(())}
#[cfg(test)]mod tests{
 use super::*;
 fn layer(n:u64)->Value{json!({"bloque":"A","capa":n,"medidas":{"terna_completa":true,"puntuacion":100.0,"N0":9},"casos":(1..=9).map(|i|json!({"caso":format!("A{i:02}"),"valor":"0","sustantivo":"correcto","formal_conforme":true,"critico_prefijado":i>=4})).collect::<Vec<_>>()})}
 #[test]fn indeterminacion_no_numerica(){let a=layer(0);let mut b=layer(1);b["casos"][0]["valor"]=json!("U");b["casos"][0]["sustantivo"]=json!("indeterminado");let r=calculate(&a,&b).unwrap();assert_eq!(r["matriz_de_transicion"][0][2],1);assert_eq!(r["regresiones_sustantivas_correcto_a_error"],0);assert_eq!(r["conclusiones_correctas_que_pasan_a_indeterminadas"],1);}
 #[test]fn orden_y_consecutividad(){let a=layer(0);assert!(calculate(&a,&layer(2)).is_err());let mut b=layer(1);b["casos"].as_array_mut().unwrap().swap(0,1);assert!(calculate(&a,&b).is_err());}
}
