<script lang="ts">
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

// import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";

// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

type T_DataLayout = {
  tests_result: {
    inet: T_TestReturnState;
    fw_installed: T_TestReturnState;
    fw_working: T_TestReturnState;
    av_installed: T_TestReturnState;
    av_working: T_TestReturnState;
    av_test: T_TestReturnState;
  };
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
  }),
  methods: {
    test_inet() {
      inet().then((result) => {
        this.tests_result.inet = result;
      });
    },
    test_fw_installed() {
      fw_installed().then((result) => {
        this.tests_result.fw_installed = result;
      });
    },
    test_fw_working() {
      fw_working().then((result) => {
        this.tests_result.fw_working = result;
      });
    },
    test_av_installed() {
      av_installed().then((result) => {
        this.tests_result.av_installed = result;
      });
    },
    test_av_working() {
      av_working().then((result) => {
        this.tests_result.av_working = result;
      });
    },
    test_av_test() {
      av_test().then((result) => {
        this.tests_result.av_test = result;
      });
    },
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
  font-weight: 500;
  font-size: 24px;
  margin-bottom: 0.5rem;
}
</style>
