import os,pathlib,hashlib,json,tomllib
r=pathlib.Path(os.environ["RUNNER_TEMP"])/"eio";e=r/"evidencia"
def identidad(p):return {"bytes":p.stat().st_size,"sha256":hashlib.file_digest(p.open("rb"),"sha256").hexdigest()}
rec=[{"ruta":"/"+str(p.relative_to(r/"web")),"archivo":str(p.relative_to(r/"web")),**identidad(p)} for p in sorted((r/"web").rglob("*")) if p.is_file()]
(r/"web/recursos.json").write_text(json.dumps(rec,indent=2)+"\n")
(e/"recursos.json").write_bytes((r/"web/recursos.json").read_bytes())
# Cotejo nuevamente antes de servir. Los pesos no se exportan como evidencia.
m=json.loads((pathlib.Path(os.environ["EIO_BASE"])/"pruebas/ENTRADAS_ENSAYO.json").read_text())
for x in m["entradas"]:
 y=identidad(r/"entradas"/x["archivo"])
 assert y=={"bytes":x["bytes"],"sha256":x["sha256"]}
herr=[]
for p in sorted((r/"cargo/registry/src").glob("*/wasm-bindgen-cli-0.2.104/Cargo.lock")):
 data=p.read_bytes();(e/"Cargo.herramienta.lock").write_bytes(data)
 herr=tomllib.loads(data.decode())["package"]
assert herr,"LOCK_INSTRUMENTAL_AUSENTE"
(e/"dependencias-herramienta.json").write_text(json.dumps(herr,indent=2)+"\n")
(e/"artefactos.json").write_text(json.dumps([
 {"ruta":str(p.relative_to(r)),**identidad(p)} for p in [r/"target/wasm32-unknown-unknown/release/eio_candidato.wasm",r/"bindgen/bin/wasm-bindgen"]],indent=2)+"\n")
