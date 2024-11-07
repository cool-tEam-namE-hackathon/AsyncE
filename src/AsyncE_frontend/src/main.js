import { MotionPlugin } from "@vueuse/motion";
import { createPinia } from "pinia";
import { createApp } from "vue";
import "@/index.css";
import router from "./routes";
import App from "@/App.vue";

const app = createApp(App);

app.use(createPinia());
app.use(MotionPlugin);
app.use(router);

app.mount("#app");
