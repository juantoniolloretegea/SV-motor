# Ficha de futura comprobación remota · INACTIVA

Objeto: resolver lock, compilar y comprobar sólo testigos NAT03, sin inferencia ni pesos. Requiere recepción y autorización nueva. Ninguna disponibilidad, cuota, contención o tarifa queda acreditada por esta ficha.

## Entorno y gasto adicional cero

Candidato conservado: un Codespace Linux x86-64 GNU de dos núcleos, sin prebuild, puertos públicos ni arranque automático. No crear otro alojamiento ni conceder privilegios. Antes de crear/arrancar: documentar identidad de cuenta/plan, saldo de cómputo y almacenamiento, consumo previo, reserva máxima y barrera efectiva que impida cargos adicionales. Una alerta o el nombre Free no bastan. Imagen/digest, versión del kernel, curl y herramientas receptoras deben identificarse. Si no se puede comprobar gasto adicional cero para cómputo y almacenamiento, fase no habilitable.

Se propone reserva máxima de 80 minutos-núcleo (dos núcleos durante 40 minutos), además del almacenamiento efectivo por su duración hasta eliminación autorizada. No se afirma que esa reserva esté disponible. Detener no elimina cargos de almacenamiento: el plan económico deberá cubrir custodia y cierre, con cota explícita de retención de **24 horas** y decisión humana de eliminación previa al vencimiento; no se autoriza borrado automático que destruya evidencia. Si la cuota no cubre esa retención, no crear.

## Presupuesto único propuesto

| Recurso/fase | Cota y parada |
|---|---|
| Instancias / campañas | una instancia; una campaña sin reintentos |
| Tiempo total | 40 min monotónicos desde inicio de preparación; incluye fallos y recuperación |
| Resolución y compilación | hasta 15 min acumulados; un lock nuevo, una compilación de bins/tests sin inferencia; jobs=1 |
| Comprobación de contención | hasta 4 min; una hoja vacía, un test benigno de kill y un test de muerte de guarda con control superior |
| Bancos sin procesos peligrosos | hasta 3 min; frontera, JSON y banco una vez cada uno |
| Casos HTTP | 13 sesiones nuevas secuenciales, una por cada fila; hasta 60 s por banco; reserva global 13 min incluyendo cierre exterior |
| Muerte de supervisor | una sesión separada, hasta 60 s, con guarda ya comprobada |
| Recuperación final | reserva mínima 4 min; no iniciar caso si invade esa reserva |
| Memoria | 4294967296 bytes de RSS agregada; muestreo objetivo 1 s; superar o no poder medir implica parada |
| Compilación | agregar Cargo/rustc/linker/build scripts y monitor; misma cota 4 GiB, sin sustituir por PSS o memory.current |
| Disco | margen libre inicial >= 8 GiB; máximo 5 GiB nuevos para fuentes/cache/build/evidencias, incluidos temporales; parar al exceder o quedar < 2 GiB |
| Evidencia de cada sesión | límites heredados: 8 MiB/archivo y 20 MiB de journals; banco 42 MiB recuperados y 1600 fragmentos en 20 s |
| Modelo | cero descargas y cero inferencias; feature inferencia desactivada |

Las reservas suman 40 min. Los tiempos de cada fase no se transfieren automáticamente para ampliar otra; si no caben, entregar incompleto. El lanzador exterior deberá imponer cotas y capturar su actuación; no basta una estimación. Tamaños reales de dependencias/build no conocidos: si la cota resulta insuficiente, detener sin ampliarla ni limpiar de oficio. El RSS del cliente exterior/receptor se registra separadamente; el agregado del ensayo mantiene la hoja completa más la guarda según el contrato.

## Orden futuro

