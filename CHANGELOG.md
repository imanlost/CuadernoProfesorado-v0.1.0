# Changelog - CuadernoDocente (versión de escritorio)

> Changelog de la app de escritorio (Tauri). Las versiones que mandan son las del `package.json`/`tauri.conf.json` (las que ve el usuario en el `.deb`). La versión web (repo CuadernoProfesorado-v1.0) lleva su propio changelog en `CHANGELOG.md`.
> Formato: cronológico inverso (lo más reciente arriba). Actualizar SIEMPRE en cada release o commit de cambios.

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
