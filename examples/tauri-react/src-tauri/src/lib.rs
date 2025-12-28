// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rgbitsoft_tauri_plugin_unified_log::unified_log;

#[tauri::command]
fn greet(name: &str) -> String {
    unified_log!(Info, "Invoking Rust ...! {}" , name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(rgbitsoft_tauri_plugin_unified_log::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
