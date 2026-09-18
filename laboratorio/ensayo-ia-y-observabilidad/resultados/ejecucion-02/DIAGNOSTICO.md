# EIO-GITHUB-01/v2 · diagnóstico documental de acceso
18/09/2026. Unidad ejecutora: UE-LOCAL-CODEX-WINDOWS. Expediente sucesor, sin ejecución experimental.

## Restricción observada
La entrega anterior conserva dos resultados distintos: rechazo de conexión a 127.0.0.1:9 en la consulta HTTPS desde PowerShell y ERR_BLOCKED_BY_CLIENT al abrir la suma oficial de Rust 1.98.0 en el navegador. Ninguno identifica por sí solo al responsable del control ni su alcance fuera de ese canal. No se han repetido solicitudes al recurso bloqueado.

## Fuentes y alcance
Se han examinado las restricciones efectivamente expuestas a la sesión y los registros propios anteriores. El entorno de ejecución local declara acceso de red restringido y controles de permisos. Las herramientas GitHub permiten operaciones concretas de repositorio. Estas capacidades no contienen una excepción expresa que aclare el alcance del rechazo de navegador respecto de la adquisición desde un ejecutor de Actions.

Se conoce el fallo de las solicitudes concretas y el régimen restringido del canal local. No se conoce qué filtro, configuración o política produjo cada error, si comparten origen, ni si el control comprende la adquisición remota propuesta. No se atribuye el rechazo al servidor Rust, al antivirus o al cortafuegos. Tampoco se deduce permiso porque otro canal pueda funcionar.

## Paso bloqueado y aclaración necesaria
No queda satisfecha la condición previa de la revisión v2: fundamento explícito de que la preparación remota respeta la restricción. Se mantiene detenida la adquisición de la distribución y, por dependencia, el lanzamiento de la preparación en Actions.

El responsable del control deberá aclarar, para https://static.rust-lang.org/dist/channel-rust-1.98.0.toml y su suma .sha256, qué canal y recurso limita el rechazo y si permite la adquisición ordinaria de la distribución oficial en el ejecutor estándar de GitHub Actions previsto para el ensayo. No basta con reiterar la autorización del proyecto. No se ha contactado con terceros ni solicitado modificar controles.

## Resultado y límites
Examen documental concluido; adquisición y ensayo no ejecutados. Contador propio acumulado: 0/3 ejecuciones de Actions; sin run ID ni duración experimental. No se ha probado el acceso desde Actions. No se ofrece dictamen de compatibilidad, integridad de distribución, inferencia, privacidad o paridad entre plataformas.

Este documento aplica la rama de parada de v2 §2 y §6. La autoridad concreta del control y su alcance remoto permanecen desconocidos. Se conservan ejecucion-01 y los contratos. Revisión receptora pendiente; sin continuación automática ni cambio de S32/BIS-03.
