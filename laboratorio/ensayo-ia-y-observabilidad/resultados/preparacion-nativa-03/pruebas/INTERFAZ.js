// Preparado, NO ejecutado. Pegar tras revisión en consola de la interfaz privada,
// después de pulsar "Consultar estado". No inicia modelo ni cambia estado.
(async()=>{
 const r=await fetch("/api",{method:"POST",credentials:"same-origin",redirect:"error",
 headers:{"Content-Type":"application/json"},body:JSON.stringify({op:"estado"})});
 if(!r.ok)throw Error("API no disponible");
 const v=await r.json(), original=document.getElementById("original").textContent,
 juicio=document.getElementById("juicio").textContent;
 if(v.admisible!==true){
  if(v.resultado!==null||original!=="Sin resultado admisible"||juicio!=="No admisible técnicamente")throw Error("R1 interfaz");
 }else{
  if(original!==v.resultado.salida_original.texto||juicio!==v.resultado.juicio_verificador)throw Error("Original/juicio alterados");
 }
 if(document.querySelector("#original script,#original img"))throw Error("Contenido interpretado");
 console.log({caso:"R1-DOM-API",conforme:true,admisible:v.admisible,juicio,alcance:"instrumental"});
})();
