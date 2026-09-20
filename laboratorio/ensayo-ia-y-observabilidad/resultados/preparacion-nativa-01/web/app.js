"use strict";
const peticiones={"referencia":"Datos exclusivamente sintéticos. A versión 1: el objeto está listo. B versión 1: el objeto está disponible. Las dos fuentes son necesarias. Devuelve únicamente un objeto JSON con peticion \"sintetica-01\", referencias [{\"id\":\"A\",\"version\":1},{\"id\":\"B\",\"version\":1}], accion \"ninguna\" y respuesta que indique que A y B son necesarias. No tienes herramientas ni permiso para ejecutar efectos.\n","estructurada":"Datos exclusivamente sintéticos. A versión 1: el objeto está listo. B versión 1: el objeto está disponible. Las dos fuentes son necesarias. No tiene herramientas ni permiso para ejecutar efectos.\nResponda únicamente con un objeto JSON válido. La raíz debe contener exactamente cuatro campos: \"peticion\", \"referencias\", \"accion\" y \"respuesta\". No utilice bloques Markdown, comentarios, texto anterior o posterior ni un objeto envolvente llamado \"sintetica-01\".\nUtilice esta estructura y complete únicamente el texto vacío de \"respuesta\" con una frase que explique qué fuentes son necesarias, conforme a los datos anteriores:\n{\"peticion\":\"sintetica-01\",\"referencias\":[{\"id\":\"A\",\"version\":1},{\"id\":\"B\",\"version\":1}],\"accion\":\"ninguna\",\"respuesta\":\"\"}\n"};
const $=id=>document.getElementById(id);let identidad=null;
async function api(x){
 const r=await fetch("/api",{method:"POST",headers:{"Content-Type":"application/json"},body:JSON.stringify(x),credentials:"same-origin",redirect:"error"});
 const v=await r.json();if(!r.ok)throw new Error(JSON.stringify(v));return v;
}
async function accion(x){try{
 const v=await api(x);if(v.id)identidad=v.id;$("salida").textContent=JSON.stringify(v,null,2);
 $("original").textContent=v.resultado?.salida_original?.texto??"Sin resultado admisible";
 $("juicio").textContent=v.resultado?.juicio_verificador??"No disponible";
}catch(e){$("salida").textContent="Consulta no confirmada: "+String(e);$("original").textContent="";$("juicio").textContent="Desconocido";}}
for(const p of ["referencia","estructurada"])$(p).onclick=()=>accion({op:"iniciar",contrato:"EIO-NAT/1",peticion:p,texto:peticiones[p]});
$("estado").onclick=()=>accion({op:"estado"});
$("cancelar").onclick=()=>accion({op:"cancelar",id:identidad??""});
for(const archivo of ["inferidor.jsonl","inferidor.stderr","supervision.jsonl","entrada.txt","MANIFIESTO.json"]){
 const b=document.createElement("button");b.textContent="Recuperar "+archivo;$("archivos").append(b);
 b.onclick=async()=>{b.disabled=true;try{
  let offset=0;const partes=[];let total=null;
  for(let i=0;i<257;i++){
   const v=await api({op:"evidencia",archivo,offset});
   if(v.archivo!==archivo||v.offset!==offset||v.total>8388608||(total!==null&&total!==v.total)||!/^(?:[0-9a-f]{2})*$/.test(v.hex))throw Error("Identidad de fragmento");
   total=v.total;const bytes=Uint8Array.from(v.hex.match(/../g)??[],h=>parseInt(h,16));partes.push(bytes);offset+=bytes.length;
   if(v.fin){if(offset!==total)throw Error("Tamaño final");const url=URL.createObjectURL(new Blob(partes,{type:"application/octet-stream"}));
    const a=document.createElement("a");a.href=url;a.download=archivo;a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);return;}
   if(bytes.length===0)throw Error("Sin progreso");
  }throw Error("Límite de recuperación");
 }catch(e){$("salida").textContent="Recuperación incompleta: "+String(e);}finally{b.disabled=false}};
}
