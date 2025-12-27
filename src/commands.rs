use tauri::{command, AppHandle, Runtime};
use crate::utils::{write_unified_log, LogLevel, LogSource};

#[command]
pub(crate) fn log_terminal<R: Runtime>(
    _app: AppHandle<R>,
    level: LogLevel,
    message: String,
) {
    // 오직 개발 모드에서만 실제 출력이 작동합니다.
    // Only in debug mode
    #[cfg(debug_assertions)]
    {
        write_unified_log(LogSource::Frontend, level, &message);
    }
}