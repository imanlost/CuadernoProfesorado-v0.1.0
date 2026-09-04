# Changelog - CuadernoDocente (versión de escritorio)

> Changelog de la app de escritorio (Tauri). Las versiones que mandan son las del `package.json`/`tauri.conf.json` (las que ve el usuario en el `.deb`). La versión web (repo CuadernoProfesorado-v1.0) lleva su propio changelog en `CHANGELOG.md`.
> Formato: cronológico inverso (lo más reciente arriba). Actualizar SIEMPRE en cada release o commit de cambios.

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
