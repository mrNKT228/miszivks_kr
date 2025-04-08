#[tauri::command]
fn test_inet() -> String {
    match std::process::Command::new("ping")
        .arg("ya.ru")
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(mut v) => {
            let result = v.wait();
            if let Ok(exit_code) = result {
                match exit_code.code() {
                    None => "failed".to_owned(),
                    Some(exit_code) => match exit_code {
                        0 => "enabled".to_owned(),
                        _ => "disabled".to_owned(),
                    },
                }
            } else {
                "failed".to_owned()
            }
        }
        Err(_) => "failed".to_owned(),
    }
}

#[tauri::command]
fn test_av_installed() -> String {
    match std::process::Command::new("sc")
        .arg("query")
        .arg("WinDefend")
        .stdout(std::process::Stdio::piped())
        .spawn()
    {
        Ok(mut v) => {
            let result = v.wait();
            if let Ok(exit_code) = result {
                match exit_code.code() {
                    None => "failed".to_owned(),
                    Some(exit_code) => match exit_code {
                        0 => "enabled".to_owned(),
                        _ => "disabled".to_owned(),
                    },
                }
            } else {
                "failed".to_owned()
            }
        }
        Err(_) => "failed".to_owned(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![test_inet, test_av_installed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
