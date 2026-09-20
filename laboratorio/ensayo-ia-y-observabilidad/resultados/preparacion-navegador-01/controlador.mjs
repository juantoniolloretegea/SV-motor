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
  if(req.method!=='GET'||!x){res.writeHead(403);res.end();accesos.push({ruta:req.url,estado:403});return;}
  const p=path.join(web,x.archivo||req.url.slice(1));
  accesos.push({ruta:req.url,bytes:x.bytes,sha256:x.sha256,estado:200,civil:Date.now()});
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
  }else if(d.method==='Runtime.exceptionThrown')excepciones.push(d.params);
 };
 guardar('cdp-version.json',await cdp('Browser.getVersion'));
 const {targetId}=await cdp('Target.createTarget',{url:'about:blank'});
 const {sessionId}=await cdp('Target.attachToTarget',{targetId,flatten:true});
 await cdp('Runtime.enable',{},sessionId);
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
}catch(e){guardar('error-controlador.json',{error:String(e),stack:e.stack});}
finally{
 guardar('accesos.json',accesos);guardar('excepciones.json',excepciones);
 if(ws?.readyState===1){try{await cdp('Browser.close');}catch{}ws.close();}
 if(chrome && chrome.exitCode===null){chrome.kill('SIGTERM');await espera(1000);if(chrome.exitCode===null)chrome.kill('SIGKILL');}
 if(server){server.closeAllConnections();await new Promise(r=>server.close(r));}
 guardar('cierre-controlador.json',{chrome_exit:chrome?.exitCode,chrome_signal:chrome?.signalCode,servidor_cerrado:!server?.listening,retorno:rc,
  alcance:'El supervisor exterior coteja la familia de procesos. No se inspeccionan procesos del PC.'});
}
process.exit(rc);
