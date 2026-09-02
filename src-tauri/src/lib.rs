use std::path::{Path, PathBuf};
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

#[derive(serde::Serialize)]
struct DbPathStatus {
    ok: bool,
    target: Option<String>,
    reason: String,
}

fn data_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    app.path().app_data_dir().map_err(|e| e.to_string())
}

/// Una carpeta es válida si contiene la subcarpeta `indexeddb` (IndexedDB de WebKitGTK).
fn is_valid_database_folder(path: &Path) -> bool {
    path.join("indexeddb").is_dir()
}

/// Comprueba si la base de datos local (IndexedDB de WebKit) está disponible.
/// Desde la Fase 2 la BD viva vive en local (directorio real), sin symlink a Dropbox:
/// el caso normal es un directorio `databases` real con `indexeddb/` dentro.
#[tauri::command]
fn check_database_path(app: tauri::AppHandle) -> DbPathStatus {
    let data_dir = match data_dir(&app) {
        Ok(d) => d,
        Err(e) => {
            return DbPathStatus {
                ok: false,
                target: None,
                reason: e,
            }
        }
    };
    let link = data_dir.join("databases");

    #[cfg(unix)]
    {
        match std::fs::symlink_metadata(&link) {
            Ok(meta) if meta.file_type().is_symlink() => match std::fs::read_link(&link) {
                Ok(target) => {
                    let ok = is_valid_database_folder(&target);
                    DbPathStatus {
                        ok,
                        target: Some(target.to_string_lossy().into_owned()),
                        reason: if ok {
                            String::new()
                        } else {
                            "El enlace apunta a una carpeta inexistente o sin base de datos (indexeddb)."
                                .into()
                        },
                    }
                }
                Err(e) => DbPathStatus {
                    ok: false,
                    target: None,
                    reason: format!("No se puede leer el enlace: {e}"),
                },
            },
            Ok(_) => DbPathStatus {
                ok: is_valid_database_folder(&link),
                target: Some(link.to_string_lossy().into_owned()),
                reason: String::new(),
            },
            Err(_) => DbPathStatus {
                ok: false,
                target: None,
                reason: "No existe la carpeta de la base de datos.".into(),
            },
        }
    }
    #[cfg(not(unix))]
    {
        let _ = link;
        DbPathStatus {
            ok: true,
            target: None,
            reason: String::new(),
        }
    }
}

fn collect_candidate_folders(base: &Path, results: &mut Vec<PathBuf>, depth: u32, limit: usize) {
    if results.len() >= limit || depth == 0 {
        return;
    }
    let Ok(entries) = std::fs::read_dir(base) else {
        return;
    };
    for entry in entries.flatten() {
        if results.len() >= limit {
            return;
        }
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_symlink() {
            continue;
        }
        let path = entry.path();
        if ft.is_dir() {
            if is_valid_database_folder(&path) {
                if !results.contains(&path) {
                    results.push(path);
                }
            } else {
                collect_candidate_folders(&path, results, depth - 1, limit);
            }
        }
    }
}

/// Busca carpetas que contienen una base de datos (`indexeddb/`) en las rutas habituales.
#[tauri::command]
fn search_database_folders(app: tauri::AppHandle) -> Vec<String> {
    let mut bases: Vec<PathBuf> = Vec::new();
    if let Ok(home) = app.path().home_dir() {
        bases.push(home.join("Desktop"));
        bases.push(home.join("Documents"));
        bases.push(home.join("Downloads"));
    }
    bases.push(PathBuf::from("/mnt/DATOS/Dropbox/INSTITUTO"));

    let mut results: Vec<PathBuf> = Vec::new();
    for base in bases {
        if results.len() >= 10 {
            break;
        }
        if base.is_dir() {
            collect_candidate_folders(&base, &mut results, 4, 10);
        }
    }

    results
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect()
}

/// Conecta la base de datos apuntando el enlace `databases` a la carpeta indicada.
#[tauri::command]
fn apply_database_folder(app: tauri::AppHandle, path: String) -> Result<String, String> {
    let target = PathBuf::from(&path);
    if !is_valid_database_folder(&target) {
        return Err(
            "La carpeta seleccionada no contiene una base de datos válida (subcarpeta 'indexeddb')."
                .into(),
        );
    }
    let data_dir = data_dir(&app)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let link = data_dir.join("databases");
        if let Ok(meta) = std::fs::symlink_metadata(&link) {
            if meta.file_type().is_symlink() || meta.is_file() {
                let _ = std::fs::remove_file(&link);
            } else if meta.is_dir() {
                let ts = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let backup = data_dir.join(format!("databases.backup-{}", ts));
                std::fs::rename(&link, &backup)
                    .map_err(|e| format!("No se pudo apartar la carpeta local existente: {e}"))?;
            }
        }
        symlink(&target, &link).map_err(|e| format!("Error creando el enlace: {e}"))?;
        Ok(format!("Base de datos conectada: {}", target.display()))
    }
    #[cfg(not(unix))]
    {
        let _ = data_dir;
        let _ = target;
        Err("Operación no disponible en esta plataforma.".into())
    }
}

