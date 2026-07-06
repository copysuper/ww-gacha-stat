# 抽卡分析实现进度

本文档用于在多会话、多次中断的情况下，快速恢复当前项目开发状态。

配合以下文件一起阅读：

- `AGENTS.md`
- `doc/reference-gacha-resource-tech-note.md`
- `doc/gacha-analysis-implementation-roadmap.md`
- `/memories/repo/project-notes.md`

---

## 1. 当前阶段

当前处于：**第 1 阶段已完成，第 2 阶段待开始**

- 第 1 阶段：基础设施搭建 ✅
- 第 2 阶段：离线抽卡分析核心 ⏳

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
- `gacha_storage`
- `gacha_merge`
- `gacha_analysis`
- `resource_sync`

### 3.2 当前还**没有**实现的业务能力

以下能力尚未开始：

- 从游戏日志提取抽卡 URL
- 手动输入 URL 解析
- 官方抽卡接口请求
- 抽卡记录本地持久化
- 新旧记录合并
- 离线抽卡分析
- 图片资源同步
- 真实设置页表单

---

## 4. 下一步唯一优先目标

下一步进入：**第 2 阶段：离线抽卡分析核心**

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

### 4.2 当前阶段不要做的事

进入第 2 阶段时，**先不要**急着做：

- 日志扫描
- URL 解析
- 网络请求
- 资源同步
- 复杂图表
- 大而全设置页

原因：

- 必须先把“分析核心”做稳
- 避免输入层和资源层干扰核心统计算法验证

---

## 5. 推荐恢复工作时的阅读顺序

如果是新会话，建议按以下顺序恢复上下文：

1. `AGENTS.md`
2. `doc/reference-gacha-resource-tech-note.md`
3. `doc/gacha-analysis-implementation-roadmap.md`
4. `doc/implementation-progress.md`
5. `/memories/repo/project-notes.md`

然后直接继续第 2 阶段实现。

---

## 6. 已验证结果

本阶段完成后已验证：

### 前端

- `pnpm check` ✅

### Rust

- `cargo check --manifest-path src-tauri/Cargo.toml` ✅

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

- 日期：2026-07-06
- 状态：第 1 阶段完成，第 2 阶段待开始
