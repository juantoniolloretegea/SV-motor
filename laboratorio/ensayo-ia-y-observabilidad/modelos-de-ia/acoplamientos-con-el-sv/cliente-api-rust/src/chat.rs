//! Adaptación del contrato común a Chat Completion; sin acceso de red propio.
#![forbid(unsafe_code)]
use crate::{need, parse, R, AVISO, LICENCIA};
use serde_json::{json, Value};

/// Sólo texto y antecedentes visibles. No traduce herramientas ni estado remoto.
pub fn solicitud(q: &Value, modelo: &str) -> R<Value> {
    need(modelo == "glm-5.3", "Modelo Z.ai no recibido")?;
    let obj = q.as_object().ok_or("Solicitud no es objeto")?;
    for key in obj.keys() {
        need(["model","instructions","input","reasoning","store","stream","tools","tool_choice","max_output_tokens","text"].contains(&key.as_str()), "Campo común sin adaptación recibida")?;
    }
    need(q["tools"] == json!([]) && q["store"] == false, "Herramientas o almacenamiento no autorizados")?;
    let instructions = q["instructions"].as_str().ok_or("Faltan instrucciones")?;
    need(instructions.contains(LICENCIA) && instructions.contains(AVISO), "Faltan derechos")?;
    let schema = q.pointer("/text/format/schema").filter(|v|v.is_object()).ok_or("Falta esquema de entrega")?;
    let limit = q["max_output_tokens"].as_u64().filter(|n| *n > 0 && *n <= 16384).ok_or("Cota de salida inválida")?;
    let effort=q.pointer("/reasoning/effort").and_then(Value::as_str).unwrap_or("max");
    need(matches!(effort,"low"|"high"|"max"),"Esfuerzo GLM no admitido")?;
    let mut messages = vec![json!({"role":"system","content":format!("{instructions}\nEntregue exclusivamente un objeto JSON válido conforme a este esquema: {schema}")})];
    let input = q["input"].as_array().ok_or("Entrada común no recibida")?;
    need(!input.is_empty() && input.len() <= 16, "Conversación fuera de cota")?;
    for (i,m) in input.iter().enumerate() {
        let fields=m.as_object().ok_or("Mensaje inválido")?;
        need(fields.len()==2 && fields.contains_key("role") && fields.contains_key("content"), "Mensaje con campos no admitidos")?;
        let role=m["role"].as_str().ok_or("Emisor ausente")?;
        need(matches!(role,"user"|"assistant") && (i!=0 || role=="user"), "Interlocutor no permitido")?;
        let text=m["content"].as_str().filter(|s|!s.is_empty()).ok_or("Sólo se admite texto íntegro")?;
        messages.push(json!({"role":role,"content":text}));
    }
    need(input.last().is_some_and(|m|m["role"]=="user"), "Falta instrucción final del operador")?;
    Ok(json!({"model":modelo,"messages":messages,"stream":true,"max_tokens":limit,
        "thinking":{"type":"enabled","clear_thinking":true},"reasoning_effort":effort,
        "response_format":{"type":"json_object"}}))
}

pub fn validar(q:&Value, modelo:&str)->R<()> {
    let obj=q.as_object().ok_or("Solicitud Chat no es objeto")?;
    need(obj.len()==7 && ["model","messages","stream","max_tokens","thinking","reasoning_effort","response_format"].iter().all(|k|obj.contains_key(*k)), "Campos Chat no autorizados")?;
    need(q["model"]==modelo && modelo=="glm-5.3" && q["stream"]==true && q["thinking"]==json!({"type":"enabled","clear_thinking":true}) && q["reasoning_effort"].as_str().is_some_and(|s|matches!(s,"low"|"high"|"max")) && q["response_format"]==json!({"type":"json_object"}), "Perfil Chat distinto")?;
    let messages=q["messages"].as_array().ok_or("Faltan mensajes")?;
    need(messages.len()>=2 && messages.len()<=17 && messages[1]["role"]=="user" && messages.last().is_some_and(|m|m["role"]=="user"),"Conversación inválida")?;
    for (i,m) in messages.iter().enumerate() {
        need(m.as_object().is_some_and(|o|o.len()==2),"Campos adicionales de mensaje")?;
        need(if i==0 {m["role"]=="system"} else {m["role"]=="user"||m["role"]=="assistant"},"Emisor no permitido")?;
        let text=m["content"].as_str().filter(|s|!s.is_empty()).ok_or("Contenido no textual")?;
        if i==0 {need(text.contains(LICENCIA)&&text.contains(AVISO),"Derechos ausentes")?;}
    }
    need(q["max_tokens"].as_u64().is_some_and(|n|n>0&&n<=16384),"Cota inválida")
}

