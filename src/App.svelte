<script lang="ts">
  import ToolCard from "./components/ToolCard.svelte";
  import SnEditor from "./components/SnEditor.svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface Tool {
    id: string;
    name: string;
    description: string;
    icon: string;
    path: string;
  }

  let tools: Tool[] = [];
  let activeTab: "tools" | "sn" = "tools";

  onMount(async () => {
    try {
      tools = await invoke("get_tools");
    } catch (e) {
      console.error("Failed to load tools:", e);
    }
  });

  async function handleLaunch(toolPath: string) {
    try {
      await invoke("launch_tool", { toolPath });
    } catch (e) {
      alert("启动失败: " + e);
    }
  }
</script>

<main>
  <header>
    <h1>iTools</h1>
    <nav>
      <button
        class:active={activeTab === "tools"}
        on:click={() => (activeTab = "tools")}
      >
        工具箱
      </button>
      <button
        class:active={activeTab === "sn"}
        on:click={() => (activeTab = "sn")}
      >
        SN 码管理
      </button>
    </nav>
  </header>

  <div class="content">
    {#if activeTab === "tools"}
      <div class="tools-grid">
        {#each tools as tool}
          <ToolCard {tool} on:launch={() => handleLaunch(tool.path)} />
        {:else}
          <p class="empty">暂无工具，请在 config/config.json 中配置</p>
        {/each}
      </div>
    {:else}
      <SnEditor />
    {/if}
  </div>
</main>

<style>
  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--border);
  }

  h1 {
    font-size: 1.8rem;
    color: var(--primary);
  }

  nav {
    display: flex;
    gap: 0.5rem;
  }

  nav button {
    padding: 0.6rem 1.2rem;
    border-radius: 0.5rem;
    background: var(--card-bg);
    color: var(--text-secondary);
    font-size: 0.95rem;
    transition: all 0.2s;
  }

  nav button:hover {
    background: var(--border);
  }

  nav button.active {
    background: var(--primary);
    color: white;
  }

  .content {
    min-height: 400px;
  }

  .tools-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1rem;
  }

  .empty {
    grid-column: 1 / -1;
    text-align: center;
    padding: 3rem;
    color: var(--text-secondary);
  }
</style>
