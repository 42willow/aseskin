// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::ipc::Response;
use tauri::Manager;
use tauri_plugin_log::fern::colors;

#[tauri::command]
fn cli_export(app_handle: tauri::AppHandle, path: String) -> Result<Response, String> {
    log::info!("Exporting {}", path);

    let temp_dir = app_handle.path().temp_dir().map_err(|e| e.to_string())?;
    let temp_file = format!("{}/aseskin_temp.png", temp_dir.display());

    log::debug!("Temp file: {:?}", temp_file);

    let output = std::process::Command::new("aseprite")
        .arg("-b") // batch mode
        .arg(&path) // input file
        .arg("--save-as")
        .arg(&temp_file) // output file
        .output()
        .map_err(|e| e.to_string())?;
    log::debug!("Aseprite output: {:?}", output);

    if !output.status.success() {
        return Err(format!(
            "Aseprite CLI failed with status: {}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let data = std::fs::read(&temp_file).map_err(|e| e.to_string())?;
    Ok(tauri::ipc::Response::new(data))
    // Ok(tauri::ipc::Response::new("".as_bytes().to_vec()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .filter(|metadata| metadata.target() != "notify::inotify") // disable notify logging
                .with_colors(
                    colors::ColoredLevelConfig::new()
                        .error(colors::Color::Red)
                        .warn(colors::Color::Yellow)
                        .info(colors::Color::Blue)
                        .debug(colors::Color::Magenta)
                        .trace(colors::Color::Cyan),
                )
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![cli_export])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
