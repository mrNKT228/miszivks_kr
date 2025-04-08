import { createApp } from "vue";
import App from "./App.vue";
// PRIMEVUE
import Aura from "@primeuix/themes/aura";
import PrimeVue from "primevue/config";
import { Button } from "primevue";
// -PRIMEVUE-

const app = createApp(App);
app.use(PrimeVue, {
  theme: {
    preset: Aura,
  },
});

// PRIMEVUE
app.component("Button", Button);
// -PRIMEVUE-

app.mount("#app");
