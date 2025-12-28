<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  let status = "Click Me!";

  function testLog() {
    console.log("console.log Test");
    console.info("console.info Test");
    console.warn("console.warn Test");
    console.error("console.error Test");
  }

  function testJson() {
    const updateInfo = {
      available: true,
      current_version: "0.0.9",
      latest_version: "0.1.0",
      body: "To Void"
    };
    
    console.log(updateInfo);
  }
  async function testServerLog() {
    try {
      await invoke('trigger_server_log'); // Rust 커맨드 호출
    } catch (e) {
      console.error("서버 호출 실패:", e);
      status = "호출 실패 ㅠㅠ";
    }
  }
</script>

<main class="container">
  <h1>Unified Log Tester</h1>
  <p>{status}</p>

  <div class="row">
    <button on:click={testLog}>Click(FrontEnd-String)</button>
    <button on:click={testJson}>Click(FrontEnd-JSON)</button>
    <button class="server-btn" on:click={testServerLog}>Server Log Trigger</button>
  </div>
</main>

<style>
  .container {
    padding: 2rem;
    text-align: center;
  }
  .row {
    display: flex;
    justify-content: center;
    gap: 1rem;
    margin-top: 1rem;
  }
  button {
    padding: 0.6em 1.2em;
    cursor: pointer;
  }
</style>