# 抽卡分析实现进度

本文档用于在多会话、多次中断的情况下，快速恢复当前项目开发状态。

配合以下文件一起阅读：

- `AGENTS.md`
- `doc/reference-gacha-resource-tech-note.md`
- `doc/gacha-analysis-implementation-roadmap.md`
- `/memories/repo/project-notes.md`

---

## 1. 当前阶段

当前处于：**第 3 阶段最小完整链路已完成**

- 第 1 阶段：基础设施搭建 ✅
- 第 2 阶段：离线抽卡分析核心 ✅
- 第 3 阶段：新旧数据合并与本地持久化完整链路 ✅（最小可验证版）

---

## 2. 已完成内容

### 2.1 Rust 骨架已建立

已完成以下模块骨架：

```text
src-tauri/src/
  app_paths/
  commands/
    mod.rs
    settings.rs
  config/
    mod.rs
    model.rs
    service.rs
  error/
    mod.rs
```

### 2.2 已完成的 Rust 能力

已具备：

- 应用配置目录 / 数据目录 / 资源目录路径解析
- 统一错误类型 `AppError`
- 应用默认配置 `AppConfig`
- 配置文件自动创建与读取
- 配置更新并持久化
- 基础日志初始化
- 两个 Tauri command：
  - `get_app_config`
  - `update_app_config`

### 2.3 前端骨架已建立

已完成以下前端基础结构：

```text
src/
  lib/
    api/tauri/
      core.ts
      settings.ts
    components/common/
      Panel.svelte
    types/
      dto.ts
  routes/
    +page.svelte
    settings/
      +page.svelte
```

### 2.4 已完成的前端能力

已具备：

- 首页业务骨架 UI
- 设置页占位页面
- Tauri command 统一调用封装
- 与 Rust 配置 DTO 对齐的前端类型定义
- 首页可读取并展示 Rust 侧配置状态

---

## 3. 当前实现状态说明

### 3.1 当前还**没有**实现的业务模块

以下模块尚未开始：

- `gacha_log`
- `gacha_params`
- `gacha_api`
- `gacha_storage`（已完成最小读取能力）
- `gacha_merge`
- `gacha_analysis`（已完成最小分析能力）
- `resource_sync`

### 3.2 当前还**没有**实现的业务能力

以下能力尚未开始：

- 从游戏日志提取抽卡 URL
- 手动输入 URL 解析
- 官方抽卡接口请求
- 抽卡记录本地持久化（完整版本尚未开始）
- 新旧记录合并
- 离线抽卡分析（核心已完成；后续仅在第 3 阶段后按需要迭代）
- 图片资源同步
- 真实设置页表单

---

## 4. 下一步唯一优先目标

下一步进入：**第 4 阶段：输入层与刷新链路**

### 4.1 下一步建议严格顺序

1. 新建 `gacha_analysis` 模块
2. 新建 `gacha_storage` 的最小读取模型
3. 定义以下核心数据结构：
   - `GachaRecord`
   - `PoolFile`
   - `AnalysisData`
   - `HitData`
4. 先实现：
   - 5 星索引提取
   - 4 星索引提取
   - 已垫统计
   - 5 星平均 / 最欧 / 最非
   - 限定判断
   - 时间范围计算
5. 暂时只支持：
   - 从本地 `pool.json` 读取
   - 输出 `AnalysisData`
6. 前端新增一个调试展示区，用于显示离线分析结果

### 4.1 当前已完成到哪一步

已完成：

- `gacha_analysis` 模块骨架
- `GachaRecord` / `PoolFile` / `AnalysisData` / `HitData` 定义
- 本地 `pool.json` 读取
- 离线分析主链路：`pool.json -> AnalysisData[]`
- Tauri command：`analyze_local_pool`
- 分析总览映射层：`PoolAnalysisSummary` / `PoolRankSummary`
- 前端第 2 阶段调试区
- 示例数据文件：`doc/examples/sample-pool.json`
- 边界示例数据：`empty-pool.json` / `only-r-pool.json` / `single-ssr-pool.json` / `skip-first-ssr-pool.json`

当前下一步建议：

1. 开始实现 `gacha_params`，支持手动输入抽卡 URL 的参数解析
2. 开始实现 `gacha_api`，按卡池请求官方抽卡记录
3. 将 `gacha_api -> gacha_merge -> gacha_storage -> gacha_analysis` 串成刷新链路

### 4.2 第 3 阶段已完成内容

已完成：

