# Recepción técnica del servicio documental MCP

Esta instrucción sustituye el encargo anterior de programación desde cero. El paquete contiene la realización y las pruebas. El encargo queda limitado a recepción, compilación y comprobación en OneCloud, sin inferencias.

1. Confirmar la identidad de la instancia conservada 864618 y que el motor permanece detenido. No crear instancias, ampliar recursos, utilizar Codespaces, copiar pesos ni modificar el motor, núcleo o evidencias anteriores.
2. Transferir este paquete desde el PC a un directorio nuevo de OneCloud. Antes de ejecutarlo, verificar la huella externa del archivo facilitada con la entrega; extraer y comprobar SHA256SUMS. No sobrescribir una recepción anterior.
3. Leer LEAME.md y COMPATIBILIDAD.md. Comprobar por inspección, sin carga de pesos, si el ejecutable instalado de mistral.rs expone el mecanismo MCP descrito. Conservar ayuda, versión y huella. Si el contraste falla, informar; no actualizar o adaptar el motor.
4. Ejecutar `bash VERIFICAR.sh` con Rust 1.98.0 ya instalado, bajo un supervisor que limite el conjunto de la compilación a 8 GiB y cuatro trabajos simultáneos. El script recompila con `--locked`, conserva registros y ejecuta únicamente el cliente técnico. No exige arrancar mistral.rs.
5. Tiempo máximo de recepción: 30 minutos desde el inicio. Reservar los últimos cinco para recopilar la entrega. Si faltan dependencias o se excede un límite, detener, conservar registros e informar. No ampliar recursos ni cambiar Cargo.lock para forzar el resultado.
6. Si aparece un error del código entregado, conservar diagnóstico y devolverlo para corrección. No introducir modificaciones extensas ni cambiar la hipótesis experimental. Las rutas absolutas y nombres de diarios pueden adaptarse, dejando constancia.
7. Dejar preparada una copia de la plantilla MCP con rutas absolutas y un diario nuevo, pero NO aplicarla al servicio de inferencia. Para el futuro, el proceso documental deberá ejecutarse sin privilegios, con catálogo de solo lectura y sin acceso al núcleo o secretos. No convertir este encargo en un despliegue permanente.
8. Entregar en el repositorio privado `SV-sala-de-maquinas`, rama `main`, dentro de `respuestas-ejecucion/MCP-DOCUMENTAL-PREPARACION-20260926/entrega-01/`. Si existe, crear la siguiente entrega numerada. Incluir este paquete, instrucciones, resultados de recepción, huellas y enlace a un commit. No publicar credenciales, claves ni datos personales. Vincular el antecedente HCL-02 sin cambiar su dictamen. No crear parte TT ni aceptación definitiva de calidad antes de la recepción humana.
9. Informar de tiempo, coste estimado y estado contractual. Cerrar cliente, servicio documental temporal y supervisor. Mantener el motor apagado, la instancia conservada y los archivos intactos. La instancia continúa facturándose.

Resultado esperado: recepción técnica conforme o impedimento documentado. La integración ejecutada con el cliente real de mistral.rs y el uso de las herramientas por Qwen seguirán pendientes.

PARADA OBLIGATORIA: no cargar el modelo, no repetir HCL-02, no ejecutar otro caso y no iniciar el paso 2. Entregar y esperar revisión.

Antecedente:
https://github.com/juantoniolloretegea/SV-sala-de-maquinas/blob/ffed6b40339c4ac19c8da18a1ceaf75c1ce78578/respuestas-ejecucion/QWEN38-HCL01-DIAGNOSTICO-20260926/continuacion-directa-20260926/INFORME.md
