use crate::{E,hash};
use serde_json::{Value,json};
use std::{fs::File,io::{BufRead,BufReader,Read},path::Path};

pub fn recorrer(path: &Path, mut recibir: impl FnMut(&Value)->Result<(),E>)->Result<Value,E> {
    let mut r=BufReader::new(File::open(path)?);
    let mut previo="0".repeat(64); let mut secuencia=0u64; let mut unix_previo=0.0;
    loop {
        let mut linea=Vec::new();
        let n=(&mut r).take(1024*1024+1).read_until(b'\n',&mut linea)?;
        if n==0 { break; }
        if n>1024*1024 || linea.last()!=Some(&b'\n') { return Err("Línea excesiva o incompleta".into()); }
        let v=crate::estricto::parse(&linea)?; let p=&v["registro"];
        if p["secuencia"]!=secuencia || p["anterior"]!=previo { return Err("Secuencia o enlace alterado".into()); }
        let exactos:std::collections::BTreeMap<String,&serde_json::value::RawValue>=serde_json::from_slice(&linea)?;
        let calculada=hash(exactos.get("registro").ok_or("Registro ausente")?.get().as_bytes());
        if v["sha256"]!=calculada { return Err("Huella del registro discordante".into()); }
        let unix=p["unix"].as_f64().ok_or("Fecha ausente")?;
        if unix<unix_previo { return Err("Fecha de registro regresiva".into()); }
        recibir(&p["datos"])?;
        previo=calculada; unix_previo=unix; secuencia+=1;
    }
    if secuencia==0 { return Err("Diario vacío".into()); }
    Ok(json!({"registros":secuencia,"ultima_huella":previo,"conforme":true}))
}

#[cfg(test)] mod tests {
    use super::*;
    use std::io::Write;
    #[test] fn detecta_alteracion_y_truncamiento() {
        let root=std::env::current_dir().unwrap().join("evidencias-pruebas"); std::fs::create_dir_all(&root).unwrap();
        let path=root.join("diario-auditoria.jsonl");
        let p=json!({"secuencia":0,"anterior":"0".repeat(64),"unix":1.0,"datos":{"dato":"original"}});
        let v=json!({"registro":p,"sha256":hash(&serde_json::to_vec(&p).unwrap())});
        let mut f=File::create(&path).unwrap(); writeln!(f,"{v}").unwrap(); drop(f);
        assert!(recorrer(&path, |_|Ok(())).is_ok());
        let mut changed=v.clone(); changed["registro"]["datos"]["dato"]=json!("alterado");
        std::fs::write(&path,format!("{changed}\n")).unwrap(); assert!(recorrer(&path, |_|Ok(())).is_err());
        std::fs::write(&path,format!("{v}")).unwrap(); assert!(recorrer(&path, |_|Ok(())).is_err());
    }
}
