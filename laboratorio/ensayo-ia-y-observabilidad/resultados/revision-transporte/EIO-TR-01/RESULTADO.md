# EIO-TR-01 · resultado diagnóstico

20/09/2026 Europe/Madrid. [Run35475928967](https://github.com/juantoniolloretegea/SV-motor/actions/runs/35475928967), workflow_id361319025, número4 intento1, completed/failure. Job105985059716. SHA solicitado/efectivo fb963fbb8ac2a94c3066e9f81072ded4087d3f9e; precompromiso publicado y releído antes del dispatch.

## Observación nueva

GET único al origen huggingface.co. HTTP302, retorno de transporte0. Host de redirección saneado: **us.aws.cdn.hf.co**. Rechazo HOST_FUERA_DE_LISTA, política hf, salto1, código65, seguimiento0. El diagnóstico obtuvo el dato buscado aunque GitHub marque failure; no se presenta como ensayo experimental satisfactorio.

[Salida literal con marcas temporales](DIAGNOSTICO.txt). Se conserva exclusivamente la salida diagnóstica saneada y el retorno del paso, extraídos del log del trabajo mediante conector. No se publica el log completo, Location, rutas de destino, consultas firmadas, cabeceras crudas ni cuerpos. No se observó otro diagnóstico de error del programa en el log; stdout/stderr no se obtuvieron como canales independientes.

Inicio run19/09/2026 23:22:05UTC; actualización final23:22:14UTC (20/09 01:22:05–01:22:14 Madrid):9s según metadatos, dentro de120s. La petición registrada comienza23:22:11.7101465UTC y comunica HTTP a23:22:11.8549101UTC; son marcas de log, no instrumentación independiente del tiempo de red.

## Alcance y límites

Plantilla activa idéntica a la preparada: timeout2min; conexión10s/solicitud30s; cuerpo64KiB/archivo128KiB; una llamada curl sin seguimiento/retry. Guardas de adquisición/ensayo false verificadas por el paso. El código se detuvo antes de cualquier petición al host observado. Sin descarga de pesos, tokenizador o Rust; sin compilación, inferencia, caché o upload-artifact. No se midieron tráfico TLS total o tamaños efectivos de cabecera/cuerpo, ni se equiparan a límites configurados.

Esta respuesta nueva no reconstruye el host perdido del run35474691239. No demuestra legitimidad del host ni autoriza incluirlo en la lista. No hubo visita, consulta o adquisición posterior a ese destino. Lista sin cambios, ninguna quinta ejecución ni reintento. Cuenta histórica experimental3/3 conservada; presupuesto extraordinario diagnóstico1/1 consumido, total4.

Metadatos: identidad/presupuesto, checkout y paso de custodia satisfactorios; diagnóstico failure65; limpieza y Complete job satisfactorios. GitHub registra Cleaning up orphan processes; el guion configura limpieza de su temporal con trap. No se aporta inventario independiente de procesos remotos ni se inventa su cobertura. Trabajo terminado, sin ejecución propia persistente conocida.

## Recepción

Se entrega para valorar la legitimidad del host y cualquier eventual cambio concreto de política antes de otra adquisición. Nada de ello se realiza aquí. Sin aceptación científica, promoción, cambio S32/BIS-03 o modificación de antecedentes. Las guardas siguen cerradas; el workflow exige exactamente4/intento1, de modo que no habilita una quinta ejecución ni Re-run.
