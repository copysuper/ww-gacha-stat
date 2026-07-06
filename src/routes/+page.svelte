<script lang="ts">
  import { goto } from "$app/navigation";
  import { getAppConfig } from "$lib/api/tauri/settings";
  import Panel from "$lib/components/common/Panel.svelte";
  import type { AppConfigState } from "$lib/types/dto";

  let appConfigState = $state<AppConfigState | null>(null);
  let loading = $state(true);
  let errorMessage = $state("");

  async function loadConfig() {
    loading = true;
    errorMessage = "";

    try {
      appConfigState = await getAppConfig();
    } catch (error) {
      errorMessage = error instanceof Error ? error.message : "读取配置失败";
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    loadConfig();
  });
</script>

<svelte:head>
  <title>鸣潮抽卡分析</title>
</svelte:head>

<main class="page-shell">
  <section class="hero">
    <div class="hero-copy">
      <p class="eyebrow">ww-gacha-stat</p>
      <h1>鸣潮抽卡分析</h1>
      <p class="summary">
        当前阶段已完成应用骨架、配置读写入口和前后端边界搭建。下一步将接入离线抽卡数据分析核心。
      </p>
    </div>

    <div class="hero-actions">
      <button class="ghost-button" type="button" onclick={() => goto('/settings')}>
        打开设置
      </button>
      <button class="primary-button" type="button" onclick={loadConfig} disabled={loading}>
        {#if loading}刷新中...{:else}刷新配置{/if}
      </button>
    </div>
  </section>

  <section class="grid">
    <Panel title="当前开发阶段" description="对照实现路线，当前正在完成第 1 阶段基础设施搭建。">
      <ul class="bullet-list">
        <li>已移除默认 Hello World 页面</li>
        <li>已建立 Rust 配置模块与统一响应结构</li>
        <li>已建立前端 Tauri API 封装入口</li>
        <li>下一步将接入本地抽卡分析与记录存储模块</li>
      </ul>
    </Panel>

    <Panel title="应用配置" description="这里先展示 Rust 侧默认配置和解析出来的应用目录。">
      {#if loading}
        <p class="status-text">正在从 Rust 读取配置...</p>
      {:else if errorMessage}
        <p class="error-text">{errorMessage}</p>
      {:else if appConfigState}
        <dl class="meta-grid">
          <div>
            <dt>配置文件</dt>
            <dd>{appConfigState.configFilePath}</dd>
          </div>
          <div>
            <dt>数据目录</dt>
            <dd>{appConfigState.resolvedDataDir}</dd>
          </div>
          <div>
            <dt>资源目录</dt>
            <dd>{appConfigState.resolvedAssetsDir}</dd>
          </div>
          <div>
            <dt>资源源</dt>
            <dd>{appConfigState.config.resourceSource}</dd>
          </div>
          <div>
            <dt>日志级别</dt>
            <dd>{appConfigState.config.logLevel}</dd>
          </div>
          <div>
            <dt>跳过首个五星</dt>
            <dd>{appConfigState.config.skipFirstSSR ? "是" : "否"}</dd>
          </div>
        </dl>
      {/if}
    </Panel>

    <Panel title="后续模块边界" description="后续会严格按这些模块向内推进，而不是继续把逻辑堆在页面里。">
      <div class="tag-list">
        <span>config</span>
        <span>gacha_log</span>
        <span>gacha_params</span>
        <span>gacha_api</span>
        <span>gacha_storage</span>
        <span>gacha_merge</span>
        <span>gacha_analysis</span>
        <span>resource_sync</span>
        <span>commands</span>
      </div>
    </Panel>
  </section>
</main>

<style>
  :global(body) {
    margin: 0;
    min-height: 100vh;
    font-family: Inter, "PingFang SC", "Microsoft YaHei", sans-serif;
    background:
      radial-gradient(circle at top left, rgba(59, 130, 246, 0.18), transparent 32%),
      radial-gradient(circle at top right, rgba(168, 85, 247, 0.16), transparent 28%),
      linear-gradient(180deg, #020617 0%, #0f172a 52%, #111827 100%);
    color: #e2e8f0;
  }

  .page-shell {
    max-width: 1180px;
    margin: 0 auto;
    padding: 3rem 1.25rem 4rem;
  }

  .hero {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    gap: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .hero-copy {
    max-width: 780px;
  }

  .eyebrow {
    margin: 0 0 0.85rem;
    color: #93c5fd;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    font-size: 0.82rem;
    font-weight: 700;
  }

  h1 {
    margin: 0;
    font-size: clamp(2rem, 6vw, 3.5rem);
    line-height: 1.08;
    color: #f8fafc;
  }

  .summary {
    margin: 1rem 0 0;
    max-width: 58rem;
    color: #cbd5e1;
    line-height: 1.7;
    font-size: 1rem;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(12, minmax(0, 1fr));
    gap: 1rem;
  }

  .grid :global(.panel:nth-child(1)) {
    grid-column: span 4;
  }

  .grid :global(.panel:nth-child(2)) {
    grid-column: span 8;
  }

  .grid :global(.panel:nth-child(3)) {
    grid-column: 1 / -1;
  }

  .hero-actions {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .primary-button {
    border: 0;
    border-radius: 999px;
    padding: 0.9rem 1.25rem;
    background: linear-gradient(135deg, #38bdf8 0%, #6366f1 100%);
    color: white;
    font-size: 0.95rem;
    font-weight: 700;
    cursor: pointer;
    box-shadow: 0 18px 45px rgba(59, 130, 246, 0.32);
  }

  .ghost-button {
    border: 1px solid rgba(148, 163, 184, 0.28);
    border-radius: 999px;
    padding: 0.9rem 1.25rem;
    background: rgba(15, 23, 42, 0.55);
    color: #e2e8f0;
    font-size: 0.95rem;
    font-weight: 700;
    cursor: pointer;
  }

  .primary-button:disabled {
    opacity: 0.7;
    cursor: wait;
  }

  .bullet-list {
    margin: 0;
    padding-left: 1.1rem;
    color: #cbd5e1;
    line-height: 1.75;
  }

  .status-text,
  .error-text {
    margin: 0;
    line-height: 1.7;
  }

  .status-text {
    color: #cbd5e1;
  }

  .error-text {
    color: #fca5a5;
  }

  .meta-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
    margin: 0;
  }

  .meta-grid div {
    padding: 0.9rem 1rem;
    border-radius: 14px;
    background: rgba(30, 41, 59, 0.7);
    border: 1px solid rgba(148, 163, 184, 0.16);
  }

  .meta-grid dt {
    margin-bottom: 0.4rem;
    color: #93c5fd;
    font-size: 0.82rem;
    font-weight: 700;
  }

  .meta-grid dd {
    margin: 0;
    color: #e2e8f0;
    word-break: break-all;
    line-height: 1.6;
  }

  .tag-list {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
  }

  .tag-list span {
    padding: 0.55rem 0.85rem;
    border-radius: 999px;
    background: rgba(59, 130, 246, 0.14);
    border: 1px solid rgba(96, 165, 250, 0.24);
    color: #bfdbfe;
    font-size: 0.9rem;
  }

  @media (max-width: 900px) {
    .hero {
      flex-direction: column;
      align-items: flex-start;
    }

    .hero-actions {
      width: 100%;
      flex-wrap: wrap;
    }

    .grid :global(.panel:nth-child(1)),
    .grid :global(.panel:nth-child(2)),
    .grid :global(.panel:nth-child(3)) {
      grid-column: 1 / -1;
    }

    .meta-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
