use serde::{Deserialize, Serialize};

pub enum LogSource {
    WebView,
    RustCore,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
    Trace,
    Log,
}

// 매크로에서 편하게 쓰기 위해 문자열 변환 기능을 넣어줍니다.
impl LogLevel {
    pub fn to_label(&self) -> &'static str {
        match self {
            LogLevel::Info  => "INFO",
            LogLevel::Warn  => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
            LogLevel::Log   => "LOG",
        }
    }
}