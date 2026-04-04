# Hoja de cambios

## v0.1.8

- se endurece el núcleo algebraico con validación de `n = b², b ≥ 3`;
- se introduce `SVTernaryError` y normalización explícita del alfabeto ternario, con interoperabilidad para `"0"`, `"1"` y `"U"`;
- `sv_version` pasa a derivarse dinámicamente del paquete instalado;
- la cobertura y el umbral de CI pasan a medir `sv_motor` completo;
- el CI incorpora `etapa_2_dominio_dev` y `etapa_5_verificacion_python`;
- FT-SV-IA/001 admite bloqueo por custodia estructural declarada en el expediente directo;
- se actualizan checklist, tablero, adversariales y deuda viva;
- se materializa puente verificable con el banco de idiomas y se registra el adaptador pendiente con la IR v0.2 del lenguaje;
- se corrigen los pseudocódigos de laboratorio para distinguir `gate_vector` de `gate_value`;
- se completa el registro global de deuda del Hito 5;
- se consolida el Hito 5 en régimen exclusivo de laboratorio, nunca de producción;
- se incorporan `docs/arquitectura/09_hito5_arquitectura_multiestrato.md`, `docs/gobierno/12_regimen_exclusivo_de_laboratorio_y_perfiles_hito_5.md`, `docs/gobierno/13_cierre_tecnico_hito_5.md` y los protocolos mínimos de reemplazo de extractores.

## v0.1.7

- Se fija el cierre del Hito 3 con orientación aplicada del motor hacia agentes especializados, interacción experto–Inteligencia Lógica, proyección poligonal compartida, capa visual especializada subordinada y programación trazable en lenguaje SV con cotejo paralelo en Python.
- Se incorpora la capa `verification/` como vara paralela de cotejo reproducible sobre JSON canónico, sin desplazar la forma canónica `.svp` ni el carril ejecutable principal.
- Se consolida la reorganización científica del repositorio y se retira de la primera línea el material transitorio de lotes.
- Se normaliza la serie documental de gobierno y se cierra la colisión entre protocolo original y revisión vigente.

## v0.1.6

- se incorpora la capa de verificación Python (`src/sv_motor/verification/`):
  - `py_runner.py`: runner ejecutable en Python puro para los dominios NLP, DEV, CUSTODIA y CUSTOM;
  - `comparator.py`: comparador de salidas JSON canónicas entre Python y backend Rust/.svp (doble vara);
  - se fija un JSON canónico local del runner Python como artefacto de cotejo; la convergencia plena con la IR v0.2 del Lenguaje SV queda diferida a un adaptador explícito;
  - U preservada como cadena "U" en todo el JSON — nunca colapsada;
  - sin dependencias externas, sin estado global, `to_dict()` con copia profunda;
- se añade laboratorio etapa_5_verificacion_python: 15 casos canónicos + 6 adversariales de doble vara, 21/21 APTO;
- se añade `docs/arquitectura/08_capa_python_de_verificacion.md`;
- se explicita que Python puede actuar como artefacto paralelo de cotejo y difusión para programadores que aún no dominan `.svp`, sin adquirir estatuto canónico;
- el backend Rust/.svp sigue siendo el motor de ejecución canónico; Python es la vara de verificación.

## v0.1.5

- se fija la dirección aplicada del motor (hito 3): agentes especializados, interacción experto–Inteligencia Lógica, proyección poligonal, capa visual subordinada y futura programación trazable en `.svp`;
- se incorporan las carpetas `docs/agentes/`, `docs/fundamentos/`, `docs/interfaces/`, `docs/estado/` con su documentación de arquitectura y gobierno;
- se registra el problema correcto de transducción determinista de intención a programa `.svp` (DV-SVM-002);
- se retira de primera línea el material transitorio de lotes anteriores;
- se actualizan registros de deuda viva, decisiones y sincronización con repositorios superiores.

## v0.1.4

- se declara `𝔇_DEV` en su capa evaluadora algebraica;
- se incorpora compuerta ejecutable mínima de custodia estructural;
- se reorganiza la capa documental activa del repositorio;
- se fija la dirección aplicada del motor hacia agentes especializados, proyección poligonal, capa visual subordinada y futura programación trazable en `.svp`;
- se retira de primera línea el material transitorio o impropio de una lectura científica principal.
