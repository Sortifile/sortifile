import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";
//import "./assets/global.css"; // global css
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import "./assets/element-variables.scss";
//import "element-plus/theme-chalk/dark/css-vars.css";

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.use(ElementPlus);
app.mount("#app");