- 新增 `gacha_merge` 模块
- 实现参考文档要求的“按时间拼接”合并策略
- 支持多卡池合并，卡池名取新旧 `pool.json` 并集
- 实现同一边界秒旧记录跳过规则
- 新增 `PoolMergeSummary` / `PoolMergeResult` 合并摘要
- 扩展 `gacha_storage`：
  - 保存 `pool.json`
  - 自动创建父目录
  - pretty JSON 写入
  - 保存前按自然日备份旧 `pool.json` 为 `pool.json.bak`
- 新增 Tauri command：`merge_local_pool`
- 前端新增：
  - `MergeLocalPoolResponse`
  - `PoolMergeResult`
  - `PoolMergeSummary`
  - `mergeLocalPool(...)` API 封装
- 新增合并示例数据：
  - `doc/examples/merge-old-pool.json`
  - `doc/examples/merge-new-pool.json`
  - `doc/examples/merge-expected-pool.json`
- 新增 6 个 `gacha_merge` 单元测试

### 4.3 当前阶段不要做的事

进入第 4 阶段前，**先不要**急着做：

- 日志扫描
- 资源同步
- 复杂图表
- 大而全设置页

原因：

- 必须先把“URL 参数解析 + 请求 + 合并 + 保存 + 分析”的主链路做稳
- 避免资源层和复杂 UI 干扰刷新链路验证

---

## 5. 推荐恢复工作时的阅读顺序

如果是新会话，建议按以下顺序恢复上下文：

1. `AGENTS.md`
2. `doc/reference-gacha-resource-tech-note.md`
3. `doc/gacha-analysis-implementation-roadmap.md`
4. `doc/implementation-progress.md`
5. `/memories/repo/project-notes.md`

然后直接继续第 4 阶段实现。

---

## 6. 已验证结果

本阶段完成后已验证：

### 前端

- `pnpm check` ✅

### Rust

- `cargo check --manifest-path src-tauri/Cargo.toml` ✅

### 第 2 阶段最小链路

- `sample-pool.json -> analyze_local_pool -> 首页调试展示` ✅

### 第 2 阶段收尾新增

- `analyze_local_pool` 已返回 `analysisList + summaryList` ✅
- 首页已优先展示 `summaryList`，原始 `ssrDataList` 保留为调试视图 ✅

### 第 2 阶段样例验证清单

- `sample-pool.json`：验证多卡池、5/4/3 星统计、pity、最近命中、常驻/限定判断 ✅
- `empty-pool.json`：验证全空卡池零值摘要与空时间范围 ✅
- `only-r-pool.json`：验证仅 3 星时 5/4 星为空且 pity 正确回落到总抽数 ✅
- `single-ssr-pool.json`：验证单 5 星时 `avg = min = max` 且限定判断正确 ✅
- `skip-first-ssr-pool.json`：验证 `skipFirstSSR = true` 时最老 5 星片段被正确裁剪 ✅

### 第 2 阶段自动化验证

- 已新增 5 个 Rust 单元测试覆盖上述样例 ✅
- `cargo test --manifest-path src-tauri/Cargo.toml gacha_analysis::service::tests -- --nocapture` ✅

### 第 3 阶段最小链路验证

- `merge-old-pool.json + merge-new-pool.json -> merge-expected-pool.json` ✅
- `gacha_merge` 6 个单元测试 ✅
- Rust 全量测试 11 个通过 ✅
- `cargo check --manifest-path src-tauri/Cargo.toml` ✅
- `cargo test --manifest-path src-tauri/Cargo.toml` ✅
- `pnpm check` ✅

---

## 7. 已知约束与注意事项

### 7.1 必须继续遵守

- 抽卡核心逻辑以 `doc/reference-gacha-resource-tech-note.md` 为准
- 新功能优先放 Rust，不要把统计算法写进页面组件
- Tauri command 输入输出结构保持稳定
- 配置优先，不要把卡池映射和常驻五星硬编码到页面层

### 7.2 已确认的实现约束

- `Panel.svelte` 已使用 Svelte 5 `Snippet` / `{@render ...}` 写法
- 后续新增通用组件时，不要退回旧 `<slot>` 写法
- 当前项目已经不是 Hello World 结构，后续应在现有骨架上增量扩展

---

## 8. 最后更新时间

- 日期：2026-07-07
- 状态：第 3 阶段最小完整链路已完成，已具备本地新旧 pool.json 合并、保存、摘要返回与自动化验证能力
