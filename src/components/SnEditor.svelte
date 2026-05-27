<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";

  let sn = "";
  let newSn = "";
  let loading = false;
  let message = "";
  let isError = false;

  onMount(async () => {
    await loadSn();
  });

  async function loadSn() {
    try {
      sn = await invoke("read_sn");
      newSn = sn;
    } catch (e) {
      sn = "读取失败";
      console.error("Failed to read SN:", e);
    }
  }

  async function handleWrite() {
    if (!newSn.trim()) {
      showMessage("SN 码不能为空", true);
      return;
    }

    loading = true;
    try {
      await invoke("write_sn", { sn: newSn });
      sn = newSn;
      showMessage("SN 码写入成功", false);
    } catch (e) {
      showMessage("写入失败: " + e, true);
    } finally {
      loading = false;
    }
  }

  function showMessage(text: string, error: boolean) {
    message = text;
    isError = error;
    setTimeout(() => (message = ""), 3000);
  }
</script>

<div class="sn-editor">
  <div class="card">
    <h2>主板 SN 码管理</h2>

    <div class="field">
      <label>当前 SN 码</label>
      <div class="current-sn">{sn || "加载中..."}</div>
    </div>

    <div class="field">
      <label for="new-sn">新 SN 码</label>
      <input
        id="new-sn"
        type="text"
        bind:value={newSn}
        placeholder="输入新的 SN 码"
        disabled={loading}
      />
    </div>

    <div class="actions">
      <button class="btn-refresh" on:click={loadSn} disabled={loading}>
        刷新
      </button>
      <button class="btn-write" on:click={handleWrite} disabled={loading || newSn === sn}>
        {loading ? "写入中..." : "写入 SN"}
      </button>
    </div>

    {#if message}
      <div class="message" class:error={isError} class:success={!isError}>
        {message}
      </div>
    {/if}

    <div class="warning">
      <strong>注意：</strong>写入 SN 码需要管理员权限，且写入成功后需重启电脑才能全局生效。
    </div>
  </div>
</div>

<style>
  .sn-editor {
    max-width: 600px;
    margin: 0 auto;
  }

  .card {
    background: var(--card-bg);
    border: 1px solid var(--border);
    border-radius: 0.75rem;
    padding: 2rem;
  }

  h2 {
    font-size: 1.4rem;
    margin-bottom: 1.5rem;
    color: var(--text);
  }

  .field {
    margin-bottom: 1.5rem;
  }

  label {
    display: block;
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--text-secondary);
    margin-bottom: 0.5rem;
  }

  .current-sn {
    padding: 0.75rem 1rem;
    background: var(--bg);
    border-radius: 0.5rem;
    font-family: monospace;
    font-size: 1.1rem;
    color: var(--text);
  }

  input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    font-size: 1rem;
    transition: border-color 0.2s;
  }

  input:focus {
    border-color: var(--primary);
  }

  input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .actions {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
  }

  button {
    flex: 1;
    padding: 0.75rem 1.5rem;
    border-radius: 0.5rem;
    font-size: 1rem;
    font-weight: 500;
    transition: all 0.2s;
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-refresh {
    background: var(--bg);
    color: var(--text);
  }

  .btn-refresh:hover:not(:disabled) {
    background: var(--border);
  }

  .btn-write {
    background: var(--primary);
    color: white;
  }

  .btn-write:hover:not(:disabled) {
    background: var(--primary-hover);
  }

  .message {
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    font-size: 0.9rem;
    margin-bottom: 1rem;
  }

  .message.success {
    background: #ecfdf5;
    color: var(--success);
  }

  .message.error {
    background: #fef2f2;
    color: var(--error);
  }

  .warning {
    padding: 1rem;
    background: #fffbeb;
    border-radius: 0.5rem;
    font-size: 0.85rem;
    color: #92400e;
    line-height: 1.5;
  }
</style>
