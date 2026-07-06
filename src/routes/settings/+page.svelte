<script lang="ts">
  import Panel from "$lib/components/common/Panel.svelte";
</script>

<svelte:head>
  <title>设置 - 鸣潮抽卡分析</title>
</svelte:head>

<main class="page-shell">
  <Panel title="设置页占位" description="后续将在这里接入游戏路径、资源源、日志级别、跳过首个五星等配置项。">
    <ul>
      <li>游戏根目录配置</li>
      <li>日志路径规则配置</li>
      <li>资源源切换</li>
      <li>日志级别切换</li>
      <li>卡池映射配置</li>
      <li>常驻五星 ID 配置</li>
    </ul>
  </Panel>
</main>

<style>
  :global(body) {
    margin: 0;
    min-height: 100vh;
    font-family: Inter, "PingFang SC", "Microsoft YaHei", sans-serif;
    background: linear-gradient(180deg, #020617 0%, #0f172a 52%, #111827 100%);
    color: #e2e8f0;
  }

  .page-shell {
    max-width: 1180px;
    margin: 0 auto;
    padding: 3rem 1.25rem 4rem;
  }

  ul {
    margin: 0;
    padding-left: 1.1rem;
    color: #cbd5e1;
    line-height: 1.8;
  }
</style>
