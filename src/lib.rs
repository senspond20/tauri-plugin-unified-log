use tauri::{
  plugin::{Builder, TauriPlugin}, Runtime,
};

pub mod commands;
pub mod utils;

#[macro_export]
macro_rules! unified_log {
    ($level:ident, $($arg:tt)*) => { 
        #[cfg(debug_assertions)]  // Only in debug mode
        {
                $crate::utils::write_unified_log(
                  $crate::utils::LogSource::Backend,
                  $crate::utils::LogLevel::$level, 
                  &format!($($arg)*)
            );
        }
    };
}
// .
/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("unified-log")
    .invoke_handler(tauri::generate_handler![commands::log_terminal])
    .build()
}
