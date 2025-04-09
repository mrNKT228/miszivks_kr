use crate::logger::{add_log, add_log_str};

#[tauri::command]
pub fn test_fw_installed() -> String {
  add_log_str(
    "Checking installed firewall",
    "Проверка установленного ПО firewall",
  );

  let brandmauer = check_windows_brandmauer_installed();

  add_log(
    format!("Windows Brandmauer is installed: {}", brandmauer),
    format!("Брандмауэр Windows установлен: {}", brandmauer),
  );

  if brandmauer {
    "enabled".to_owned()
  } else {
    "disabled".to_owned()
  }
}

fn check_windows_brandmauer_installed() -> bool {
  match std::process::Command::new("netsh.exe")
    .arg("advfirewall")
    .stdout(std::process::Stdio::piped())
    .spawn()
  {
    Ok(_) => true,
    Err(error) => {
      add_log(
        format!("Error spawning firewall command: {}", error),
        format!("Ошибка запуска команды фаервола: {}", error),
      );
      false
    }
  }
}
