use std::{io::{Read, Write}, net::TcpStream, time::Duration};

fn get(path: &str) -> String {
    let mut stream = TcpStream::connect("127.0.0.1:3000").expect("Servicio local inaccesible");
    stream.set_read_timeout(Some(Duration::from_secs(15))).unwrap();
    write!(stream, "GET {path} HTTP/1.0\r\nHost: localhost\r\nConnection: close\r\n\r\n").unwrap();
    let mut response = String::new();
    stream.take(1_048_576).read_to_string(&mut response).unwrap();
    assert!(response.starts_with("HTTP/1.0 200 ") || response.starts_with("HTTP/1.1 200 "), "HTTP local no conforme");
    response.split_once("\r\n\r\n").expect("Sin cabecera HTTP").1.to_string()
}

fn main() {
    let page = get("/");
    assert!(page.contains("id=\"connection\"") && page.contains("id=\"lastCheck\""));
    assert!(page.contains("id=\"pendingNotice\""));
    assert!(page.contains("Juan Antonio Lloret Egea") && page.contains("CC BY-NC-ND 4.0"));
    let js = get("/app.js");
    assert!(js.contains("request_status") && js.contains("sessionStorage"));
    let css = get("/style.css");
    assert!(css.contains("data-state"));
    // No se muestran claves de sesión ni se solicita contenido de expedientes.
    println!("{{\"schema\":\"EIO-INSTALACION-LOCAL-1\",\"resultado\":\"conforme\",\"pagina_http\":200,\"javascript_http\":200,\"css_http\":200,\"aviso_sv_presente\":true,\"indicador_y_ultima_comprobacion_presentes\":true,\"recuperacion_de_peticion_presente\":true,\"inferencias\":0,\"acceso_externo_comprobado\":false,\"limite\":\"Comprueba el servicio HTTP local y el contenido servido; no ejecuta JavaScript ni acredita autenticación o acceso desde el navegador externo.\"}}");
}
