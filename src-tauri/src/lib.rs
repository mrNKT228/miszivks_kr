mod logger;
mod tests;

use tests::av_installed::test_av_installed;
use tests::av_test::test_av_test;
use tests::av_working::test_av_working;
use tests::fw_installed::test_fw_installed;
use tests::fw_working::test_fw_working;
use tests::inet::test_inet;

use logger::get_log;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      test_inet,
      test_fw_installed,
      test_fw_working,
      test_av_installed,
      test_av_working,
      test_av_test,
      get_log
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
