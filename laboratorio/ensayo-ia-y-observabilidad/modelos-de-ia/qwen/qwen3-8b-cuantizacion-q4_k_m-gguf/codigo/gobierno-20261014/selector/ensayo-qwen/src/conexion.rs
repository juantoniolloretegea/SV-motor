//! Acoplamiento experimental: la decisión del Selector gobierna la llamada física.
//! Dos dispositivos preparados, peticiones consecutivas, sin servicio público.
use super::*;
use selector::{Capacidad, Carga, Clase, Cobertura, Criterio, Decision, Destino, Estado,
    Politica, Recurso, Solicitud};

const VIGENCIA: u64 = 120_000;
const PLAZO_DECISION: u64 = 5_000;
const RESERVA: u64 = 1024 * 1024 * 1024;

pub(super) fn nombre(c: Clase) -> &'static str {
    match c { Clase::Cpu=>"CPU", Clase::Gpu=>"GPU", Clase::Tpu=>"TPU", Clase::Npu=>"NPU" }
}
fn ms(t: Instant) -> u64 { t.elapsed().as_millis() as u64 }
pub(super) fn mediana(v: &[u64]) -> R<u64> {
    if v.len()!=3 || v.contains(&0) { return Err("Se requieren tres tiempos positivos".into()); }
    let mut x=v.to_vec(); x.sort_unstable(); Ok(x[1])
}
fn carga(entrada: &[u32]) -> Carga {
    Carga { contrato:"qwen3-8b-secuencia-fijada-16-posiciones/v1".into(),
        revision:MODELO.into(),
        perfil:format!("Q4_K_M;residentes;contexto64;CPU4;T4-sm75;mwv;hw_dp4a;secuencia={:x}",Sha256::digest(serde_json::to_vec(entrada).unwrap())),
        dimensiones:vec![entrada.len(),151936] }
}
pub(super) fn destino(dispositivo: &str, revision: &str) -> Destino {
    Destino { servidor:"sv-selector-kaggle-sesion-local".into(), dispositivo:dispositivo.into(), adaptador_revision:revision.into() }
}
fn djson(d: &Destino) -> Value {
    json!({"servidor":d.servidor,"dispositivo":d.dispositivo,"adaptador_revision":d.adaptador_revision})
}
pub(super) fn decision_json(d: &Decision) -> Value {
    json!({"solicitud":d.solicitud,"politica_revision":d.politica_revision,"destino":djson(&d.destino),"clase":nombre(d.clase),"criterio":format!("{:?}",d.criterio),"evidencia_capacidad":d.evidencia_capacidad,"observado_ms":d.observado_ms,"decidida_ms":d.decidida_ms,"descartes":d.descartes.iter().map(|r|json!({"destino":djson(&r.destino),"motivo":format!("{:?}",r.motivo)})).collect::<Vec<_>>()})
}
pub(super) fn solicitud_json(s: &Solicitud) -> Value {
    json!({"id":s.id,"carga":{"contrato":s.carga.contrato,"revision":s.carga.revision,"perfil":s.carga.perfil,"dimensiones":s.carga.dimensiones},"memoria_adicional_bytes":s.memoria_adicional_bytes,"margen_bytes":s.margen_bytes,"residencia_obligatoria":s.residencia_obligatoria.as_ref().map(djson)})
}
fn politica(destinos: &[Destino], cpu: bool) -> Politica {
    Politica { revision:if cpu {"conexion-cpu-explicita/v1"} else {"conexion-menor-tiempo-total/v1"}.into(),
        destinos_permitidos:destinos.to_vec(), preferencia:vec![Clase::Cpu,Clase::Gpu],
        clase_obligatoria:if cpu {Some(Clase::Cpu)} else {None}, vigencia_ms:VIGENCIA, comparar_estimaciones:true }
}
pub(super) fn pjson(p: &Politica) -> Value {
    json!({"revision":p.revision,"destinos_permitidos":p.destinos_permitidos.iter().map(djson).collect::<Vec<_>>(),"preferencia":p.preferencia.iter().map(|c|nombre(*c)).collect::<Vec<_>>(),"clase_obligatoria":p.clase_obligatoria.map(nombre),"vigencia_ms":p.vigencia_ms,"comparar_estimaciones":p.comparar_estimaciones})
}
pub(super) fn recursos(c: &Carga, destinos: &[Destino], tiempos: &[u64], evidencia: &str, observado: u64, memoria: [u64;2]) -> Vec<Recurso> {
    [Clase::Cpu,Clase::Gpu].into_iter().enumerate().map(|(i,clase)| Recurso {
        destino:destinos[i].clone(), clase, estado:Estado::Comprobado, observado_ms:observado,
        memoria_disponible_bytes:Some(memoria[i]), disponible:true,
        capacidades:vec![Capacidad { carga:c.clone(), cobertura:Cobertura::Completa,
            evidencia:evidencia.into(), estimacion_total_us:Some(tiempos[i]) }] }).collect()
}
pub(super) fn rjson(rs: &[Recurso]) -> Value {
    json!(rs.iter().map(|r|json!({"destino":djson(&r.destino),"clase":nombre(r.clase),"estado":format!("{:?}",r.estado),"observado_ms":r.observado_ms,"memoria_disponible_bytes":r.memoria_disponible_bytes,"disponible":r.disponible,"capacidades":r.capacidades.iter().map(|c|json!({"contrato":c.carga.contrato,"revision":c.carga.revision,"perfil":c.carga.perfil,"dimensiones":c.carga.dimensiones,"cobertura":format!("{:?}",c.cobertura),"evidencia":c.evidencia,"estimacion_total_us":c.estimacion_total_us})).collect::<Vec<_>>()})).collect::<Vec<_>>())
}

