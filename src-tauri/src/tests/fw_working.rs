use crate::logger::{add_log, add_log_str};

const CHECK_ADDR: &str = "http://fimatic-s.ru/";

#[tauri::command]
pub fn test_fw_working() -> String {
  add_log(
    format!("Checking connection to addr {}", CHECK_ADDR),
    format!("Проверка доступности адреса {}", CHECK_ADDR),
  );
  match std::process::Command::new("ping")
    .arg(CHECK_ADDR)
    .stdout(std::process::Stdio::piped())
    .spawn()
  {
    Ok(mut v) => {
      let result = v.wait();
      if let Ok(exit_code) = result {
        match exit_code.code() {
          None => "failed".to_owned(),
          Some(exit_code) => match exit_code {
            0 => {
              add_log_str(
                "Address is UP, firewall is disabled",
                "Адрес доступен, фаервол не работает",
              );
              "disabled".to_owned()
            }
            _ => {
              add_log_str(
                "Address is DOWN, firewall is enabled",
                "Адрес не доступен, фаервол работает",
              );
              "enabled".to_owned()
            }
          },
        }
      } else {
        "failed".to_owned()
      }
    }
    Err(_) => "failed".to_owned(),
  }
}
