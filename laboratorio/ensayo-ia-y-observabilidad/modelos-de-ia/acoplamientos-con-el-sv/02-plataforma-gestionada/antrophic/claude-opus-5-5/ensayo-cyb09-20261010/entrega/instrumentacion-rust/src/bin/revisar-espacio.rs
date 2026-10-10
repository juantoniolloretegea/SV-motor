//! Inventario administrativo y retirada limitada de copias previamente custodiadas.
//! © 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, env, fs, io::{Read, Write}, path::{Component, Path, PathBuf}, time::UNIX_EPOCH};
type R<T> = Result<T, Box<dyn std::error::Error>>;
const BASE: &str = "inventarios/revision-espacio-20261009";
const ROOT: &str = r"C:\laboratorio\watson-local\lenguaje-computacion-sv";
const PREFIJOS: &[&str] = &["compilacion/oauth", "compilacion/visor-egui", "compilacion/visor-web"];
fn leer(p: impl AsRef<Path>) -> R<Value> { Ok(serde_json::from_slice(&fs::read(p)?)?) }
fn escribir(p: impl AsRef<Path>, v: &Value) -> R<()> {
    let mut f = fs::OpenOptions::new().create_new(true).write(true).open(p)?;
    f.write_all(&serde_json::to_vec_pretty(v)?)?; f.sync_all()?; Ok(())
}
fn sum(f: &mut fs::File) -> R<String> {
    let mut h = Sha256::new(); let mut b = vec![0; 1024*1024];
    loop { let n=f.read(&mut b)?; if n==0 { break; } h.update(&b[..n]); }
    Ok(format!("{:x}", h.finalize()))
}
fn ns(m: &fs::Metadata) -> R<String> { Ok(m.modified()?.duration_since(UNIX_EPOCH)?.as_nanos().to_string()) }
fn seguro(root: &Path, rel: &str) -> R<PathBuf> {
    let r = Path::new(rel);
    if r.is_absolute() || !r.components().all(|c| matches!(c, Component::Normal(_))) { return Err("Ruta no relativa segura".into()); }
    let mut p=root.to_path_buf();
    for c in r.components() {
        p.push(c); let m=fs::symlink_metadata(&p)?;
        #[cfg(windows)] { use std::os::windows::fs::MetadataExt; if m.file_attributes() & 0x400 != 0 { return Err("Punto de reanálisis excluido".into()); } }
        if m.file_type().is_symlink() { return Err("Enlace excluido".into()); }
    }
    let p=p.canonicalize()?;
    if p==root || !p.starts_with(root) { return Err("Fuera del perímetro".into()); } Ok(p)
}
fn elegible(rel: &str) -> bool {
    let a: Vec<_>=rel.split('/').collect();
    PREFIJOS.iter().any(|p| rel.starts_with(&format!("{p}/"))) &&
        a.iter().skip(3).any(|p| ["deps","build","incremental",".fingerprint"].contains(p))
}
fn visitar(root: &Path, rel: &str, archivos: &mut Vec<String>, errores: &mut Vec<Value>) {
    let p=match seguro(root,rel) { Ok(p)=>p, Err(e)=>{errores.push(json!({"ruta":rel,"causa":e.to_string()}));return;} };
    let entries=match fs::read_dir(p) { Ok(e)=>e, Err(e)=>{errores.push(json!({"ruta":rel,"causa":e.to_string()}));return;} };
    for e in entries {
        match e {
            Ok(e)=>{
                let s=format!("{rel}/{}",e.file_name().to_string_lossy());
                match seguro(root,&s) {
                    Ok(p) if p.is_dir()=>visitar(root,&s,archivos,errores),
                    Ok(p) if p.is_file()=>archivos.push(s),
                    Ok(_)=>errores.push(json!({"ruta":s,"causa":"Tipo no regular"})),
                    Err(e)=>errores.push(json!({"ruta":s,"causa":e.to_string()})),
                }
            }, Err(e)=>errores.push(json!({"ruta":rel,"causa":e.to_string()})),
        }
    }
}
fn custodia() -> R<Value> {
    let ed=leer(format!("{BASE}/EDICION-ACTUAL.json"))?;
    if ed["draft"]!=false || ed["id"]!=406092276_u64 { return Err("Edición no conforme".into()); }
    let antiguos=leer(format!("{BASE}/ACTIVOS-CUSTODIA-PREVIA.json"))?;
    let actuales=ed["assets"].as_array().ok_or("Activos ausentes")?;
    for a in antiguos.as_array().ok_or("Custodia sin activos")? {
        let b=actuales.iter().find(|b| b["name"]==a["nombre"]).ok_or("Activo ya no presente")?;
        if b["id"]!=a["id_activo"] || b["size"]!=a["bytes"] { return Err("Activo remoto sustituido".into()); }
    }
    let v=leer(format!("{BASE}/recuperado/MANIFIESTO-COMPILACIONES.json"))?;
    let a=antiguos.as_array().unwrap().iter().find(|a| a["nombre"]=="MANIFIESTO-COMPILACIONES.json").unwrap();
    let mut f=fs::File::open(format!("{BASE}/recuperado/MANIFIESTO-COMPILACIONES.json"))?;
    if f.metadata()?.len()!=a["bytes"].as_u64().unwrap() || sum(&mut f)?!=a["sha256"].as_str().unwrap() || v["conforme"]!=true { return Err("Manifiesto recuperado distinto".into()); }
    let previo=leer(format!("{BASE}/CONSTANCIA-CUSTODIA-PREVIA.json"))?;
    if previo["conforme"]!=true || previo["archivos_contenidos"]!=28464 || previo["archivo_sha256"]!=v["archivo_sha256"] { return Err("Reconstrucción previa no acreditada".into()); }
    Ok(v)
}
fn main() -> R<()> {
    let root=Path::new(ROOT).canonicalize()?;
    if env::current_dir()?.canonicalize()?!=root { return Err("Directorio de trabajo distinto".into()); }
    let modo=env::args().nth(1).ok_or("Modo inspeccionar o retirar requerido")?;
    let m=custodia()?;
    if modo=="inspeccionar" {
        let mut indices=BTreeMap::<u64,BTreeMap<String,String>>::new();
        for a in m["archivos"].as_array().ok_or("Manifiesto sin archivos")? {
            indices.entry(a["bytes"].as_u64().unwrap()).or_default().insert(a["sha256"].as_str().unwrap().to_owned(),a["ruta"].as_str().unwrap().to_owned());
        }
        let mut rutas=vec![]; let mut errores=vec![];
        for p in PREFIJOS { visitar(&root,p,&mut rutas,&mut errores); }
        let mut candidatas=vec![]; let mut inventario=BTreeMap::<String,[u64;4]>::new();
        for rel in rutas {
            let p=seguro(&root,&rel)?; let meta=fs::metadata(&p)?;
            let grupo=PREFIJOS.iter().find(|a| rel.starts_with(&format!("{a}/"))).unwrap().to_string();
            let t=inventario.entry(grupo).or_default(); t[0]+=1;t[1]+=meta.len();
            if !elegible(&rel) {continue;}
            if let Some(posibles)=indices.get(&meta.len()) {
                let mut f=fs::File::open(&p)?; let h=sum(&mut f)?;
                if let Some(origen)=posibles.get(&h) {
                    if ns(&fs::metadata(&p)?)?!=ns(&meta)? { errores.push(json!({"ruta":rel,"causa":"Cambio concurrente"}));continue; }
                    t[2]+=1;t[3]+=meta.len();candidatas.push(json!({"ruta":rel,"bytes":meta.len(),"sha256":h,"modificado_ns":ns(&meta)?,"ruta_en_archivo_remoto":origen}));
                }
            }
        }
        let total:u64=candidatas.iter().map(|c|c["bytes"].as_u64().unwrap()).sum();
        let plan=json!({"naturaleza":"Sólo copias idénticas a contenidos previamente reconstruidos y cotejados; fuentes y productos finales excluidos", "edicion":"compilaciones-instrumentales-conservacion-20261007-v1", "manifiesto_sha256":"a26c70cd67b771adc467b83e3886885d123da2f80aa656813c4ff27920784263", "inventario":inventario,"candidatas":candidatas,"bytes_logicos":total,"errores_o_exclusiones":errores});
        escribir(format!("{BASE}/PLAN-RUST.json"),&plan)?;
        println!("{}",json!({"inventario":plan["inventario"],"candidatas":plan["candidatas"].as_array().unwrap().len(),"bytes_logicos":total,"exclusiones":plan["errores_o_exclusiones"].as_array().unwrap().len()}));
    } else if modo=="retirar" {
        let plan=leer(format!("{BASE}/PLAN-RUST.json"))?; let mut retirados=vec![];let mut preservados=vec![];let mut total=0_u64;
        let mut registro=fs::OpenOptions::new().create_new(true).write(true).open(format!("{BASE}/RETIRADAS.jsonl"))?;
        for c in plan["candidatas"].as_array().ok_or("Plan sin candidatas")? {
            let rel=c["ruta"].as_str().ok_or("Ruta")?;
            let intento=(|| -> R<()> {
                if !elegible(rel) {return Err("Ruta no elegible".into());}
                let p=seguro(&root,rel)?;
                let mut opciones=fs::OpenOptions::new();opciones.read(true);
                #[cfg(windows)] { use std::os::windows::fs::OpenOptionsExt; opciones.share_mode(1|4); }
                let mut f=opciones.open(&p)?; let meta=f.metadata()?;
                if meta.len()!=c["bytes"].as_u64().unwrap() || ns(&meta)?!=c["modificado_ns"].as_str().unwrap() || sum(&mut f)?!=c["sha256"].as_str().unwrap() { return Err("Cambio concurrente; se conserva".into()); }
                let origen=m["archivos"].as_array().unwrap().iter().find(|a| a["ruta"]==c["ruta_en_archivo_remoto"]).ok_or("Origen no presente")?;
                if origen["sha256"]!=c["sha256"] || origen["bytes"]!=c["bytes"] { return Err("Custodia no correspondiente".into()); }
                if seguro(&root,rel)?!=p {return Err("Cambio de perímetro".into());}
                writeln!(registro,"{}",json!({"antes_de_retirada":c}))?;registro.sync_all()?;
                fs::remove_file(&p)?;
                writeln!(registro,"{}",json!({"retirada_completa":c}))?;registro.sync_all()?;Ok(())
            })();
            match intento {Ok(())=>{total+=c["bytes"].as_u64().unwrap();retirados.push(c.clone());},Err(e)=>preservados.push(json!({"ruta":rel,"causa":e.to_string()}))}
        }
        let resultado=json!({"retirados":retirados,"preservados_por_cambio_o_error":preservados,"bytes_logicos_retirados":total,"evidencias_unicas_retiradas":0,"fuentes_y_productos_finales_retirados":0});
        escribir(format!("{BASE}/RESULTADO-RUST.json"),&resultado)?;
        println!("{}",json!({"archivos_retirados":resultado["retirados"].as_array().unwrap().len(),"bytes_logicos_retirados":total,"preservados":resultado["preservados_por_cambio_o_error"].as_array().unwrap().len()}));
    } else { return Err("Modo no permitido".into()); } Ok(())
}
