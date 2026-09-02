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

/// Comprueba si el enlace `databases` (IndexedDB de WebKit) apunta a una carpeta válida.
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
            Ok(meta) if meta.is_dir() => DbPathStatus {
                ok: false,
                target: Some(link.to_string_lossy().into_owned()),
                reason: "La base de datos está en una carpeta local sin enlazar (se esperaba un enlace a Dropbox)."
                    .into(),
            },
            Ok(_) => DbPathStatus {
                ok: false,
                target: Some(link.to_string_lossy().into_owned()),
                reason: "La ruta de la base de datos no es un enlace válido.".into(),
            },
            Err(_) => DbPathStatus {
                ok: false,
                target: None,
                reason: "No existe el enlace de la base de datos.".into(),
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

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            check_database_path,
            search_database_folders,
            apply_database_folder,
            select_database_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
