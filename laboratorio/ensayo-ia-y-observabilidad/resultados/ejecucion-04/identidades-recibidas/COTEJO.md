# Cotejo receptor de las identidades aportadas

Fecha de cierre documental: 19/09/2026.
Base del candidato: [ae3d7dfbcae493f22173a3a7b9071fd363c7f24a](https://github.com/juantoniolloretegea/SV-motor/commit/ae3d7dfbcae493f22173a3a7b9071fd363c7f24a).
Resultado: completados los dos pendientes documentales de identidad y el cotejo estático de plantilla. No constituye ejecución del ensayo ni aceptación científica.

## 1. Material recibido y método

La dirección aportó tres archivos tras solicitarse las rutas oficiales indicadas abajo. Se inspeccionaron los adjuntos disponibles en el entorno receptor, sin instalar ni extraer el paquete. Se midieron sus bytes con stat y se calculó SHA-256 con sha256sum. El [manifiesto](MANIFIESTO.tsv) conserva tamaños, huellas y resultados.

La comprobación `sha256sum --check --strict rust-std-1.98.0-wasm32-unknown-unknown.tar.xz.sha256`, con el paquete en el mismo directorio, terminó con código 0 y la salida:

```text
rust-std-1.98.0-wasm32-unknown-unknown.tar.xz: OK
```

| Archivo | Bytes | Resultado |
| --- | ---: | --- |
| Paquete estándar WASM | 22 441 312 | SHA-256 recalculado igual al fichero de suma aportado |
| Fichero de suma | 112 | Copia íntegra conservada |
| Configuración del tokenizador | 9 732 | JSON válido; identidad calculada sobre el adjunto original |

El cotejo acredita concordancia entre los bytes recibidos y la suma aportada. La procedencia de la descarga se atribuye a la aportación de la dirección; no se presenta como una descarga receptora independiente desde Rust o Hugging Face ni como validación de firma digital. El ejecutor deberá verificar su propia adquisición antes de instalar o consumir componentes.

Fuentes solicitadas:

- [Paquete Rust 1.98.0 para wasm32-unknown-unknown](https://static.rust-lang.org/dist/rust-std-1.98.0-wasm32-unknown-unknown.tar.xz).
- [Suma oficial prevista](https://static.rust-lang.org/dist/rust-std-1.98.0-wasm32-unknown-unknown.tar.xz.sha256).
- [Configuración Qwen en c1899de289a04d12100db370d81485cdf75e47ca](https://huggingface.co/Qwen/Qwen3-0.6B/raw/c1899de289a04d12100db370d81485cdf75e47ca/tokenizer_config.json).

La configuración y la suma se conservan sin reformatear. El paquete binario aportado no se incorpora a Git; la adquisición experimental sigue utilizando su URL oficial y la identidad fijada.

## 2. Cotejo estático de la plantilla

Se leyó chat_template del [archivo recibido](tokenizer_config.json) y se contrastó con [inferencia/adaptador.rs](https://github.com/juantoniolloretegea/SV-motor/blob/ae3d7dfbcae493f22173a3a7b9071fd363c7f24a/laboratorio/ensayo-ia-y-observabilidad/inferencia/adaptador.rs). No se ejecutó un intérprete Jinja, un modelo ni una nueva prueba compilada.

Condiciones exactas del cotejo: una lista con un único mensaje de rol user; contenido de tipo string; tools ausente o vacío; sin mensaje system; add_generation_prompt=true; enable_thinking=false.

Deducción por ramas:

1. La rama de tools no emite contenido y la alternativa system tampoco lo emite.
2. El recorrido inverso sólo calcula el índice de la última consulta; no añade texto.
3. Para el único mensaje user, la plantilla concatena el delimitador de comienzo, el rol, un salto LF, la petición, el delimitador de fin y otro LF.
4. add_generation_prompt añade el comienzo del mensaje assistant y LF.
5. enable_thinking=false añade el bloque de pensamiento vacío, con los mismos saltos LF que el literal Rust.

Resultado común, representado con escapes visibles; cada `\n` denota un LF y `{peticion}` denota el contenido insertado, no texto literal:

```text
<|im_start|>user\n{peticion}<|im_end|>\n<|im_start|>assistant\n<think>\n\n</think>\n\n
```

La concatenación coincide con el literal format! del adaptador bajo esas condiciones. La configuración también declara eos_token=<|im_end|>, con identificador 151645, distinto de pad_token=<|endoftext|>, identificador 151643; add_bos_token=false y add_prefix_space=false.

Conclusión: cotejo estático favorable para la especialización declarada. No se acredita equivalencia general para conversaciones múltiples, herramientas, mensajes system, pensamiento habilitado o contenido no textual. Tampoco se acredita todavía equivalencia de tokenización o comportamiento de generación. El adaptador no lee tokenizer_config.json durante la inferencia: es una referencia para su literal especializado.

## 3. Integración de la evidencia

Se completan bytes, SHA-256 y plantilla_cotejada en pruebas/ENTRADAS_ENSAYO.json y se enlaza la copia custodiada. No se cambian pesos, revisiones, dependencias, configuración de generación, código Rust o esperados.

La guarda global ensayo_habilitado no se modifica en esta recepción. Su apertura corresponde al precompromiso experimental completo bajo la continuación ya autorizada. Los dos requisitos documentales aquí tratados dejan de ser pendientes; no se exige repetir su obtención local.

El contador contrastado antes de esta publicación es 2/3. No se lanzó Actions, no hubo instalación, compilación o inferencia y no se modificó S32/BIS-03. Los resultados de funcionamiento, costes, seguridad activa ejercitada y navegador siguen pendientes de sus pruebas.

## 4. Procedencia y licencia de la copia de configuración

Archivo procedente de Qwen/Qwen3-0.6B, revisión indicada; se conserva sin modificaciones. La distribución del modelo declara Apache-2.0. Se acompaña una copia del [texto de Apache License 2.0](LICENSE-APACHE-2.0.txt), sin atribuir esa licencia al SV. El texto de licencia se obtuvo del fichero estándar disponible en el entorno receptor; no se presenta como un archivo descargado de Qwen. La fuente original permanece enlazada para sus avisos y condiciones.
