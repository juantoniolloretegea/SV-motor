"""Emite originales gzip/base64, con identidades calculadas antes del transporte."""
import pathlib,os,json,hashlib,gzip,base64
r=pathlib.Path(os.environ["RUNNER_TEMP"])/"eio";ev=r/"evidencia"
rutas=list(sorted(ev.rglob("*")))
if (r/"web").exists():rutas+=list(sorted((r/"web").rglob("*")))
archivos=[];segmentos=[];acumulado=0;limite=20971520
for p in rutas:
 if not p.is_file() or p.is_symlink():continue
 if p.stat().st_size>limite:
  archivos.append({"ruta":str(p.relative_to(r)),"bytes":p.stat().st_size,"sha256":hashlib.file_digest(p.open("rb"),"sha256").hexdigest(),"estado":"EXCEDE_COTA_ORIGINAL"});continue
 data=p.read_bytes();z=gzip.compress(data,mtime=0);b=base64.b64encode(z).decode()
 ident={"ruta":str(p.relative_to(r)),"bytes":len(data),"sha256":hashlib.sha256(data).hexdigest(),"gzip_bytes":len(z),"gzip_sha256":hashlib.sha256(z).hexdigest(),"codificacion":"gzip+base64","segmentos":(len(b)+11999)//12000}
 if acumulado+len(b)+8192>limite:
  ident["estado"]="EXCEDE_COTA_TRANSPORTE"
 else:
  ident["estado"]="EMITIDO";acumulado+=len(b)+8192
  for n in range(0,len(b),12000):segmentos.append({"ruta":ident["ruta"],"n":n//12000,"base64":b[n:n+12000]})
 archivos.append(ident)
manifest={"formato":"EIO-CUSTODIA-01","archivos":archivos,"completa":all(x["estado"]=="EMITIDO" for x in archivos),"cota_texto_bytes":limite,"excluidos":"pesos y tokenizer: identidades en recursos.json, transporte expresamente excluido; ejecutable instrumental solo identidad","ambito":"mismo ejecutor; no independencia ante host malicioso"}
linea="EIO_MANIFIESTO "+json.dumps(manifest,separators=(",",":"))
salida=[linea]+["EIO_SEGMENTO "+json.dumps(x,separators=(",",":")) for x in segmentos]+["EIO_FIN_CUSTODIA"]
if sum(len(x.encode())+1 for x in salida)>limite:
 print("EIO_CUSTODIA_INCOMPLETA LIMITE_TEXTO")
 print(linea)
else:
 for x in salida:print(x,flush=True)