// No se acepta un destino dado por la IA. La decisión se recalcula inmediatamente
// antes del envío con inventario actualizado y se consume una sola vez.
pub(super) fn validar_envio(d: &Decision, s: &Solicitud, p: &Politica, rs: &[Recurso], ahora: u64, carga_efectiva: &Carga) -> R<()> {
    if &s.carga != carga_efectiva || d.solicitud != s.id || d.politica_revision != p.revision
        || ahora.checked_sub(d.decidida_ms).is_none_or(|edad|edad>PLAZO_DECISION) {
        return Err("Solicitud, carga, política o vigencia de decisión discrepante".into());
    }
    let nueva=selector::seleccionar(s,p,rs,ahora).map_err(|e|format!("Revalidación rechazada: {e:?}"))?;
    if nueva.destino!=d.destino || nueva.clase!=d.clase || nueva.criterio!=d.criterio
        || nueva.evidencia_capacidad!=d.evidencia_capacidad || d.criterio!=Criterio::MenorTiempoTotalEstimado {
        return Err("Destino o fundamento cambió; no se sustituye el dispositivo".into());
    }
    if !matches!(d.clase,Clase::Cpu|Clase::Gpu) { return Err("Adaptador físico no disponible".into()); }
    Ok(())
}
pub(super) fn memoria(nv: &nvml_wrapper::Nvml, uuid: &str) -> R<[u64;2]> {
    let mut s=sysinfo::System::new(); s.refresh_memory();
    Ok([s.available_memory(),nv.device_by_uuid(uuid)?.memory_info()?.free])
}
pub(super) fn comprobar_gpu(g: &OwnedQuantizedModelCuda, reloj: &RelojGpu, nv: &nvml_wrapper::Nvml, uuid: &str) -> R<()> {
    let id=reloj.identidad()?;
    if id["uuid"]!=uuid || id["cc"]!=75 || !g.supports_gpu_resident() {
        return Err("Identidad o compatibilidad GPU cambió".into());
    }
    if nv.device_by_uuid(uuid)?.running_compute_processes()?.iter().all(|p|p.pid!=std::process::id()) {
        return Err("Proceso propio no identificado por NVML".into());
    }
    Ok(())
}
struct Salida { vectores: Vec<Vec<f32>>, registro: Value, tiempo: u64 }
fn ejecutar(g: &mut OwnedQuantizedModelCuda, reloj: &RelojGpu, clase: Clase, entrada: &[u32], inicio_salidas: usize, dir: &Path, id: &str) -> R<Salida> {
    // Mismo alcance temporal en CPU/GPU: caché nueva, cálculo, transferencias,
    // recuperación, observación por posición y conservación de vectores.
    fase(id);
    let t=Instant::now(); let lanzamientos=traza::total(); let antes=cpu()?;
    let mut cache=OwnedQuantizedKVCache::new(g.model().config().num_layers,g.model().config().kv_dim(),64);
    if clase==Clase::Gpu { g.executor_mut().reset_kv_cache_gpu(); g.executor().synchronize()?; }
    let mut posiciones=Vec::new(); let mut vectores=Vec::new();
    for (i,&token) in entrada.iter().enumerate() {
        let t=Instant::now(); let n=traza::total(); let a=cpu()?;
        let (v,eventos)=match clase {
            Clase::Cpu=>(g.model().forward_single_with_cache(token,&mut cache,i)?,None),
            Clase::Gpu=>{
                let e=reloj.iniciar(g.executor().sv_execution_stream().raw() as usize)?;
                let v=g.forward_gpu_resident(token,&mut cache,i)?;
                let ms=e.terminar()?; g.executor().synchronize()?; (v,Some(ms))
            },
            _=>return Err("Clase sin adaptador ejecutable".into()),
        };
        if v.len()!=151936 || v.iter().any(|x|!x.is_finite()) { return Err("Salida numérica incompleta".into()); }
        let b=cpu()?;
        let l=traza::total()-n;
        if (clase==Clase::Cpu && l!=0) || (clase==Clase::Gpu && l==0) { return Err("Ejecución física no coincide con destino".into()); }
        if i>=inicio_salidas { vector(dir,&format!("{id}-{:02}.f32",i-inicio_salidas),&v)?; vectores.push(v); }
        posiciones.push(json!({"posicion":i,"token":token,"pared_us":t.elapsed().as_micros(),"cpu_antes":a,"cpu_despues":b,"intervalo_gpu_ms":eventos,"lanzamientos_gpu":l}));
    }
    drop(cache);
    let despues=cpu()?;
    let tiempo=t.elapsed().as_micros() as u64;
    let registro=json!({"id":id,"clase_ejecutada":nombre(clase),"tiempo_servicio_us":tiempo,"cpu_antes":antes,"cpu_despues":despues,"lanzamientos_gpu":traza::total()-lanzamientos,"posiciones":posiciones,"salidas":vectores.len(),"alcance_temporal":"Desde creación de caché hasta recuperación, observación y conservación íntegra de vectores; modelo ya preparado. No incluye calibración, selección ni evaluación numérica posterior."});
    guardar(dir,&format!("{id}.json"),&registro)?;
    Ok(Salida {vectores,registro,tiempo})
}
fn contraste(a: &[Vec<f32>], b: &[Vec<f32>]) -> R<Value> {
    if a.len()!=8 || b.len()!=8 { return Err("Ocho salidas requeridas".into()); }
    let v=a.iter().zip(b).map(|(x,y)|comparar(x,y)).collect::<R<Vec<_>>>()?;
    Ok(json!({"conforme":v.iter().all(|x|x["conforme"]==true),"comparaciones":v}))
}
fn ensayo_conexion(modelo: &Path,d: &Path) -> R<Value> {
    let origen=Instant::now();
    fase("identidad_conexion");
    if fs::metadata(modelo)?.len()!=BYTES || hash(modelo)?!=MODELO { return Err("Pesos no conformes".into()); }
    let nv=nvml_wrapper::Nvml::init()?;
    let inventario=(0..nv.device_count()?).map(|i|->R<Value>{let v=nv.device_by_index(i)?;let m=v.memory_info()?;Ok(json!({"indice":i,"uuid":v.uuid()?,"nombre":v.name()?,"memoria_total":m.total,"memoria_libre":m.free}))}).collect::<R<Vec<_>>>()?;
    let uuid=inventario.first().and_then(|v|v["uuid"].as_str()).ok_or("GPU ausente")?.to_string();
    if !inventario[0]["nombre"].as_str().unwrap_or("").contains("T4") || memoria(&nv,&uuid)?[0]<12*RESERVA || memoria(&nv,&uuid)?[1]<8*RESERVA { return Err("Hardware o memoria fuera del contraste".into()); }
    let cpu_descripcion=fs::read_to_string("/proc/cpuinfo")?;
    let cpu_id=format!("CPU-{:x}",Sha256::digest(cpu_descripcion.as_bytes()));
    guardar(d,"INVENTARIO.json",&json!({"gpu":inventario,"cpu_id":cpu_id,"cpu_descripcion":cpu_descripcion,"cpu_logicas":std::thread::available_parallelism()?.get(),"controlador":nv.sys_driver_version()?,"tpu":"No observado ni ensayado","npu":"No observado ni ensayado"}))?;
    fase("preparacion_modelo"); let prep=Instant::now();
    let mapped=MappedGGUFModel::from_path(modelo)?;
    let vocab=mapped.model.vocabulary().ok_or("Vocabulario ausente")?;
    let bpe=ByteLevelBpe::from_gguf(&mapped.model.metadata,&vocab).map_err(|e|format!("Tokenización: {e:?}"))?;
    let p=bpe.encode(PROMPT); let cont=bpe.encode(CONT);
    if p.len()!=9 || cont.len()<7 || vocab.len()!=151936 { return Err("Entrada fuera del contraste anterior".into()); }
    let mut entrada=p.clone(); entrada.extend(&cont[..7]);
    let m=OwnedQuantizedModel::from_mapped(&mapped)?;
    if m.config().num_layers!=36 || m.config().hidden_dim!=4096 || m.config().num_heads!=32 || m.config().num_kv_heads!=8 {return Err("Modelo diferente".into());}
    guardar(d,"ENTRADA.json",&json!({"texto":PROMPT,"continuacion":CONT,"secuencia":entrada,"tokens_prompt":p,"primera_salida":8,"salidas":8,"vocabulario":151936,"formato":"f32 little-endian","tipo":"Secuencia fijada; no examen de aptitud ni generación libre"}))?;
    drop((mapped,bpe,vocab));
    let carga_cpu_us=prep.elapsed().as_micros();
    let mut g=OwnedQuantizedModelCuda::with_max_seq_len(m,0,64).map_err(|e|format!("GPU rechazada, sin sustitución: {e}"))?;
    let reloj=RelojGpu::nuevo()?;
    comprobar_gpu(&g,&reloj,&nv,&uuid)?;
    let perfil=serde_json::to_value(g.executor().gpu_profile())?;
    if perfil["q4k"]!="mwv" || perfil["q6k"]!="hw_dp4a" || ["hgemm_decode","fp8_prefill","fp8_decode","w4a16_interleaved","fused_gate_up"].iter().any(|k|perfil[*k]!=false) {return Err("Perfil GPU discrepante".into());}
    guardar(d,"PREPARACION.json",&json!({"total_us":prep.elapsed().as_micros(),"carga_cpu_us":carga_cpu_us,"perfil":perfil,"gpu":reloj.identidad()?,"paridad":{"estado":g.parity.status,"coseno":g.parity.cosine,"umbral":g.parity.threshold},"vram":g.vram_report(),"residencia":"Pesos CPU y GPU preparados; nueva caché para cada ejecución"}))?;
    let mut tiempos=[Vec::new(),Vec::new()]; let mut referencia=Vec::new(); let mut contrastes=Vec::new();
    // Tres pares, orden alterno; no se usan medidas históricas de otro dispositivo.
    for ronda in 0..3 {
        let orden=if ronda%2==0 {[Clase::Cpu,Clase::Gpu]} else {[Clase::Gpu,Clase::Cpu]};
        let mut resultados=Vec::new();
        for clase in orden {
            let salida=ejecutar(&mut g,&reloj,clase,&entrada,8,d,&format!("calibracion-{ronda}-{}",nombre(clase)))?;
            tiempos[usize::from(clase==Clase::Gpu)].push(salida.tiempo);
            resultados.push((clase,salida));
        }
        let cpu=&resultados.iter().find(|(c,_)|*c==Clase::Cpu).unwrap().1;
        let gpu=&resultados.iter().find(|(c,_)|*c==Clase::Gpu).unwrap().1;
        let c=contraste(&cpu.vectores,&gpu.vectores)?;
        if c["conforme"]!=true { return Err("Calibración numérica no conforme; no se selecciona".into()); }
        contrastes.push(c);
        if ronda==0 {referencia=cpu.vectores.clone();}
    }
    let estimaciones=[mediana(&tiempos[0])?,mediana(&tiempos[1])?];
    let medido=ms(origen);
    guardar(d,"CALIBRACION.json",&json!({"observado_ms":medido,"cpu_us":tiempos[0],"gpu_us":tiempos[1],"medianas_us":estimaciones,"contrastes":contrastes,"vigencia_ms":VIGENCIA,"condicion":"Misma sesión, modelo preparado, secuencia exacta, proceso sin concurrencia de solicitudes; no rendimiento universal"}))?;
    let evidencia=format!("CALIBRACION.json:sha256:{}",hash(&d.join("CALIBRACION.json"))?);
    let revision=hash(&std::env::current_exe()?)?;
    let destinos=[destino(&cpu_id,&revision),destino(&uuid,&revision)];
    let carga_ensayo=carga(&entrada);
    let mut solicitudes=Vec::new();
    for (id,solo_cpu) in [("solicitud-automatica",false),("solicitud-cpu",true)] {
        fase(&format!("admision-{id}")); let recorrido=Instant::now();
        let s=Solicitud {id:id.into(),carga:carga_ensayo.clone(),memoria_adicional_bytes:RESERVA/2,margen_bytes:RESERVA/2,residencia_obligatoria:None};
        let p=politica(&destinos,solo_cpu);
        comprobar_gpu(&g,&reloj,&nv,&uuid)?;
        let rs=recursos(&carga_ensayo,&destinos,&estimaciones,&evidencia,medido,memoria(&nv,&uuid)?);
        let decision=selector::seleccionar(&s,&p,&rs,ms(origen)).map_err(|e|format!("Selector rechazó: {e:?}"))?;
        guardar(d,&format!("{id}-DECISION.json"),&json!({"solicitud":solicitud_json(&s),"politica":pjson(&p),"recursos":rjson(&rs),"decision":decision_json(&decision)}))?;
        let frescos=recursos(&carga_ensayo,&destinos,&estimaciones,&evidencia,medido,memoria(&nv,&uuid)?);
        validar_envio(&decision,&s,&p,&frescos,ms(origen),&carga(&entrada))?;
        comprobar_gpu(&g,&reloj,&nv,&uuid)?;
        // Único lugar de envío: la clase procede de seleccionar, sin preferencia añadida.
        let antes=traza::total();
        let salida=ejecutar(&mut g,&reloj,decision.clase,&entrada,8,d,id)?;
        let total=recorrido.elapsed().as_micros();
        let c=contraste(&referencia,&salida.vectores)?;
        let registro=json!({"decision":decision_json(&decision),"ejecucion":salida.registro,"recorrido_desde_admision_us":total,"lanzamientos_gpu":traza::total()-antes,"contraste":c,"coincidencia_dispositivo":true});
        guardar(d,&format!("{id}-CIERRE.json"),&registro)?;
        if c["conforme"]!=true {return Err("Solicitud seleccionada no conforme".into());}
        solicitudes.push(registro);
    }
    // Caso negativo real en el control: TPU exigida, sin adaptador. Cero cálculo.
    let s=Solicitud { id:"rechazo-tpu-no-disponible".into(),carga:carga_ensayo.clone(),memoria_adicional_bytes:RESERVA/2,margen_bytes:RESERVA/2,residencia_obligatoria:None };
    let mut p=politica(&destinos,false); p.preferencia.push(Clase::Tpu); p.clase_obligatoria=Some(Clase::Tpu);
    let rs=recursos(&carga_ensayo,&destinos,&estimaciones,&evidencia,medido,memoria(&nv,&uuid)?);
    let antes=traza::total();
    let rechazo=selector::seleccionar(&s,&p,&rs,ms(origen));
    if !matches!(rechazo,Err(selector::Error::NingunDestino(_))) || traza::total()!=antes {return Err("Rechazo de clase no disponible incorrecto".into());}
    guardar(d,"RECHAZO.json",&json!({"solicitud":solicitud_json(&s),"politica":pjson(&p),"resultado":format!("{rechazo:?}"),"ejecuciones":0,"lanzamientos_gpu":0}))?;
    fs::write(d.join("BIBLIOTECAS-CARGADAS.txt"),fs::read("/proc/self/maps")?)?;
    Ok(json!({"nombre":"SV-Aprender","selector":selector::NOMBRE,"selector_sha256":format!("{:x}",Sha256::digest(SELECTOR.as_bytes())),"calibraciones":6,"solicitudes":solicitudes,"rechazos_sin_calculo":1,"conforme":true,"alcance":"Conexión experimental de decisión y ejecución real CPU/T4, con ambos modelos preparados; no integración productiva del Árbitro-Director, no validación del nodo, no TPU/NPU física ni aptitud del modelo"}))
}
pub fn trabajar(m: &Path,d: &Path) -> R<()> {
    let monitor=Monitor::iniciar(d)?;
    let resultado=ensayo_conexion(m,d); let observacion=monitor.terminar()?;
    match resultado {
        Ok(v)=>{let conforme=v["conforme"]==true && observacion["fallos"]==0;
            guardar(d,"RESULTADO.json",&json!({"conforme":conforme,"conexion":v,"telemetria":observacion}))?;
            if !conforme {return Err("Observación no conforme".into());}
        },
        Err(e)=>{guardar(d,"RESULTADO.json",&json!({"conforme":false,"error":e.to_string(),"telemetria":observacion}))?;return Err(e);}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn ejemplo() -> (Solicitud,Politica,Vec<Recurso>) {
        let c=carga(&[1,2]); let ds=[destino("cpu","rev"),destino("gpu","rev")];
        let s=Solicitud{id:"solicitud".into(),carga:c.clone(),memoria_adicional_bytes:1,margen_bytes:1,residencia_obligatoria:None};
        (s,politica(&ds,false),recursos(&c,&ds,&[20,10],"sha:prueba",0,[100,100]))
    }
    #[test] fn envio_respeta_decision_y_cpu_expresa() {
        let (s,mut p,rs)=ejemplo();
        let d=selector::seleccionar(&s,&p,&rs,10).unwrap();assert_eq!(d.clase,Clase::Gpu);
        validar_envio(&d,&s,&p,&rs,11,&s.carga).unwrap();
        p.clase_obligatoria=Some(Clase::Cpu);let d=selector::seleccionar(&s,&p,&rs,10).unwrap();assert_eq!(d.clase,Clase::Cpu);
        validar_envio(&d,&s,&p,&rs,11,&s.carga).unwrap();
    }
    #[test] fn no_sustituye_destino_ni_revision() {
        let (s,p,mut rs)=ejemplo(); let d=selector::seleccionar(&s,&p,&rs,10).unwrap();
        rs[1].disponible=false;assert!(validar_envio(&d,&s,&p,&rs,11,&s.carga).is_err());
        rs[1].disponible=true;rs[1].destino.dispositivo="otra_gpu".into();assert!(validar_envio(&d,&s,&p,&rs,11,&s.carga).is_err());
    }
    #[test] fn rechaza_otro_contenido_y_decision_vencida() {
        let (s,p,rs)=ejemplo();let d=selector::seleccionar(&s,&p,&rs,10).unwrap();
        assert!(validar_envio(&d,&s,&p,&rs,11,&carga(&[2,1])).is_err());
        assert!(validar_envio(&d,&s,&p,&rs,5011,&s.carga).is_err());
        assert!(validar_envio(&d,&s,&p,&rs,9,&s.carga).is_err());
    }
    #[test] fn memoria_insuficiente_no_reenvia() {
        let (s,p,mut rs)=ejemplo();let d=selector::seleccionar(&s,&p,&rs,10).unwrap();
        rs[1].memoria_disponible_bytes=Some(0);assert!(validar_envio(&d,&s,&p,&rs,11,&s.carga).is_err());
    }
    #[test] fn capacidad_vencida_no_se_renueva_con_observar_memoria() {
        let (s,p,rs)=ejemplo();assert!(selector::seleccionar(&s,&p,&rs,VIGENCIA+1).is_err());
    }
    #[test] fn mediana_y_datos_incompletos() {
        assert_eq!(mediana(&[30,10,20]).unwrap(),20);assert!(mediana(&[1,2]).is_err());assert!(mediana(&[0,2,3]).is_err());
    }
}
