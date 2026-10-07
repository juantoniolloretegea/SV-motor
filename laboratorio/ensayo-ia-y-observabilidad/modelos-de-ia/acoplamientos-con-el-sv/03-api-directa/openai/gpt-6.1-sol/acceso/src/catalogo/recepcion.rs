use serde_json::{json,Value};
use std::collections::{BTreeMap,BTreeSet};
type R<T> = Result<T,String>;
const MODEL:&str="gpt-6-astra";
fn correlated_text(events:&[Value],item:&Value,index:u64,part:u64,summary:bool,expected:&str)->R<()> {
    let (delta_kind,done_kind,part_key)=if summary {("response.reasoning_summary_text.delta","response.reasoning_summary_text.done","summary_index")} else {("response.output_text.delta","response.output_text.done","content_index")};
    let mut text=String::new();let mut done=None;let mut seen=false;
    for e in events.iter().filter(|e|e["type"]==delta_kind||e["type"]==done_kind) {
        if e["output_index"].as_u64()!=Some(index)||e[part_key].as_u64()!=Some(part){continue;}
        if e["item_id"]!=item["id"]{return Err("Identidad de fragmento discordante".into());}
        seen=true;
        if e["type"]==delta_kind {if done.is_some(){return Err("Fragmento posterior al cierre".into());}text.push_str(e["delta"].as_str().ok_or("Fragmento no textual")?);}
        else if done.replace(e["text"].as_str().ok_or("Cierre no textual")?).is_some(){return Err("Cierre textual repetido".into());}
    }
    if !seen||done!=Some(expected)||text!=expected{return Err("Fragmentos y texto concluido discordantes".into());}Ok(())
}

pub fn extract(events:&[Value])->R<Value> {
    for(i,e)in events.iter().enumerate(){if e["sequence_number"].as_u64()!=Some(i as u64){return Err("Secuencia incompleta".into());}}
    let first=events.first().ok_or("Sin eventos")?;let last=events.last().unwrap();
    if first["type"]!="response.created"||last["type"]!="response.completed"||last["response"]["status"]!="completed"||last["response"]["model"]!=MODEL||first["response"]["id"].as_str().is_none()||first["response"]["id"]!=last["response"]["id"] {return Err("Identidad o terminación no acreditadas".into());}
    let mut items=BTreeMap::new();let mut ids=BTreeSet::new();
    for e in events.iter().filter(|e|e["type"]=="response.output_item.done") {
        let ix=e["output_index"].as_u64().ok_or("Índice ausente")?;let item=&e["item"];let id=item["id"].as_str().ok_or("Identidad de contenido ausente")?;
        if !ids.insert(id)||items.insert(ix,item).is_some(){return Err("Contenido concluido repetido".into());}
    }
    if items.is_empty()||items.keys().copied().ne(0..items.len() as u64){return Err("Índices de contenido incompletos".into());}
    // No admitir fragmentos huérfanos ni llamadas ajenas al contrato de esta prueba.
    for e in events.iter().filter(|e|matches!(e["type"].as_str(),Some("response.output_text.delta"|"response.output_text.done"|"response.reasoning_summary_text.delta"|"response.reasoning_summary_text.done"))) {
        let ix=e["output_index"].as_u64().ok_or("Fragmento sin índice")?;let item=items.get(&ix).ok_or("Fragmento huérfano")?;
        let summary=e["type"].as_str().unwrap().starts_with("response.reasoning_summary");
        let key=if summary{"summary_index"}else{"content_index"};let list=if summary{"summary"}else{"content"};
        let part=e[key].as_u64().ok_or("Fragmento sin posición")? as usize;
        if e["item_id"]!=item["id"]||item[list].as_array().is_none_or(|a|part>=a.len()){return Err("Correlación de fragmento inválida".into());}
    }
    let mut summaries=Vec::new();let mut messages=Vec::new();let mut refusals=Vec::new();
    for(&ix,item)in &items {match item["type"].as_str(){
        Some("reasoning")=>{for(part,s)in item["summary"].as_array().ok_or("Resumen sin lista")?.iter().enumerate(){
            if s["type"]!="summary_text"{return Err("Tipo de resumen no admitido".into());}let t=s["text"].as_str().ok_or("Resumen no textual")?;
            correlated_text(events,item,ix,part as u64,true,t)?;summaries.push(t.to_owned());
        }},
        Some("message")=>{if item["role"]!="assistant"||item["status"]!="completed"{return Err("Mensaje no concluido".into());}
            for(part,p)in item["content"].as_array().ok_or("Mensaje sin contenido")?.iter().enumerate(){match p["type"].as_str(){
                Some("output_text")=>{let t=p["text"].as_str().ok_or("Mensaje no textual")?;correlated_text(events,item,ix,part as u64,false,t)?;messages.push(t.to_owned());},
                Some("refusal")=>refusals.push(p["refusal"].clone()),_=>return Err("Contenido de mensaje no previsto".into())
            }}
        },_=>return Err("Salida ajena al contrato sin herramientas".into())
    }}
    if let Some(a)=last["response"]["output"].as_array(){if !a.is_empty() && a.iter().ne(items.values().copied()){return Err("Contenido terminal discordante".into());}}
    let u=&last["response"]["usage"];
    if u["input_tokens"].as_u64().zip(u["output_tokens"].as_u64()).and_then(|(a,b)|a.checked_add(b))!=u["total_tokens"].as_u64()||u["total_tokens"].is_null(){return Err("Uso ausente o incoherente".into());}
    if messages.len()!=1||!refusals.is_empty(){return Err("No hay una única respuesta textual sin rechazo".into());}
    Ok(json!({"texto_original":messages[0],"resumen_proveedor":summaries,"elementos_salida_concluidos":items.values().collect::<Vec<_>>(),"uso_proveedor":u}))
}
