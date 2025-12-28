import "./style.css";
import App from "./App.svelte";
import { mount } from 'svelte';

import { initUnifiedLog } from "@rgbitsoft/tauri-plugin-unified-log";
// import { initUnifiedLog } from "../../../guest-js/index.js";

// Bridges all browser logs (console.log, warn, error, ..) to the Rust terminal
initUnifiedLog({
  printInBrowser: true,   // Keep logs visible in the browser console
  prettyJson: false       // Set true to format JSON objects for better readability
});

const app = mount(App, {
  target: document.getElementById("app"),
});

export default app;
