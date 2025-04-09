use std::{
  sync::Mutex,
  time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct LogEntry {
  pub(crate) ru_msg: String,
  pub(crate) en_msg: String,
  pub(crate) timestamp: u64,
}

static LOG: Mutex<Vec<LogEntry>> = Mutex::new(Vec::new());

pub fn add_log_str(en: &str, ru: &str) {
  add_log(en.to_owned(), ru.to_owned());
}

pub fn add_log(en: String, ru: String) {
  println!("{}", en);
  let mut log = LOG.lock().unwrap();
  log.push(LogEntry {
    ru_msg: ru,
    en_msg: en,
    timestamp: get_timestamp(),
  })
}

#[tauri::command]
pub fn get_log() -> Vec<LogEntry> {
  let log = LOG.lock().unwrap();
  log.iter().map(|log_entry| log_entry.clone()).collect()
}

pub fn get_timestamp() -> u64 {
  match SystemTime::now().duration_since(UNIX_EPOCH) {
    Ok(duration) => duration.as_secs(),
    Err(_) => {
      panic!("Time went backwards!");
    }
  }
}
