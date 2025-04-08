<script lang="ts">
import Button from "primevue/button";
import { PropType } from "vue";
import { T_TestReturnState } from "../layouts/test.layout";

type T_ButtonSeverity =
  | "secondary"
  | "success"
  | "info"
  | "warn"
  | "help"
  | "danger"
  | "contrast"
  | undefined;

export default {
  name: "Test",
  props: {
    title: {
      type: String,
      required: true,
    },
    state: {
      type: String as PropType<T_TestReturnState>,
      required: true,
    },
  },
  emits: ["start"],
  computed: {
    buttonLabel(): string {
      switch (this.state) {
        case "disabled":
          return "Не успешно";
        case "enabled":
          return "Успешно";
        case "failed":
          return "Ошибка";
        case "not_checked_yet":
          return "Запустить";
        case "running":
          return "В процессе";
        case undefined:
          return "Запустить";
        default:
          return `Неизвестное состояние "${this.state}"`;
      }
    },
    buttonSeverity(): T_ButtonSeverity {
      switch (this.state) {
        case "disabled":
          return "warn";
        case "enabled":
          return "success";
        case "failed":
          return "danger";
        case "not_checked_yet":
          return undefined;
        case "running":
          return "info";
        case undefined:
          return undefined;
        default:
          return undefined;
      }
    },
  },
  components: {
    Button,
  },
};
</script>

<template>
  <div class="test">
    <span class="test-name">{{ title }}</span>
    <Button
      class="test-action"
      :label="buttonLabel"
      :severity="buttonSeverity"
      :loading="state == 'running'"
      @click="$emit('start')"
    ></Button>
  </div>
</template>

<style lang="css" scoped>
.test {
  width: 100%;
  display: flex;
  gap: 1rem;
  justify-content: space-between;
  align-items: center;

  transition: all 250ms ease-in-out;
  padding: 0.5rem;
  border-radius: 0.6rem;
  flex-wrap: wrap;
}

.test:hover {
  background-color: #252525;
}

.test-name {
  color: rgb(179, 179, 179);
}

.test-action {
  width: 9rem;
}
</style>
