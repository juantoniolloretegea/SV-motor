//! Cliente técnico por proceso, sin motor ni pesos. Plazo de 30 s por respuesta.
use serde_json::{json,Value};
use std::{io::{self,BufRead,BufReader,Write},path::Path,process::{Child,ChildStdin,Command,Stdio},sync::mpsc,time::Duration};
use sv_mcp_documental::{Catalog,PROTOCOL};
struct Client {child:Child,input:Option<ChildStdin>,rx:mpsc::Receiver<Result<Value,String>>,id:u64}
impl Drop for Client {fn drop(&mut self){let _=self.child.kill();let _=self.child.wait();}}
impl Client {
    fn new(bin:&str,catalog:&str,hash:&str,log:&str)->Result<Self,Box<dyn std::error::Error>> {
        let mut child=Command::new(bin).args([catalog,hash,log,"256"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit()).spawn()?;
        let input=child.stdin.take().ok_or("Falta entrada")?;let output=child.stdout.take().ok_or("Falta salida")?;
        let (tx,rx)=mpsc::channel();
        std::thread::spawn(move||{for line in BufReader::new(output).lines(){
            let value=line.map_err(|e|e.to_string()).and_then(|s|serde_json::from_str(&s).map_err(|e|e.to_string()));
            if tx.send(value).is_err(){break;}
        }});
        Ok(Self {child,input:Some(input),rx,id:0})
    }
    fn send(&mut self,v:Value)->Result<(),Box<dyn std::error::Error>> {let input=self.input.as_mut().ok_or("Entrada cerrada")?;writeln!(input,"{v}")?;input.flush()?;Ok(())}
    fn request(&mut self,method:&str,params:Value)->Result<Value,Box<dyn std::error::Error>> {
        self.id+=1;let id=self.id;
        self.send(json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}))?;
        let v=self.rx.recv_timeout(Duration::from_secs(30)).map_err(|_|"Sin respuesta válida dentro de 30 segundos")?.map_err(io::Error::other)?;
        if v["id"]!=id || v["jsonrpc"]!="2.0" {return Err("Correlación JSON-RPC incorrecta".into());}
        if v.get("error").is_some(){return Err(format!("Error RPC: {}",v["error"]).into());}
        Ok(v["result"].clone())
    }
    fn tool(&mut self,name:&str,args:Value)->Result<Value,Box<dyn std::error::Error>> {
        self.request("tools/call",json!({"name":name,"arguments":args}))
    }
}
fn check(condition:bool,message:&str)->Result<(),Box<dyn std::error::Error>> {if condition {Ok(())}else{Err(message.to_string().into())}}
fn payload(v:&Value)->Result<Value,Box<dyn std::error::Error>> {Ok(serde_json::from_str(v["content"][0]["text"].as_str().ok_or("Falta texto MCP")?)?)}
fn run()->Result<(),Box<dyn std::error::Error>> {
    let a:Vec<String>=std::env::args().collect();
    if a.len()!=5 {return Err("Uso: comprobar-mcp BIN_SERVIDOR CATALOGO SHA256 DIARIO_NUEVO".into());}
    let catalog=Catalog::load(Path::new(&a[2]),&a[3],false).map_err(io::Error::other)?;
    let mut c=Client::new(&a[1],&a[2],&a[3],&a[4])?;
    let init=c.request("initialize",json!({"protocolVersion":PROTOCOL,"capabilities":{},"clientInfo":{"name":"sv-comprobacion","version":"0.1.0"}}))?;
    check(init["protocolVersion"]==PROTOCOL,"Versión MCP incorrecta")?;
    c.send(json!({"jsonrpc":"2.0","method":"notifications/initialized"}))?;
    let listing=c.request("tools/list",json!({}))?;
    let names:Vec<&str>=listing["tools"].as_array().ok_or("Listado inválido")?.iter().filter_map(|v|v["name"].as_str()).collect();
    check(names==vec!["buscar_documentos","leer_documento"],"Herramientas inesperadas")?;
    let found=c.tool("buscar_documentos",json!({"consulta":"leucemia","limite":5}))?;
    check(found["isError"]==false && payload(&found)?["coincidencias"].as_u64().unwrap_or(0)>0,"Búsqueda sin resultado")?;
    let mut count=0;
    for d in &catalog.documents {for s in &d.sections {
        let mut page=0;let mut recovered=String::new();
        loop {
            let r=c.tool("leer_documento",json!({"documento":d.id,"seccion":s.id,"pagina":page}))?;
            check(r["isError"]==false,"Error al recuperar sección")?;
            let p=payload(&r)?;recovered.push_str(p["texto"].as_str().ok_or("Página sin texto")?);
            count+=1;
            if p["siguiente_pagina"].is_null(){break;}
            page=p["siguiente_pagina"].as_u64().ok_or("Cursor inválido")?;
            check(count<110,"Número excesivo de páginas")?;
        }
        check(recovered==s.text,"La reconstrucción no coincide con la fuente conservada")?;
    }}
    for args in [json!({"documento":"../privado","seccion":"x"}),json!({"documento":"inexistente","seccion":"x"})] {
        let r=c.tool("leer_documento",args)?;
        check(r["isError"]==true && r["is_error"]==true,"El error no fue reconocido por ambos contratos")?;
    }
    let bad=c.tool("buscar_documentos",json!({"consulta":"texto","limite":99}))?;
    check(bad["isError"]==true,"No se aplica el límite")?;
    c.request("ping",json!({}))?;
    let calls=c.id;
    // Cierre por EOF y espera acotada: no deja un servicio en segundo plano.
    drop(c.input.take());
    let deadline=std::time::Instant::now()+Duration::from_secs(5);
    loop {
        if let Some(status)=c.child.try_wait()? {check(status.success(),"Servicio terminó con error")?;break;}
        if std::time::Instant::now()>=deadline {return Err("Servicio no cerró tras EOF".into());}
        std::thread::sleep(Duration::from_millis(10));
    }
    println!("{}",json!({"estado":"conforme","alcance":"cliente técnico propio por proceso; sin inferencia ni carga de pesos", "paginas_reconstruidas":count,"peticiones":calls,"catalogo_sha256":a[3],"cliente_mistral_rs_real":"pendiente"}));
    Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("Comprobación detenida: {e}");std::process::exit(1);}}
