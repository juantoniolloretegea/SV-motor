"""Supervisor exterior NAV02: criterio RSS previo; complemento asíncrono."""
import os,sys,time,json,signal,subprocess,pathlib,shutil,traceback,threading,queue
root=pathlib.Path(os.environ["RUNNER_TEMP"])/"eio"
ev=root/"evidencia";ev.mkdir(parents=True,exist_ok=True)
fase,seg,*orden=sys.argv[1:];limite=min(int(seg),int(os.environ["EIO_FIN"])-time.time()-5)
def guardar(p,obj):
 q=pathlib.Path(str(p)+".tmp")
 with q.open("w") as h:
  h.write(json.dumps(obj,ensure_ascii=False,indent=2)+"\n");h.flush();os.fsync(h.fileno())
 q.replace(p)
def linea(p,obj):
 data=(json.dumps(obj,ensure_ascii=False,separators=(",",":"))+"\n").encode()
 if p.exists() and p.stat().st_size+len(data)>4194304:raise RuntimeError("COTA_JOURNAL:"+p.name)
 with p.open("ab",buffering=0) as h:h.write(data);os.fsync(h.fileno())
def stat(pid):
 s=(pathlib.Path("/proc")/str(pid)/"stat").read_text();a=s[s.rfind(")")+2:].split()
 return {"ppid":int(a[1]),"pgrp":int(a[2]),"sesion":int(a[3]),"inicio":a[19],"estado":a[0],"rss":int(a[21])*os.sysconf("SC_PAGE_SIZE"),"comm":s[s.find("(")+1:s.rfind(")")]}
errores_proc=[]
def proc():
 r={}
 for x in pathlib.Path("/proc").iterdir():
  if not x.name.isdigit():continue
  try:r[int(x.name)]=stat(int(x.name))
  except (OSError,ValueError,IndexError) as e:
   if len(errores_proc)<2048:errores_proc.append({"pid":int(x.name),"error":str(e),"civil_unix":time.time()})
 return r
def complemento(pid,v):
 t=time.monotonic();res={"pid":pid,"inicio":v["inicio"],"inicio_lectura_mono":t,"civil_unix":time.time(),"pss_bytes":None,"private_clean_bytes":None,"private_dirty_bytes":None,"rol":"desconocido"}
 try:
  if stat(pid)["inicio"]!=v["inicio"]:raise RuntimeError("PID_REUTILIZADO")
  cmd=(pathlib.Path("/proc")/str(pid)/"cmdline").read_bytes().split(b"\0")
  tipo=next((x.decode(errors="replace") for x in cmd if x.startswith(b"--type=")),None)
  res["rol"]=tipo or ("controlador-node" if v["comm"]=="node" else v["comm"])
  campos={}
  for l in (pathlib.Path("/proc")/str(pid)/"smaps_rollup").read_text().splitlines():
   a=l.split()
   if a and a[0] in ("Pss:","Private_Clean:","Private_Dirty:"):
    if len(a)!=3 or a[2]!="kB":raise RuntimeError("UNIDAD_SMAPS")
    campos[a[0]]=int(a[1])*1024
  for clave,campo in (("pss_bytes","Pss:"),("private_clean_bytes","Private_Clean:"),("private_dirty_bytes","Private_Dirty:")):
   if campo not in campos:raise RuntimeError("CAMPO_AUSENTE:"+campo)
   res[clave]=campos[campo]
  if stat(pid)["inicio"]!=v["inicio"]:raise RuntimeError("IDENTIDAD_POST_LECTURA")
 except Exception as e:
  res.update({"error":str(e),"pss_bytes":None,"private_clean_bytes":None,"private_dirty_bytes":None})
 res["fin_lectura_mono"]=time.monotonic();return res
