import "./style.css";
import App from "./App.svelte";
import { mount } from 'svelte';

import { initUnifiedLog } from "@rgbitsoft/tauri-plugin-unified-log";
// import { initUnifiedLog } from "../../../guest-js/index.js";

// 🔥 입맛대로 설정하는 로그의 안식
initUnifiedLog({
  printInBrowser: true,   
  prettyJson: false       
});

const app = mount(App, {
  target: document.getElementById("app"),
});

export default app;
