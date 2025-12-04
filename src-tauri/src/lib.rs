use std::path::Path;

use serde_json::json;
use tauri::Emitter;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_file(app_handle: tauri::AppHandle, filepath: String) -> Result<(), String> {
    let path = Path::new(&filepath);

    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .ok_or("[ERROR] - Could not read file name")?
        .to_string();

    println!("File Name: {}", file_name);

    app_handle.emit("file-added", json!({
        file_name: "file"
    }))
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            add_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
