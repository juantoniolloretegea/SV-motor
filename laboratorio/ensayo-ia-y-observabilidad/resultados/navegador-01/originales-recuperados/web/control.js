const ENTRADA='{"peticion":"nav-control-01","referencias":[{"id":"A","version":1},{"id":"B","version":1}],"accion":"marcar","respuesta":"Control sintético finito A y B."}';
const registro={agente:navigator.userAgent,origen:location.origin,casos:[],mensajes:[],rechazos_tardios:[],errores:[]};
globalThis.eio={finalizado:false,registro};
const cancelados=new Set(); const vivos=new Map();
function recibir(d,esperado,canal){
 if(cancelados.has(d.id)||d.id!==esperado||!vivos.has(canal)){
  registro.rechazos_tardios.push({id:d.id,esperado,tipo:d.tipo,mono:performance.now()});return false;
 }
 registro.mensajes.push({...d,recibido_mono:performance.now()});return true;
}
function tarea(id,op,limite,extra={}){
 return new Promise((resolve,reject)=>{
  const w=new Worker('/worker.js',{type:'module'});vivos.set(w,id);
  const inicio=performance.now();let listo=false,sonda=false,timerStop;
  const cerrar=()=>{clearTimeout(timer);clearTimeout(timerStop);cancelados.add(id);vivos.delete(w);w.onmessage=null;w.onerror=null;w.terminate();};
  const timer=setTimeout(()=>{cerrar();reject(Error(id+':PLAZO_EXTERIOR'));},limite);
  w.onerror=e=>{cerrar();reject(Error(id+':'+e.message));};
  w.onmessage=({data:d})=>{
   if(!recibir(d,id,w))return;
   if(d.tipo==='error'){cerrar();reject(Error(d.error));return;}
   if(d.tipo==='listo')listo=true;
   if(op==='sonda' && d.tipo==='sonda-inicio'){
    sonda=true;const inicioSonda=performance.now();
    timerStop=setTimeout(()=>{
     const orden=performance.now();cerrar();
     const antes=registro.rechazos_tardios.length;
     // Control negativo del receptor: una respuesta sintética del ID cancelado.
     const aceptada=recibir({id,tipo:'resultado',literal:'CONTROL_TARDIO_SINTETICO'},id,w);
     resolve({id,pasa:listo&&sonda&&!aceptada&&registro.rechazos_tardios.length===antes+1,
      inicio, inicio_sonda:inicioSonda,orden_terminacion:orden,transcurrido_ms:orden-inicioSonda,
      fin:performance.now(),canal_cerrado:true,control_tardio_aceptado:aceptada,
      limite:'terminate y cierre observados; ausencia de mensajes no acredita cese físico de toda actividad'});
    },2000);
   }
   if(d.tipo==='resultado' && op!=='sonda'){
    const fin=performance.now();cerrar();resolve({id,listo,inicio,fin,mensaje:d});
   }
  };
  w.postMessage({id,op,entrada:ENTRADA,...extra});
 });
}
async function ejecutar(){
 try{
  const a=await tarea('NAV-01','control',10000);
  const ar=JSON.parse(a.mensaje.literal);
  registro.casos.push({...a,pasa:a.listo&&ar.pasa});
  if(!registro.casos.at(-1).pasa)throw Error('NAV-01');
  const b=await tarea('NAV-02','control',10000);
  const br=JSON.parse(b.mensaje.literal);
  registro.casos.push({...b,pasa:b.listo&&br.pasa&&br.resultado==='OK'&&br.objeto===true});
  if(!registro.casos.at(-1).pasa)throw Error('NAV-02');
  const s=await tarea('NAV-03-cancelada','sonda',10000);
  const nueva=await tarea('NAV-03-nueva','control',Math.max(1,10000-(performance.now()-s.inicio)));
  const nr=JSON.parse(nueva.mensaje.literal);
  registro.casos.push({id:'NAV-03',pasa:s.pasa&&s.transcurrido_ms>=2000&&nueva.fin-s.inicio<=10000&&nr.pasa,sonda:s,nueva});
  if(!registro.casos.at(-1).pasa)throw Error('NAV-03');
  const c=await tarea('NAV-04','control',10000,{omitir:true});
  const cr=JSON.parse(c.mensaje.literal);
  registro.casos.push({...c,pasa:c.listo&&cr.pasa&&cr.resultado==='EVENTO_AUSENTE'&&cr.objeto===false});
  if(!registro.casos.at(-1).pasa)throw Error('NAV-04');
  const m=await tarea('NAV-05','modelo',120000);
  const mr=JSON.parse(m.mensaje.literal);
  registro.casos.push({...m,ejecucion_completada:true,cobertura_eventos:mr.cobertura_eventos,contrato:mr.contrato,aceptacion_contractual:mr.contrato==='OK'&&mr.cobertura_eventos});
 }catch(e){registro.errores.push({error:String(e),stack:e.stack,mono:performance.now()});}
 finally{
  for(const [w,id]of vivos){cancelados.add(id);w.terminate();}vivos.clear();
  registro.fin_civil=Date.now();registro.workers_controlados_abiertos=vivos.size;
  globalThis.eio.finalizado=true;
 }
}
ejecutar();