/// Conserva usage sin atribuir un precio liquidado ni sumar detalles dos veces.
pub fn uso(u:&Value)->R<Value> {
    let a=u["prompt_tokens"].as_u64().ok_or("Entrada no comunicada")?;
    let b=u["completion_tokens"].as_u64().ok_or("Salida no comunicada")?;
    let total=u["total_tokens"].as_u64().ok_or("Total no comunicado")?;
    need(a.checked_add(b)==Some(total),"Tokens discordantes")?;
    let cached=u.pointer("/prompt_tokens_details/cached_tokens");
    let reasoning=u.pointer("/completion_tokens_details/reasoning_tokens");
    for(v,bound)in [(cached,a),(reasoning,b)]{if let Some(v)=v {need(v.as_u64().is_some_and(|n|n<=bound),"Detalle de uso inconsistente")?;}}
    Ok(json!({"input_tokens":a,"output_tokens":b,"total_tokens":total,
        "cached_tokens":cached,"reasoning_tokens":reasoning,"coste_liquidado_usd":null}))
}

#[derive(Default)]
pub struct Flujo {
    pending:Vec<u8>, pub eventos:Vec<Value>, id:Option<String>, modelo:Option<String>,
    texto:String, razonamiento:String, terminado:bool, done:bool, usage:Option<Value>, bytes:usize,
}
impl Flujo {
    pub fn feed(&mut self, bytes:&[u8])->R<Vec<String>> {
        self.bytes=self.bytes.checked_add(bytes.len()).ok_or("Desbordamiento")?;
        need(self.bytes<=16*1024*1024,"SSE excede cota")?;
        self.pending.extend_from_slice(bytes); let mut kinds=vec![];
        while let Some(pos)=self.pending.iter().position(|b|*b==b'\n') {
            let line=self.pending.drain(..=pos).collect::<Vec<_>>();
            let line=std::str::from_utf8(&line).map_err(|_|"SSE no UTF-8")?.trim_end_matches(['\r','\n']);
            if line.is_empty()||line.starts_with(':'){continue;}
            let data=line.strip_prefix("data:").ok_or("Campo SSE no recibido")?.trim();
            need(!self.done,"Datos después de DONE")?;
            if data=="[DONE]" {need(self.terminado,"DONE sin terminación")?; self.done=true;continue;}
            let v=parse(data.as_bytes())?;
            need(v.get("error").is_none()&&v.get("web_search").is_none(),"Error o búsqueda no autorizada")?;
            let id=v["id"].as_str().filter(|s|!s.is_empty()).ok_or("Falta identidad")?;
            let model=v["model"].as_str().filter(|s|!s.is_empty()).ok_or("Falta modelo")?;
            if let Some(old)=&self.id {need(old==id&&self.modelo.as_deref()==Some(model),"Identidad cambiante")?;}
            else {self.id=Some(id.into());self.modelo=Some(model.into());}
            if let Some(o)=v.get("object") {need(o=="chat.completion.chunk","Objeto SSE distinto")?;}
            let choices=v["choices"].as_array().ok_or("Faltan alternativas")?;
            need(choices.len()<=1,"Más de una alternativa")?;
            if let Some(c)=choices.first() {
                need(!self.terminado && c["index"]==0,"Alternativa o secuencia distinta")?;
                let d=c["delta"].as_object().ok_or("Delta inválido")?;
                need(d.keys().all(|k|["role","content","reasoning_content"].contains(&k.as_str())),"Herramienta o delta ajeno")?;
                if let Some(r)=d.get("role"){need(r=="assistant","Emisor ajeno")?;}
                for (field,target,kind) in [("content",&mut self.texto,"response.output_text.delta"),("reasoning_content",&mut self.razonamiento,"chat.reasoning_content.delta")] {
                    if let Some(t)=d.get(field).filter(|t|!t.is_null()) {
                        let s=t.as_str().ok_or("Delta no textual")?;target.push_str(s);
                        if !s.is_empty(){kinds.push(kind.into());}
                    }
                }
                if let Some(f)=c.get("finish_reason").filter(|v|!v.is_null()) {need(f=="stop","Terminación no completa")?;self.terminado=true;kinds.push("chat.completed".into());}
            } else {need(self.terminado && v.get("usage").is_some(),"Fragmento vacío inesperado")?;}
            let global=v.get("usage").filter(|v|!v.is_null());
            let alternative=choices.first().and_then(|c|c.get("usage")).filter(|v|!v.is_null());
            if let (Some(a),Some(b))=(global,alternative){need(a==b,"Dos representaciones de uso discordantes")?;}
            if let Some(u)=global.or(alternative) {
                need(self.terminado,"Uso prematuro")?;uso(u)?;
                if let Some(previous)=&self.usage{need(previous==u,"Uso repetido discordante")?;}else{self.usage=Some(u.clone());}
            }
            self.eventos.push(v);
        }
        Ok(kinds)
    }
    pub fn recibir(&self,modelo:&str)->R<Value> {
        need(self.done&&self.terminado&&self.pending.iter().all(u8::is_ascii_whitespace),"SSE truncado o sin cierre")?;
        need(self.modelo.as_deref()==Some(modelo)&&!self.texto.is_empty(),"Modelo distinto o respuesta vacía")?;
        let u=self.usage.as_ref().ok_or("Uso final no comunicado; no se presume cero")?;
        Ok(json!({"texto_original":self.texto,"razonamiento_comunicado":self.razonamiento,
            "uso_proveedor":u,"uso_normalizado":uso(u)?,"id_proveedor":self.id,"modelo_proveedor":self.modelo,
            "eventos":self.eventos.len(),"reconstruccion_desde_deltas":true,
            "concordancia_sse_texto":null,"alcance":"No existe copia final independiente en este protocolo; el texto se reconstruye de los deltas conservados"}))
    }
}

