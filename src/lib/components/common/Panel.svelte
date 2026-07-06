<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    description?: string;
    children?: Snippet;
    actions?: Snippet;
  }

  let { title, description = "", children, actions }: Props = $props();
</script>

<section class="panel">
  <header class="panel-header">
    <div>
      <h2>{title}</h2>
      {#if description}
        <p>{description}</p>
      {/if}
    </div>
    {@render actions?.()}
  </header>

  <div class="panel-body">
    {@render children?.()}
  </div>
</section>

<style>
  .panel {
    border: 1px solid rgba(148, 163, 184, 0.22);
    border-radius: 18px;
    background: rgba(15, 23, 42, 0.72);
    backdrop-filter: blur(14px);
    box-shadow: 0 20px 60px rgba(15, 23, 42, 0.28);
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 1rem;
    padding: 1.25rem 1.25rem 0;
  }

  .panel-header h2 {
    margin: 0;
    font-size: 1.05rem;
    color: #f8fafc;
  }

  .panel-header p {
    margin: 0.45rem 0 0;
    color: #94a3b8;
    font-size: 0.92rem;
    line-height: 1.5;
  }

  .panel-body {
    padding: 1.25rem;
  }
</style>
