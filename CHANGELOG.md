# Changelog - CuadernoDocente (versión de escritorio)

> Changelog de la app de escritorio (Tauri). Las versiones que mandan son las del `package.json`/`tauri.conf.json` (las que ve el usuario en el `.deb`). La versión web (repo CuadernoProfesorado-v1.0) lleva su propio changelog en `CHANGELOG.md`.
> Formato: cronológico inverso (lo más reciente arriba). Actualizar SIEMPRE en cada release o commit de cambios.

## [2026-11-03] - v2.10.5: el cursor también en el diario y en las sesiones del calendario
- **Corregido**: al **Calificar**, el cursor no caía en «Calificación Única» sino en el primer criterio (faltaba una referencia en el campo de nota única). Ahora cae donde debe.
- **Añadido**: **autofoco en el Diario de Clase**: al entrar en la pestaña (o al cambiar de fecha) el cursor ya está en el cuadro de anotaciones del primer bloque del día.
- **Añadido**: **autofoco al abrir una sesión del calendario**: el cursor va directo a las anotaciones del diario, sin clicar.
- **Interno**: sin cambios en datos ni en cálculos.

## [2026-11-03] - v2.10.4: Color de la nota de evaluación, autofoco y pequeñas incomodidades de uso
- **Corregido**: la columna **Nota Ev.** salía con **fondo gris** en casi todas las filas (solo se veían las notas «Bien», en amarillo). La celda llevaba un color de fondo fijo (`bg-slate-100`) junto al color del semáforo; al competir dos clases de fondo gana la que va después en la hoja de estilos, y por eso el gris tapaba el verde, el lima, el naranja y el rojo. Ahora el gris queda solo como respaldo cuando no hay nota. Afecta a las tres copias del código (escritorio, tauri y web).
- **Mejorado**: al abrir las ventanas de **Nueva/Editar Tarea**, **Calificar** y **Nueva Tarea del calendario**, el cursor ya está listo para escribir (en Calificar, directamente en «Calificación Única»). El cursor se coloca al final del texto existente, sin seleccionarlo, para no machacar nada al editar.
- **Corregido**: la ventana **ya no se cierra** al seleccionar texto con el ratón y soltar fuera del recuadro. Antes, cualquier clic que terminara en el fondo cerraba el modal; ahora solo cierra si el clic empezó en el fondo. Se ha arreglado en el componente común, así que vale para todas las ventanas de la app.
- **Mejorado**: las **sesiones de una unidad didáctica se pueden reordenar arrastrando** desde el asa de puntos que aparece a la izquierda de cada sesión, además de con las flechas de siempre.
- **Corregido**: la **viñeta suelta** que aparecía a la izquierda de «Copia de Seguridad» en Ajustes (era una etiqueta de lista fuera de su lista) y el descuadre de esa casilla. El acceso a Copia de Seguridad se muestra ahora en **ámbar**, para distinguirlo del resto de la configuración.
- **Interno**: sin cambios en datos ni en cálculos.

## [2026-09-28] - v2.10.3: Encabezados fijos al desplazar (calendario, calificaciones, informes y diario de clase)
- **Mejorado**: Las **cabeceras de las pantallas quedan fijas al desplazar** el contenido (antes se desplazaba todo y se perdía de vista el contexto). Ahora, mientras se revisan las filas de abajo, se sigue viendo en todo momento la tarea o el criterio sobre el que se está trabajando:
  - **Calendario**: quedan fijos el mes con sus flechas y el botón «Hoy», los botones de vista (Mes/Semana/Día) y la línea de días de la semana (Lun-Vie); en la vista Semana, la franja con las fechas de los cinco días.
  - **Calificaciones**: quedan fijas las pestañas de clase y de evaluación y la fila de nombres de **Tareas** (la columna Alumn@ ya lo estaba). La tabla tiene ahora su propio desplazamiento vertical y horizontal.
  - **Informes**: en las cuatro pestañas (Criterios, Competencias Específicas, Competencias Clave y Descriptores Operativos) queda fija la cabecera con el título, el selector de periodo y la fila de criterios, competencias o descriptores.
  - **Diario de Clase**: queda fijo el bloque con el título, las flechas de navegación y la fecha.
