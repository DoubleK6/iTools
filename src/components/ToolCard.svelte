<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { convertFileSrc, invoke } from "@tauri-apps/api/core";

  export let tool: {
    id: string;
    name: string;
    description: string;
    icon: string;
    path: string;
  };

  const dispatch = createEventDispatcher();
  let imgError = false;
  $: iconSrc = tool.icon ? convertFileSrc(tool.icon) : "";

  function handleClick() {
    dispatch("launch");
  }

  function writeLog(level: "INFO" | "WARN" | "ERROR", message: string) {
    invoke("write_log", { level, message }).catch(() => {});
  }

  function handleImageLoad() {
    writeLog("INFO", `Icon image loaded for ${tool.id}: ${tool.icon} -> ${iconSrc}`);
  }

  function handleImageError() {
    imgError = true;
    writeLog("WARN", `Icon image failed for ${tool.id}: ${tool.icon} -> ${iconSrc}`);
  }
</script>

<button class="card" on:click={handleClick}>
  <div class="icon">
    {#if iconSrc && !imgError}
      <img src={iconSrc} alt={tool.name} on:load={handleImageLoad} on:error={handleImageError} />
    {:else}
      <span class="placeholder">{tool.name[0]}</span>
    {/if}
  </div>
  <h3>{tool.name}</h3>
  <p>{tool.description}</p>
</button>

<style>
  .card {
    background: var(--card-bg);
    border: 1px solid var(--border);
    border-radius: 0.75rem;
    padding: 1.5rem;
    text-align: center;
    transition: all 0.2s;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.75rem;
  }

  .card:hover {
    border-color: var(--primary);
    box-shadow: 0 4px 12px rgba(79, 70, 229, 0.15);
    transform: translateY(-2px);
  }

  .icon {
    width: 64px;
    height: 64px;
    border-radius: 1rem;
    background: var(--bg);
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  .icon img {
    width: 48px;
    height: 48px;
    object-fit: contain;
  }

  .placeholder {
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--primary);
  }

  h3 {
    font-size: 1rem;
    font-weight: 600;
  }

  p {
    font-size: 0.85rem;
    color: var(--text-secondary);
    line-height: 1.4;
  }
</style>
