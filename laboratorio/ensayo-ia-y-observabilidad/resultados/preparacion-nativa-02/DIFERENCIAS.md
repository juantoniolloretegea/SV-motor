# Diferencias frente a fed3892

Corte exacto antecedente: fed3892c31aaec6c8d7721fcb06c72245b13bca2, carpeta preparacion-nativa-01. Esta carpeta 02 es independiente; no se edita la 01.

| Zona | Cambio acotado |
|---|---|
| nativa/mod.rs | Admision común; campo sello obligatorio; versión EIO-NAT/2; casos negativos; módulos custodia/parada |
| nativa/custodia.rs | Extrae escritor a hilo exclusivo con cola 64, barrera ordinal, cierre por consumo y snapshot inmutable |
| nativa/parada.rs | Escalada dedicada monotónica 250 ms, avisos sin espera y heartbeat separado |
| nativa/supervisor.rs | Usa custodia asíncrona; no write/sync de evidencia; ACK/sello; descarta tardíos; membresía cgroup y guarda exterior en RSS |
| nativa/guarda.rs | Mecanismo exterior inactivo, cgroup v2 delegado previamente, heartbeat y limpieza/confirmación |
| nativa/cotejo.rs | Reconoce campos nuevos del manifiesto y versión, mantiene cotejo sin reparar |
| nativa/testigo.rs | Retorno 77 tras resultado; hijo que mantiene pipes y emite tarde; recepción TERM; FIFO real para E/S bloqueante |
| web/ | Indicador único de admisibilidad; original rechazado visible; evidencia parcial y sello fijo por recuperación |
| pruebas/ | Amplía frontera; api-real por HTTP con recuperación concurrente; observador independiente; comprobación DOM preparada |
| Cargo.toml | Paquete 0.2.1 y bins guarda/api-real/observador; dependencias/versiones/features intactas |
| Configuración | Sigue inactiva; añade propuesta de guarda, sin configuración de plataforma operativa |
| Documentación | Sucesión de contrato y estado separado R1/R2/R3/B01–B08; alternativa Qwen interfaz retirada |

Se reutilizan literalmente banco.rs, casos.json, dos peticiones, ENTRADAS_ENSAYO.json, adaptador Candle, inferidor, telemetría, biblioteca de comprobación, evidencia histórica y lock de antecedente, salvo cambios de versión de frontera donde proceda. El manifiesto determina exactitud archivo por archivo; esta tabla no sustituye ese cotejo.

El plan de casos de NAT01 se conserva literalmente en antecedentes/ORACULOS-NAT01.md. Los originales del repositorio no se sustituyen. No cambios de pesos, tokenizador, plantilla, EOS, semilla/ArgMax, límites 128/2048/120 s, 4 GiB o verificador JSON. No nueva dependencia, resolución de lock ni prueba ejecutada.
