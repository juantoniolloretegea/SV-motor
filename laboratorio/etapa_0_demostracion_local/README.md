# Demostración end-to-end local — Fase 0

**Fecha y Versión: V.1 del conjunto**  
**Fecha:** 4 de abril de 2026  
**Versión del conjunto:** V.1 del conjunto  
**Autor del corpus:** Juan Antonio Lloret Egea  
**ORCID:** 0000-0002-6634-3351  
**Institución:** ITVIA — IA eñ™  
**ISSN:** 2695-6411  
**Licencia:** CC BY-NC-ND 4.0  
**Titularidad y autoría:** © Juan Antonio Lloret Egea, 2026. Este conjunto se distribuye con atribución explícita de autoría y bajo la licencia indicada, sin autorización para apropiación de la paternidad intelectual del Sistema Vectorial SV.  

---


Esta etapa cierra materialmente la demostración local mínima del frente motor.

## Objeto

Demostrar, en un único comando reproducible, la cadena completa:

entrada tipada → validación del paquete Ω_NLP → cadena algebraica del agente → clasificación K₃ → política de salida → estructura verificable bajo FT-SV-IA/001.

## Comando canónico

```bash
python -m pip install -e ".[dev]"
sv-nlp --modo direct   --obs-file laboratorio/etapa_0_demostracion_local/entrada_observables_demo.json   --session-file laboratorio/etapa_0_demostracion_local/sesion_demo_ft_sv_ia.json   --out laboratorio/etapa_0_demostracion_local/salida_demo_end_to_end_local.json
```

## Variante para comprobar U_d(B)

```bash
sv-nlp --modo direct   --obs-file laboratorio/etapa_0_demostracion_local/entrada_observables_demo_ud_b.json   --session-file laboratorio/etapa_0_demostracion_local/sesion_demo_ft_sv_ia.json   --out laboratorio/etapa_0_demostracion_local/salida_demo_end_to_end_local_ud_b.json
```

## Criterio de cierre

La Fase 0 queda materialmente demostrada si el comando ejecuta sin red, la salida reproduce la clasificación esperada, el archivo almacenado coincide con la salida regenerada y el bloque de estado aparece únicamente cuando existe algo que declarar.
