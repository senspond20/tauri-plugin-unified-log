use chrono::Local;
use colored::*;
use std::process;
use std::thread;
use crate::{LogLevel, LogSource};

#[macro_export]
macro_rules! unified_log {
    ($level:ident, $($arg:tt)*) => {
        #[cfg(debug_assertions)]  // Only in debug mode
        {
                $crate::logger::write_unified_log(
                  $crate::LogSource::RustCore,
                  $crate::LogLevel::$level,
                  &format!($($arg)*)
            );
        }
    };
}

pub fn write_unified_log(source: LogSource, level: LogLevel, message: &str) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S.%.3f").to_string();
    let pid = process::id();
    
    let source_str = match source {
        LogSource::WebView => "WEB_VIEW",
        LogSource::RustCore  => "RUST_CORE",
    };
    
    let level_str = level.to_label();
    // 색상 입히기
    let colored_level = match level_str {
        "ERROR" => level_str.red().bold(),
        "WARN"  => level_str.yellow().bold(),
        "INFO"  => level_str.cyan().bold(),
        _       => level_str.green().bold(),
    };
    
    let colored_source = if let LogSource::WebView = source {
        source_str.magenta().bold()
    } else {
        source_str.blue().bold()
    };

    // 현재 스레드 정보 낚아채기
    let current = thread::current();
    let thread_name = current.name();
    
    // 명칭 결정 로직: 이름이 "main"이면 그대로 쓰고, 아니면 ID를 사용 ㅡㅡㅋㅋ
    let display_thread = match thread_name {
        Some("main") => "main".to_string(),
        _ => {
            let id = format!("{:?}", current.id())
                .replace("ThreadId(", "")
                .replace(")", "");
            format!("thread-{}", id)
        }
    };
    // [시간] [레벨] [PID] [Thread] [소스] : 메시지
    println!(
        "{} {:5} {} [{:>5}] {:<6} : {}",
        now.dimmed(),
        colored_level,
        pid.to_string().dimmed(),
        display_thread,
        colored_source,
        message
    );
}