use base64::prelude::*;
use tauri::Manager;

fn read_file_to_string(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_session_file(path: String) -> Result<String, String> {
    read_file_to_string(&path)
}

#[tauri::command]
fn write_export_file(path: String, content: String, encoding: String) -> Result<(), String> {
    let bytes = if encoding == "base64" {
        BASE64_STANDARD.decode(content).map_err(|e| e.to_string())?
    } else {
        content.into_bytes()
    };
    std::fs::write(&path, &bytes).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_app_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .invoke_handler(tauri::generate_handler![read_session_file, write_export_file, get_app_version])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_title("Mira");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
