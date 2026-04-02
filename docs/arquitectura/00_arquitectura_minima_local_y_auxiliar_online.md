# Arquitectura mínima local y auxiliar online

## Principio rector

Todo lo algebraicamente determinista permanece local. Lo online, cuando comparezca, sólo puede actuar como extracción subordinada o soporte auxiliar.

## Siempre local

- umbral, clasificación y compuertas;
- política de salida;
- trazabilidad del dictamen;
- manifiestos y huellas;
- verificación de integridad del paquete.

## Auxiliar y eventualmente online

- extracción de observables desde texto o imagen natural;
- automatización de pruebas;
- publicación estática;
- sincronización entre sedes.

## Lo que no puede salir

- el dictamen final;
- la lógica de cierre en K3;
- la semántica de la compuerta;
- la trazabilidad completa del paquete de custodia.

## Demostración local de Fase 0

La Fase 0 queda materialmente demostrada mediante una interfaz de línea de órdenes mínima en modo `direct`. Esta vía no activa modelos opcionales de entorno local o en línea. Su función es cerrar el circuito verificable de terminal a `K₃`.

```bash
sv-nlp --modo direct   --obs-file laboratorio/etapa_0_demostracion_local/entrada_observables_demo.json   --session-file laboratorio/etapa_0_demostracion_local/sesion_demo_ft_sv_ia.json   --out laboratorio/etapa_0_demostracion_local/salida_demo_end_to_end_local.json
```

## Compuerta equivalente a FT-SV-IA/001 en código

Antes de cualquier activación real de extractores opcionales, el repositorio dispone ya de un módulo ejecutable `src/sv_motor/protocols/ft_sv_ia.py` que materializa, para el carril local, la declaración de sesión, el bloque de estado y la unicidad de acción requerida del humano soberano.
