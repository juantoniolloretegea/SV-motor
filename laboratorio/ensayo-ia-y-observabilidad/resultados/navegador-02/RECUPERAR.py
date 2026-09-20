"""Recuperación offline de archivos publicados como gzip/base64; no ejecutar el ensayo."""
import pathlib,base64,gzip,hashlib,json,sys
base=pathlib.Path(__file__).resolve().parent
dest=pathlib.Path(sys.argv[1]).resolve()
if dest.exists():raise SystemExit("Se exige un destino nuevo; no sobrescribir originales")
m=json.loads((base/"MANIFIESTO_EMISOR.json").read_text())
dest.mkdir(parents=True)
for x in m["archivos"]:
 if x["estado"]!="EMITIDO":raise SystemExit("Custodia incompleta: "+x["ruta"])
 rel=pathlib.PurePosixPath(x["ruta"])
 if rel.is_absolute() or ".." in rel.parts:raise SystemExit("Ruta no admitida")
 partes=sorted((base/"transporte").glob(x["ruta"].replace("/","__")+".*.b64"))
 z=base64.b64decode("".join(p.read_text().strip() for p in partes),validate=True)
 assert len(z)==x["gzip_bytes"] and hashlib.sha256(z).hexdigest()==x["gzip_sha256"]
 data=gzip.decompress(z)
 assert len(data)==x["bytes"] and hashlib.sha256(data).hexdigest()==x["sha256"]
 p=dest.joinpath(*rel.parts);p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(data)
 print(x["ruta"],len(data),x["sha256"],"CONFORME_IDENTIDAD")
