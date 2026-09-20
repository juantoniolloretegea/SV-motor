use axum::{Router,routing::{get,post},extract::{State,DefaultBodyLimit},http::{HeaderMap,StatusCode,header},response::{Html,IntoResponse,Response},body::Bytes,Json};
use eio_candidato::nativa::*;
use serde_json::json;
use std::{sync::Arc,os::unix::net::UnixStream,time::Duration};
#[derive(Clone)]struct App{origen:String,socket:String,cupo:Arc<tokio::sync::Semaphore>}
fn salida(code:StatusCode,v:serde_json::Value)->Response{
 let mut r=(code,Json(v)).into_response();let h=r.headers_mut();
 h.insert(header::CACHE_CONTROL,"no-store".parse().unwrap());
 h.insert("x-content-type-options","nosniff".parse().unwrap());r
}
fn permitido(h:&HeaderMap,a:&App)->bool{
 h.get(header::ORIGIN).and_then(|x|x.to_str().ok())==Some(a.origen.as_str()) &&
 h.get(header::HOST).and_then(|x|x.to_str().ok())==a.origen.strip_prefix("https://") &&
 h.get(header::CONTENT_TYPE).and_then(|x|x.to_str().ok())==Some("application/json")
}
async fn api(State(a):State<App>,h:HeaderMap,body:Bytes)->Response{
 if !permitido(&h,&a){return salida(StatusCode::FORBIDDEN,json!({"error":"ORIGEN_O_TIPO"}))}
 let orden:Orden=match serde_json::from_slice(&body){Ok(x)=>x,Err(_)=>return salida(StatusCode::BAD_REQUEST,json!({"error":"ESTRUCTURA"}))};
 if let Orden::Iniciar{contrato,peticion,texto}=&orden{if let Err(e)=validar_inicio(contrato,peticion,texto){return salida(StatusCode::BAD_REQUEST,json!({"error":e}))}}
 let Ok(permiso)=a.cupo.clone().try_acquire_owned() else{return salida(StatusCode::TOO_MANY_REQUESTS,json!({"error":"OCUPADO"}))};
 // Bloqueo acotado de IPC, nunca inferencia dentro de Tokio.
 let r=tokio::task::spawn_blocking(move||->Result<serde_json::Value,String>{
  let _permiso=permiso;
  let mut s=UnixStream::connect(&a.socket).map_err(|e|e.to_string())?;
  s.set_read_timeout(Some(Duration::from_secs(2))).map_err(|e|e.to_string())?;
  s.set_write_timeout(Some(Duration::from_millis(100))).map_err(|e|e.to_string())?;
  send(&mut s,&orden).map_err(|e|e.to_string())?;
  recv(&mut s,2*MAX_FRAME).map_err(|e|e.to_string())
 }).await;
 match r{Ok(Ok(v))=>{let c=if v.get("error").is_some(){StatusCode::CONFLICT}else{StatusCode::OK};salida(c,v)},
 _=>salida(StatusCode::SERVICE_UNAVAILABLE,json!({"error":"SUPERVISOR_NO_CONFIRMADO","resultado":"desconocido"}))}
}
async fn pagina()->Response{
 let mut r=Html(include_str!("../web/index.html")).into_response();
 r.headers_mut().insert("content-security-policy","default-src 'none'; script-src 'self'; connect-src 'self'; style-src 'none'; frame-ancestors 'none'; base-uri 'none'; form-action 'none'".parse().unwrap());
 r.headers_mut().insert(header::CACHE_CONTROL,"no-store".parse().unwrap());r
}
async fn js()->impl IntoResponse{([(header::CONTENT_TYPE,"text/javascript"),(header::CACHE_CONTROL,"no-store"),(header::X_CONTENT_TYPE_OPTIONS,"nosniff")],include_str!("../web/app.js"))}
#[tokio::main(flavor="multi_thread",worker_threads=2)]
async fn main()->Result<(),Error>{
 let v:Vec<String>=std::env::args().collect();if v.len()!=3{return Err("servidor SOCKET ORIGEN".into())}
 let app=App{socket:v[1].clone(),origen:v[2].clone(),cupo:Arc::new(tokio::sync::Semaphore::new(4))};
 let r=Router::new().route("/",get(pagina)).route("/app.js",get(js)).route("/api",post(api))
 .layer(DefaultBodyLimit::max(MAX_INPUT)).with_state(app);
 let listener=tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
 axum::serve(listener,r).await?;Ok(())
}
