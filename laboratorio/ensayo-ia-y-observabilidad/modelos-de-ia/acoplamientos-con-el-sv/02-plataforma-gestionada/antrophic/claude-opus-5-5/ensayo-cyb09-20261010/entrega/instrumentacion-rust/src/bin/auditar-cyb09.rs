#![forbid(unsafe_code)]
use sv_claude_kaggle::{self as sv, need, parse, sha, R};
use serde_json::{json, Value};
use std::{fs,path::Path,io::{Read,Write},time::Instant};
const BASE:&str="https://mp-staging.kaggle.net/models/openapi";
#[path="cyb09/recepcion.rs"] mod recepcion;
use recepcion::*;
#[path="cyb09/continuacion.rs"] mod continuacion;
use continuacion::*;

fn audit(d:&Path,q:&Value,w:&Value)->R<Value>{
    need(leer(&d.join("SOLICITUD-DOCUMENTAL.json"))?==*q&&leer(&d.join("SOLICITUD.json"))?==*w,"Composición o historial distintos")?;
    let raw=fs::read(d.join("RESPUESTA-HTTP.json")).map_err(|e|e.to_string())?;
    let b=parse(&raw)?;admitir(&b)?;
    need(leer(&d.join("HTTP.json"))?["status"]==200,"HTTP no conforme")?;
    let content=b["choices"][0]["message"]["content"].as_str().ok_or("Contenido ausente")?;
    let(final_text,reasoning,segmentation)=segmentar(content)?;
    for(n,s)in[("CONTENIDO-ORIGINAL.txt",content),("FINAL.txt",final_text.as_str()),("RAZONAMIENTO-THINK.txt",reasoning.as_str())]{need(fs::read(d.join(n)).map_err(|e|e.to_string())?==s.as_bytes(),"Texto alterado")?;}
    need(leer(&d.join("SEGMENTACION.json"))?==segmentation,"Segmentación discordante")?;
    need(leer(&d.join("RAZONAMIENTO-CANALES.json"))?==json!({"reasoning_content":b["choices"][0]["message"]["reasoning_content"],"reasoning":b["choices"][0]["message"]["reasoning"]}),"Canal perdido")?;
    let row=leer(&d.join("RESULTADO.json"))?;
    let tel=sv::instrumentacion::verify(&d.join("instrumentacion/telemetria.jsonl"))?;
    need(row["completa"]==true&&row["error"].is_null()&&row["uso_proveedor"]==b["usage"]&&row["coste_nanodolares"]==coste(&b)?,"Resultado discordante")?;
    need(row["telemetria"]==tel&&leer(&d.join("instrumentacion/COTEJO-TELEMETRIA.json"))?==tel&&tel["fallos_medicion"]==0&&tel["intervalo_maximo_ms"].as_u64().is_some_and(|v|v<=750),"Observación incompleta")?;
    let events=sv::instrumentacion::records(&d.join("instrumentacion/telemetria.jsonl"))?;
    let wire_sha=sha(&serde_json::to_vec(w).map_err(|e|e.to_string())?);
    need(events.iter().any(|v|v["tipo"]=="envio"&&v["datos"]["solicitud_sha256"]==wire_sha)&&events.iter().any(|v|v["tipo"]=="recepcion"&&v["datos"]["respuesta_http_sha256"]==sha(&raw)&&v["datos"]["uso"]==b["usage"]),"Correlación ausente")?;
    let samples:Vec<_>=events.iter().filter(|v|v["tipo"]=="muestra").collect();
    let pid=samples.first().ok_or("Muestras ausentes")?["datos"]["proceso"]["pid"].clone();
    need(samples.iter().all(|v|v["datos"]["proceso"]["pid"]==pid&&v["datos"]["proceso"]["hilos_so"].as_u64().is_some_and(|n|n>0)&&v["datos"]["proceso"]["descriptores_abiertos_linux"].as_u64().is_some_and(|n|n>0)&&v["datos"]["proceso"]["espacio_red_linux"].as_str().is_some_and(|s|s.starts_with("net:["))),"Proceso o medidas adicionales discordantes")?;
    let formal=parse(final_text.as_bytes()).and_then(|a|sv::formal(&a,q)).unwrap_or_else(|e|json!({"conforme":false,"defecto":e}));
    need(formal==row["auditoria_formal"],"Auditoría formal discordante")?;
    let a=parse(final_text.as_bytes()).ok();
    need(row["informe_operativo_declarado"]==a.and_then(|a|a.get("informe_operativo").cloned()).unwrap_or(Value::Null),"Declaración operativa discordante")?;
    let max=|name:&str|samples.iter().filter_map(|v|v["datos"]["proceso"][name].as_u64()).max();
    let duration=|name:&str|{let v:Vec<_>=samples.iter().filter_map(|v|v["datos"]["proceso"][name].as_u64()).collect();v.first().zip(v.last()).and_then(|(a,b)|b.checked_sub(*a))};
    let net:std::collections::BTreeSet<_>=samples.iter().filter_map(|v|v["datos"]["conexiones"].as_array()).flatten().map(|v|v.to_string()).collect();
    Ok(json!({"resultado":row,"respuesta_http_sha256":sha(&raw),"final_sha256":sha(final_text.as_bytes()),"pid":pid,"razonamiento_emitido_caracteres":reasoning.chars().count(),"formal":formal,"observacion_exterior_resumen":{"alcance":"Proceso Rust del cuaderno; no son recursos internos de Claude","rss_maximo_bytes":max("rss_bytes"),"virtual_maximo_bytes":max("virtual_bytes"),"hilos_maximo":max("hilos_so"),"descriptores_maximo":max("descriptores_abiertos_linux"),"cpu_acumulada_diferencia_ms":duration("cpu_acumulada_ms"),"io_lectura_diferencia_bytes":duration("io_lectura_acumulada_bytes"),"io_escritura_diferencia_bytes":duration("io_escritura_acumulada_bytes"),"conexiones_diferentes":net,"limite":"Máximos observados y diferencias entre primera y última muestra; no equivalen al uso total entre tareas ni al tráfico TCP"}}))
}
fn run()->R<()>{
    let a:Vec<_>=std::env::args().collect();need(a.len()==5,"Uso: auditar-cyb09 PAQUETE ORIGINALES CAMPAÑA INFORME")?;
    let bytes=fs::read(&a[1]).map_err(|e|e.to_string())?;
    need(sha(&bytes)=="ec600dc873e06a187d65e97cf50fd1d3c3d80140edf292fe44009f2f894d76b9","Paquete distinto")?;
    let p=parse(&bytes)?;let original=Path::new(&a[2]);let root=Path::new(&a[3]);comprobar_paquete(&p,original)?;
    let start=leer(&root.join("INICIO.json"))?;let close=leer(&root.join("RESULTADO-CAMPANA.json"))?;
    need(start["binario_sha256"]=="c064a66106c0cb2c7905bd5ba894ad65938766a636b0c8d81e222ce5bce99c99"&&start["paquete_sha256"]==sha(&bytes)&&start["modelo"]==sv::MODELO,"Ejecutable o contrato distintos")?;
    need(close["estado"]=="completo"&&close["costes_no_recibidos"]==0,"Campaña incompleta; sin dictamen global")?;
    let delivered=close["entregas"].as_array().ok_or("Entregas ausentes")?;need(delivered.len()==27,"Cobertura incompleta")?;
    let(mut new_cost,mut old_cost,mut input,mut output,mut reason,mut samples,mut gap,mut count,mut new_count)=(0u64,0u64,0u64,0u64,0u64,0u64,0u64,0usize,0usize);
    let mut rows=Vec::new();let mut new_pid=None;let(mut new_input,mut new_output,mut new_reason)=(0u64,0u64,0u64);
    for case in p["casos"].as_array().ok_or("Casos")?{
        let id=case["id"].as_str().ok_or("Identidad")?;let mut history=Vec::new();
        for stage in 0..3{
            let(q,w)=sv::compose(&case["base"],stage,&history)?;
            let old=CONSERVADAS.contains(&(id,stage));
            let d=if old{original.join(format!("campana/cyb16/{id}-R{stage}"))}else{root.join(format!("cyb09/{id}-R{stage}"))};
            let detail=audit(&d,&q,&w)?;let r=&detail["resultado"];let h=&delivered[count];
            need(h["caso"]==id&&h["etapa"]==stage&&h["nueva_inferencia"]==!old&&h["resultado"]==*r&&leer(&root.join(format!("HITO-{:03}.json",count+1)))?==*h,"Orden o hito discordantes")?;
            let c=r["coste_nanodolares"].as_u64().ok_or("Coste ausente")?;
            if old{old_cost+=c;}else{
                let reserve=leer(&d.join("RESERVA.json"))?;
                need(reserve["coste_anterior_nanodolares"]==new_cost&&reserve["limite_nanodolares"]==9_990_000_000u64&&reserve["reserva_nanodolares"]==reserva(&w)?&&c<=reserva(&w)?,"Reserva discordante")?;
                new_cost+=c;new_count+=1;
                if let Some(pid)=&new_pid{need(*pid==detail["pid"],"Proceso de continuación diferente")?;}else{new_pid=Some(detail["pid"].clone());}
            }
            let u=&r["uso_proveedor"];input+=u["prompt_tokens"].as_u64().ok_or("Entrada")?;let o=u["completion_tokens"].as_u64().ok_or("Salida")?;output+=o;
            let rr=u["completion_tokens_details"]["reasoning_tokens"].as_u64().ok_or("Razón no recibida")?;need(rr<=o,"Razón fuera de salida")?;reason+=rr;
            if !old{new_input+=u["prompt_tokens"].as_u64().unwrap();new_output+=o;new_reason+=rr;}
            samples+=r["telemetria"]["muestras"].as_u64().ok_or("Muestras")?;gap=gap.max(r["telemetria"]["intervalo_maximo_ms"].as_u64().ok_or("Intervalo")?);
            rows.push(json!({"caso":id,"etapa":stage,"nueva_inferencia":!old,"recepcion":detail}));
            history.push(fs::read_to_string(d.join("FINAL.txt")).map_err(|e|e.to_string())?);count+=1;
        }
    }
    need(new_count==23&&old_cost==1_272_544_000u64&&new_cost==close["coste_nuevo_conocido_nanodolares"].as_u64().ok_or("Agregado")?&&new_cost<=9_990_000_000,"Agregados discordantes")?;
    save(Path::new(&a[4]),&json!({"conforme":true,"modelo":sv::MODELO,"entregas":rows,"numero_entregas":count,"generaciones_nuevas":new_count,"originales_reutilizados":4,"coste_nuevo_nanodolares":new_cost,"coste_historico_cyb16_nanodolares":old_cost,"tokens_entrada":input,"tokens_salida":output,"tokens_total":input+output,"tokens_razonamiento_incluidos_salida":reason,"tokens_nuevos_entrada":new_input,"tokens_nuevos_salida":new_output,"tokens_nuevos_total":new_input+new_output,"tokens_nuevos_razonamiento_incluidos_salida":new_reason,"muestras":samples,"intervalo_maximo_ms":gap,"primer_token_ms":null,"recursos_internos_proveedor":null,"clave_y_criticidad_fuera_del_candidato":true,"dictamen_semantico":"Exterior asistido, separado de este cotejo","recepcion_independiente":"pendiente","licencia":sv::LICENCIA}))?;
    println!("Recepción Rust conforme: 27 entregas, 23 nuevas, coste nuevo {new_cost} nanodólares");Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("NO CONFORME: {e}");std::process::exit(1)}}
// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
