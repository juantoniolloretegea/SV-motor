use std::{fs,path::Path,io::Write};use serde_json::{Value,json};use sv_arbitro_comprobaciones::huella;
fn main()->Result<(),Box<dyn std::error::Error>>{
 let a=std::env::args().collect::<Vec<_>>();let root=Path::new(&a[1]);let id=&a[2];let dir=root.join("entrega/casos").join(id);
 let raw=fs::read(dir.join("RESPUESTA-FINAL-ORIGINAL.txt"))?;let v:Value=serde_json::from_slice(&raw)?;let plan:Value=serde_json::from_slice(&fs::read(root.join("realizacion/contraste/config/plan.json"))?)?;
 let caso=plan["casos"].as_array().ok_or("Casos ausentes")?.iter().find(|v|v["id"]==*id).ok_or("Caso ausente")?;
 let doc=caso["documento"].as_str().ok_or("Documento ausente")?;let text=fs::read_to_string(root.join("fuentes/corpus").join(format!("{doc}.txt")))?;let mut items=Vec::new();
 for e in v["evidencias"].as_array().ok_or("Evidencias ausentes")?{
  let fragment=e["fragmento"].as_str().ok_or("Fragmento ausente")?;let segments=fragment.split("...").flat_map(|s|s.split('…')).map(str::trim).filter(|s|!s.is_empty()).collect::<Vec<_>>();
  let mut pos=0;let mut spans=Vec::new();let mut complete=!segments.is_empty();let mut previous_end=None;
  for seg in segments{if let Some(found)=text[pos..].find(seg){let start=pos+found;let end=start+seg.len();let cs=text[..start].chars().count();let ce=text[..end].chars().count();let gap=previous_end.map(|p|&text[p..start]);spans.push(json!({"segmento":seg,"inicio_caracter":cs,"fin_caracter_exclusivo":ce,"paginas":[cs/2000,(ce-1)/2000],"texto_omitido_anterior":gap}));pos=end;previous_end=Some(end);}else{complete=false;spans.push(json!({"segmento":seg,"encontrado":false}));break;}}
  let page=e["pagina"].as_u64();let localizador=page.map(|p|complete&&spans.iter().all(|s|s["paginas"][0]==p&&s["paginas"][1]==p));items.push(json!({"original":e,"coincidencia_continua":text.contains(fragment),"elipsis_explicita":fragment.contains("...")||fragment.contains('…'),"segmentos_literales_en_orden":complete,"segmentos":spans,"localizador_de_pagina_conforme":localizador,"documento_seccion_conformes":e["documento"]==doc&&e["seccion"]=="S1"}));
 }
 let result=json!({"id":id,"final_sha256":huella(&raw),"corpus_sha256":huella(text.as_bytes()),"citas":items,"limite":"Cotejo literal de segmentos y localización; la suficiencia y el efecto de las omisiones requieren adjudicación externa. No modifica la salida."});let mut f=fs::OpenOptions::new().create_new(true).write(true).open(dir.join("COTEJO-CITAS.json"))?;f.write_all(&serde_json::to_vec_pretty(&result)?)?;f.sync_all()?;println!("{}",serde_json::to_string_pretty(&result)?);Ok(())
}
