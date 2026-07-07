<script lang="ts">
  import { goto } from "$app/navigation";
  import {
    analyzeLocalPool,
    extractLatestGachaUrl,
    loadCachedGachaParams,
    parseGachaUrl,
    refreshGachaData,
  } from "$lib/api/tauri/gacha";
  import { getAppConfig } from "$lib/api/tauri/settings";
  import Panel from "$lib/components/common/Panel.svelte";
  import type {
    AnalyzeLocalPoolResponse,
    AppConfigState,
    ExtractLatestGachaUrlResponse,
    PoolRankSummary,
    RefreshGachaDataResponse,
  } from "$lib/types/dto";

  let appConfigState = $state<AppConfigState | null>(null);
  let loading = $state(true);
  let errorMessage = $state("");
  let analyzeLoading = $state(false);
  let analyzeErrorMessage = $state("");
  let analyzeResult = $state<AnalyzeLocalPoolResponse | null>(null);
  let samplePoolPath = $state("/workspaces/ww-gacha-stat/doc/examples/sample-pool.json");
  let logFilePath = $state("");
  let gachaRecordUrl = $state("");
  let refreshPlayerId = $state("");
  let logLoading = $state(false);
  let parseLoading = $state(false);
  let cacheLoading = $state(false);
  let refreshLoading = $state(false);
  let refreshErrorMessage = $state("");
  let refreshStatusMessage = $state("");
  let cachedParamsJson = $state("");
  let logExtractResult = $state<ExtractLatestGachaUrlResponse | null>(null);
  let refreshResult = $state<RefreshGachaDataResponse | null>(null);

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

  async function parseAndSaveGachaUrl() {
    const url = gachaRecordUrl.trim();
    refreshErrorMessage = "";
    refreshStatusMessage = "";

    if (!url) {
      refreshErrorMessage = "请先粘贴抽卡记录 URL";
      return;
    }

    parseLoading = true;

    try {
      const result = await parseGachaUrl(url, true);
      refreshPlayerId = result.parsed.playerId;
      cachedParamsJson = JSON.stringify(result.parsed.params, null, 2);
      refreshStatusMessage = result.dataFilePath
        ? `已解析并缓存参数：${result.dataFilePath}`
        : "已解析参数";
    } catch (error) {
      refreshErrorMessage = error instanceof Error ? error.message : "解析抽卡 URL 失败";
    } finally {
      parseLoading = false;
    }
  }

  async function extractUrlFromLog() {
    refreshErrorMessage = "";
    refreshStatusMessage = "";
    logLoading = true;

    try {
      const result = await extractLatestGachaUrl(logFilePath);
      logExtractResult = result;
      gachaRecordUrl = result.result.latest.url;
      refreshPlayerId = result.parsed.playerId;
      cachedParamsJson = JSON.stringify(result.parsed.params, null, 2);
      refreshStatusMessage = `已从日志第 ${result.result.latest.lineNumber} 行提取并缓存参数：${result.dataFilePath}`;
    } catch (error) {
      refreshErrorMessage = error instanceof Error ? error.message : "从日志提取抽卡 URL 失败";
    } finally {
      logLoading = false;
    }
  }

  async function loadParamsFromCache() {
    const playerId = refreshPlayerId.trim();
    refreshErrorMessage = "";
    refreshStatusMessage = "";

    if (!playerId) {
      refreshErrorMessage = "请先输入玩家 ID";
      return;
    }

    cacheLoading = true;

    try {
      const result = await loadCachedGachaParams(playerId);
      cachedParamsJson = JSON.stringify(result.params, null, 2);
      refreshStatusMessage = `已读取缓存参数：${result.dataFilePath}`;
    } catch (error) {
      refreshErrorMessage = error instanceof Error ? error.message : "读取缓存参数失败";
    } finally {
      cacheLoading = false;
    }
  }

  async function refreshFromCachedParams() {
    const playerId = refreshPlayerId.trim();
    refreshErrorMessage = "";
    refreshStatusMessage = "";

    if (!playerId) {
      refreshErrorMessage = "请先输入玩家 ID，或先解析抽卡 URL";
      return;
    }

    refreshLoading = true;

    try {
      refreshResult = await refreshGachaData(playerId);
      refreshStatusMessage = `刷新完成：${refreshResult.poolFilePath}`;
    } catch (error) {
      refreshErrorMessage = error instanceof Error ? error.message : "刷新抽卡数据失败";
    } finally {
      refreshLoading = false;
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
        当前阶段已完成抽卡 URL 解析、参数缓存、官方接口请求和最小刷新链路，正在进行端到端联调入口验证。
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
    <Panel title="当前开发阶段" description="对照实现路线，当前正在完成第 4 阶段刷新链路调试入口。">
      <ul class="bullet-list">
        <li>已完成本地 `pool.json -> AnalysisData[] -> summaryList` 离线分析链路</li>
        <li>已完成 `parse_gacha_url`、`data.json` 参数缓存和 `refresh_gacha_data`</li>
        <li>当前新增最小刷新入口，用于真实抽卡 URL 端到端联调</li>
        <li>下一步根据真实接口表现决定是否补分页、请求头或错误码细化</li>
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

    <Panel title="第 4 阶段刷新链路调试区" description="可从日志提取或手动粘贴抽卡 URL，缓存参数后按玩家 ID 调用 Rust 刷新链路。">
      <div class="debug-controls stacked-controls">
        <label class="field-group full-width-field">
          <span>游戏日志文件路径（可选，不填则使用配置中的游戏根目录和日志相对路径）</span>
          <input bind:value={logFilePath} placeholder="例如：/path/to/Wuthering Waves/Client/Saved/Logs/Client.log" />
        </label>

        <div class="action-row">
          <button class="ghost-button" type="button" onclick={extractUrlFromLog} disabled={logLoading}>
            {#if logLoading}提取中...{:else}从日志提取并缓存 URL{/if}
          </button>
        </div>

        <label class="field-group full-width-field">
          <span>抽卡记录 URL</span>
          <textarea bind:value={gachaRecordUrl} rows="4" placeholder="粘贴从游戏日志中提取到的 index.html#/record?... URL"></textarea>
        </label>

        <div class="action-row">
          <button class="primary-button" type="button" onclick={parseAndSaveGachaUrl} disabled={parseLoading}>
            {#if parseLoading}解析中...{:else}解析并缓存 URL{/if}
          </button>
        </div>

        <label class="field-group">
          <span>玩家 ID</span>
          <input bind:value={refreshPlayerId} placeholder="解析 URL 后会自动填入，也可手动输入" />
        </label>

        <div class="action-row">
          <button class="ghost-button" type="button" onclick={loadParamsFromCache} disabled={cacheLoading}>
            {#if cacheLoading}读取中...{:else}读取缓存参数{/if}
          </button>
          <button class="primary-button" type="button" onclick={refreshFromCachedParams} disabled={refreshLoading}>
            {#if refreshLoading}刷新中...{:else}刷新抽卡数据{/if}
          </button>
        </div>
      </div>

      {#if refreshErrorMessage}
        <p class="error-text analysis-feedback">{refreshErrorMessage}</p>
      {/if}

      {#if refreshStatusMessage}
        <p class="status-text analysis-feedback">{refreshStatusMessage}</p>
      {/if}

      {#if cachedParamsJson}
        <section class="history-block">
          <h4>缓存参数预览</h4>
          <pre class="json-preview">{cachedParamsJson}</pre>
        </section>
      {/if}

      {#if logExtractResult}
        <section class="history-block">
          <h4>日志提取结果</h4>
          <dl class="meta-grid">
            <div>
              <dt>日志文件</dt>
              <dd>{logExtractResult.result.logFilePath}</dd>
            </div>
            <div>
              <dt>匹配 URL 数量</dt>
              <dd>{logExtractResult.result.totalUrlCount}</dd>
            </div>
            <div>
              <dt>最新 URL 行号</dt>
              <dd>{logExtractResult.result.latest.lineNumber}</dd>
            </div>
            <div>
              <dt>缓存文件</dt>
              <dd>{logExtractResult.dataFilePath}</dd>
            </div>
          </dl>
        </section>
      {/if}

      {#if refreshResult}
        <div class="refresh-result-block">
          <dl class="meta-grid">
            <div>
              <dt>玩家 ID</dt>
              <dd>{refreshResult.playerId}</dd>
            </div>
            <div>
              <dt>pool.json</dt>
              <dd>{refreshResult.poolFilePath}</dd>
            </div>
            <div>
              <dt>data.json</dt>
              <dd>{refreshResult.dataFilePath}</dd>
            </div>
            <div>
              <dt>合并后总数</dt>
              <dd>{refreshResult.mergeResult.totalMergedCount}</dd>
            </div>
          </dl>

          <section class="history-block">
            <h4>接口返回概览</h4>
            <ul class="result-list compact-result-list">
              {#each refreshResult.apiPoolResults as poolResult}
                <li>
                  <strong>{poolResult.poolName}</strong>
                  <span>类型 {poolResult.cardPoolType}</span>
                  <span>{poolResult.recordCount} 条</span>
                </li>
              {/each}
            </ul>
          </section>

          <section class="history-block">
            <h4>合并摘要</h4>
            <ul class="result-list compact-result-list">
              {#each refreshResult.mergeResult.summaries as summary}
                <li>
                  <strong>{summary.poolName}</strong>
                  <span>新 {summary.newCount}</span>
                  <span>旧 {summary.oldCount}</span>
                  <span>合并 {summary.mergedCount}</span>
                </li>
              {/each}
            </ul>
          </section>

          <section class="history-block">
            <h4>刷新后总览</h4>
            <div class="analysis-grid">
              {#each refreshResult.summaryList as summary}
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
                      <dt>5 星数量</dt>
                      <dd>{formatRankSummary(summary.ssr)}</dd>
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
                </article>
              {/each}
            </div>
          </section>
        </div>
      {/if}
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

  .grid :global(.panel:nth-child(4)),
  .grid :global(.panel:nth-child(5)) {
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

  .stacked-controls {
    align-items: stretch;
  }

  .action-row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .field-group {
    flex: 1 1 480px;
    display: flex;
    flex-direction: column;
    gap: 0.55rem;
  }

  .full-width-field {
    flex-basis: 100%;
  }

  .field-group span {
    color: #93c5fd;
    font-size: 0.82rem;
    font-weight: 700;
  }

  .field-group input,
  .field-group textarea {
    width: 100%;
    border: 1px solid rgba(148, 163, 184, 0.26);
    border-radius: 14px;
    padding: 0.9rem 1rem;
    background: rgba(15, 23, 42, 0.65);
    color: #e2e8f0;
    font: inherit;
    box-sizing: border-box;
  }

  .field-group textarea {
    min-height: 7rem;
    resize: vertical;
    line-height: 1.6;
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

  .refresh-result-block {
    margin-top: 1rem;
  }

  .json-preview {
    margin: 0;
    max-height: 16rem;
    overflow: auto;
    padding: 1rem;
    border-radius: 14px;
    border: 1px solid rgba(148, 163, 184, 0.16);
    background: rgba(15, 23, 42, 0.62);
    color: #bfdbfe;
    line-height: 1.55;
    white-space: pre-wrap;
    word-break: break-all;
  }

  .result-list {
    margin: 0;
    padding: 0;
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.65rem;
  }

  .result-list li {
    display: grid;
    grid-template-columns: 1.4fr repeat(3, minmax(0, 0.7fr));
    gap: 0.75rem;
    padding: 0.75rem 0.85rem;
    border-radius: 12px;
    background: rgba(15, 23, 42, 0.46);
    color: #cbd5e1;
    font-size: 0.9rem;
  }

  .compact-result-list li {
    align-items: center;
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
    .grid :global(.panel:nth-child(3)),
    .grid :global(.panel:nth-child(4)),
    .grid :global(.panel:nth-child(5)) {
      grid-column: 1 / -1;
    }

    .meta-grid {
      grid-template-columns: 1fr;
    }

    .analysis-grid,
    .analysis-meta-grid {
      grid-template-columns: 1fr;
    }

    .history-list li,
    .result-list li {
      grid-template-columns: 1fr;
    }
  }
</style>
