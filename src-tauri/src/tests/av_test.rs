use std::{fs::File, io::Write, path::Path};

use directories::UserDirs;

use crate::logger::{add_log, add_log_str, get_timestamp};

const EICAR: &[u8; 70] =
  b"\"X5O!P%@AP[4\\PZX54(P^)7CC)7}$EICAR-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*\"";

#[tauri::command]
pub fn test_av_test() -> String {
  match UserDirs::new() {
    Some(user_dirs) => match user_dirs.document_dir() {
      Some(path) => {
        add_log(
          format!(
            "Found user \"Documents\" directory: {}",
            path.to_str().unwrap()
          ),
          format!(
            "Найдена пользовательская папка \"Документы\": {}",
            path.to_str().unwrap()
          ),
        );
        match inject_eicar(path) {
          Ok(_) => return "enabled".to_owned(),
          Err(_) => return "disabled".to_owned(),
        }
      }
      None => {
        add_log_str(
          "Error getting user \"Document\" dir path",
          "Ошибка получения пользовательской директории \"Документы\"",
        );
        return "failed".to_owned();
      }
    },
    None => {
      add_log_str(
        "Error getting user home dir path",
        "Ошибка получения пользовательской директории",
      );
      return "failed".to_owned();
    }
  }
  "enabled".to_owned()
}

pub fn inject_eicar(path: &Path) -> Result<(), ()> {
  let timestamp = get_timestamp();
  let path = path.join(format!("eicar_test_file_{}.txt", timestamp));

  if path.exists() {
    add_log(
      format!(
        "Error creating EICAR file with path: \"{}\" (File already exist)",
        path.to_str().unwrap()
      ),
      format!(
        "Ошибка создания файла EICAR по адресу: \"{}\" (Файл уже существует)",
        path.to_str().unwrap()
      ),
    );
    return Err(());
  }

  match File::create(&path) {
    Ok(mut file) => match file.write_all(EICAR) {
      Ok(_) => {
        add_log(
          format!(
            "EICAR file created with path \"{}\":",
            path.to_str().unwrap()
          ),
          format!(
            "Файл EICAR успешно записан по адресу \"{}\":",
            path.to_str().unwrap()
          ),
        );
        Ok(())
      }
      Err(error) => {
        add_log(
          format!(
            "Error writing to EICAR file with path \"{}\": {}",
            path.to_str().unwrap(),
            error
          ),
          format!(
            "Ошибка записи в файл EICAR по адресу \"{}\": {}",
            path.to_str().unwrap(),
            error
          ),
        );
        Err(())
      }
    },
    Err(error) => {
      add_log(
        format!(
          "Error creating EICAR file with path \"{}\": {}",
          path.to_str().unwrap(),
          error
        ),
        format!(
          "Ошибка создания файла EICAR по адресу \"{}\": {}",
          path.to_str().unwrap(),
          error
        ),
      );
      Err(())
    }
  }
}
