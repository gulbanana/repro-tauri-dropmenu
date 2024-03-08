<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  async function onMenu() {
    await invoke("menu");
  }
</script>

<main class="container">
  <div class="row">
    <p>
      <button on:click={onMenu}>Open context menu</button>
    </p>
  </div>
</main>
