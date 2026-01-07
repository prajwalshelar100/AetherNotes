// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;

use tauri::Manager;

mod notes;
mod vault;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("Failed to get app data dir: {}", e))?;

            let vault = vault::Vault::new(app_data_dir)?;
            app.manage(vault);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::list_notes,
            commands::create_note,
            commands::update_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
