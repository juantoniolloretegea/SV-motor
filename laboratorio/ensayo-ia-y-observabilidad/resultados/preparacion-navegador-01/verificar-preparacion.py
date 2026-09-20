import pathlib,json,hashlib,ast,subprocess
base=pathlib.Path(".")
m=json.loads((base/"MANIFIESTO.json").read_text())
for x in m["archivos"]:
 p=base/x["ruta"];data=p.read_bytes()
 assert len(data)==x["bytes"] and hashlib.sha256(data).hexdigest()==x["sha256"],x["ruta"]
for p in base.glob("*.py"):ast.parse(p.read_text(),filename=str(p))
for p in base.glob("*.sh"):subprocess.run(["bash","-n",str(p)],check=True)
subprocess.run(["node","--check","controlador.mjs"],check=True)
print("PRECOMPROMISO_IDENTIDAD_Y_SINTAXIS_OK")
