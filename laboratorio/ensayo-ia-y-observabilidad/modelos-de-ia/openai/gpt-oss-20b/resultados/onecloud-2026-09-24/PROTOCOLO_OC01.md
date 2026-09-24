# Protocolo previo OC-01 · gpt-oss-20b en OneCloud

Fecha: 2026-09-24. Unidad W-S39-02. S39 / TT-0012. Autorización humana expresa de continuar las pruebas después de la conciliación documental. Antecedente: recuperación Motor a74632b0b6dde70629863113134082dc3f31d521; Lenguaje c68020992d19b041574992355de023961f6713d6. Investigación lateral EIO, no integración del núcleo ni promoción productiva.

## Pregunta y criterio previo

¿El motor candidato con la corrección de difusión CPU MXFP4 obtiene una respuesta aritmética coherente en el nuevo anfitrión, utilizando la misma entrada de completions que el intento 24?

Aceptación semántica del caso: respuesta explícita de cinco elementos (5 o cinco) sin una cifra contradictoria, referida a tres elementos más dos. La valoración se realizará sobre el texto íntegro conservado, sin sustituirlo por un resumen del agente. La aceptación HTTP, la generación y la corrección semántica se evalúan por separado. Una salida limitada a 16 tokens que no complete una respuesta se registra como tal; no se amplía su presupuesto retrospectivamente.

## Identidades

- mistral.rs 0.9.3, fuente 24dbf5c256f232176ee5949485ba264049407fbe más CORRECCION_MXFP4_CPU.patch publicado en el antecedente.
- Candidata: SHA-256 2d6856918349d85a073fea59190c59886e780a62bcd94c6d7789060d04e99fb1.
- Controlador Rust 0.1.24: SHA-256 dfc8afe8ce7c76019fc35687c481927b429ea731e69058444b86a6c151c6554f.
- GGUF: ggml-org/gpt-oss-20b-GGUF@b97cbb20d1995efd41dce8c4dd1ddf86e8db375b, 12 109 566 624 B, SHA-256 27cd6c432c7672cb812a92f611cf3ba7bbc35928262bb1e1253ff4ee6ae35901.
- Tokenizador y Harmony recuperados: se conservará manifiesto de hashes y cotejo origen/destino antes de inferencia.
- Entorno: Ubuntu 26.04, EPYC7502/KVM, 12 CPU virtuales, 64 GB nominales; inventario exacto conservado en la recepción previa. Sin pretensión de equivalencia con el anfitrión anterior.

## Configuración y presupuesto

Se reutiliza el controlador y su petición literal: inventario de tres elementos más dos; Harmony con canal final abierto; /v1/completions; temperatura 0; max_tokens 16; logprobs 1; stream false; echo_prompt true. CPU, dtype bf16, 24 capas en CPU, contexto 1024, una secuencia, caché de prefijos 0 y paged attention off.

Un intento; ventana 540 s, carga 180 s, petición 300 s, límite virtual por proceso 32768 MiB y reserva de entorno 512 MiB. Mínimo de carga histórico 13 300 137 984 B. La ejecución conserva guardas internas y se envuelve con límite exterior temporal 570 s y parada de hasta 15 s, límite agregado de memoria del servicio 32 GiB, swap 0 y grupo de procesos limitado. La configuración efectiva de systemd y cgroup se conservará; declararla no equivale a comprobar su imposición.

El servicio usa red privada con bucle local y no expone la inferencia al exterior. Los pesos y tokenizador se cargan localmente; no se habilitan herramientas ni ejecución de código generado por el modelo. Las pruebas instrumentales se compilan con Rust 1.98.0 y se ejecutan en el anfitrión nuevo antes de la inferencia.

## Observación y cierre

Conservar protocolo previo, manifestaciones de identidad, compilador, comprobaciones de guardas, propiedades efectivas del servicio, SUCESOS.jsonl completo, motor.log, salida del controlador, contadores y estado final. Comprobar terminación del servicio y ausencia de sus procesos. Los límites del muestreo interno y de la lectura del cgroup se declaran por separado; no se anuncian garantías generales.

Si la candidata produce una respuesta correcta, el siguiente contraste para atribución causal debe ejecutarse en este mismo anfitrión contra el motor sin parche, con configuración comparable e identidades explícitas. Si falla, se conserva el intento y se estudia antes de modificar una variable. Cualquier segunda prueba tendrá su motivo, configuración y número propios. Este protocolo no predeclara éxito ni cierra TT-0012.
