// Enlace candidato no ejecutado. Sólo recursos del mismo origen en bucle local.
// pkg se generaría con wasm-bindgen-cli =0.2.104: no incluido ni instalado.
import init, { caso } from "./pkg/eio_candidato.js";
export async function ejecutar() {
  const url = new URL(location.href);
  if (!["127.0.0.1", "[::1]"].includes(url.hostname)) throw new Error("Sólo bucle local");
  const obtener = async (ruta, max) => {
    const r = await fetch(ruta, {redirect:"error", credentials:"omit"});
    if (!r.ok) throw new Error("HTTP "+r.status);
    const n = Number(r.headers.get("content-length"));
    if (!Number.isFinite(n) || n<=0 || n>max) throw new Error("Tamaño no acreditado");
    const bytes = new Uint8Array(await r.arrayBuffer());
    if (bytes.length!==n) throw new Error("Tamaño distinto");
    return bytes;
  };
  await init();
  const pesos=await obtener("../descargas/Qwen3-0.6B-Q4_K_M.gguf",500000000);
  const tokenizer=await obtener("../descargas/tokenizer.json",33554432);
  const peticion=new TextDecoder("utf-8",{fatal:true}).decode(await obtener("./peticion.txt",8192));
  return caso(pesos,tokenizer,peticion);
}
// El supervisor exterior debe poder terminar TODO el navegador; no usar setTimeout
// como supuesto mecanismo de interrupción de una llamada WASM síncrona.