/// Abre el selector de carpetas y conecta la base de datos elegida.
#[tauri::command]
fn select_database_folder(app: tauri::AppHandle) -> Result<String, String> {
    let folder = app
        .dialog()
        .file()
        .set_title("Selecciona la carpeta que contiene la base de datos (subcarpeta 'indexeddb')")
        .blocking_pick_folder()
        .ok_or("Operación cancelada.")?;
    let path = folder.into_path().map_err(|e| e.to_string())?;
    apply_database_folder(app, path.to_string_lossy().into_owned())
}

// ---------------------------------------------------------------------------
// Fase 2: copia de seguridad automática cifrada (GPG asimétrico) + rotación
// ---------------------------------------------------------------------------

fn sanitize(s: &str, fallback: &str) -> String {
    let out: String = s
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .take(64)
        .collect();
    if out.is_empty() {
        fallback.to_string()
    } else {
        out
    }
}

/// Lee `~/.config/cuaderno-docente/config.json` con `backup_dir` y `gpg_recipient`.
fn load_backup_config() -> Result<(PathBuf, String), String> {
    let home = std::env::var("HOME").map_err(|e| format!("No se pudo obtener HOME: {e}"))?;
    let config_path = PathBuf::from(&home)
        .join(".config")
        .join("cuaderno-docente")
        .join("config.json");
    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("No se pudo leer la configuración de copias ({}): {e}", config_path.display()))?;
    let cfg: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Configuración de copias inválida: {e}"))?;
    let backup_dir = cfg
        .get("backup_dir")
        .and_then(|v| v.as_str())
        .ok_or("Falta 'backup_dir' en la configuración de copias.")?
        .to_string();
    let recipient = cfg
        .get("gpg_recipient")
        .and_then(|v| v.as_str())
        .ok_or("Falta 'gpg_recipient' en la configuración de copias.")?
        .to_string();
    Ok((PathBuf::from(backup_dir), recipient))
}

/// Conserva solo las `keep` copias más recientes (por fecha de modificación).
fn rotate_backups(dir: &Path, keep: usize) -> Result<(), String> {
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with("backup_") && name.ends_with(".db.gpg") {
            files.push(path);
        }
    }
    if files.len() <= keep {
        return Ok(());
    }
    files.sort_by(|a, b| {
        let ta = std::fs::metadata(a)
            .and_then(|m| m.modified())
            .unwrap_or(std::time::UNIX_EPOCH);
        let tb = std::fs::metadata(b)
            .and_then(|m| m.modified())
            .unwrap_or(std::time::UNIX_EPOCH);
        tb.cmp(&ta)
    });
    for f in files.iter().skip(keep) {
        let _ = std::fs::remove_file(f);
    }
    Ok(())
}

/// Exporta la base de datos, la cifra con GPG (clave pública) y la guarda en la carpeta de copias.
#[tauri::command]
fn auto_backup(data: Vec<u8>, workspace: String, timestamp: String) -> Result<String, String> {
    let (backup_dir, recipient) = load_backup_config()?;
    if !backup_dir.is_dir() {
        return Err(format!("La carpeta de copias no existe: {}", backup_dir.display()));
    }

    let ws = sanitize(&workspace, "default");
    let ts = sanitize(&timestamp, "0");
    let base = format!("backup_{}_{}", ws, ts);
    let plain = backup_dir.join(format!("{}.db", base));
    let enc = backup_dir.join(format!("{}.db.gpg", base));

    std::fs::write(&plain, &data).map_err(|e| format!("No se pudo escribir la copia temporal: {e}"))?;

    let status = std::process::Command::new("gpg")
        .args([
            "--batch",
            "--yes",
            "--trust-model",
            "always",
            "--recipient",
            &recipient,
            "--encrypt",
            "--output",
        ])
        .arg(&enc)
        .arg(&plain)
        .status()
        .map_err(|e| format!("No se pudo ejecutar gpg: {e}"))?;

    if !status.success() {
        let _ = std::fs::remove_file(&plain);
        return Err("gpg no pudo cifrar la copia (¿clave pública disponible?).".into());
    }
    let _ = std::fs::remove_file(&plain);

    rotate_backups(&backup_dir, 5)?;

    Ok(enc.to_string_lossy().into_owned())
}

#[tauri::command]
fn exit_app(app: tauri::AppHandle) {
    app.exit(0);
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            check_database_path,
            search_database_folders,
            apply_database_folder,
            select_database_folder,
            auto_backup,
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
