const fs=require('fs'),assert=require('assert'),crypto=require('crypto');
const base=__dirname, input=JSON.parse(fs.readFileSync(base+'/COMPARACION_DOCUMENTAL_ENTRADAS.json','utf8'));
const raw=fs.readFileSync(base+'/verificacion/DOCUMENTAL_ORIGINAL.jsonl');
const results=raw.toString('utf8').trim().split('\n').map(JSON.parse);
assert.equal(results.length,4);assert.equal(new Set(results.map(x=>x.case)).size,4);
const equal=(a,b)=>JSON.stringify(Object.entries(a).sort())===JSON.stringify(Object.entries(b).sort());
const rows=results.map(r=>{
 const source=input.cases.find(x=>x.id===r.case);assert(source);
 const expected=['DOC01','DOC04'].includes(r.case)?{estado:'documentado',fuente:'OP-IMM-001-P10@1.0',cita:'Compara un recuento absoluto de neutrófilos válido con el intervalo de referencia aplicable.'}:{estado:'sin_respaldo',fuente:'',cita:''};
 const text=r.response.choices[0].text;let parsed;try{parsed=JSON.parse(text)}catch{}
 const accepted=r.response.choices[0].finish_reason==='stop'&&!!parsed&&equal(parsed,expected);
 assert.equal(accepted,r.accepted);assert(accepted);
 const old=source.qwen_result;let qwenStrict=false;try{qwenStrict=equal(JSON.parse(old.result.raw),expected)}catch{}
 assert.equal(qwenStrict,old.verification.accepted);
 return {id:r.case,pregunta:source.user,esperado:expected,gpt_oss:{respuesta:text,fin:r.response.choices[0].finish_reason,conforme:accepted,entrada_tokens:r.input_tokens,salida_tokens:r.response.usage.completion_tokens,segundos:r.seconds},qwen:{respuesta:old.result.raw,conforme:qwenStrict,entrada_tokens:old.input_tokens,salida_tokens:old.result.tokens,segundos:old.seconds},alcance:'Comparación de condiciones históricas distintas; sin atribución causal exclusiva al modelo.'};
});
const report={esquema:'EIO-EVALUACION-DOCUMENTAL-1',fecha_utc:new Date().toISOString(),original_publicado:'verificacion/0.2.2/evidencias/documental-01/RESULTADOS.jsonl',original_local_sha256:crypto.createHash('sha256').update(raw).digest('hex'),nota_huella:'Huella del archivo local recibido para análisis; la procedencia canónica se identifica por commit y ruta.',referencia_commit:'1b58b75',numero_condiciones:4,repeticiones_por_condicion:1,gpt_oss_conformes:rows.filter(r=>r.gpt_oss.conforme).length,qwen_conformes:rows.filter(r=>r.qwen.conforme).length,filas:rows,limites:['Pasaje OP-IMM-001-P10@1.0, no todo el universo de inmunología.','Objetos JSON, citas y límites de fuente; no decisión clínica.','Cambian modelo, cuantización, plantilla, temperatura y plazo; no ensayo causal apareado.','El cliente documental consulta el motor directamente; no recorre la API de conversación ni aporta sus tramos OpenTelemetry. El motor se muestrea por cgroup.']};
fs.writeFileSync(base+'/verificacion/EVALUACION_DOCUMENTAL.json',JSON.stringify(report,null,2)+'\n');
console.log(JSON.stringify({gpt_oss_conformes:report.gpt_oss_conformes,qwen_conformes:report.qwen_conformes,filas:rows.map(r=>({id:r.id,gpt:r.gpt_oss.respuesta,qwen:r.qwen.respuesta}))}));