q=queue.Queue(maxsize=1);detener=threading.Event();complemento_error=[];saltos=0
def observar():
 try:
  while not detener.is_set() or not q.empty():
   try:seq,vivos=q.get(timeout=.1)
   except queue.Empty:continue
   t=time.monotonic();cpu=time.thread_time()
   datos=[complemento(pid,v) for pid,v in vivos.items()]
   propio=stat(os.getpid())
   linea(ev/"navegador.complemento.jsonl",{"muestra_rss":seq,"inicio_mono":t,"fin_mono":time.monotonic(),"cpu_hilo_s":time.thread_time()-cpu,"procesos":datos,"supervisor_exterior":complemento(os.getpid(),propio),"supervisor_rss_bytes":propio["rss"],"incluido_umbral":False,"perimetro":"hilo en supervisor exterior; sólo observa, no ejecuta trabajo del modelo"})
 except Exception:complemento_error.append(traceback.format_exc())
if limite<=0:sys.exit(93)
inicio=time.time();mono=time.monotonic();vigilados={};medidas=[];causa="normal";p=None;hilo=None
ruta=ev/fase;ultimo=None
if fase=="navegador":
 for nombre in ["navegador.muestras.jsonl","navegador.complemento.jsonl"]:linea(ev/nombre,{"tipo":"apertura","civil_unix":time.time()})
 hilo=threading.Thread(target=observar,daemon=True);hilo.start()
try:
 with open(str(ruta)+".stdout","wb") as out,open(str(ruta)+".stderr","wb") as err:
  p=subprocess.Popen(orden,stdout=out,stderr=err,start_new_session=True)
  while True:
   ciclo=time.monotonic();cpu=time.thread_time();tabla=proc()
   cambio=True
   while cambio:
    cambio=False
    for pid,v in tabla.items():
     if pid==p.pid or v["pgrp"]==p.pid or v["sesion"]==p.pid or (v["ppid"] in vigilados and tabla.get(v["ppid"],{}).get("inicio")==vigilados[v["ppid"]]):
      if pid not in vigilados:vigilados[pid]=v["inicio"];cambio=True
   vivos={pid:v for pid,v in tabla.items() if vigilados.get(pid)==v["inicio"] and v["estado"]!="Z"}
   faltantes=[pid for pid in vigilados if pid not in tabla]
   rss=sum(v["rss"] for v in vivos.values())
   usado=int(subprocess.check_output(["du","-s","-B1",str(root)],text=True,timeout=5).splitlines()[0].split()[0])
   usado+=int(subprocess.check_output(["du","-s","-B1",os.environ["GITHUB_WORKSPACE"]],text=True,timeout=5).split()[0])
   libre=shutil.disk_usage(root).free
   evidencia=sum(x.stat().st_size for x in ev.rglob("*") if x.is_file())
   medida={"secuencia":len(medidas)+1,"mono_s":time.monotonic()-mono,"mono_absoluto":time.monotonic(),"civil_unix":time.time(),"rss_bytes":rss,"disco_bytes":usado,"libre_bytes":libre,"evidencia_bytes":evidencia,"pids":list(vivos)}
   medidas.append(medida)
   if time.monotonic()-mono>=limite:causa="tiempo"
   if usado>10737418240 or libre<2147483648 or evidencia>20971520:causa="disco"
   if fase=="navegador" and rss>4294967296:causa="memoria"
   if fase=="navegador" and (not vivos and p.poll() is None):causa="rss-no-disponible"
   if fase=="navegador" and any((pathlib.Path("/proc")/str(pid)).exists() for pid in faltantes):causa="rss-no-disponible"
   # Decisión y señal antes de mediciones complementarias o persistencia nueva.
   if causa!="normal":
    for pid,v in vivos.items():
     try:os.kill(pid,signal.SIGTERM)
     except ProcessLookupError:pass
   if fase=="navegador":
    medida.update({"procesos":[{"pid":pid,**v} for pid,v in vivos.items()],"desaparecidos":faltantes,"errores_proc":errores_proc[:],"intervalo_efectivo_s":None if ultimo is None else ciclo-ultimo,"observador_inicio_mono":ciclo,"observador_fin_mono":time.monotonic(),"cpu_hilo_supervisor_s":time.thread_time()-cpu,"causa_decidida":causa,"supervisor_pid":os.getpid(),"supervisor_rss_bytes":stat(os.getpid())["rss"]})
    errores_proc.clear();ultimo=ciclo
    linea(ev/"navegador.muestras.jsonl",medida)
    guardar(ev/"supervisor-activo.json",{"civil_unix":time.time(),"pid":os.getpid(),"familia_pid":p.pid,"rss_disponible":bool(vivos),"causa":causa,"secuencia":medida["secuencia"],"captura_muestras":True,"hilo_complementario_vivo":hilo.is_alive()})
    try:q.put_nowait((medida["secuencia"],vivos))
    except queue.Full:saltos+=1
   medida["coste_ciclo_completo_s"]=time.monotonic()-ciclo
   if causa!="normal" or p.poll() is not None:break
   time.sleep(1)
