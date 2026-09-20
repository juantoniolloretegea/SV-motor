// Control de infraestructura: servidor limitado y CDP. Sin semantica SV.
import http from 'node:http';
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import {spawn,execFileSync} from 'node:child_process';
const root=process.env.RUNNER_TEMP+'/eio', web=root+'/web', ev=root+'/evidencia';
const manifest=JSON.parse(fs.readFileSync(web+'/recursos.json','utf8'));
const permitidos=new Map(manifest.map(x=>[x.ruta,x]));
permitidos.set('/recursos.json',{archivo:'recursos.json'});
let seqEscritura=0,ultimaCaptura=0,costeJournalMs=0;const totales={};
function journal(nombre,dato){
 const t=performance.now(),fila={secuencia_escritura:++seqEscritura,recepcion_controlador_mono:t,recepcion_controlador_civil:Date.now(),dato};
 const line=JSON.stringify(fila)+'\n',p=ev+'/'+nombre;
 if((fs.existsSync(p)?fs.statSync(p).size:0)+Buffer.byteLength(line)>2097152)throw Error('COTA_JOURNAL:'+nombre);
 const fd=fs.openSync(p,'a');try{fs.writeSync(fd,line);fs.fsyncSync(fd);}finally{fs.closeSync(fd);}
 costeJournalMs+=performance.now()-t;
 totales[nombre]=(totales[nombre]||0)+1;
 return {coste_acumulado_ms:costeJournalMs,secuencia_escritura:seqEscritura,fin_escritura_mono:performance.now(),coste_ms:performance.now()-t};
}
for(const n of ['accesos.jsonl','excepciones.jsonl','marcas.jsonl','eventos.jsonl'])journal(n,{tipo:'apertura'});
const accesos=[];let chrome,ws,server;let rc=1;
const excepciones=[];const pendientes=new Map();let sec=0;
const espera=ms=>new Promise(r=>setTimeout(r,ms));
function guardar(nombre,obj){fs.writeFileSync(ev+'/'+nombre,JSON.stringify(obj,null,2)+'\n');}
async function cdp(method,params={},sessionId){
 const id=++sec;
 return new Promise((resolve,reject)=>{
  const timer=setTimeout(()=>{pendientes.delete(id);reject(Error('CDP_PLAZO:'+method));},10000);
  pendientes.set(id,{resolve,reject,timer});
  ws.send(JSON.stringify({id,method,params,...(sessionId?{sessionId}:{})}));
 });
}
try{
 if(typeof WebSocket!=='function')throw Error('NODE_WEBSOCKET_NO_DISPONIBLE');
 const browser=['/usr/bin/google-chrome','/usr/bin/google-chrome-stable','/usr/bin/chromium','/usr/bin/chromium-browser'].find(p=>fs.existsSync(p));
 if(!browser)throw Error('NAVEGADOR_AUSENTE');
 guardar('versiones-navegador.json',{node:process.version,browser,version:execFileSync(browser,['--version'],{encoding:'utf8'}).trim(),
  ejecutable_sha256:crypto.createHash('sha256').update(fs.readFileSync(fs.realpathSync(browser))).digest('hex'),
  image:process.env.ImageVersion});
 server=http.createServer((req,res)=>{
  const x=permitidos.get(req.url);
  if(req.method!=='GET'||!x){res.writeHead(403);res.end();accesos.push({ruta:req.url,estado:403});journal('accesos.jsonl',accesos.at(-1));return;}
  const p=path.join(web,x.archivo||req.url.slice(1));
  accesos.push({ruta:req.url,bytes:x.bytes,sha256:x.sha256,estado:200,civil:Date.now()});
  journal('accesos.jsonl',accesos.at(-1));
  const tipo=p.endsWith('.wasm')?'application/wasm':p.endsWith('.js')?'text/javascript':p.endsWith('.html')?'text/html':p.endsWith('.json')?'application/json':'application/octet-stream';
  res.writeHead(200,{'Content-Type':tipo,'Cache-Control':'no-store',
   'Content-Security-Policy':"default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; worker-src 'self'; connect-src 'self'; base-uri 'none'; frame-ancestors 'none'"});
  fs.createReadStream(p).pipe(res);
 });
 await new Promise(r=>server.listen(0,'127.0.0.1',r));
 const port=server.address().port;
 const perfil=root+'/perfil-chrome';fs.mkdirSync(perfil);
 const args=['--headless=new','--disable-gpu','--no-first-run','--no-default-browser-check','--disable-background-networking','--disable-component-update','--disable-sync','--remote-debugging-address=127.0.0.1','--remote-debugging-port=0','--user-data-dir='+perfil,'about:blank'];
 guardar('orden-navegador.json',{browser,args,origen:'http://127.0.0.1:'+port});
 const out=fs.openSync(ev+'/chrome.stdout','w'),err=fs.openSync(ev+'/chrome.stderr','w');
 chrome=spawn(browser,args,{stdio:['ignore',out,err],env:{PATH:'/usr/bin:/bin',HOME:perfil,TMPDIR:root+'/tmp',LANG:'C.UTF-8'}});
 fs.closeSync(out);fs.closeSync(err);
 const iniciado=performance.now();
 while(!fs.existsSync(perfil+'/DevToolsActivePort')){
  if(chrome.exitCode!==null||performance.now()-iniciado>10000)throw Error('ARRANQUE_CHROME');
  await espera(100);
 }
 const [debugPort,socketPath]=fs.readFileSync(perfil+'/DevToolsActivePort','utf8').trim().split('\n');
 ws=new WebSocket('ws://127.0.0.1:'+debugPort+socketPath);
 await new Promise((resolve,reject)=>{ws.onopen=resolve;ws.onerror=reject;});
 ws.onmessage=event=>{
  const d=JSON.parse(String(event.data));
  if(d.id&&pendientes.has(d.id)){
   const p=pendientes.get(d.id);pendientes.delete(d.id);clearTimeout(p.timer);
   if(d.error)p.reject(Error(JSON.stringify(d.error)));else p.resolve(d.result);
  }else if(d.method==='Runtime.exceptionThrown'){
   excepciones.push(d.params);journal('excepciones.jsonl',d.params);
  }else if(d.method==='Runtime.bindingCalled' && d.params.name==='eioCaptura'){
   try{
    const evento=JSON.parse(d.params.payload);
    if(evento.secuencia!==ultimaCaptura+1)throw Error('SECUENCIA_CAPTURA');
    ultimaCaptura=evento.secuencia;
    const escritura=journal('eventos.jsonl',evento);
    if(evento.tipo==='marca')journal('marcas.jsonl',evento);
    if(evento.tipo.startsWith('excepcion'))journal('excepciones.jsonl',evento);
    let permiso=null;
    if(evento.tipo==='preflight'){
     const estado=JSON.parse(fs.readFileSync(ev+'/supervisor-activo.json','utf8'));
     const muestras=fs.readFileSync(ev+'/navegador.muestras.jsonl','utf8').trim().split('\n');
     const muestra=JSON.parse(muestras.at(-1));
     const eventos=fs.readFileSync(ev+'/eventos.jsonl','utf8');
     const casos=evento.dato.casos;
     const ok=estado.causa==='normal'&&estado.rss_disponible&&estado.familia_pid===process.pid&&
      Date.now()/1000-estado.civil_unix<3&&muestra.procesos.length>0&&
      totales['accesos.jsonl']>1&&totales['marcas.jsonl']>1&&totales['excepciones.jsonl']>=1&&
      eventos.includes('"terminacion-worker"')&&eventos.includes('"rechazo-tardio"')&&
      casos.length===4&&casos.every(x=>x.pasa);
     permiso={ok,estado,muestra_rss:muestra.secuencia,ultima_captura:ultimaCaptura,
      nav03_conservada:eventos.includes('"terminacion-worker"')&&eventos.includes('"rechazo-tardio"'),complemento_journal_presente:fs.statSync(ev+'/navegador.complemento.jsonl').size>0};
     guardar('preflight-custodia.json',permiso);journal('eventos.jsonl',{tipo:'decision-preflight',permiso});
    }
    const exp='globalThis.eioPersistida='+ultimaCaptura+';'+
     (permiso?'globalThis.eioPermisoModelo='+JSON.stringify(permiso)+';':'')+
     'globalThis.eioUltimaEscritura='+JSON.stringify(escritura);
    cdp('Runtime.evaluate',{expression:exp,contextId:d.params.executionContextId},d.sessionId).catch(e=>{journal('excepciones.jsonl',{error:String(e),fase:'ack-captura'});});
   }catch(e){
    journal('excepciones.jsonl',{error:String(e),fase:'captura-binding'});
    process.exitCode=96; if(chrome)chrome.kill('SIGTERM');
   }
  }
 };
 guardar('cdp-version.json',await cdp('Browser.getVersion'));
 const {targetId}=await cdp('Target.createTarget',{url:'about:blank'});
 const {sessionId}=await cdp('Target.attachToTarget',{targetId,flatten:true});
 await cdp('Runtime.enable',{},sessionId);
 await cdp('Runtime.addBinding',{name:'eioCaptura'},sessionId);
 await cdp('Page.enable',{},sessionId);
 await cdp('Page.navigate',{url:'http://127.0.0.1:'+port+'/index.html'},sessionId);
 const t=performance.now();
 let snapshot;
 while(performance.now()-t<170000){
  const x=await cdp('Runtime.evaluate',{expression:'globalThis.eio ? JSON.stringify(globalThis.eio) : null',returnByValue:true},sessionId);
  if(x.exceptionDetails)throw Error('CDP_EVALUACION');
  if(x.result.value){
   snapshot=JSON.parse(x.result.value);guardar('navegador.json',snapshot);
   if(snapshot.finalizado)break;
  }
  await espera(500);
 }
 if(!snapshot?.finalizado)throw Error('PLAZO_NAVEGADOR');
 const c=snapshot.registro.casos;
 rc=(excepciones.length===0&&c.length===5&&c.slice(0,4).every(x=>x.pasa)&&c[4].ejecucion_completada&&c[4].cobertura_eventos)?(c[4].aceptacion_contractual?0:2):3;
}catch(e){journal('excepciones.jsonl',{error:String(e),stack:e.stack});guardar('error-controlador.json',{error:String(e),stack:e.stack});}
finally{
 guardar('accesos.json',accesos);guardar('excepciones.json',excepciones);
 if(ws?.readyState===1){try{await cdp('Browser.close');}catch{}ws.close();}
 if(chrome && chrome.exitCode===null){chrome.kill('SIGTERM');await espera(1000);if(chrome.exitCode===null)chrome.kill('SIGKILL');}
 if(server){server.closeAllConnections();await new Promise(r=>server.close(r));}
 guardar('cierre-controlador.json',{chrome_exit:chrome?.exitCode,chrome_signal:chrome?.signalCode,servidor_cerrado:!server?.listening,retorno:rc,
  alcance:'El supervisor exterior coteja la familia de procesos. No se inspeccionan procesos del PC.'});
}
process.exit(rc);
