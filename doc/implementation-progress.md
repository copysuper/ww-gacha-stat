# 抽卡分析实现进度

本文档用于在多会话、多次中断的情况下，快速恢复当前项目开发状态。

配合以下文件一起阅读：

- `AGENTS.md`
- `doc/reference-gacha-resource-tech-note.md`
- `doc/gacha-analysis-implementation-roadmap.md`
- `/memories/repo/project-notes.md`

---

## 1. 当前阶段

当前处于：**第 4 阶段已开始：输入层与刷新链路**

- 第 1 阶段：基础设施搭建 ✅
- 第 2 阶段：离线抽卡分析核心 ✅
- 第 3 阶段：新旧数据合并与本地持久化完整链路 ✅（最小可验证版）
- 第 4 阶段：输入层与刷新链路 🚧（已完成手动 URL、明文日志 URL 提取骨架、data.json 参数缓存、gacha_api、最小刷新链路与前端调试入口；**加密日志解密尚未接入**）

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

以下模块尚未开始或未完整实现：

- `gacha_log`（已完成路径定位 + 明文扫描提取骨架，并接入参数缓存；**尚未按 `new-ui-dev` 解密 `Client.log`**）
- `gacha_params`（已完成手动 URL 参数解析，并可解析日志提取结果）
- `gacha_api`（已完成最小官方接口请求层）
- `gacha_storage`（已完成 pool.json 读取/写入与 data.json 参数缓存）
- `gacha_merge`（已完成本地 pool.json 合并）
- `gacha_analysis`（已完成离线分析核心）
- `resource_sync`

### 3.2 当前还**没有**实现的业务能力

以下能力尚未开始：

- 从游戏日志提取抽卡 URL（已完成最小版，但**尚未接入加密日志解密**；当前按明文扫描，对现行 `Client.log` 会失败）
- 手动输入 URL 解析（已完成）
- 官方抽卡接口请求（已完成最小请求层，待真实账号联调）
- 抽卡记录本地持久化（pool.json 已完成；data.json 参数缓存已完成）
- 新旧记录合并（已完成本地新旧 pool.json 合并）
- 离线抽卡分析（核心已完成；后续按真实刷新链路需要迭代）
- 图片资源同步
- 真实设置页表单

---

## 4. 下一步唯一优先目标

下一步进入：**第 4 阶段：输入层与刷新链路**

### 4.1 下一步建议严格顺序

1. 按 `new-ui-dev` 的 `CardPoolGetUrlTask` 为 `gacha_log` 接入 `Client.log` 解密
2. 使用真实加密日志验证：`extract_latest_gacha_url -> refresh_gacha_data`
3. 根据真实接口表现确认是否需要分页、额外请求头或错误码细化
4. 再补资源同步、真实设置页和复杂 UI

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

1. 先按 `new-ui-dev` 接入加密日志解密
2. 使用真实加密日志 / 抽卡 URL 端到端联调刷新链路
3. 根据真实接口表现确认是否需要分页、请求头或错误码细化
4. 后续进入资源同步

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

### 4.3 第 4 阶段当前已完成内容

已完成：

- 新增 `gacha_params` 模块
- 定义 `RequestParams` / `ParsedGachaParams`
- 实现 `parse_gacha_url_params`
- 支持从手动输入的抽卡记录 URL 中解析 query 参数
- 按参考文档完成字段映射：
  - `player_id -> playerId`
  - `record_id -> recordId`
  - `resources_id -> cardPoolId`
  - `gacha_type -> cardPoolType`
  - `svr_id -> serverId`
  - `lang -> languageCode`
- 保留未知 query 参数，避免丢失官方接口后续可能需要的字段
- 支持 hash fragment 后的 query：`index.html#/record?...`
- 新增 Tauri command：`parse_gacha_url`
- `parse_gacha_url` 支持通过 `saveToCache` 可选保存 `data/{playerId}/data.json`
- 新增 `load_cached_gacha_params` Tauri command
- 前端新增：
  - `RequestParams`
  - `ParsedGachaParams`
  - `ParseGachaUrlResponse`
  - `LoadCachedGachaParamsResponse`
  - `parseGachaUrl(...)` API 封装
  - `loadCachedGachaParams(...)` API 封装
- 新增 4 个 `gacha_params` 单元测试
- 新增 2 个 `gacha_storage` 参数缓存单元测试

### 4.4 第 4 阶段刷新链路当前已完成内容

已完成：

- 新增 `gacha_api` 模块
- 按 `playerId` 选择官方请求地址：
  - `1*`：`https://gmserver-api.aki-game2.com/gacha/record/query`
  - 其他：`https://gmserver-api.aki-game2.net/gacha/record/query`
