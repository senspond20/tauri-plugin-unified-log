// build.rs
const COMMANDS: &[&str] = &["log_terminal"]; 

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .build();
}