- **Mejorado**: el desplazamiento pasa del borde de la ventana al panel de contenido: la cabecera general («Cuaderno Docente») y el menú lateral ya no se mueven nunca.
- **Interno**: solo cambios de maquetación (clases de Tailwind); sin ninguna modificación en los datos, en los cálculos ni en los archivos guardados.
- **Retirado**: dependencia `@google/genai`, que ya no se usaba (la app de escritorio funciona sin conexión y sin CDN de IA).

## [2026-09-08] - v2.10.2: Hotfix crítico — WASM de sql.js correcto y recuperación blindada
- **Corregido (crítico)**: El build 2.10.1 pedía `sql-wasm-browser.wasm` (el que resuelve Vite para el import `sql.js`) pero el script solo copiaba `sql-wasm.wasm`. El fetch daba 404 → `initSqlJs()` lanzaba → la app entraba en la pantalla de "No se pudo cargar la base de datos" **aunque la BD fuera válida**. Ahora el script copia ambos WASM y la app arranca sin red.
- **Corregido (crítico)**: El flujo de recuperación **ya no se muestra si la BD local es válida** y el fallo es del motor (WASM): se distingue `check_database_path` (BD ausente → recuperación) de un error de inicialización (BD sana → mensaje de error, sin tocar la carpeta de datos).
- **Corregido (crítico)**: `searchDatabase` **ya no autoaplica** la única carpeta encontrada. En 2.10.1 eso renombró la BD local válida (`databases` → `databases.backup-*`) para conectar un residuo antiguo de Dropbox y dejó la app sin responder. Ahora siempre lista los resultados y exige elección explícita.
- **Corregido**: La lista de recuperación se muestra también con **una sola** carpeta encontrada (antes exigía `> 1` y con 1 sola no ofrecía nada tras quitar el autoapply).
- **Corregido**: `recoverBrokenUI` espera a `document.readyState === 'complete'` + 300 ms antes de comprobar el CSS (2 frames daban falsos positivos de "UI sin estilos" con caché fría de WebKitGTK).

## [2026-09-08] - v2.10.1: Build autocontenido (sin CDN), copia diaria inmutable y auto-recuperación de la UI
- **Corregido**: El `.deb` ya **no depende de Internet en runtime**. El `index.html` embe bido cargaba Tailwind CSS, React y sql.js desde CDN (`cdn.tailwindcss.com`, `aistudiocdn.com`, `cdnjs.cloudflare.com`, Google Fonts): si la red bloqueaba esos dominios (p. ej. proxy del instituto) la UI arrancaba sin maquetar. Ahora Tailwind se compila a CSS estático (PostCSS), React/ReactDOM y sql.js se empaquetan locales (Vite) y la fuente Inter se sirve local (fontsource). Única copia afectada: repo Tauri; la versión web (Vercel) sigue con CDN.
- **Añadido**: **Copia diaria inmutable** de seguridad (`backup_diario_*`): la rotación ya no puede borrar el histórico de días anteriores al hacer varios cierres seguidos; conserva 15 copias de sesión + 60 diarias (~2 meses).
- **Añadido**: **Auto-recuperación al arrancar**: si la UI carga sin estilos (caché WebKit corrupta), la app limpia `WebKitCache`/`CacheStorage` y se recarga sola una vez (nuevo comando `clear_webkit_cache`).

