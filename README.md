# 📓 Cuaderno del Profesorado — Versión de Escritorio (Tauri)

<p align="center">
  <img src="https://raw.githubusercontent.com/imanlost/CuadernoProfesorado-v1.0/main/public/cuadernoprofesor.svg" alt="Cuaderno Profesorado" width="200">
</p>

<p align="center">
  <a href="http://creativecommons.org/licenses/by-nc/4.0/"><img src="https://licensebuttons.net/l/by-nc/4.0/88x31.png" alt="CC BY-NC 4.0"></a>
</p>

[![Release](https://img.shields.io/github/v/release/imanlost/CuadernoProfesorado-v0.1.0?label=%C3%9Altima%20versi%C3%B3n)](https://github.com/imanlost/CuadernoProfesorado-v0.1.0/releases)
[![Web](https://img.shields.io/badge/Web-Vercel-black?logo=vercel)](https://cuadernodocentev2.vercel.app/)
[![Tauri](https://img.shields.io/badge/Desktop-Tauri-FFC131?logo=tauri)](https://tauri.app/)

**Aplicación de escritorio para la gestión académica docente con soberanía de datos y evaluación competencial LOMLOE.**

Este repositorio contiene la versión de escritorio del Cuaderno del Profesorado, empaquetada con **Tauri 2** para Windows, macOS y Linux. Es el mismo proyecto que la [versión web](https://github.com/imanlost/CuadernoProfesorado-v1.0), pero con acceso nativo al sistema de archivos y base de datos local real (sin depender del navegador).

Toda la información del alumnado permanece en tu equipo: la base de datos vive en tu disco, nunca en servidores externos. **Tus datos no abandonan tu dispositivo.**

---

## 🚀 Instalación

Descarga el instalador para tu sistema desde la **[página de releases](https://github.com/imanlost/CuadernoProfesorado-v0.1.0/releases)** (cada versión genera automáticamente los instaladores de los tres sistemas):

| Sistema | Archivo |
|---|---|
| **Linux** | `.deb` (Debian/Ubuntu) o AppImage |
| **Windows** | `.msi` o `.exe` |
| **macOS** | `.dmg` |

> También se distribuye al profesorado de Navarra desde [esta carpeta compartida](https://drive.google.com/drive/folders/14ipQlfZ7RB2Pz2XT9q7xXI2pbT2F108Y).

### Notas por sistema

- **Linux**: si la aplicación no abre, puede faltar una pieza del sistema llamada WebKit. Instálala con:
  ```
  sudo apt-get install libwebkit2gtk-4.1-0
  ```
- **macOS**: al no tener firma de desarrollador de Apple (99 USD/año), Gatekeeper bloqueará la primera apertura. Solución en el [manual de usuario](https://github.com/imanlost/CuadernoProfesorado-v1.0/blob/main/MANUAL_USUARIO_ESCRITORIO.md) (sección 1.3): `sudo xattr -cr /Applications/CuadernoDocente.app` o «Abrir de todas formas» en Preferencias del Sistema.
- **Windows**: si aparece el aviso azul «Windows protegió tu equipo», pulsa **Más información → Ejecutar de todas formas** (el instalador no está firmado con certificado de pago).

La versión de escritorio incluye **copias de seguridad automáticas**: al cerrar la aplicación y cada 30 minutos exporta tu base de datos cifrada (GPG asimétrico) a la carpeta de backups que elijas, con rotación de las 5 copias más recientes. La clave privada solo vive en tu gestor de contraseñas: ni la aplicación ni la nube pueden leer tus datos sin ella.

---

## ✨ Características Principales

### 📅 Planificación y Calendario
- **Calendario académico** con vistas de mes, semana y día
- **Festivos y vacaciones** personalizables que el calendario respeta automáticamente
- **Horario semanal** configurable con recreos y franjas horarias
- **Unidades Didácticas (UD)** arrastrables con número de sesiones y vinculación a saberes básicos
- **Colocación automática** de sesiones en días lectivos, esquivando fines de semana y festivos
- **Sesiones coloreables** para identificar visualmente tipos de clase

### 🧮 Evaluación Competencial LOMLOE
- **Dos modos de cálculo de nota final**:
  - *Clásico*: media ponderada por categorías (exámenes, trabajos, actitud...)
  - *LOMLOE Puro*: media ponderada de Competencias Específicas con pesos configurables
- **Cascada curricular completa**: cada tarea se vincula a criterios de evaluación, que alimentan competencias específicas, que nutren descriptores operativos del Perfil de Salida
- **Ponderación personalizable**: pesos configurables para cada Competencia Específica (%), cada categoría de tareas y cada periodo de evaluación
- **Umbral de aprobado** configurable (por defecto 5 sobre 10)

### 📝 Instrumentos de Evaluación
- **Listas de Cotejo**: evaluación binaria (Sí/No) con pesos por ítem
- **Escalas de Valoración**: niveles numéricos personalizables (ej. 1-5)
- **Rúbricas**: matrices con niveles y descriptores textuales por criterio
- **Importación/exportación CSV** de instrumentos
- Todos los instrumentos normalizan automáticamente a escala 0-10

### 🔄 Sistema de Recuperaciones
- **Recuperación de tarea**: una tarea de recuperación sobrescribe automáticamente la nota de la tarea original suspendida
- **Recuperación de evaluación**: categorías tipo «Recuperación de Evaluación» que reemplazan la media del trimestre si el alumno estaba suspenso
- **Inyección en cascada**: la nota de recuperación se proyecta hacia abajo en todos los criterios de evaluación del periodo

### 📊 Informes y Estadísticas
- **Informes competenciales** por alumno y grupo (Criterios, Competencias Específicas, Competencias Clave, Descriptores Operativos)
- **Filtro por periodo**: 1ª, 2ª, 3ª Evaluación o Curso Completo
- **Estadísticas gráficas** del grupo (Recharts): distribución de notas, porcentaje de aprobados, medias
- **Exportación de informes** para actas de evaluación

### 👥 Gestión de Alumnado
- **Carga masiva** de alumnado: escribe o pega una columna de nombres (Excel, Séneca, Raíces), con revisión previa y sin duplicados
- **Etiquetas ACNEAE**: anota necesidades educativas (RE, ACS, etc.) con contador por alumno
- **Anotaciones por alumno** con fecha y marcador de aviso importante (⚠ en la lista y en la ficha)
- **Vista de resumen** por alumno con todas sus calificaciones
- **Desglose de nota** clicable: muestra la fórmula matemática exacta de cada calificación
- **Avisos flotantes** no intrusivos y diálogos de confirmación centrados para las acciones que borran datos

### 📚 Gestión Curricular
- **Importación CSV** del currículo LOMLOE con fusión inteligente de descriptores (evita duplicados)
- **Exportación CSV** para respaldo o edición externa
- **Gestor visual** de Competencias Clave, Descriptores Operativos, Competencias Específicas, Criterios de Evaluación y Saberes Básicos
- **Editor in-situ** de todos los elementos curriculares con vinculación cruzada

### 💾 Persistencia y Copias de Seguridad
- **Base de datos nativa** en tu disco (sin depender del navegador)
- **Múltiples espacios de trabajo**: bases de datos independientes para distintos cursos o especialidades
- **Copias de seguridad automáticas cifradas** (GPG asimétrico): al cerrar la app y cada 30 minutos, con rotación de las 5 copias más recientes
- **Exportación e importación manual** de la base de datos completa (`.db`)
- **Transición de curso**: botón «Nuevo Curso» que vacía alumnado y notas pero conserva currículo, instrumentos y configuración, avanzando las fechas un año
- **Recuperación ante fallos**: si la base de datos no se puede cargar al arrancar, la aplicación ofrece buscar la carpeta correcta automáticamente o seleccionarla manualmente — nunca una pantalla en blanco

### 📖 Diario de Clase
- Registro diario por clase con notas de texto
- Sincronización bidireccional con el Calendario
- Colores por sesión para categorización visual

---

## 🗺️ Flujo de Trabajo Recomendado

1. **Primer arranque**: configura curso, periodos, festivos y umbral de aprobado en **Ajustes ⚙️**; crea cursos, clases y alumnado; importa el currículo LOMLOE desde CSV.
2. **Día a día**: planifica Unidades Didácticas en el Calendario, califica tareas en el Cuaderno y registra incidencias en el Diario de Clase.
3. **Final de evaluación**: consulta los informes competenciales y las estadísticas del grupo, y descarga una copia de seguridad cifrada desde **Ajustes → Copia de Seguridad**.

---

## 🛡️ Soberanía de Datos y RGPD

- **Todos los datos se almacenan en local**: base de datos nativa en tu disco, nunca en servidores externos.
- **No hay servidor backend**: la aplicación es 100% cliente (React + SQLite).
- **No se envían datos a terceros**: ni analíticas, ni telemetría, ni cookies de seguimiento.
- **Backups cifrados**: las copias automáticas se cifran con tu clave pública GPG; la clave privada solo existe en tu gestor de contraseñas.
- **Cumplimiento RGPD**: al no tratar datos en servidores externos, minimizas los riesgos legales.

---

## 🔧 Desarrollo

### Requisitos

- **Node.js** LTS (v18 o superior)
- **Rust** (stable) — instala con [rustup](https://rustup.rs/)
- **Linux**: dependencias de Tauri (`libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, etc. — [guía oficial](https://tauri.app/start/prerequisites/))

### Puesta en marcha

```bash
npm install
npm run tauri dev      # desarrollo con ventana nativa
npm run tauri build    # genera los instaladores en src-tauri/target/release/
```

> ⚠️ Este repositorio contiene ramas de código específicas de Tauri (acceso nativo a archivos y diálogos). No sobrescribas `App.tsx` o `SettingsModal.tsx` con las versiones de la web sin preservar esas ramas.

---

## 📚 Documentación

- 📖 **[Manual de Usuario (Escritorio)](https://github.com/imanlost/CuadernoProfesorado-v1.0/blob/main/MANUAL_USUARIO_ESCRITORIO.md)** — Guía completa de la versión Tauri
- 🌐 **[Manual de Usuario (Web)](https://github.com/imanlost/CuadernoProfesorado-v1.0/blob/main/MANUAL_USUARIO_WEB.md)** — Guía de la versión de navegador
- 📋 **[README del proyecto (versión web)](https://github.com/imanlost/CuadernoProfesorado-v1.0)** — Características completas, modo LOMLOE Puro y formato CSV curricular

---

## 📄 Licencia

[Creative Commons Attribution-NonCommercial 4.0 International (CC BY-NC 4.0)](http://creativecommons.org/licenses/by-nc/4.0/)

Esto significa que puedes:
- ✅ Usar la aplicación gratuitamente con fines educativos
- ✅ Compartirla con compañeros docentes
- ✅ Modificar el código para adaptarlo a tus necesidades
- ❌ Usarla con fines comerciales sin permiso

---

**Hecho por un docente, para docentes.**
