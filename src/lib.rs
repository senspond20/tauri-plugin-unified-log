use tauri::{
  plugin::{Builder, TauriPlugin}, Runtime,
};
pub mod types;  
pub mod logger;
pub mod commands;

// 매크로가 내부 타입을 참조하기 쉽게 재배치
pub use types::{LogLevel, LogSource};

// .
/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("unified-log")
    .invoke_handler(tauri::generate_handler![commands::log_terminal])
    .build()
}
