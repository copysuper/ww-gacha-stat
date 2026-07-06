<script lang="ts">
  import { goto } from "$app/navigation";
  import { analyzeLocalPool } from "$lib/api/tauri/gacha";
  import { getAppConfig } from "$lib/api/tauri/settings";
  import Panel from "$lib/components/common/Panel.svelte";
  import type {
    AnalyzeLocalPoolResponse,
    AppConfigState,
    PoolRankSummary,
  } from "$lib/types/dto";

  let appConfigState = $state<AppConfigState | null>(null);
  let loading = $state(true);
  let errorMessage = $state("");
  let analyzeLoading = $state(false);
  let analyzeErrorMessage = $state("");
  let analyzeResult = $state<AnalyzeLocalPoolResponse | null>(null);
  let samplePoolPath = $state("/workspaces/ww-gacha-stat/doc/examples/sample-pool.json");

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

  async function runSampleAnalysis() {
    analyzeLoading = true;
    analyzeErrorMessage = "";

    try {
      analyzeResult = await analyzeLocalPool(samplePoolPath);
    } catch (error) {
      analyzeErrorMessage = error instanceof Error ? error.message : "离线分析失败";
    } finally {
      analyzeLoading = false;
    }
  }

  function formatAverage(value: number) {
    return Number.isInteger(value) ? String(value) : value.toFixed(2);
  }

  function formatRate(rate: number) {
    return `${(rate * 100).toFixed(2)}%`;
  }

  function formatRankSummary(rank: PoolRankSummary) {
    return `${rank.count} / ${formatRate(rank.rate)}`;
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
        当前阶段已完成离线抽卡分析最小链路，正在补齐总览映射、边界样例和第 2 阶段收尾验证。
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
    <Panel title="当前开发阶段" description="对照实现路线，当前正在完成第 2 阶段离线抽卡分析核心收尾。">
      <ul class="bullet-list">
        <li>已完成本地 `pool.json -> AnalysisData[]` 离线分析链路</li>
        <li>已完成 Tauri 离线分析命令与前端调试入口</li>
        <li>当前正在补齐总览摘要 DTO，降低页面与统计算法耦合</li>
        <li>下一步将补边界样例并准备进入第 3 阶段数据合并</li>
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

    <Panel title="第 2 阶段调试区" description="当前验证离线分析链路，并优先展示总览摘要：本地 pool.json -> AnalysisData / Summary。">
      <div class="debug-controls">
        <label class="field-group">
          <span>本地 `pool.json` 路径</span>
          <input bind:value={samplePoolPath} placeholder="输入本地 pool.json 路径" />
        </label>

        <button class="primary-button" type="button" onclick={runSampleAnalysis} disabled={analyzeLoading}>
          {#if analyzeLoading}分析中...{:else}运行离线分析{/if}
        </button>
      </div>

      {#if analyzeErrorMessage}
        <p class="error-text analysis-feedback">{analyzeErrorMessage}</p>
      {/if}

      {#if analyzeResult}
        <p class="status-text analysis-feedback">
          已分析文件：<code>{analyzeResult.filePath}</code>
        </p>

        <div class="analysis-grid">
          {#each analyzeResult.summaryList as summary, index}
            <article class="analysis-card">
              <header class="analysis-card-header">
                <div>
                  <h3>{summary.poolName}</h3>
                  <p>
                    {#if summary.isEmpty}
                      无记录
                    {:else}
                      {summary.startDate} ~ {summary.endDate}
                    {/if}
                  </p>
                </div>
              </header>

              <dl class="analysis-meta-grid">
                <div>
                  <dt>总抽数</dt>
                  <dd>{summary.totalCount}</dd>
                </div>
                <div>
                  <dt>5 星已垫</dt>
                  <dd>{summary.ssr.currentPity}</dd>
                </div>
                <div>
                  <dt>4 星已垫</dt>
                  <dd>{summary.sr.currentPity}</dd>
                </div>
                <div>
                  <dt>5 星数量</dt>
                  <dd>{formatRankSummary(summary.ssr)}</dd>
                </div>
                <div>
                  <dt>4 星数量</dt>
                  <dd>{formatRankSummary(summary.sr)}</dd>
                </div>
                <div>
                  <dt>3 星数量</dt>
                  <dd>{formatRankSummary(summary.r)}</dd>
                </div>
                <div>
                  <dt>5 星平均</dt>
                  <dd>{formatAverage(summary.ssr.avg)}</dd>
                </div>
                <div>
                  <dt>5 星最欧 / 最非</dt>
                  <dd>{summary.ssr.min} / {summary.ssr.max}</dd>
                </div>
                <div>
                  <dt>4 星平均</dt>
                  <dd>{formatAverage(summary.sr.avg)}</dd>
                </div>
                <div>
                  <dt>4 星最欧 / 最非</dt>
                  <dd>{summary.sr.min} / {summary.sr.max}</dd>
                </div>
                <div>
                  <dt>限定 / 常驻 5 星</dt>
                  <dd>{summary.ssrEventCount} / {summary.ssrPermanentCount}</dd>
                </div>
                <div>
                  <dt>最近 5 星</dt>
                  <dd>
                    {#if summary.latestSsr}
                      {summary.latestSsr.name} · {summary.latestSsr.count} 抽
                    {:else}
                      暂无
                    {/if}
                  </dd>
                </div>
              </dl>

              <section class="history-block">
                <h4>5 星历史（调试）</h4>
                {#if analyzeResult.analysisList[index]?.ssrDataList.length === 0}
                  <p class="status-text">暂无 5 星记录</p>
                {:else}
                  <ul class="history-list">
                    {#each analyzeResult.analysisList[index]?.ssrDataList ?? [] as hit}
                      <li>
                        <strong>{hit.name}</strong>
                        <span>{hit.count} 抽</span>
                        <span>{hit.event ? "限定" : "常驻"}</span>
                        <span>{hit.date}</span>
                      </li>
                    {/each}
                  </ul>
                {/if}
              </section>
            </article>
          {/each}
        </div>
      {/if}
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

  .debug-controls {
    display: flex;
    align-items: end;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .field-group {
    flex: 1 1 480px;
    display: flex;
    flex-direction: column;
    gap: 0.55rem;
  }

  .field-group span {
    color: #93c5fd;
    font-size: 0.82rem;
    font-weight: 700;
  }

  .field-group input {
    width: 100%;
    border: 1px solid rgba(148, 163, 184, 0.26);
    border-radius: 14px;
    padding: 0.9rem 1rem;
    background: rgba(15, 23, 42, 0.65);
    color: #e2e8f0;
    font: inherit;
    box-sizing: border-box;
  }

  .analysis-feedback {
    margin-top: 1rem;
  }

  .analysis-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
    margin-top: 1rem;
  }

  .analysis-card {
    border-radius: 18px;
    padding: 1rem;
    background: rgba(30, 41, 59, 0.7);
    border: 1px solid rgba(148, 163, 184, 0.16);
  }

  .analysis-card-header h3 {
    margin: 0;
    color: #f8fafc;
    font-size: 1.05rem;
  }

  .analysis-card-header p {
    margin: 0.45rem 0 0;
    color: #94a3b8;
    font-size: 0.9rem;
  }

  .analysis-meta-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 0.75rem;
    margin: 1rem 0 0;
  }

  .analysis-meta-grid div {
    padding: 0.75rem 0.85rem;
    border-radius: 12px;
    background: rgba(15, 23, 42, 0.46);
  }

  .analysis-meta-grid dt {
    margin-bottom: 0.35rem;
    color: #93c5fd;
    font-size: 0.78rem;
    font-weight: 700;
  }

  .analysis-meta-grid dd {
    margin: 0;
    color: #e2e8f0;
    line-height: 1.5;
  }

  .history-block {
    margin-top: 1rem;
  }

  .history-block h4 {
    margin: 0 0 0.75rem;
    color: #f8fafc;
    font-size: 0.98rem;
  }

  .history-list {
    margin: 0;
    padding: 0;
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.65rem;
  }

  .history-list li {
    display: grid;
    grid-template-columns: 1.2fr 0.7fr 0.7fr 1.3fr;
    gap: 0.75rem;
    padding: 0.75rem 0.85rem;
    border-radius: 12px;
    background: rgba(15, 23, 42, 0.46);
    color: #cbd5e1;
    font-size: 0.9rem;
  }

  code {
    color: #bfdbfe;
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

    .analysis-grid,
    .analysis-meta-grid {
      grid-template-columns: 1fr;
    }

    .history-list li {
      grid-template-columns: 1fr;
    }
  }
</style>
