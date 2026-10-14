//! Restricción de red del proceso local de traducción en Linux x86_64.
use std::{io, fs::{File, OpenOptions}, path::Path, os::{fd::AsRawFd, unix::fs::OpenOptionsExt}};
#[repr(C)] struct Filtro { codigo:u16, si:u8, no:u8, valor:u32 }
#[repr(C)] struct Programa { largo:u16, instrucciones:*const Filtro }
unsafe extern "C" { fn prctl(op:i32, ...) -> i32; fn syscall(n:i64, ...) -> i64; }
fn control(n:i64)->io::Result<()> { if n<0 { Err(io::Error::last_os_error()) } else { Ok(()) } }
pub fn sin_red()->io::Result<()> {
    if !cfg!(all(target_os="linux",target_arch="x86_64")) { return Err(io::Error::other("Plataforma no recibida")); }
    // El proceso recibe únicamente los tres conductos estándar; no hereda conexiones.
    control(unsafe {syscall(436,3u32,u32::MAX,0u32)})?;
    control(unsafe {prctl(38,1,0,0,0)} as i64)?;
    let mut f=vec![
        Filtro{codigo:0x20,si:0,no:0,valor:4},
        Filtro{codigo:0x15,si:1,no:0,valor:0xc000003e},
        Filtro{codigo:0x06,si:0,no:0,valor:0x80000000},
        Filtro{codigo:0x20,si:0,no:0,valor:0},
    ];
    // socket, connect, socketpair, execve, execveat y variantes x32.
    for n in [41,42,53,59,101,310,311,322] {
        for nr in [n,n|0x40000000] {
            f.push(Filtro{codigo:0x15,si:0,no:1,valor:nr});
            f.push(Filtro{codigo:0x06,si:0,no:0,valor:0x00050001});
        }
    }
    f.push(Filtro{codigo:0x06,si:0,no:0,valor:0x7fff0000});
    let p=Programa{largo:f.len() as u16,instrucciones:f.as_ptr()};
    control(unsafe {prctl(22,2,&p as *const Programa,0,0)} as i64)?;
    // Se ejecuta antes de crear hilos, que heredan la restricción.
    let error=std::net::TcpListener::bind("127.0.0.1:0").err().ok_or_else(||io::Error::other("Red no restringida"))?;
    if error.raw_os_error()!=Some(1) { return Err(error); }
    eprintln!("Red del proceso deshabilitada; comprobación local EPERM conforme.");
    Ok(())
}

#[repr(C)] struct Reglas { fs:u64 }
#[repr(C,packed)] struct Ruta { acceso:u64, fd:i32 }
pub fn solo_modelo(modelo:&Path)->io::Result<Vec<File>> {
    // Reutiliza el criterio de objetos explícitos del perfil documental del SV.
    if std::fs::read_dir("/proc/self/task")?.count()!=1 {return Err(io::Error::other("Aislamiento requerido antes de crear hilos"));}
    let abi=unsafe {syscall(444,std::ptr::null::<u8>(),0usize,1u32)};
    if abi<6 {return Err(io::Error::other("Landlock ABI 6 o posterior requerido"));}
    let r=Reglas{fs:(1<<16)-1};
    let fd=unsafe {syscall(444,&r,std::mem::size_of::<Reglas>(),0u32)};control(fd)?;
    let mut anclas=Vec::new();
    let recibido=(|| {
        for (path,acceso) in [(modelo,12u64),(Path::new("/proc/self/status"),4u64)] {
            let f=OpenOptions::new().read(true).custom_flags(0x200000|0x80000).open(path)?;
            let regla=Ruta{acceso,fd:f.as_raw_fd()};
            control(unsafe {syscall(445,fd as i32,1i32,&regla,0u32)})?;
            anclas.push(f);
        }
        control(unsafe {syscall(446,fd as i32,0u32)})?;
        Ok::<(),io::Error>(())
    })();
    control(unsafe {syscall(3,fd as i32)})?;
    recibido?;
    // Archivo público presente: el rechazo prueba el límite sin leer credenciales.
    let err=std::fs::read("/etc/hostname").err().ok_or_else(||io::Error::other("Lectura ajena no restringida"))?;
    if err.kind()!=io::ErrorKind::PermissionDenied {return Err(err);}
    eprintln!("Lectura limitada al modelo y a métricas del proceso; Landlock ABI {abi}.");
    Ok(anclas)
}