#[cfg(test)] mod tests {
    use super::*;
    fn request()->Value {json!({"instructions":format!("{LICENCIA}\n{AVISO}"),"input":[{"role":"user","content":"Fuente á"}],"tools":[],"store":false,"max_output_tokens":1024,"text":{"format":{"schema":{"type":"object"}}}})}
    fn chunk(delta:Value,finish:Value)->Value {json!({"id":"prueba","model":"glm-5.3","choices":[{"index":0,"delta":delta,"finish_reason":finish}]})}
    fn stream()->String {let mut end=chunk(json!({}),json!("stop"));end["usage"]=json!({"prompt_tokens":5,"completion_tokens":2,"total_tokens":7});format!("data: {}\n\ndata: {}\n\ndata: [DONE]\n\n",chunk(json!({"role":"assistant","content":"año"}),Value::Null),end)}
    #[test]fn adapta_sin_perder_contexto(){let q=solicitud(&request(),"glm-5.3").unwrap();validar(&q,"glm-5.3").unwrap();assert_eq!(q["messages"][1]["content"],"Fuente á");assert!(q.get("tools").is_none());}
    #[test]fn rechaza_funciones_y_estado(){for field in ["conversation","previous_response_id","functions"]{let mut q=request();q[field]=json!("x");assert!(solicitud(&q,"glm-5.3").is_err());}let mut q=request();q["input"][0]["role"]=json!("tool");assert!(solicitud(&q,"glm-5.3").is_err());}
    #[test]fn fragmentacion_unicode_y_uso(){let wire=stream();for size in [1,2,7,1024]{let mut f=Flujo::default();for b in wire.as_bytes().chunks(size){f.feed(b).unwrap();}let r=f.recibir("glm-5.3").unwrap();assert_eq!(r["texto_original"],"año");assert_eq!(r["uso_normalizado"]["total_tokens"],7);assert!(r["concordancia_sse_texto"].is_null());assert!(f.recibir("otro").is_err());}}
    #[test]fn cierre_obligatorio(){let mut f=Flujo::default();f.feed(stream().replace("data: [DONE]\n\n","").as_bytes()).unwrap();assert!(f.recibir("glm-5.3").is_err());let mut f=Flujo::default();assert!(f.feed(b"data: [DONE]\n").is_err());}
    #[test]fn no_herramientas_ni_terminacion_parcial(){for delta in [json!({"tool_calls":[]}),json!({"function_call":{}})]{assert!(Flujo::default().feed(format!("data: {}\n",chunk(delta,Value::Null)).as_bytes()).is_err());}assert!(Flujo::default().feed(format!("data: {}\n",chunk(json!({}),json!("length"))).as_bytes()).is_err());}
    #[test]fn identidad_y_datos_posteriores(){let wire=stream().replacen("\"prueba\"","\"otra\"",1);assert!(Flujo::default().feed(wire.as_bytes()).is_err());let mut f=Flujo::default();f.feed(stream().as_bytes()).unwrap();assert!(f.feed(b"data: {}\n").is_err());}
    #[test]fn uso_no_inventado(){assert!(uso(&json!({"prompt_tokens":5,"completion_tokens":2,"total_tokens":8})).is_err());assert!(uso(&json!({"prompt_tokens":5,"completion_tokens":2,"total_tokens":7,"prompt_tokens_details":{"cached_tokens":6}})).is_err());let u=uso(&json!({"prompt_tokens":5,"completion_tokens":2,"total_tokens":7})).unwrap();assert!(u["reasoning_tokens"].is_null()&&u["coste_liquidado_usd"].is_null());}
    #[test]fn uso_final_separado_y_ausente(){let mut f=Flujo::default();let a=chunk(json!({"content":"ok"}),json!("stop"));f.feed(format!("data: {a}\n").as_bytes()).unwrap();let u=json!({"id":"prueba","model":"glm-5.3","choices":[],"usage":{"prompt_tokens":5,"completion_tokens":2,"total_tokens":7}});f.feed(format!("data: {u}\ndata: [DONE]\n").as_bytes()).unwrap();f.recibir("glm-5.3").unwrap();let mut f=Flujo::default();f.feed(format!("data: {a}\ndata: [DONE]\n").as_bytes()).unwrap();assert!(f.recibir("glm-5.3").is_err());}
    #[test]fn uso_en_alternativa_con_concordancia(){let mut c=chunk(json!({"content":"ok"}),json!("stop"));let u=json!({"prompt_tokens":5,"completion_tokens":2,"total_tokens":7});c["choices"][0]["usage"]=u.clone();let mut f=Flujo::default();f.feed(format!("data: {c}\ndata: [DONE]\n").as_bytes()).unwrap();assert_eq!(f.recibir("glm-5.3").unwrap()["uso_proveedor"],u);c["usage"]=json!({"prompt_tokens":6,"completion_tokens":2,"total_tokens":8});assert!(Flujo::default().feed(format!("data: {c}\n").as_bytes()).is_err());}
    #[test]fn uso_duplicado_entre_tramas_solo_si_identico(){let mut c=chunk(json!({"content":"ok"}),json!("stop"));let u=json!({"prompt_tokens":5,"completion_tokens":2,"total_tokens":7});c["choices"][0]["usage"]=u.clone();let mut final_chunk=json!({"id":"prueba","model":"glm-5.3","choices":[],"usage":u});let mut f=Flujo::default();f.feed(format!("data: {c}\ndata: {final_chunk}\ndata: [DONE]\n").as_bytes()).unwrap();assert_eq!(f.recibir("glm-5.3").unwrap()["uso_normalizado"]["total_tokens"],7);final_chunk["usage"]["total_tokens"]=json!(8);assert!(Flujo::default().feed(format!("data: {c}\ndata: {final_chunk}\n").as_bytes()).is_err());}
    #[test]fn perfil_cero_no_inicia_envio(){let p=crate::Perfil{proveedor:"Z.ai".into(),modelo:"glm-5.3".into(),endpoint:"https://api.z.ai/api/paas/v4/chat/completions".into(),presupuesto_ticks:0,exigir_zdr:false,entrada_ticks_por_token:14000,salida_ticks_por_token:44000,cuota_gratuita_tokens:None};let mut q=request();crate::proteger(&mut q,&p).unwrap();let error=crate::enviar(&p,"CREDENCIAL_SINTETICA_NO_VALIDA",&q,std::path::Path::new("NO_DEBE_EXISTIR"),1).unwrap_err();assert!(error.contains("preparación"));}
    #[test]fn historia_visible_integra_con_instrucciones_de_procedencia(){let mut q=request();q["input"]=json!([{"role":"user","content":"fuente"},{"role":"user","content":"instrucción histórica"},{"role":"assistant","content":"respuesta anterior íntegra"},{"role":"user","content":"verificación neutral"}]);let r=solicitud(&q,"glm-5.3").unwrap();validar(&r,"glm-5.3").unwrap();assert_eq!(&r["messages"].as_array().unwrap()[1..],q["input"].as_array().unwrap());}
}

// © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
