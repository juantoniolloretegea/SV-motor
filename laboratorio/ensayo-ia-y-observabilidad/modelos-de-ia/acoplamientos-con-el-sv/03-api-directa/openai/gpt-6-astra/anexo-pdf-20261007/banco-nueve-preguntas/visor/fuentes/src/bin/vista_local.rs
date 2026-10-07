#![forbid(unsafe_code)]
use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    path::PathBuf,
    time::{Duration, Instant},
};
fn main() {
    let archivo = PathBuf::from(std::env::args().nth(1).expect("HTML autónomo"));
    let bytes = fs::read(&archivo).expect("HTML");
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let url = format!("http://{}/", listener.local_addr().unwrap());
    fs::write(archivo.parent().unwrap().join("VISTA-LOCAL.json"),serde_json::to_vec_pretty(&serde_json::json!({"url":url,"pid":std::process::id(),"alcance":"Sólo el HTML incorporado; sin acceso a otros archivos ni inferencia","caducidad_minutos":120})).unwrap()).unwrap();
    let fin = Instant::now() + Duration::from_secs(7200);
    while Instant::now() < fin {
        let (mut stream, _) = match listener.accept() {
            Ok(s) => s,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(50));
                continue;
            }
            Err(_) => break,
        };
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .unwrap();
        let mut req = [0; 8192];
        let n = match stream.read(&mut req) {
            Ok(n) => n,
            Err(_) => continue,
        };
        let line = String::from_utf8_lossy(&req[..n]);
        let primera = line.lines().next().unwrap_or("");
        let descarga = primera == "GET /POLIGONO-EGUI.html HTTP/1.1";
        if primera == "GET / HTTP/1.1" || descarga {
            let extra = if descarga {
                "Content-Disposition: attachment; filename=POLIGONO-EGUI.html\r\n"
            } else {
                ""
            };
            let h=format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\n{extra}Connection: close\r\n\r\n",bytes.len());
            let _ = stream.write_all(h.as_bytes());
            let _ = stream.write_all(&bytes);
        } else {
            let _ = stream.write_all(
                b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            );
        }
    }
}
