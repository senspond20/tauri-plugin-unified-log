# Tauri Plugin unified-log

unified logging for Tauri v2.0+

```shell
cargo add rgbitsoft-tauri-plugin-unified-log
```


```rs
fn main() {
    tauri::Builder::default()
        .plugin(rgbitsoft_tauri_plugin_unified_log::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```