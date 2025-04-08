import { invoke } from "@tauri-apps/api/core";
import { T_TestReturnState } from "./layouts/test.layout";

export function inet(): Promise<T_TestReturnState> {
  return new Promise((resolve, _) => {
    console.log('Starting "inet" test');
    invoke("test_inet")
      .then((result) => {
        console.log(`"inet" test result: ${result}`);
        resolve(result as T_TestReturnState);
      })
      .catch((error) => {
        console.error('"inet" test error:', error);
        resolve("failed");
      });
  });
}

export function fw_installed(): Promise<T_TestReturnState> {
  return new Promise((resolve, _) => {
    console.log('Starting "fw_installed" test');
    invoke("test_fw_installed")
      .then((result) => {
        console.log(`"fw_installed" test result: ${result}`);
        resolve(result as T_TestReturnState);
      })
      .catch((error) => {
        console.error('"fw_installed" test error:', error);
        resolve("failed");
      });
  });
}

export function fw_working(): Promise<T_TestReturnState> {
  return new Promise((resolve, _) => {
    console.log('Starting "fw_working" test');
    invoke("test_fw_working")
      .then((result) => {
        console.log(`"fw_working" test result: ${result}`);
        resolve(result as T_TestReturnState);
      })
      .catch((error) => {
        console.error('"fw_working" test error:', error);
        resolve("failed");
      });
  });
}

export function av_installed(): Promise<T_TestReturnState> {
  return new Promise((resolve, _) => {
    console.log('Starting "av_installed" test');
    invoke("test_av_installed")
      .then((result) => {
        console.log(`"av_installed" test result: ${result}`);
        resolve(result as T_TestReturnState);
      })
      .catch((error) => {
        console.error('"av_installed" test error:', error);
        resolve("failed");
      });
  });
}

export function av_working(): Promise<T_TestReturnState> {
  return new Promise((resolve, _) => {
    console.log('Starting "av_working" test');
    invoke("test_av_working")
      .then((result) => {
        console.log(`"av_working" test result: ${result}`);
        resolve(result as T_TestReturnState);
      })
      .catch((error) => {
        console.error('"av_working" test error:', error);
        resolve("failed");
      });
  });
}

export function av_test(): Promise<T_TestReturnState> {
  return new Promise((resolve, _) => {
    console.log('Starting "av_test" test');
    invoke("test_av_test")
      .then((result) => {
        console.log(`"av_test" test result: ${result}`);
        resolve(result as T_TestReturnState);
      })
      .catch((error) => {
        console.error('"av_test" test error:', error);
        resolve("failed");
      });
  });
}
