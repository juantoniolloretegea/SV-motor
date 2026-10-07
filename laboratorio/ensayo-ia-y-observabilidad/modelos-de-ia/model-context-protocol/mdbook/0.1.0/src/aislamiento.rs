use serde_json::json;
use std::{
    fs, io,
    net::{SocketAddr, TcpStream},
    path::Path,
    time::Duration,
};
#[repr(C)]
struct Filter {
    code: u16,
    jt: u8,
    jf: u8,
    k: u32,
}
#[repr(C)]
struct Prog {
    len: u16,
    filter: *const Filter,
}
unsafe extern "C" {
    fn prctl(option: i32, ...) -> i32;
}
/// Linux x86_64: bloquear toda creación de sockets y conexión de red en el proceso MCP.
pub fn no_network() -> io::Result<()> {
    #[cfg(not(target_arch = "x86_64"))]
    return Err(io::Error::other("Arquitectura seccomp no admitida"));
    let f = [
        Filter {
            code: 0x20,
            jt: 0,
            jf: 0,
            k: 4,
        },
        Filter {
            code: 0x15,
            jt: 1,
            jf: 0,
            k: 0xc000003e,
        },
        Filter {
            code: 0x06,
            jt: 0,
            jf: 0,
            k: 0x80000000,
        },
        Filter {
            code: 0x20,
            jt: 0,
            jf: 0,
            k: 0,
        },
        Filter {
            code: 0x15,
            jt: 2,
            jf: 0,
            k: 41,
        },
        Filter {
            code: 0x15,
            jt: 1,
            jf: 0,
            k: 42,
        },
        Filter {
            code: 0x15,
            jt: 0,
            jf: 1,
            k: 0x40000029,
        },
        Filter {
            code: 0x06,
            jt: 0,
            jf: 0,
            k: 0x00050001,
        },
        Filter {
            code: 0x06,
            jt: 0,
            jf: 0,
            k: 0x7fff0000,
        },
    ];
    let p = Prog {
        len: f.len() as u16,
        filter: f.as_ptr(),
    };
    if unsafe { prctl(38, 1, 0, 0, 0) } != 0 || unsafe { prctl(22, 2, &p) } != 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}
pub fn probe(cat: &Path) -> serde_json::Value {
    let external = TcpStream::connect_timeout(
        &"1.1.1.1:443".parse::<SocketAddr>().unwrap(),
        Duration::from_millis(300),
    );
    let local = TcpStream::connect_timeout(
        &"127.0.0.1:1234".parse::<SocketAddr>().unwrap(),
        Duration::from_millis(300),
    );
    json!({"catalogo_legible":fs::read(cat).is_ok(),"red_externa_error":external.err().map(|e|e.raw_os_error()),"red_local_error":local.err().map(|e|e.raw_os_error()),"alcance":"rechazo de creación/conexión de sockets en este proceso; no inspecciona otros recursos"})
}
