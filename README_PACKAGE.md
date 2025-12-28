# Tauri Plugin unified-log

unified logging for Tauri v2.0+

```shell
cargo add rgbitsoft-tauri-plugin-unified-log
```

### Setup

#### Step1. Register Plugin(Rust)

> Initialize the plugin in your main.rs or lib.rs:

```rs
use rgbitsoft_tauri_plugin_unified_log;

fn main() {
    tauri::Builder::default()
        .plugin(rgbitsoft_tauri_plugin_unified_log::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### Step2. Configure Permissions

> Tauri 2.0 requires explicit security permissions. Add unified-log:default to your capability configuration (e.g., src-tauri/capabilities/default.json)

```json
{
  "permissions": [
    "core:default",
    "unified-log:default"
  ]
}
```