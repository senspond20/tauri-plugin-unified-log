import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { initUnifiedLog } from "@rgbitsoft/tauri-plugin-unified-log";
initUnifiedLog()

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
