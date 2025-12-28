// Learn more about Tauri commands at https://v2.tauri.app/develop/calling-rust/#commands
use rgbitsoft_tauri_plugin_unified_log::unified_log;

#[tauri::command]
fn trigger_server_log() {
    unified_log!(Info, "Invoking Rust Backend...!");
    unified_log!(Warn, "Invoking Rust Backend...! {}", "Warning Message");
    unified_log!(Error, "Invoking Rust Backend...! {}", "Error Message");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![trigger_server_log])
        .plugin(rgbitsoft_tauri_plugin_unified_log::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
