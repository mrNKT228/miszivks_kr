use std::path::Path;

use crate::logger::{add_log, add_log_str};

#[tauri::command]
pub fn test_av_installed() -> String {
  add_log_str(
    "Checking installed antivirus",
    "Проверка установленного антивирусного ПО",
  );

  let defender = check_windows_defender_installed();
  let kaspersky = check_kaspersky_installed();

  add_log(
    format!(
      "Windows Defender is installed: {}, Kaspersky is installed: {}",
      defender, kaspersky
    ),
    format!(
      "Windows Defender установлен: {}, Kaspersky установлен: {}",
      defender, kaspersky
    ),
  );

  if defender || kaspersky {
    "enabled".to_owned()
  } else {
    "disabled".to_owned()
  }
}

fn check_windows_defender_installed() -> bool {
  Path::new("C:\\Program Files\\Windows Defender").exists()
}

fn check_kaspersky_installed() -> bool {
  Path::new("C:\\Program Files (x86)\\Kaspersky Lab").exists()
}
