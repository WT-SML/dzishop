// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod invoke;
mod tools;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tools::set_window_shadow(app);
            Ok(())
        })
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![invoke::get_directory_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
