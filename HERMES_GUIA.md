# Guía de Actualización — CuadernoProfesorado Tauri

> **Para Hermes Agent.** Consulta este documento SIEMPRE antes de tocar cualquier archivo del proyecto Tauri.
> La regla de oro: **los backups son la base funcional; la versión web solo aporta novedades.**

---

## 1. REGLA DE ORO

```
BACKUP (Tauri funcional) + NUEVAS FUNCIONALIDADES (web) = Tauri actualizado
```

**NUNCA:**
- Copies `App.tsx` ni `SettingsModal.tsx` del ZIP de AI Studio directamente al proyecto Tauri.
- Uses `@tauri-apps/plugin-dialog` ni `@tauri-apps/plugin-fs` — los backups usan **File System Access API** (estándar web) que funciona en el webview de Tauri.
- Modifiques `package.json`, `vite.config.ts`, `index.html`, `tsconfig.json` ni `index.tsx` del proyecto Tauri.

**SIEMPRE:**
- Restaura desde los `.backup` como base.
- Añade SOLO las funciones/líneas nuevas de la versión web.
- Verifica con `npm run tauri dev` antes de commitear.

---

## 2. UBICACIONES CLAVE

| Ruta | Propósito |
|---|---|
| `/mnt/DATOS/PROYECTOS/CUADERNO/cuadernoprofesorado` | ZIP de AI Studio descomprimido aquí. Se prueba con `npm run dev`. Es la **versión web pura**. |
| `/mnt/DATOS/PROYECTOS/CUADERNO/cuadernoprofesoradoGitHub/tauri-cuaderno` | **Repositorio Tauri** (v0.1.0). Se prueba con `npm run tauri dev`. |
| `…/tauri-cuaderno/src/` | Carpeta fuente del proyecto Tauri. Aquí se copian a mano los archivos (NUNCA App.tsx ni SettingsModal.tsx directamente). |
| `…/tauri-cuaderno/src/App.tsx.backup` | **App.tsx funcional para Tauri**. Usa File System Access API, sin imports de Tauri. |
| `…/tauri-cuaderno/src/components/SettingsModal.tsx.backup` | **SettingsModal.tsx funcional para Tauri**. Props: `onSaveToLocalFile`, `onOpenLocalFile`, `localFileName`. NO tiene `onDisconnectLocalFile`, `onRequestFilePermission`, `filePermissionGranted`. |
| `/mnt/DATOS/PROYECTOS/CUADERNO/fusion_cuaderno` | **Repositorio web** (v1.0). Versión exacta de AI Studio para `npm run dev`. |

---

## 3. REPOSITORIOS GITHUB

| Repo | URL | Qué contiene | CI |
|---|---|---|---|
| **v1.0 (web)** | `https://github.com/imanlost/CuadernoProfesorado-v1.0` | App tal cual de AI Studio. El usuario descarga ZIP y lanza `npm run dev`. | No |
| **v0.1.0 (Tauri)** | `https://github.com/imanlost/CuadernoProfesorado-v0.1.0` | Tauri wrapper + workflow de compilación. Se dispara con tag `vX.X`. | Sí (`build-deb.yml`) |

---

## 4. FLUJO DE ACTUALIZACIÓN (para Hermes)

### 4.1 Cuando el usuario trae un ZIP nuevo de AI Studio

1. El ZIP ya está descomprimido en `/mnt/DATOS/PROYECTOS/CUADERNO/cuadernoprofesorado`
2. El usuario ya probó `npm run dev` y funciona

### 4.2 Archivos que se copian DIRECTAMENTE al proyecto Tauri

Estos se copian **tal cual** del ZIP a `tauri-cuaderno/src/`:

```
constants.ts
types.ts
components/*     (toda la carpeta)
services/*       (toda la carpeta)
```

### 4.3 Archivos que NUNCA se copian directamente

| Archivo | Por qué |
|---|---|
| `App.tsx` | Tiene lógica específica de archivos (File System Access API) que difiere de la web |
| `SettingsModal.tsx` | Tiene props diferentes entre web y Tauri |
| `package.json` | Contiene dependencias de Tauri (`@tauri-apps/api`, `@tauri-apps/cli`) |
| `vite.config.ts` | Tiene `port: 1420` y `strictPort: true` para Tauri |
| `index.html` | Versión adaptada para Tauri |
| `tsconfig.json` | Configuración específica del proyecto Tauri |
| `index.tsx` | Punto de entrada Tauri |

