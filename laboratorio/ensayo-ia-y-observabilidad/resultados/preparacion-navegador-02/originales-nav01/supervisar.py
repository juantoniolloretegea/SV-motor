"""Supervisor del runner: plazos, disco y familia observada; muestreo de 1 s."""
import os,sys,time,json,signal,subprocess,pathlib,shutil,traceback
root=pathlib.Path(os.environ["RUNNER_TEMP"])/"eio"
ev=root/"evidencia";ev.mkdir(parents=True,exist_ok=True)
fase,seg,*orden=sys.argv[1:]; limite=min(int(seg),int(os.environ["EIO_FIN"])-time.time()-5)
def guardar(p,obj):p.write_text(json.dumps(obj,ensure_ascii=False,indent=2)+"\n")
def proc():
 r={}
 for x in pathlib.Path("/proc").iterdir():
  if not x.name.isdigit():continue
  try:
   s=(x/"stat").read_text();a=s[s.rfind(")")+2:].split()
   r[int(x.name)]={"ppid":int(a[1]),"pgrp":int(a[2]),"sesion":int(a[3]),"inicio":a[19],"estado":a[0],"rss":int(a[21])*os.sysconf("SC_PAGE_SIZE")}
  except (OSError,ValueError,IndexError):pass
 return r
if limite<=0:sys.exit(93)
inicio=time.time();mono=time.monotonic();vigilados={};medidas=[];causa="normal";p=None
ruta=ev/fase
try:
 with open(str(ruta)+".stdout","wb") as out,open(str(ruta)+".stderr","wb") as err:
  p=subprocess.Popen(orden,stdout=out,stderr=err,start_new_session=True)
  while True:
   tabla=proc()
   cambio=True
   while cambio:
    cambio=False
    for pid,v in tabla.items():
     if pid==p.pid or v["pgrp"]==p.pid or v["sesion"]==p.pid or (v["ppid"] in vigilados and tabla.get(v["ppid"],{}).get("inicio")==vigilados[v["ppid"]]):
      if pid not in vigilados:vigilados[pid]=v["inicio"];cambio=True
   vivos={pid:v for pid,v in tabla.items() if vigilados.get(pid)==v["inicio"] and v["estado"]!="Z"}
   rss=sum(v["rss"] for v in vivos.values())
   usado=int(subprocess.check_output(["du","-s","-B1",str(root)],text=True,timeout=5).splitlines()[0].split()[0])
   # Checkout se mide tambien; sin sumar dos veces subcarpetas.
   usado+=int(subprocess.check_output(["du","-s","-B1",os.environ["GITHUB_WORKSPACE"]],text=True,timeout=5).split()[0])
   libre=shutil.disk_usage(root).free
   evidencia=sum(x.stat().st_size for x in ev.rglob("*") if x.is_file())
   medidas.append({"mono_s":time.monotonic()-mono,"civil_unix":time.time(),"rss_bytes":rss,"disco_bytes":usado,"libre_bytes":libre,"evidencia_bytes":evidencia,"pids":list(vivos)})
   if time.monotonic()-mono>=limite:causa="tiempo"
   if usado>10737418240 or libre<2147483648 or evidencia>20971520:causa="disco"
   if fase=="navegador" and rss>4294967296:causa="memoria"
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
 tabla=proc()
 residuales=[pid for pid,v in tabla.items() if vigilados.get(pid)==v["inicio"] and v["estado"]!="Z"]
 resultado={"fase":fase,"orden":orden,"inicio_unix":inicio,"fin_unix":time.time(),"segundos":time.monotonic()-mono,"limite_s":limite,"retorno":rc,"causa":causa,"pico_rss_bytes":max([x["rss_bytes"] for x in medidas]or[0]),"residuales_observados":residuales,"intervalo_nominal_s":1,"perimetro":"grupo/sesion y descendientes observados; PID+inicio; excluye procesos fugados antes de observarlos y otros procesos del host"}
 guardar(pathlib.Path(str(ruta)+".fase.json"),resultado)
 guardar(pathlib.Path(str(ruta)+".medidas.json"),medidas)
 print(json.dumps(resultado),flush=True)
sys.exit(rc if causa=="normal" and not residuales and rc>=0 else 92)
