# Servicio documental y cliente local 0.1.2

## Contrato y alcance

Versión sucesora de 0.1.1. Conserva el servicio documental Rust y añade un cliente HTTP local con supervisión independiente del proceso que realiza la comunicación y la conservación. Requiere Rust 1.98.0 y las dependencias de Cargo.lock. No constituye una evaluación clínica ni una recepción general del servicio.

El cliente admite un mensaje de usuario textual, destino fijo de bucle local, recuento no generativo y una petición generativa. Desactiva proxies, redirecciones y reintentos. No proporciona al modelo herramientas, búsqueda web ni ejecución de código. El plan, la pregunta y la rúbrica pertenecen a la configuración externa privada y no se distribuyen en esta versión.

## Cambios

- `supervision.rs`: lectura no bloqueante del canal de control, consulta no bloqueante de terminación y señal al grupo de procesos al alcanzar el umbral temporal.
- `cliente-minimo.rs`: el trabajador conserva petición, respuesta y trazas antes de confirmar; la conservación del informe del supervisor se realiza en otro trabajador, dentro del presupuesto restante. Las operaciones documentales usan una cota de 30 segundos.
- `observar-minimo.rs`: lectura del cgroup del conjunto experimental. Una métrica no disponible se representa mediante null.
- El cliente usa `/v1/messages` y `/v1/messages/count_tokens`, interfaces implementadas por mistral.rs. Su denominación no implica un servicio externo.

La API heredada `Custodia::transact` permanece conservada y no recibe una garantía temporal nueva. La supervisión adicional corresponde a la ruta de `cliente-minimo`. Las etiquetas de instrumentación heredadas no deben confundirse con una recepción nueva del antecedente documental.

## Comprobaciones dirigidas

Se comprobaron un FIFO sin datos y una escritura superior a la capacidad de un FIFO sin lector activo. Ambos trabajadores fueron interrumpidos y recogidos. Un receptor HTTP en un espacio de red separado confirmó la igualdad de bytes, incluidos caracteres Unicode. La conservación del cuerpo se verifica antes de la confirmación del trabajador.

Estos controles no prueban la corrección integral de un adaptador compilado, ni garantizan la recuperación de un sistema operativo o dispositivo bloqueado de forma no interrumpible. La parada del motor depende de la unidad de servicio supervisora y debe configurarse y verificarse separadamente; un timeout HTTP es insuficiente.

## Construcción

```sh
cargo +1.98.0 build --locked --offline -j4 --bin sv-mcp-documental --bin cliente-minimo --bin observar-minimo
```

El modo offline requiere las dependencias presentes. La construcción y los controles preceden a la carga del modelo. El cliente no está destinado a operar sin una política externa de recursos, aislamiento, conservación y parada.

## Fuentes técnicas

- [Antecedente 0.1.1](https://github.com/juantoniolloretegea/SV-motor/tree/f3a888a028bfc8a4ffa687fcabcf263e93b623ae/laboratorio/ensayo-ia-y-observabilidad/modelos-de-ia/model-context-protocol/0.1.1).
- [Adaptador local de mistral.rs, revisión fijada](https://github.com/EricLBuehler/mistral.rs/blob/2370966bb91e2e3dafa0b1521b87c50fd5c01244/mistralrs-server-core/src/anthropic.rs).

Los resultados operativos, las preguntas y las rúbricas no forman parte de esta publicación.
