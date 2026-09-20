# Diferencias NAT03 respecto de d82bbe2

Base de implementación: d82bbe2f5f396eb31da5b095ba0849404cb352b0, preparacion-nativa-02. Publicación sucesora en carpeta nueva preparacion-nativa-03, conciliada con main 057dba8a4a1fd2774e35f61e6135fd096679502a; se conserva el avance ajeno README 2.3.

No se encontró candidata/entrega NAT03 en los árboles remotos consultados ni en las dos rutas locales exactas comprobadas. No se efectuó inventario general del PC. Las 40 identidades de contenido de la base (sin autohuella de su manifiesto) concordaron con el manifiesto canónico leído por commit.

## Cambios funcionales acotados

| Archivos | Reparo / propósito |
|---|---|
| pruebas/api_real.rs, pruebas/oraculos_nat03.rs, pruebas/CASOS-NAT03.json | NAT02-A: caso exacto, sello exigido/prohibido, prefijo y error específicos; recuperación total acotada y con progreso |
| pruebas/frontera.rs, pruebas/lib.rs | NAT02-A: cuatro controles de sensibilidad nuevos y módulo compartido; once tests fuente en total, ninguno ejecutado |
| nativa/custodia.rs | NAT02-A: recibos de trabajos en journal existente; NAT02-B: conservar diagnóstico de error y preparar desconexión |
| nativa/supervisor.rs | NAT02-B: registro fallido de waitpid no abandona control; error de observación no afirma reap; diagnóstico volátil, invalidación y cierre explícito |
| nativa/mod.rs, nativa/testigo.rs | NAT02-B: caso escritor_desconectado; sin cambiar dependencias, protocolo de entrada ni modelos |
| README, CONTRATO, DISENO, MATRIZ, HABILITACION, ORACULOS | concordancia con cambios y límites; no resultado dinámico nuevo |
| FICHA_REMOTA_INACTIVA.md | presupuesto futuro, gasto cero por comprobar, compilación separada de pruebas peligrosas, contención y custodia |
| MANIFIESTO.json, este documento | identidades y trazabilidad de copia/derivación |

Los estados interrumpida por fallo de custodia son invalidación técnica; no afirman que se haya enviado señal a un hijo ya recogido. La API añade diagnóstico, sin interfaz nueva. No se amplía el diseño completo.

## Copias literales

- Cargo.toml
- antecedentes/Cargo.lock
- antecedentes/ORACULOS-NAT01.md
- configuracion/INACTIVA.json
- configuracion/codespaces-propuesta.json
- configuracion/guarda-INACTIVA.json
- inferencia/adaptador.rs
- nativa/cotejo.rs
- nativa/guarda.rs
- nativa/inferidor.rs
- nativa/parada.rs
- nativa/servidor.rs
- observabilidad/telemetria.rs
- pruebas/ENTRADAS_ENSAYO.json
- pruebas/INTERFAZ.js
- pruebas/banco.rs
- pruebas/casos.json
- pruebas/json_complementario.rs
- pruebas/observador.rs
- pruebas/peticion.txt
- pruebas/registro-futuro.sh
- resultados/continuacion-06/INFERENCIAS.jsonl
- resultados/revision-05/continuacion-06/peticion-estructurada.txt
- resultados/revision-06/json-01/CASOS.json
- web/app.js
- web/index.html

Cargo.toml, fuentes de inferencia/verificador/telemetría, entradas y resultados históricos, interfaz, guarda y escalada permanecen literales. El lock se conserva sólo en antecedentes; no existe lock resuelto para NAT03.

## Estado de verificación

Inspección estática y cálculo de tamaños/SHA-256 sobre UTF-8; comparación de copias con manifiesto base canónico, parseo de JSON y comprobación de referencias. No se ejecutaron fuentes ni bancos y no se invocó compilador, Cargo, servidor, workflow, Codespaces o inferidor. La tipificación Rust, disponibilidad del kernel, carreras y tiempos siguen pendientes.

Los archivos reutilizados son antecedentes o fuentes; los datos históricos no son resultados de NAT03. La recepción deberá decidir los dos reparos y cualquier habilitación posterior.