1. Recibir commit y cotejar manifiesto; comprobar coste, entorno, rutas nuevas y herramientas existentes. Rust/Cargo exactos 1.98.0-x86_64-unknown-linux-gnu, sin cambiar selección predeterminada.
2. En directorio aislado, resolver **una vez** un lock para Cargo.toml conservado, sin copiar antecedentes/Cargo.lock como lock vigente. Registrar descargas/versiones/licencias y SHA-256 del lock. No ejecutar builds de inferencia; el resolvedor puede consultar metadatos de dependencias opcionales: no equivale a permiso para pesos ni otra red no prevista.
3. Compilar una vez con selección exacta, --locked, --no-default-features, --bins --tests, CARGO_BUILD_JOBS=1 y destino aislado. Registrar orden efectiva y cualquier fallo; sin ajustes de dependencias para hacerlo pasar. La compilación necesita entorno autorizado y acotado; **no depende de haber probado previamente la guarda de testigos**.
4. Ejecutar una vez las pruebas fuente de frontera (11 esperadas), los 35 controles JSON y 24 controles del banco, usando binarios producidos y conservando salidas. Un fallo detiene la campaña.
5. Sólo después de comprobar la contención descrita abajo: lanzar mediante guarda las 13 sesiones HTTP y la sesión separada de muerte del supervisor, sin modelo. Identidades de configuraciones habilitadas sólo en copias remotas nuevas; registrar SHA antes de cada sesión. No modificar configuraciones publicadas. La recepción podrá habilitar sólo un subconjunto explícito; no inventar un caso.
6. Recuperar evidencia incremental y final desde receptor exterior, detener plataforma y verificarlo. Eliminar instancia/almacenamiento únicamente con autoridad prevista y después de cotejo receptor. Si la recuperación falla, parar, conservar prefijos y comunicar límite/consumo; no declarar un conjunto completo.

Las órdenes exactas, imagen, hojas, PID y sedes se fijarán en la habilitación con datos reales; los marcadores de documentos anteriores no son órdenes autorizadas ahora.

## Contención: comprobaciones y evidencias exigidas

Antes de testigos bloqueantes o terminación forzada, comprobar una hoja cgroup v2 **domain, exclusiva, delegada y vacía**. Guardar ruta canónica, tipo, stat sin descendientes, procs vacío, events populated 0, permisos efectivos de lectura/escritura y autoridad previa de delegación. No sudo ni cambio de permisos/controladores.

Un proceso benigno acotado deberá incorporarse a esa hoja antes de actuar; registrar PID/start_ticks y membresía desde ambos lados, así como la guarda fuera de ella. Probar cgroup.kill con retorno real, posterior populated 0 y waitpid del hijo. Lectura fallida no acredita vacío. Registrar relojes monotónicos por proceso sin restarlos entre orígenes distintos.

Comprobar que los procesos contenidos no pueden salir de la hoja ni crear descendientes cgroup conforme al perímetro. Tener controles escribibles por la misma identidad no demuestra aislamiento contra esa identidad: documentar permisos efectivos y la limitación, sin afirmar contención hostil no probada.

Antes de los casos peligrosos, demostrar el control superior independiente de la guarda: con proceso benigno contenido, terminar deliberadamente la guarda en un único ensayo autorizado; el controlador exterior de plataforma deberá detectar ausencia, detener el entorno y permitir constatar ausencia de procesos/evidencia del cierre. Registrar acción, retorno, tiempos, estado de plataforma y recuperación posterior. La guarda preparada no cubre por sí sola su muerte, procesos no terminables o bloqueo del kernel. Si el control superior no existe o no puede verificarse dentro de cotas, las pruebas afectadas son **no habilitables** en ese entorno. No pedir privilegios ni cambiar alojamiento de oficio. Un reinicio necesario para el resto sólo podrá figurar expresamente en la habilitación dentro de la misma instancia/cuota; esta ficha no lo concede.

Para cada sesión, el receptor exterior conserva stdout/stderr/retorno de guarda y banco. Cierra supervisor después de recuperar; exige cgroup.kill sin error, populated 0 y supervisor recogido, además de ausencia de servicios residuales. Un banco conforme no basta para cerrar una sesión.

## Custodia y recepción

Registrar fases separadas: resolución, compilación, ejecución sintética, cierre y recuperación. Conservar órdenes exactas, tiempos/fuentes, versiones, configuración, originales y errores. Emisor inventaría rutas relativas, tamaños y SHA-256; se excluye la autohuella del manifiesto y se publica separadamente. Receptor descarga copia distinta y coteja byte a byte/huellas, sin reinterpretar texto renderizado.

Si no existe sello, recuperar manualmente prefijos sólo tras parada y con etiqueta NO_SELLADO_INCOMPLETO; incluir últimas respuestas API y evidencia exterior. No fabricar manifiesto de éxito ni rellenar lagunas. Si el receptor no puede descargar, conservar paquete transferible y registrar causa/error exactos; recuperación independiente pendiente.

Estado actual: **ficha preparada e inactiva; ejecución remota, gasto cero, permisos, contención y recuperación receptora no acreditados**.
