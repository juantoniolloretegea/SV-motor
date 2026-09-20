import init,{control,caso,sonda_no_cooperativa} from '/pkg/eio_candidato.js';
let iniciada=false;
self.onmessage=async ({data:d})=>{
 if(iniciada)return;
 iniciada=true;
 try{
  const manifest=await (await fetch('/recursos.json',{cache:'no-store'})).json();
  const recursos=[];
  async function bytes(path){
   const r=await fetch(path,{cache:'no-store'});if(!r.ok)throw Error('HTTP_RECURSO');
   const a=await r.arrayBuffer();
   const hash=Array.from(new Uint8Array(await crypto.subtle.digest('SHA-256',a)),b=>b.toString(16).padStart(2,'0')).join('');
   const x=manifest.find(x=>x.ruta===path);
   if(!x||x.bytes!==a.byteLength||x.sha256!==hash)throw Error('IDENTIDAD_RECURSO');
   recursos.push({ruta:path,bytes:a.byteLength,sha256:hash});return a;
  }
  await bytes('/pkg/eio_candidato.js');
  const wasm=await bytes('/pkg/eio_candidato_bg.wasm');
  await init({module_or_path:wasm});
  postMessage({id:d.id,tipo:'listo',recursos,mono:performance.now(),civil:Date.now()});
  if(d.op==='sonda'){postMessage({id:d.id,tipo:'sonda-inicio',mono:performance.now()});sonda_no_cooperativa();}
  else if(d.op==='control'){
   const literal=control(d.entrada,!!d.omitir);
   postMessage({id:d.id,tipo:'resultado',literal,recursos});
  }else if(d.op==='modelo'){
   const pesos=new Uint8Array(await bytes('/Qwen3-0.6B-Q4_K_M.gguf'));
   const tokenizer=new Uint8Array(await bytes('/tokenizer.json'));
   const peticion=new TextDecoder('utf-8',{fatal:true}).decode(await bytes('/peticion.txt'));
   postMessage({id:d.id,tipo:'inferencia-inicio',recursos,mono:performance.now()});
   const literal=caso(pesos,tokenizer,peticion);
   postMessage({id:d.id,tipo:'resultado',literal,recursos});
  }else throw Error('OPERACION');
 }catch(e){postMessage({id:d.id,tipo:'error',error:String(e),stack:e.stack});}
};
