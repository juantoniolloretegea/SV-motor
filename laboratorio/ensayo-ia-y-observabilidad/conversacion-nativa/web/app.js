'use strict';
const $=id=>document.getElementById(id);let state=null,caseId=localStorage.getItem('eio.case')||'',chatId=localStorage.getItem('eio.chat')||'',current=null,busy=false,polling=false,lastRender='',connected=false,lastFailureConnection=false;
function connectionError(message){const e=Error(message);e.connection=true;return e;}
function fail(e){lastFailureConnection=!!e.connection;$('error').textContent=e.message||String(e);$('error').hidden=false;if(e.connection){connected=false;$('connection').textContent='Servicio no disponible';$('reconnect').hidden=false;controls();}}
function controls(){$('send').disabled=!connected||!!state?.active||!chatId||busy;$('newChat').disabled=!connected;$('preview').disabled=!connected;$('export').disabled=!connected||!caseId;$('createCase').disabled=!connected;}
function saveDraft(){try{sessionStorage.setItem('eio.draft',JSON.stringify({chat:chatId,text:$('question').value,title:$('caseTitle').value}));}catch{}}
function restoreDraft(){try{const d=JSON.parse(sessionStorage.getItem('eio.draft')||'null');if(d){$('caseTitle').value=d.title||'';if(d.chat===chatId)$('question').value=d.text||'';}}catch{}}
function resetPreview(){$('budget').textContent='El contexto se cuenta antes de enviar.';$('contextText').textContent='Todavía no se ha preparado una petición.';}
$('question').addEventListener('input',()=>{saveDraft();resetPreview();});$('caseTitle').addEventListener('input',saveDraft);
$('retryConnection').onclick=()=>refresh();$('reloadPage').onclick=()=>{saveDraft();location.reload();};
function clearError(){$('error').hidden=true;}
async function api(op,data={}){
 const controller=new AbortController(),timer=setTimeout(()=>controller.abort(),15000);
 try{
  const r=await fetch('/api',{method:'POST',headers:{'Content-Type':'application/json','X-EIO-Session':document.querySelector('meta[name=eio-session]').content},credentials:'same-origin',redirect:'error',signal:controller.signal,body:JSON.stringify({op,...data})});
  const body=await r.text(),type=(r.headers.get('content-type')||'').toLowerCase();
  if(!body.trim())throw connectionError('El servicio no ha devuelto contenido (HTTP '+r.status+'). Compruebe que el Codespace está iniciado y que el servicio de conversación está en ejecución. Su texto permanece en esta página.');
  if(!type.includes('application/json'))throw connectionError('La conexión ha devuelto una página ajena al servicio de conversación (HTTP '+r.status+'). Compruebe el Codespace y su sesión de GitHub.');
  let v;try{v=JSON.parse(body);}catch{throw connectionError('Se recibió una respuesta incompleta del servicio (HTTP '+r.status+'). No se repetirá automáticamente su petición.');}
  if(v===null||typeof v!=='object'||Array.isArray(v))throw connectionError('La respuesta recibida no tiene el formato del servicio.');
  if(r.status===403)throw connectionError(v.error||'La sesión ha cambiado. Pulse «Recargar la página»; se conservará el texto pendiente.');
  if(!r.ok||v.error)throw Error(v.error||'El servicio no ha completado la operación (HTTP '+r.status+').');
  return v;
 }catch(e){if(e.connection)throw e;if(e.name==='AbortError')throw connectionError('El servicio no respondió dentro del plazo de conexión. No se repetirá automáticamente su petición.');if(e instanceof TypeError)throw connectionError('No se pudo conectar con el servicio. Compruebe el Codespace y su sesión de GitHub.');throw e;}finally{clearTimeout(timer);}
}
function profile(){return{thinking:$('thinking').checked,max_output:Number($('maxOutput').value),seconds:Number($('seconds').value),seed:299792458};}
function el(tag,text,cls){const e=document.createElement(tag);if(text!==undefined)e.textContent=text;if(cls)e.className=cls;return e;}
function renderState(){const cases=Object.values(state.cases);if(!state.cases[caseId])caseId=cases[0]?.id||'';const list=$('caseList');list.replaceChildren();for(const c of cases){const b=el('button',c.title,c.id===caseId?'selected':'');b.onclick=()=>selectCase(c.id);list.append(b);}
 const chats=state.chats.filter(c=>c.case_id===caseId);if(!chats.some(c=>c.id===chatId))chatId=chats[0]?.id||'';
 $('chatSelect').replaceChildren(...chats.map(c=>{const e=el('option',c.title);e.value=c.id;return e;}));$('chatSelect').value=chatId;
 $('caseName').textContent=state.cases[caseId]?.title||'Conversación con Qwen';$('workspace').hidden=!caseId;$('empty').hidden=!!caseId;$('export').disabled=!caseId;
 $('connection').textContent=state.active?'Qwen está trabajando · '+state.active.seconds+' s':'Servicio disponible · '+state.identity.model+' · CPU';
 $('storage').textContent='Registro conservado: '+(state.storage_bytes/1048576).toFixed(2)+' MiB de '+state.storage_limit/1048576+' MiB. Ningún expediente se elimina automáticamente.';
 localStorage.setItem('eio.case',caseId);localStorage.setItem('eio.chat',chatId);controls();
}
function detail(parent,label,text){const d=el('details');d.append(el('summary',label),el('pre',text));parent.append(d);}
function renderChat(data){current=data;const key=JSON.stringify(data);if(key===lastRender)return;lastRender=key;const box=$('messages'),atEnd=box.scrollTop+box.clientHeight>=box.scrollHeight-70;const oldScroll=box.scrollTop;const opened=[...box.querySelectorAll('details')].map(d=>d.open);
 if(!data.chat.turns.length){if(box.dataset.chat!==chatId||box.querySelector('article'))box.innerHTML='<div class="welcome"><span class="seal">SV</span><h2>Un contexto explícito.<br>Una conversación conservada.</h2><p>Escriba una pregunta para comenzar.</p></div>';}
 else{const children=[];for(const t of data.chat.turns){const u=el('article',undefined,'user');u.append(el('div','Usted','who'),el('div',t.user,'body'));children.push(u);const a=el('article',undefined,t.status!=='fin_normal'?'incomplete':'');a.append(el('div','Qwen · '+(t.profile.thinking?'con razonamiento':'respuesta directa'),'who'));let raw=t.raw,answer=t.answer,thinking=t.thinking,active=data.active?.id===t.id?data.active:null;
 if(active){raw=active.raw;const i=raw.indexOf('</think>');if(i>=0){thinking=raw.slice(0,i).replace(/^<think>/,'').trim();answer=raw.slice(i+8).trim();}else if(t.profile.thinking){thinking=raw.replace(/^<think>/,'');answer='';}else answer=raw;}
 a.append(el('div',answer||(active?active.phase:(thinking?'La generación terminó sin una respuesta final separada.':'No se obtuvo una respuesta.')),'body'));
 if(thinking)detail(a,'Razonamiento generado por Qwen',thinking);
 const r=t.result||{};const status=active?'En curso · '+active.seconds+' s':t.status.replaceAll('_',' ');a.append(el('div',status+(r.tokens!==undefined?' · '+r.tokens+' unidades generadas':'')+(r.seconds?' · '+r.seconds.toFixed(2)+' s':'')+(r.first_output_seconds?' · primera salida '+r.first_output_seconds.toFixed(2)+' s':''),'meta'));
 if(r.error)a.append(el('p',r.error,'note'));detail(a,'Contexto utilizado · '+t.context.input_tokens+' unidades',t.context.prompt);detail(a,'Suceso de finalización',JSON.stringify(t.result,null,2));children.push(a);}
 box.replaceChildren(...children);[...box.querySelectorAll('details')].forEach((d,i)=>{d.open=!!opened[i];});}
 if(atEnd||box.dataset.chat!==chatId)box.scrollTop=box.scrollHeight;else box.scrollTop=oldScroll;box.dataset.chat=chatId;
 $('cancel').hidden=!data.active;if(!data.active)$('cancel').textContent='Detener generación';$('audit').textContent=JSON.stringify(data.events.map(e=>({secuencia:e.seq,fecha:new Date(Number(e.utc_ms)).toISOString(),suceso:e.kind,sha256:e.sha256,peticion:e.data.request_id})),null,2);
}
async function refresh(){if(polling)return;polling=true;try{state=await api('state');connected=true;if(lastFailureConnection){clearError();lastFailureConnection=false;}$('reconnect').hidden=true;renderState();if(chatId)renderChat(await api('get_chat',{chat_id:chatId}));}catch(e){fail(e);$('connection').textContent='No se ha podido consultar el servicio';}finally{polling=false;}}
async function selectCase(id){caseId=id;chatId='';clearError();resetPreview();await refresh();}
async function newChat(){if(!caseId)return;const n=state.chats.filter(c=>c.case_id===caseId).length+1;const v=await api('create_chat',{case_id:caseId,title:'Conversación '+n});chatId=v.id;resetPreview();await refresh();$('question').focus();}
$('caseForm').onsubmit=async e=>{e.preventDefault();clearError();try{const v=await api('create_case',{title:$('caseTitle').value});caseId=v.id;$('caseTitle').value='';saveDraft();state=await api('state');await newChat();}catch(e){fail(e);}};
$('newChat').onclick=()=>newChat().catch(fail);$('chatSelect').onchange=async()=>{chatId=$('chatSelect').value;resetPreview();await refresh();};
async function preview(){if(!chatId)throw Error('Cree primero una conversación');const v=await api('preview',{chat_id:chatId,text:$('question').value,profile:profile()});$('contextText').textContent=v.context.prompt;$('budget').textContent=v.context.input_tokens+' de entrada + '+v.context.reserved_output+' reservadas / '+v.context.limit+' disponibles'+(v.fits?'':' · NO CABE');return v;}
$('preview').onclick=async()=>{clearError();try{await preview();$('contextDetails').open=true;}catch(e){fail(e);}};
$('composer').onsubmit=async e=>{e.preventDefault();if(busy)return;busy=true;clearError();$('send').disabled=true;try{const v=await preview();const req={chat_id:chatId,text:$('question').value,profile:profile(),request_id:crypto.randomUUID(),context_sha256:v.context.sha256};await api('send',req);$('question').value='';saveDraft();await refresh();}catch(e){fail(e);}finally{busy=false;controls();}};
$('cancel').onclick=async()=>{if(!current?.active)return;clearError();try{await api('cancel',{request_id:current.active.id});$('cancel').textContent='Cancelación solicitada';}catch(e){fail(e);}};
$('export').onclick=async()=>{clearError();try{const data=await api('export',{case_id:caseId});const url=URL.createObjectURL(new Blob([JSON.stringify(data,null,2)],{type:'application/json'}));const a=el('a');a.href=url;a.download=caseId+'.json';a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);}catch(e){fail(e);}};
$('thinking').onchange=()=>{$('profileNote').textContent=$('thinking').checked?'Perfil recomendado por Qwen: temperatura 0,6; Top‑P 0,95; Top‑K 20.':'Perfil recomendado por Qwen: temperatura 0,7; Top‑P 0,8; Top‑K 20.';};
restoreDraft();controls();refresh();setInterval(()=>{if(connected||document.visibilityState==='visible')refresh();},3000);