### 4.4 Cómo manejar App.tsx y SettingsModal.tsx

**Paso 1:** Restaurar desde backup
```bash
cp src/App.tsx.backup src/App.tsx
cp src/components/SettingsModal.tsx.backup src/components/SettingsModal.tsx
```

**Paso 2:** Comparar con la versión web para encontrar novedades
```bash
diff src/App.tsx /mnt/DATOS/PROYECTOS/CUADERNO/cuadernoprofesorado/App.tsx
diff src/components/SettingsModal.tsx /mnt/DATOS/PROYECTOS/CUADERNO/cuadernoprofesorado/components/SettingsModal.tsx
```

**Paso 3:** Añadir SOLO las funciones/líneas nuevas de la versión web al backup. Ejemplos de novedades que se han añadido en el pasado:
- Función `startNewCourse` (Inicio de Nuevo Curso)
- Función `disconnectLocalFile`
- Mejoras en `saveToLocalFile` y `openLocalFile` (fallbacks)
- Persistencia del fileHandle en IndexedDB

**Paso 4:** Probar
```bash
cd /mnt/DATOS/PROYECTOS/CUADERNO/cuadernoprofesoradoGitHub/tauri-cuaderno
npm run tauri dev
```

### 4.5 Flujo Git

```bash
cd /mnt/DATOS/PROYECTOS/CUADERNO/cuadernoprofesoradoGitHub/tauri-cuaderno
git add .
git commit -m "vX.Y: descripción"
git tag vX.Y
git push && git push --tags
```

El tag `vX.Y` dispara el workflow de GitHub Actions que compila los instaladores (.deb, .msi, .dmg).

---

## 5. DIFERENCIAS CLAVE ENTRE VERSIÓN WEB Y TAURI

### 5.1 App.tsx

| Aspecto | Web (AI Studio) | Tauri (backup) |
|---|---|---|
| Manejo de archivos | File System Access API + fallbacks | File System Access API (mismo API, funciona en webview) |
| Imports Tauri | No | No (no se usan plugins nativos) |
| `startNewCourse` | Sí | No (hay que añadirlo manualmente) |
| `disconnectLocalFile` | Sí | No (hay que añadirlo manualmente) |
| `filePermissionGranted` | Sí | No |
| `requestFilePermission` | Sí | No |
| Persistencia fileHandle en IndexedDB | Sí (`getFileHandle`, `setFileHandle`) | No |

### 5.2 SettingsModal.tsx

| Aspecto | Web (AI Studio) | Tauri (backup) |
|---|---|---|
| Props BackupManager | `onSaveToLocalFile`, `onOpenLocalFile`, `onDisconnectLocalFile`, `onRequestFilePermission`, `localFileName`, `filePermissionGranted` | `onSaveToLocalFile`, `onOpenLocalFile`, `localFileName` |
| Exportar | `showSaveFilePicker` con fallback a descarga | Descarga directa (`<a>` download) |
| Botón "Iniciar Nuevo Curso" | Sí (`startNewCourse`) | No (hay que añadirlo) |

---

## 6. ERRORES COMUNES

| Error | Causa | Solución |
|---|---|---|
| `Missing script: "tauri"` | Se sobreescribió `package.json` | Restaurar desde `actualizar.txt` (tiene el JSON completo) |
| `Waiting for frontend dev server on port 1420` | Se sobreescribió `vite.config.ts` | Restaurar con `port: 1420, strictPort: true` |
| App.tsx no compila en Tauri | Se usaron plugins `@tauri-apps/plugin-*` en lugar de File System Access API | Restaurar desde backup |
| SettingsModal no encuentra props | Se copió la versión web con props que el App.tsx Tauri no pasa | Restaurar desde backup y añadir solo las props nuevas necesarias |

---

## 7. VERSIONES

| Repo | Último tag | Fecha |
|---|---|---|
| CuadernoProfesorado-v1.0 (web) | v2.6 | 2026-06-09 |
| CuadernoProfesorado-v0.1.0 (Tauri) | v2.8 | 2026-06-09 |

---

## 8. REFERENCIAS

- Guía manual original: `actualizar.txt` (en la raíz del proyecto Tauri)
- Backups: `src/App.tsx.backup`, `src/components/SettingsModal.tsx.backup`
- Workflow CI: `.github/workflows/build-deb.yml`
