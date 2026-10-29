# Entornos físicos, máquinas virtuales y reproducción

29 de octubre de 2026 · Revisión 1.0

## Estado y alcance

La preparación de Q19 corresponde al estudio de un canal temporal y su contención física en un ASUS VivoBook X515EA con Intel Core i7-1165G7. El cierre instrumental previo es acotado: no acredita tiempo efectivo completo por evento ni integridad continua de los contadores. El escenario físico aún debe recibir referencia funcional, controles, contención y conservación del trabajo legítimo.

La evidencia de este soporte no se generaliza al catálogo Intel, a AMD, ARM, Apple ni a otros procesadores. Se documentarán asimismo unidades de procesamiento gráfico (GPU), tensorial (TPU) y neuronal (NPU), y otros aceleradores cuando intervengan, distinguiendo capacidades declaradas, disponibles y efectivamente observadas. La ausencia de medición no significa ausencia de riesgo.

## Ficha obligatoria de cada entorno recibido

| Objeto | Datos que deben conservarse |
|---|---|
| Identidad | Identificador estable del entorno, función, propietario o responsable institucional, alcance autorizado y fechas. |
| Procesamiento | Fabricante, modelo, arquitectura, revisión, topología de núcleos e hilos; cachés, memoria, afinidad y recursos asignados. |
| Estado del sistema | Sistema operativo, núcleo, firmware y microcódigo observados; controladores y protecciones relevantes; diferencias respecto a la referencia. |
| Virtualización | Hipervisor y versión, máquina invitada, procesadores y memoria virtuales, red, discos, dispositivos y paso directo, si existe. |
| Material | Fuentes por revisión inmutable, compilador, dependencias fijadas, opciones, imagen y disco de referencia con tamaños y huellas. |
| Receta | Preparación, arranque, observación, ejecución, parada, retorno y comprobación de recuperación, con resultados esperados. |
| Medición | Eventos, unidades, reloj, muestreo, pérdidas, multiplexación, incertidumbre, vigencia y cobertura real. |
| Contención | Fronteras de red y almacenamiento, privilegios, destinos permitidos, límites de recursos y parada independiente de los procesos descendientes. |
| Evidencia | Referencia, controles negativos, variantes, trazas originales, recepción y restricciones de publicación. |

Estos campos son requisitos; no son valores ya medidos. Cada expediente enlazará una ficha concreta e inmutable y declarará lo que no se ha observado.

## Entorno virtual y firmware

Una máquina virtual puede servir para comprobar aplicaciones, permisos, restauración y arranque emulado cuando reproduzca la propiedad examinada. No demuestra por sí sola un efecto físico de caché del anfitrión ni el comportamiento del firmware real. La propuesta con el emulador QEMU y Open Virtual Machine Firmware (OVMF), basada en EFI Development Kit II (EDK II), permanece separada de los veinticinco parámetros y pendiente de una ejecución recibida.

No se modifica el firmware de este portátil. Las imágenes de prueba serán independientes y desechables; no se usarán discos físicos ni dispositivos de la instalación vigente por simple disponibilidad. El soporte de trabajo externo tampoco aísla memoria, procesador o caché.

## Qué puede afirmarse como reproducible

- **Documental:** revisión y artefactos disponibles, huellas verificables y método consultable.
- **Computacional:** fuentes y dependencias reconstruibles; corrector ejecutable sobre los originales conservados.
- **Experimental:** receta y entorno suficientes para repetir el fenómeno y sus controles dentro de límites declarados.

Cada nivel necesita su propia evidencia. Una ficha vacía, una imagen descargada o una máquina arrancada no acreditan el nivel siguiente. No se declara una máquina virtual de examen como disponible hasta comprobar su preparación, aislamiento, observación y recuperación.

© 2026 Juan Antonio Lloret Egea. Algunos derechos reservados. | ORCID: 0000-0002-6634-3351 | Instituto Tecnológico Virtual de la Inteligencia Artificial para el Español™ (ITVIA) | IA eñ™ – La Biblia de la IA™ | ISSN 2695-6411 | Licencia Creative Commons Atribución-NoComercial-SinDerivadas 4.0 Internacional (CC BY-NC-ND 4.0).
