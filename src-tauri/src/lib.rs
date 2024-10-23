mod lcu;
#[cfg(test)]
mod tests;

use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub static DIRS: Lazy<ProjectDirs> = Lazy::new(|| {
    ProjectDirs::from("com", "winstonyli", "Roanluder")
        .expect("could not retrieve valid home directory path from OS")
});

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
