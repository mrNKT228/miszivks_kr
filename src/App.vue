<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import Test from "./components/Test.vue";
import { T_TestReturnState } from "./layouts/test.layout";
import {
  av_installed,
  av_test,
  av_working,
  fw_installed,
  fw_working,
  inet,
} from "./tests";
import { formatDate } from "./misc";

type T_Log = { en_msg: string; ru_msg: string; timestamp: number }[];

type T_DataLayout = {
  tests_result: {
    inet: T_TestReturnState;
    fw_installed: T_TestReturnState;
    fw_working: T_TestReturnState;
    av_installed: T_TestReturnState;
    av_working: T_TestReturnState;
    av_test: T_TestReturnState;
  };
  log: T_Log;
  russian_language: boolean;
};

export default {
  name: "App",
  components: {
    Test,
  },
  data: (): T_DataLayout => ({
    tests_result: {
      inet: "not_checked_yet",
      fw_installed: "not_checked_yet",
      fw_working: "not_checked_yet",
      av_installed: "not_checked_yet",
      av_working: "not_checked_yet",
      av_test: "not_checked_yet",
    },
    log: [],
    russian_language: true,
  }),
  methods: {
    test_inet() {
      this.tests_result.inet = "running";
      inet().then((result) => {
        this.tests_result.inet = result;
      });
    },
    test_fw_installed() {
      this.tests_result.fw_installed = "running";
      fw_installed().then((result) => {
        this.tests_result.fw_installed = result;
      });
    },
    test_fw_working() {
      this.tests_result.fw_working = "running";
      fw_working().then((result) => {
        this.tests_result.fw_working = result;
      });
    },
    test_av_installed() {
      this.tests_result.av_installed = "running";
      av_installed().then((result) => {
        this.tests_result.av_installed = result;
      });
    },
    test_av_working() {
      this.tests_result.av_working = "running";
      av_working().then((result) => {
        this.tests_result.av_working = result;
      });
    },
    test_av_test() {
      this.tests_result.av_test = "running";
      av_test().then((result) => {
        this.tests_result.av_test = result;
      });
    },

    get_log() {
      invoke("get_log").then((log) => {
        console.log("Received log:", log);
        this.log = log as T_Log;
      });
    },
  },
  computed: {
    logByLanguage(): { msg: string; time: string }[] {
      if (this.russian_language) {
        return this.log.map((log_entry) => ({
          msg: log_entry.ru_msg,
          time: formatDate(new Date(log_entry.timestamp * 1000)),
        }));
      } else {
        return this.log.map((log_entry) => ({
          msg: log_entry.en_msg,
          time: formatDate(new Date(log_entry.timestamp * 1000)),
        }));
      }
    },
  },
  mounted() {
    setInterval(() => {
      this.get_log();
    }, 1000);
  },
};
</script>

<template>
  <main class="container">
    <div class="test-block-container">
      <div class="title">Проверка межсетевого экрана</div>
      <Test
        title="Проверка подключения к Интернету"
        :state="tests_result.inet"
        @start="test_inet()"
      ></Test>
      <Test
        title="Проверка наличия установленного межсетевого экрана"
        :state="tests_result.fw_installed"
        @start="test_fw_installed()"
      ></Test>
      <Test
        title="Проверка работоспособности межсетевого экрана"
        :state="tests_result.fw_working"
        @start="test_fw_working()"
      ></Test>
    </div>
    <div class="test-block-container">
      <div class="title">Проверка антивирусного ПО</div>
      <Test
        title="Проверка наличия установленного антивируса"
        :state="tests_result.av_installed"
        @start="test_av_installed()"
      ></Test>
      <Test
        title="Проверка работоспособности антивирусного ПО"
        :state="tests_result.av_working"
        @start="test_av_working()"
      ></Test>
      <Test
        title="Тестирование антивирусного ПО"
        :state="tests_result.av_test"
        @start="test_av_test()"
      ></Test>
    </div>
    <div class="test-block-container">
      <div class="title">
        <span> Лог </span>
        <div class="language-button">
          <span>Русский: </span>
          <ToggleSwitch v-model="russian_language"></ToggleSwitch>
        </div>
      </div>
      <div class="log">
        <div class="log-entry" v-for="log_entry in logByLanguage">
          <span class="date">{{ log_entry.time }}</span>
          <span class="message">
            {{ log_entry.msg }}
          </span>
        </div>
      </div>
    </div>
  </main>
</template>

<style lang="css" scoped>
.container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem;
}
.test-block-container {
  display: flex;
  flex-direction: column;
  align-items: start;
  gap: 1rem;
  border: 1px solid gray;
  border-radius: 1rem;
  padding: 1.5rem;
}

.test-block-container .title {
  width: 100%;
  font-weight: 500;
  font-size: 24px;
  margin-bottom: 0.5rem;
  display: flex;
  justify-content: space-between;
}

.log {
  max-height: 15rem;
  height: 15rem;
  overflow-y: auto;
  gap: 0.5rem;
  display: flex;
  flex-direction: column;
  width: 100%;
  padding-bottom: 0.2rem;
  padding-top: 0.2rem;
}
.log-entry {
  padding: 0.4rem;
  width: 100%;
  background-color: #2a2a2a;
  border-radius: 0.4rem;
  display: flex;
  gap: 0.5rem;
}

.log-entry .date {
  color: #898989;
  font-weight: 300;
}

.log-entry .message {
  user-select: text;
}

.language-button {
  display: flex;
  align-items: center;
  font-size: 1rem;
  gap: 0.5rem;
}
</style>
