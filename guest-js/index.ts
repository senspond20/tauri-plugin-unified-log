import { invoke } from '@tauri-apps/api/core';

interface UnifiedLogOptions {
  /** 개발 시 브라우저 콘솔 출력 여부 (기본값: true) */
  printInBrowser?: boolean;
  /** JSON 객체 들여쓰기 출력 여부 (기본값: true) */
  prettyJson?: boolean;
}

export function initUnifiedLog(options: UnifiedLogOptions = {}) {
  const { 
    printInBrowser = true,
    prettyJson = false 
  } = options;
  
  const isDev = (import.meta as any).env?.DEV ?? (process as any).env.NODE_ENV === 'development';
  const levels: Array<keyof Console> = ["log", "info", "warn", "error"];

  if (!isDev) {
    levels.forEach((level) => { (console as any)[level] = () => {}; });
    return;
  }

  levels.forEach((level) => {
    const original = (console as any)[level];
    
    (console as any)[level] = (...args: any[]) => {
      if (printInBrowser) original.apply(console, args);

      const message = args
        .map((arg) => {
          if (typeof arg === "object" && arg !== null) {
            return prettyJson ? `\n${JSON.stringify(arg, null, 2)}` : JSON.stringify(arg);
          }
          return String(arg);
        })
        .join(" ");

      // 플러그인 네임스페이스 규칙 준수: plugin:이름|함수
      invoke('plugin:unified-log|log_terminal', { level, message }).catch((e) => {console.error("Error:", e);});
    };
  });
}