- 按卡池覆盖 `cardPoolType`，保留 `data.json` 中未知参数
- 默认卡池列表已按参考文档配置到 `AppConfig`
- 兼容旧配置 `cardPools` 为空时自动使用默认卡池
- 新增 Tauri command：`refresh_gacha_data`
- 刷新链路已串联：
  - 读取 `data/{playerId}/data.json`
  - 请求官方卡池记录
  - 转换为 `PoolFile`
  - 与本地 `pool.json` 合并
  - 保存 `pool.json` 与 `data.json`
  - 返回 `analysisList` / `summaryList`
- 前端新增：
  - `GachaApiPoolResult`
  - `RefreshGachaDataResponse`
  - `refreshGachaData(...)` API 封装
- 新增 2 个 `gacha_api` 单元测试

### 4.5 第 4 阶段前端刷新入口当前已完成内容

已完成：

- 首页新增“第 4 阶段刷新链路调试区”
- 支持粘贴抽卡记录 URL 并调用 `parseGachaUrl(url, true)` 缓存参数
- 支持按玩家 ID 读取 `data/{playerId}/data.json` 缓存参数
- 支持按玩家 ID 调用 `refreshGachaData(playerId)` 刷新抽卡数据
- 页面展示：
  - 缓存参数预览
  - 接口各卡池返回数量
  - 合并摘要
  - 刷新后的总览摘要
- 更新首页当前阶段文案，指向第 4 阶段端到端联调

### 4.6 第 4 阶段日志提取当前已完成内容

已完成：

- 新增 `gacha_log` 模块
- 支持从配置的 `gameRootDir + gameLogFileRelativePath` 定位日志文件
  - 默认相对路径：`Client/Saved/Logs/Client.log`
- 支持前端手动传入日志文件路径用于调试
- 当前实现仍按**明文/字节扫描**提取 URL（历史方案）
- 同一日志内收集所有匹配 URL，并取最后一个作为最新抽卡记录 URL
- 提取后复用 `gacha_params` 解析 URL 参数
- 新增 Tauri command：`extract_latest_gacha_url`
- 命令成功后自动写入 `data/{playerId}/data.json` 参数缓存
- 前端新增：
  - `ExtractedGachaUrl`
  - `GachaLogExtractResult`
  - `ExtractLatestGachaUrlResponse`
  - `extractLatestGachaUrl(...)` API 封装
- 首页“第 4 阶段刷新链路调试区”新增从日志提取并缓存 URL 的入口
- 新增 5 个 `gacha_log` 单元测试（基于明文样例）

**已知缺口（必须修）：**

- 现行游戏 `Client.log` 已加密
- 参考仓库正确实现位于 **`new-ui-dev`** 分支的 `CardPoolGetUrlTask`
- 正确流程应为：
  1. 整文件读字节
  2. 逐字节 XOR 解密：`((b & 0x0F) % 2 == 1) ? b ^ 0xA5 : b ^ 0xEF`
  3. UTF-8 解码
  4. 正则 `https.*/aki/gacha/index.html#/record[?=&\w\-]+` 取最后一个匹配
- 当前项目尚未接入上述解密，因此对真实加密日志会“读不出 URL”
- 相关规格已补进 `doc/reference-gacha-resource-tech-note.md` 第 5.1 节

最近验证结果：

- `cargo fmt --manifest-path src-tauri/Cargo.toml` ✅
- `cargo check --manifest-path src-tauri/Cargo.toml` ✅
- `cargo test --manifest-path src-tauri/Cargo.toml` ✅，24 tests passed
- `pnpm check` ✅，0 errors / 0 warnings

### 4.7 当前阶段不要做的事

在补上加密日志解密前，**先不要**急着做：

- 资源同步
- 复杂图表
- 大而全设置页

原因：

- 必须先把“日志解密 + URL 参数解析 + 请求 + 合并 + 保存 + 分析”的主链路做稳
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

### 第 4 阶段前端刷新入口验证

- 首页新增刷新链路调试区 ✅
- Rust 全量测试 19 个通过 ✅
- `cargo check --manifest-path src-tauri/Cargo.toml` ✅
- `cargo test --manifest-path src-tauri/Cargo.toml` ✅
- `pnpm check` ✅

### 第 4 阶段输入层验证

- `gacha_params` 4 个单元测试 ✅
- 已覆盖字段映射、URL 编码解码、缺少必要参数、非 record URL ✅
- `gacha_storage` 2 个参数缓存单元测试 ✅
- 已覆盖按玩家 ID 写入/读取 `data.json`、拒绝不安全玩家 ID ✅
- `gacha_api` 2 个单元测试 ✅
- 已覆盖玩家 ID 分服 URL 选择、请求体覆盖 `cardPoolType` 且保留未知参数 ✅
- Rust 全量测试 19 个通过 ✅
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
- 状态：第 4 阶段已完成手动抽卡 URL 参数解析、data.json 参数缓存、gacha_api、最小刷新链路和首页刷新调试入口，下一步使用真实抽卡 URL 联调并根据接口表现补分页/请求头/错误码细化
