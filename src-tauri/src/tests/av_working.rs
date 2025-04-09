use std::{
  fs::{self},
  io::Read,
  path::Path,
};

use crate::logger::{add_log, add_log_str};

#[tauri::command]
pub fn test_av_working() -> String {
  add_log_str(
    "Checking working antivirus",
    "Проверка работоспособности антивирусного ПО",
  );
  let defender;
  let kaspersky;

  match check_windows_defender_working() {
    Ok(state) => {
      defender = state;
    }
    Err(_) => {
      add_log_str(
        "Windows defender check failed",
        "Ошибка проверки Windows Defender",
      );
      return "failed".to_owned();
    }
  };

  match check_kaspersky_working() {
    Ok(state) => {
      kaspersky = state;
    }
    Err(_) => {
      add_log_str("Kaspersky check failed", "Ошибка проверки Kaspersky");
      return "failed".to_owned();
    }
  };

  if defender || kaspersky {
    "enabled".to_owned()
  } else {
    "disabled".to_owned()
  }
}

fn check_windows_defender_working() -> Result<bool, ()> {
  // println!("Checking windows defender is working");
  match std::process::Command::new("sc")
    .arg("query")
    .arg("WinDefend")
    .stdout(std::process::Stdio::piped())
    .spawn()
  {
    Ok(mut v) => {
      let mut result = String::new();
      let stdout = v.stdout.take();
      if let Some(mut response) = stdout {
        match response.read_to_string(&mut result) {
          Ok(_) => {
            if result.split("\n").collect::<Vec<&str>>()[3].contains("STOPPED") {
              add_log_str(
                "Windows defender is NOT working",
                "Windows defender НЕ запущен",
              );
              Ok(false)
            } else {
              add_log_str("Windows defender is working", "Windows defender запущен");
              Ok(true)
            }
          }
          Err(error) => {
            add_log(
              format!("Error searching for windows defender: {}", error),
              format!("Ошибка поиска Windows Defender: {}", error),
            );
            Err(())
          }
        }
      } else {
        Err(())
      }
    }
    Err(error) => {
      add_log(
        format!(
          "Failed to spawn command to check windows defender is enabled: {}",
          error
        ),
        format!(
          "Ошибка запуска команды проверки работоспособности Windows Defender: {}",
          error
        ),
      );
      Err(())
    }
  }
}

fn check_kaspersky_working() -> Result<bool, ()> {
  // println!("Checking kaspersky is working");

  let exec_path = match get_kaspersky_path() {
    Some(path) => path,
    None => {
      add_log_str("Kaspersky executable path not found", "Kaspersky не найден");
      return Err(());
    }
  };

  match std::process::Command::new(exec_path)
    .arg("STATUS")
    .stdout(std::process::Stdio::piped())
    .spawn()
  {
    Ok(mut v) => {
      let mut result = String::new();
      let stdout = v.stdout.take();
      if let Some(mut response) = stdout {
        match response.read_to_string(&mut result) {
          Ok(_) => {
            let status = parse_kaspersky_status(result);
            if status {
              add_log_str("Kaspersky is working", "Kaspersky запущен");
            } else {
              add_log_str("Kaspersky is NOT working", "Kaspersky НЕ запущен");
            }
            Ok(status)
          }
          Err(error) => {
            add_log(
              format!("Error requesting Kaspersky task status: {}", error),
              format!("Ошибка запроса статуса Kaspersky: {}", error),
            );
            Err(())
          }
        }
      } else {
        Err(())
      }
    }
    Err(error) => {
      add_log(
        format!(
          "Failed to spawn command to check Kaspersky is enabled: {}",
          error
        ),
        format!(
          "Ошибка запуска команды проверки работоспособности Kaspersky: {}",
          error
        ),
      );
      Err(())
    }
  }
}

fn get_kaspersky_path() -> Option<String> {
  let base_path = Path::new("C:\\Program Files (x86)\\Kaspersky Lab");
  let versions = match fs::read_dir(base_path) {
    Ok(paths) => paths,
    Err(error) => {
      add_log(
        format!("Error reading Kaspersky home directory: {}", error),
        format!("Ошибка чтения домашней директории Kaspersky: {}", error),
      );
      return None;
    }
  };

  let mut versions_array: Vec<String> = Vec::new();

  for version in versions {
    let version = version.unwrap();
    let version_name = version.file_name().to_str().unwrap().to_owned();

    versions_array.push(version_name.clone());

    if version_name.contains("KES.") {
      add_log(
        format!(
          "Found Kaspersky version: {}",
          version.file_name().to_str().unwrap_or("ERROR")
        ),
        format!(
          "Найдена версия Kaspersky: {}",
          version.file_name().to_str().unwrap_or("ОШИБКА")
        ),
      );
      let kaspersky_path = version.path();
      return Some(String::from(
        kaspersky_path.join("avp.com").to_str().unwrap(),
      ));
    }
  }

  add_log(
    format!("Kaspersky not found, all versions: {:?}", versions_array),
    format!("Kaspersky не найден, все версии: {:?}", versions_array),
  );

  None
}

fn parse_kaspersky_status(status: String) -> bool {
  let lines: Vec<&str> = status.split("\n").collect();

  let mut file_monitoring = false;
  let mut exploit_prevention = false;
  let mut proc_mon = false;
  let mut protection = false;

  for line in lines {
    if line.contains("File_Monitoring") && line.contains("running") {
      file_monitoring = true
    }
    if line.contains("ExploitPrevention") && line.contains("running") {
      exploit_prevention = true
    }
    if line.contains("ProcMon") && line.contains("running") {
      proc_mon = true
    }
    if line.contains("Protection") && line.contains("running") {
      protection = true
    }
  }

  return file_monitoring && exploit_prevention && proc_mon && protection;
}
