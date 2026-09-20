# EIO-JSON-01 · resultado y cierre

20/09/2026. Holmes / UE-LOCAL-CODEX-WINDOWS. **Ejecución satisfactoria del banco acotado, pendiente de revisión receptora.**

[Run35496525105](https://github.com/juantoniolloretegea/SV-motor/actions/runs/35496525105), número7/intento1, completed/success; job106040396115. Corte ejecutado **81fd7ea90c354e43edd222f9984c177e1fdfb7ab**, publicado y releído antes del único dispatch fase=json. Encargo privado057fee28af67a20a8b54ec8367350d192bae459e y paquete públicoa97214deaacf5e95b667bfd9f2a7b5f74e6b3e2c autorizados expresamente. Seis ejecuciones anteriores comprobadas terminadas; sin reintentos ni número8.

## Resultados separados

- Regresión original: **24/24**, retorno0; esperados sin modificar.
- Complemento: **35/35**, cero discrepancias, retorno0.
- Sensibilidad del comparador: **6/6**, incluida en los35; no seis capacidades adicionales del receptor.
- Fallos técnicos: ninguno en las cuatro fases; aprovisionamiento, compilación, regresión y complemento retornaron0.
- Inferencias nuevas: **0**. No pesos, tokenizador ni componente WASM adquiridos por este complemento.

Resumen literal del test:
```
{"conforme":true,"controles":35,"fallos":0,"inferencias_nuevas":0,"tipo":"EIO-JSON-01-resumen"}
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
Rust ejecuta un test de integración con35 controles internos; no se confunde 1test con1control. J01 aparece junto al prefijo del arnés de test en stdout; se conserva en el original y se separa sólo para CONTROLES.jsonl. Todos los35 registros individuales están presentes y concuerdan.

J01–J18 prueban el receptor tipado real, incluidos duplicados/escape, texto posterior, límites numéricos y Markdown. Los recorridos verifican textoUTF8 y orden de tokens/referencias, sin exigir igualdad de representaciones JSON ni normalizar Unicode. Las dos salidas históricas siguen rechazadas como ESTRUCTURA. A/B y B/A son admisibles bajo este contrato de cobertura y no acreditan un vector posicional SV. Las seis alteraciones sobre copias fueron detectadas.

## Identidad, instalación y compilación

Ocho archivos del paquete y once fuentes cotejados antes del lanzamiento (COTEJO_PREVIO.json); PAQUETE.sha256 identificado por blob/huella propia, sin autohuella circular. Diff desde EIO-06 sólo añade el paquete. El runner vuelve a cotejar paquete/fuentes antes y después y compara la copia temporal del banco.

Tres distribuciones nativas verificadas por tamaño y SHA256:
- rustc79745456bytes,0e37cb339f447fc44d6d781073bacacebfdc5612f2600e4c7e84c266f5f3aced.
- cargo11669560bytes,2f512d170d3dd23e16ababcda32ee2e6d5172d861a7af1f504e0b1e270cafab9.
- std30715556bytes,f5022e6c95a5ad23cca2513dc8281200f585fa188de6370aa37b128a43f876a3.

Versiones completas en salidas/aprovisionamiento-json.stdout.txt: rustc1.98.0(88d9e12ae2026-08-18),cargo1.98.0(797e8a9bc2026-08-05),hostx86_64-unknown-linux-gnu,LLVM22.1.8. Instalación sólo en prefijo efímero del runner. cargo fetch --locked recupera fuentes del lock; no inferencia habilitada. Compilación/pruebas --locked --offline --no-default-features con dos trabajos. Cargo.toml/lock, receptor, observabilidad, banco original e históricos no cambiados.

## Recursos y tiempos observados

GitHub registra inicio07:18:46UTC y actualización completed07:19:28UTC del20/09/2026 (09:19:28Madrid);42s entre marcas, no sello de mensaje humano. Runner Ubuntu24.04.5, imagen ubuntu-24.04/20260907.300.1.

30 muestras: disco máximo1648562176bytes, libre mínimo90769326080bytes, evidencia máxima28672bytes. RSS aprovisionamiento142564KiB, compilación692636KiB. Regresión/complemento sin muestras por brevedad: pico0 no acredita consumo nulo. Cotas20min/1140s,10GiB/reserva2GiB/20MiB conservadas. El corte4GiB de inferencia/navegador no se atribuye a estas fases. Muestreo nominal1s no garantiza máximos entre muestras.

Cuatro cierres informan residuales_al_wait=0 y residuales_tras_limpieza=0. Limpieza/finalización del job observada; no prueba de vigilancia independiente ni inventario exhaustivo del host.

## Custodia y limitación de integridad

JOB_106040396115.txt conserva íntegra la cadena recuperada del conector de logs, con timestamps/ANSI; no se afirma identidad del ZIP original. Salidas de las cuatro fases y medidas separadas en salidas/ como segmentos de ese log. Retornos en FASES.tsv, instrucciones exactas en log/controlador fijado.

MANIFIESTO.tsv fija tamaños UTF8 y SHA256 de los archivos documentales recuperados/derivados. Se releen y comparan tras publicar. **No hay huellas ni tamaños emitidos de los archivos efímeros de cada fase: no puede acreditarse por contraste independiente su identidad binaria con estos segmentos.** No se inventa esa evidencia. Stderr vacío observado en regresión y medidas vacías observadas en ambas pruebas se distinguen de ausencia de segmento. RECUPERACION.json documenta extracción y líneas. Retornos y recuentos sí están observados. La recepción decidirá la suficiencia de esta custodia; no se declara cumplido ese requisito binario independiente.

## Cierre y perímetro

Ambas guardas se cierran en este commit; workflow queda restringido a número7/intento1. Histórico conservado: campaña3/3, diagnóstico1/1,EIO-05 1/1,EIO-06 1/1,JSON-01 1/1. Ninguna repetición o continuación implícita.

Coordinación por conector GitHub y pestaña temporal, procesamiento documental/SHA256 en memoria. Consulta preliminar de rutas privadas produjo salida excesivamente amplia; no contenidos ajenos. Una consulta de directorio público no fue procesable; README directo disponible. Sin denegación efectiva de ejecución, instalaciones/archivos/compilaciones locales, inferencia nueva, credenciales nuevas, ampliación de hosts, caches de proyecto, upload-artifact o servicios residentes. Sin cambios canónicos/núcleo/S32/BIS-03/dominos/otros laboratorios. Sin cámara/micrófono/capturas del usuario/WSL/VSCode/almacenes de credenciales/X:/Z: ni comunicaciones a terceros.

El resultado sólo acredita estos controles y sensibilidad limitada. No resuelve el rechazo del modelo ni acredita veracidad semántica, ausencia de manipulación de origen, privacidad/H1, seguridad integral, identidad de vectores SV o ejecución en navegador. Pendientes de navegador, costes y estudio futuro de agentes conservan sus ámbitos. **Parada para revisión receptora.**