## [2026-09-07] - v2.10.0: Colores por nivel en horario y calendario, exportación del horario (JSON/CSV) y borrado de planificación
- **Añadido**: Las clases se colorean automáticamente **por nivel educativo** (azules 1º ESO, naranjas 2º ESO, verdes 3º ESO, violetas 4º ESO, amarillos 1º Bachillerato, rojos 2º Bachillerato) en el horario semanal, la vista Semana y el calendario de sesiones, con **tonalidades diferenciadas** entre grupos del mismo nivel.
- **Añadido**: **Selector de color por clase** en Ajustes > Clases y Alumnado (botón «Automático» restaura el color de su nivel; bola de color junto a la clase).
- **Añadido**: Botones **«Exportar Horario (JSON)»** y **«Exportar CSV»** en Ajustes > Horario Semanal, con **diálogo nativo de guardado** (patrón dual `__TAURI_INTERNALS__`). El JSON (esquema v1, incluye la versión de la app en `generador`) se importa en la app móvil MiHorario; el CSV para hojas de cálculo.
- **Añadido**: Botón **«Borrar planificación»** en la pestaña Programación (UUDD) con confirmación.

## [2026-09-04] - v2.9.1: Guardado automático de medidas ACNEAE
- **Corregido**: Las medidas ACNEAE del selector de la pestaña Anotaciones se guardan automáticamente en cuanto se marca o desmarca una casilla, sin necesidad de añadir una anotación con texto (antes solo se persistían al pulsar «Añadir anotación», que exigía comentario).
- Commits: fix `e602fe7`, bump `6e28347`. CI run 33884393033.

## [2026-09-04] - v2.9.0: Etiqueta REP (repetidor/a) y selector de medidas ACNEAE
- **Añadido**: Nueva etiqueta ACNEAE **REP** (repetidor/a), siempre como la medida menos prioritaria (100; última bolita, incluso por debajo de FPEX/NN que caen a 99).
- **Añadido**: Selector de medidas ACNEAE al añadir anotaciones en la ficha del alumno/a.
- Commits: `336a57a`. CI run 33881790510.

## [2026-09-03] - v2.8.0: Anotaciones por alumno, avisos flotantes y diálogos propios
- **Añadido**: Pestaña **Anotaciones** en la ficha del alumno/a (texto, fecha, aviso importante ⚠ con indicador en lista y ficha).
- **Añadido**: Sistema de **avisos flotantes** (toasts) sustituyendo a los `alert()` nativos (descentrados en WebKitGTK).
- **Añadido**: **Diálogos de confirmación propios y centrados** sustituyendo a los `confirm()` nativos en acciones destructivas.
- **Añadido**: Modales de **importación en bloque editables** (escribir o pegar, revisar y redistribuir antes de guardar).
- **Corregido**: El pegado directo en los textareas de importación ya funciona (antes exigía pegar en un txt intermedio).
- Commit `09896b7`.

## [2026-09-02] - v2.7.0: Backup automático cifrado (GPG asimétrico) + BD viva local
- **Añadido**: La base de datos viva pasa a ubicación local canónica (sin symlink a Dropbox). Backups automáticos cifrados con GPG asimétrico (clave pública; la privada vive solo en KeePass) cada 30 min y al cerrar la app, con rotación de las últimas 5 copias en BackupNOTAS.
- **Añadido**: Cierre limpio de la app en Wayland/labwc (backup antes de salir).
- Commit: F2 (ver skill `cuaderno-profesorado-release`, sección Fase 2).

## [2026-09-02] - v2.6.1: Health-check de la BD al arranque
- **Añadido**: Pantalla de recuperación al arranque si el enlace de la base de datos está roto o es un directorio vacío (nunca más pantalla en blanco ni datos de ejemplo silenciosos). Búsqueda automática de rutas válidas + selector manual con diálogo nativo.
- Commit: F1 (ver skill `cuaderno-profesorado-release`, sección Fase 1).

## [2026-06-12] - v2.9 del ZIP (serie antigua de AI Studio): Modo LOMLOE Puro
- Serie antigua del ZIP (v2.5–v2.13.2), cuando la app se versionaba como el CHANGELOG del ZIP. No confundir con la serie real v2.6.1+.
- **Añadido**: Modo LOMLOE Puro (evaluación competencial): campo Peso (%) en competencias específicas, cálculo con `comp.weight`, badge de peso en la UI.
- **Añadido**: Orden alfabético de las clases.