except Exception:
 causa="supervisor"
 pathlib.Path(str(ruta)+".supervisor-error.txt").write_text(traceback.format_exc())
finally:
 if p:
  for sig in (signal.SIGTERM,signal.SIGKILL):
   tabla=proc()
   for pid,v in tabla.items():
    if (vigilados.get(pid)==v["inicio"] or v["pgrp"]==p.pid or v["sesion"]==p.pid) and v["estado"]!="Z":
     try:os.kill(pid,sig)
     except ProcessLookupError:pass
   if sig==signal.SIGTERM:time.sleep(1)
  try:rc=p.wait(timeout=3)
  except subprocess.TimeoutExpired:rc=94
 else:rc=95
 detener.set()
 if hilo:hilo.join(timeout=2)
 tabla=proc();residuales=[pid for pid,v in tabla.items() if vigilados.get(pid)==v["inicio"] and v["estado"]!="Z"]
 resultado={"fase":fase,"orden":orden,"inicio_unix":inicio,"fin_unix":time.time(),"segundos":time.monotonic()-mono,"limite_s":limite,"retorno":rc,"causa":causa,"pico_rss_bytes":max([x["rss_bytes"] for x in medidas]or[0]),"residuales_observados":residuales,"intervalo_nominal_s":1,"perimetro":"grupo/sesion y descendientes observados; PID+inicio; excluye fugados antes de observarlos y otros procesos del host"}
 guardar(pathlib.Path(str(ruta)+".fase.json"),resultado)
 guardar(pathlib.Path(str(ruta)+".medidas.json"),medidas)
 if fase=="navegador":
  journals={}
  for nombre in ["accesos.jsonl","excepciones.jsonl","marcas.jsonl","eventos.jsonl","navegador.muestras.jsonl","navegador.complemento.jsonl"]:
   ar=ev/nombre;seq=[];errores=[]
   if ar.exists():
    for n,l in enumerate(ar.read_bytes().splitlines(keepends=True),1):
     try:
      if not l.endswith(b"\n"):raise ValueError("LINEA_INCOMPLETA")
      obj=json.loads(l);seq.append(obj.get("secuencia",obj.get("secuencia_escritura")))
     except Exception as e:errores.append({"linea":n,"error":str(e)})
   else:errores.append({"error":"ARCHIVO_AUSENTE"})
   journals[nombre]={"lineas_completas":len(seq),"ultima_secuencia":seq[-1] if seq else None,"errores":errores}
  guardar(ev/"atestacion-supervisor.json",{"resultado":resultado,"journals":journals,"cierre_normal_controlador_presente":(ev/"cierre-controlador.json").exists(),"captura":"incremental hasta última línea; emisiones no recibidas y señales en tránsito desconocidas","complementos_omitidos_cola_llena":saltos,"errores_complemento":complemento_error,"observador_hilo_pendiente":bool(hilo and hilo.is_alive()),"supervisor_pid":os.getpid(),"supervisor_rss_final":stat(os.getpid())["rss"]})
 print(json.dumps(resultado),flush=True)
sys.exit(rc if causa=="normal" and not residuales and rc>=0 else 92)
