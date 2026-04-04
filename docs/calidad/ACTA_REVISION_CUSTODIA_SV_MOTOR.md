> **ALCANCE HISTÓRICO**
> Este documento describe una revisión temprana del repositorio y no debe leerse como fotografía exhaustiva del estado actual.
> El estado consolidado posterior queda recogido en `ACTA_TECNICA_DE_SINCRONIZACION_INTERREPOSITORIAL_2026_04_03.md` y en la deuda viva vigente.

# Acta de revisión de custodia estructural — SV-motor

## Identificación
- Repositorio: `SV-motor`
- Tipo de revisión: primera revisión pública fuerte
- Fecha: 2026-04-02
- Responsable de revisión: revisión técnica interna
- Commit / tag / snapshot revisado: completar con el commit visible en `History` del clon o de GitHub web

## Material revisado
- Árbol del repositorio: raíz `SV-motor-main` con `.github/`, `docs/`, `laboratorio/`, `scripts/`, `src/` y `tests/`.
- README y documentos de gobierno: presentes y coherentes con una sede pública subordinada del frente motor.
- Paquete Python y pruebas: `pyproject.toml` presente; instalación editable ejecutada; `35` pruebas superadas.
- Laboratorio: presente `laboratorio/etapa_1_nucleo_local/` con pseudocódigo, script, salidas JSON, análisis y dictamen.
- Registros de calidad: presentes en `docs/calidad/`, con registros mínimos, matrices, deuda viva y procedimiento de revisión.
- Artefactos relevantes adicionales: `scripts/run_local_checks.sh`, `ninguno.

## Dependencias contrastadas
- Pliego: contraste favorable. El repositorio no se presenta como doctrina soberana ni como motor completo.
- FT-SV-IA/001: contraste favorable. La capa externa se mantiene subordinada y no se presenta como sede del dictamen.
- Células especializadas: contraste favorable. El frente motor se trata como carril subordinado y no como familia madura cerrada.
- Célula de seguridad estructural: contraste favorable con deuda viva. No se detecta backend clandestino ni semántica implícita, pero falta demostración end-to-end auditada.
- Documento 3 NLP: contraste favorable. El repositorio no sobreactúa el estado piloto del frente NLP.
- Hito 1 del proyecto motor: contraste favorable. Se respeta la cadena de prevalencia y la prohibición de declarar resuelto el motor.

## Hallazgos principales
### Hechos constatados
- Existe un paquete local instalable y coherente.
- La suite actual devuelve 35 pruebas superadas.
- El script `scripts/run_local_checks.sh` ejecuta pruebas y laboratorio con dictamen final `APTO`.
- Existe laboratorio mínimo reproducible de la Etapa 1 del núcleo local.
- El bloque `docs/calidad/` ya no es decorativo: contiene registros, matrices, procedimiento y deuda viva.

### Indeterminaciones honestas (U)
- El snapshot exacto del commit auditado no puede fijarse desde el ZIP descargado y debe completarse con la vista `History` o la web de GitHub.
- El estado de ejecución remota de GitHub Actions no queda verificado por este lote local.
- No queda acreditada aún una demostración end-to-end local completa del frente motor.

### Deuda viva
- Falta una demostración end-to-end estrictamente local y subordinada.
- Falta un primer contraste auditado de sincronización material con el repositorio del lenguaje y el estado vigente del frente NLP.
- Falta una primera lectura de terceros sobre el laboratorio del motor más allá de la revisión técnica interna.

### Cambios improcedentes detectados
- Ninguno detectado en el lote auditado.

## Dictamen
- Estado: APTO CON DEUDA VIVA Y SINCRONIZACIÓN OBLIGATORIA
- Justificación breve: el repositorio ya acredita núcleo local mínimo, pruebas, laboratorio y trazabilidad pública básica, pero no autoriza todavía proclamación de motor completo ni integración fuerte.

## Revisión adversarial
### Riesgos de sobreactuación detectados
- Presentar el frente como motor ya resuelto.
- Presentar la fase actual como si ya implicara backend soberano.
- Apoyarse en la madurez del frente NLP más allá de su estatuto piloto.

### Riesgos de deriva técnica detectados
- Abrir integración fuerte con lenguaje sin compuerta previa.
- Dejar que una capa externa o online cierre el dictamen.
- Confundir laboratorio mínimo con prueba de completitud del motor.

### Riesgos de desalineación con el SV
- Tratar FT-SV-IA/001 como motor nativo.
- Doblar el lenguaje desde el frente motor.
- Declarar cerrada la U donde solo hay apertura legítima.

## Cierre
- ¿Puede avanzar?: Sí, pero solo a la siguiente fase legítima y con restricción fuerte.
- ¿Debe sincronizar antes?: Sí, con lenguaje, seguridad estructural y estado vigente del frente NLP.
- ¿Debe parar?: Debe parar si la siguiente fase exige gramática nueva, IR nueva, backend nuevo o cierre delegado a capas externas.
