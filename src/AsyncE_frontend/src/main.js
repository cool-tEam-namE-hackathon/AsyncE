import { createApp } from "vue";
import { createPinia } from "pinia";
import { MotionPlugin } from "@vueuse/motion";
import "@/index.css";
import router from "./routes";
import App from "@/App.vue";

const app = createApp(App);

app.use(createPinia());
app.use(MotionPlugin);
app.use(router);

app.mount("#app");
