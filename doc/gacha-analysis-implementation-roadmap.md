qq

# 鸣潮抽卡分析技术实现路线

本文档基于以下两份文档生成，并贴合当前仓库的真实技术栈与项目状态：

- `AGENTS.md`
- `doc/reference-gacha-resource-tech-note.md`

当前项目现状：

- 前端是一个基础 `SvelteKit + Tauri` 页面
- Rust 侧仍是 `greet` Hello World 示例
- 还没有抽卡分析、日志解析、配置管理、资源同步等实际业务模块

本文档目标不是直接写代码，而是给出**适合本仓库落地的实现路线**，方便后续按阶段推进开发。

---

## 1. 目标与实现原则

### 1.1 最终目标

在当前 `SvelteKit + Tauri + Rust` 项目中实现一个可维护、可扩展、性能优先的鸣潮抽卡分析桌面应用，至少支持：

- 从游戏日志中自动提取抽卡 URL
- 手动输入从日志提取出的抽卡 URL
- 请求官方抽卡记录接口
- 按玩家保存本地抽卡记录与请求参数缓存
- 对本地抽卡记录执行分析
- 在前端展示总览页与详情页
- 使用本地头像/武器图标资源
- 后续为导入、手动补录、其他输入方式预留扩展点

### 1.2 总体原则

严格遵循：

- 配置优先
- Rust 承担核心计算与 IO
- UI 与业务逻辑低耦合
- 对扩展开放
- 日志充分
- 可读性优先，适合 Rust 初学者维护

---

## 2. 与当前项目结构的衔接方案

### 2.1 当前项目已有结构

前端：

```text
src/
  routes/
    +layout.ts
    +page.svelte
```

Tauri / Rust：

```text
src-tauri/
  src/
    lib.rs
    main.rs
```

当前状态非常适合从零开始按模块化方式铺开，不存在历史包袱。

### 2.2 建议目标结构

#### 前端目录建议

```text
src/
  routes/
    +layout.ts
    +page.svelte
    settings/
      +page.svelte
  lib/
    api/
      tauri/
        gacha.ts
        settings.ts
        resources.ts
    components/
      common/
      gacha/
    features/
      gacha/
        components/
        stores/
        mappers/
        types/
    stores/
      app.ts
    types/
      dto.ts
```

#### Rust 目录建议

```text
src-tauri/src/
  main.rs
  lib.rs
  commands/
    mod.rs
    gacha.rs
    settings.rs
    resources.rs
  config/
    mod.rs
    model.rs
    service.rs
  error/
    mod.rs
  gacha_log/
    mod.rs
    parser.rs
  gacha_params/
    mod.rs
    parser.rs
  gacha_api/
    mod.rs
    client.rs
    model.rs
  gacha_storage/
    mod.rs
    model.rs
    service.rs
  gacha_merge/
    mod.rs
  gacha_analysis/
    mod.rs
    model.rs
    service.rs
  resource_sync/
    mod.rs
    model.rs
    service.rs
  app_paths/
    mod.rs
```

说明：

- `commands` 只负责对外暴露 Tauri command，不写核心逻辑。
- 每个业务能力单独模块化，后续扩展 Excel 导入、手动补录时不会污染核心分析模块。

---

## 3. 分层设计

### 3.1 Rust 作为核心业务层

Rust 负责：

- 配置读取和持久化
- 游戏日志扫描
- 抽卡 URL 提取
- URL 参数解析
- 抽卡 API 请求
- 本地 JSON 读写
- 新旧记录合并
- 抽卡分析计算
- 资源同步

这样做的原因：

- IO 和计算放在 Rust 更高性能
- 避免前端承载复杂业务逻辑
- 便于以后增加 CLI、测试、后台任务

### 3.2 前端作为展示与交互层

前端只负责：

- 输入游戏路径、手动 URL、配置项
- 发起 command
- 展示分析结果
- 展示刷新状态、错误信息、同步状态
- 切换卡池、查看详情、显示图表

### 3.3 Tauri command 作为稳定边界

建议所有前后端交互都走清晰的 DTO：

- 输入 DTO：命令参数
- 输出 DTO：稳定 JSON 结构
- 错误 DTO：统一错误格式

不要把 Rust 内部结构直接暴露给前端页面。

---

## 4. 配置策略

根据 `AGENTS.md` 的“配置优先”原则，第一阶段就应该建立配置系统。

### 4.1 建议配置项

```ts
interface AppConfig {
  gameRootDir?: string;
  gameLogFileRelativePath?: string;
  resourceSource: "github" | "gitee";
  dataDir?: string;
  assetsDir?: string;
  logLevel: "error" | "warn" | "info" | "debug" | "trace";
  skipFirstSSR: boolean;
  baseSsrIds: string[];
  cardPools: Array<{
    name: string;
    type: string;
  }>;
}
```

### 4.2 配置落地方式

建议：

- 由 Rust 持久化配置文件
- 前端通过 command 读取/保存配置
- 默认值由 Rust 提供

不要把卡池映射、常驻 5 星 ID、资源源地址写死在 Svelte 页面里。

---

## 5. 数据模型设计

### 5.1 Rust 内部模型

按文档直接定义：

- `GachaRecord`
- `RequestParams`
- `AnalysisData`
- `HitData`
- `PoolFile`

建议做两层模型：

1. **内部领域模型**
   - 更适合 Rust 计算
2. **对前端输出 DTO**
   - 字段稳定、方便序列化

### 5.2 前端类型

前端不要自己发明一套不同字段名。

建议在：

```text
src/lib/types/dto.ts
```

中维护与 Rust command 返回一致的类型定义。

---

## 6. 功能模块实现顺序

下面是适合当前仓库的实际推进顺序。

### 阶段 1：基础设施搭建

目标：把 Hello World 项目变成可承载业务的应用骨架。

#### Rust

- 删除 `greet` 示例 command
- 建立 `commands/mod.rs`
- 建立统一错误类型
- 建立日志初始化
- 建立配置读取/保存模块
- 建立应用数据目录和资源目录路径模块

#### 前端

- 清理 `+page.svelte` 的默认示例 UI
- 建立页面基础布局
- 建立基础状态管理
- 建立调用 Tauri command 的 API 包装层

#### 产出

- 能启动一个空壳业务界面
- 能读取/修改配置
- 能看到日志输出

---

### 阶段 2：抽卡分析离线核心

目标：**先不接网络、不接日志**，只把抽卡分析算法跑通。

#### Rust

实现：

- `gacha_storage`
  - 读取 `pool.json`
  - 写入 `pool.json`
- `gacha_analysis`
  - `skipFirstSSR`
  - 星级索引提取
  - 已垫统计
  - 5 星统计
  - 4 星统计
  - 限定判断
  - 时间范围计算

#### 前端

- 提供加载本地 mock 数据的调试入口
- 展示总览数据
- 展示单池详情数据

#### 产出

- 只靠本地 JSON 就能验证算法是否正确
- 前端能展示基本统计结果

> 这是最重要的第一业务阶段。先把“分析”做成稳定核心，再加输入来源。

---

### 阶段 3：新旧数据合并与本地持久化

目标：建立稳定的数据落盘机制。

#### Rust

实现：

- `gacha_merge`
  - 按时间拼接合并
  - 同秒记录处理逻辑
- `gacha_storage`
  - `data/{playerId}/pool.json`
  - `data/{playerId}/data.json`
  - `pool.json.bak` 每日备份规则

#### 前端

- 增加“导入本地数据/重载数据”的调试入口
- 展示保存结果和错误提示

#### 产出

- 合并策略可重复验证
- 本地存储结构固定下来

---

### 阶段 4：日志 URL 提取与手动 URL 输入

目标：接入两种输入方式。

#### 4.1 自动从日志提取

Rust：

- `gacha_log/`
  - 根据配置找到日志文件：`{gameRootDir}/Client/Saved/Logs/Client.log`
  - **先按 `new-ui-dev` 规则整文件解密 `Client.log`**
    - 对每个字节：`((b & 0x0F) % 2 == 1) ? b ^ 0xA5 : b ^ 0xEF`
    - 再按 UTF-8 解码
  - 在解密后的文本上用正则匹配抽卡 URL
  - 取最后一个匹配项

说明：

- 当前游戏日志已加密，不能再按旧分支的“直接读文本”方式实现
- 实现与文档都以参考仓库 `new-ui-dev` 的 `CardPoolGetUrlTask` 为准

#### 4.2 手动输入 URL

前端：

- 新增一个输入框/弹窗
- 允许用户粘贴日志中提取出来的 URL
- 前端只做基础非空校验

Rust：

- `gacha_params/parser.rs`
  - 解析 URL query
  - 转换为 `RequestParams`
  - 校验必要字段是否存在

#### 设计重点

“自动日志提取”和“手动输入 URL”最后都应收敛成统一的：

```text
RequestParams
```

即：

- 输入来源不同
- 后续抓取、合并、分析流程完全一致

#### 产出

- 支持自动获取 URL
- 支持手动输入 URL
- 输入层扩展成功与核心分析解耦

---

### 阶段 5：抽卡接口请求

目标：从官方接口获取真实记录。

#### Rust

实现：

- `gacha_api/client.rs`
  - 按 `playerId` 判断 CN / GLOBAL 接口
  - 遍历卡池逐池请求
  - 解析响应
  - 记录每池请求日志
- 与 `gacha_merge`、`gacha_storage` 联动

#### 建议依赖

建议 Rust 增加：

- `reqwest`：HTTP 请求
- `tokio`：异步运行时

原因：

- 成熟稳定
- 易于维护
- 日志、超时、错误处理体验更好

#### 前端

- “获取抽卡数据”按钮
- “刷新当前玩家数据”按钮
- 展示获取进度、成功/失败状态

#### 产出

- 能从官方接口拉取真实数据
- 能落盘保存并触发分析

---

### 阶段 6：总览页与详情页 UI

目标：把分析结果变成可用界面。

#### 总览页

建议展示：

- 角色活动池
- 武器活动池
- 角色常驻池
- 武器常驻池

每个卡池展示：

- 卡池名
- 时间范围
- 总抽数
- 5 星已垫
- 4 星已垫
- 5 星平均
- 5 星数量占比
- 4 星数量占比
- 最近 5 星历史

#### 详情页

建议展示：

- 总抽数
- 总消耗
- 5/4/3 星数量和占比
- 5 星平均/最欧/最非
- 限定平均
- 5 星历史列表
- 饼图

#### 前端技术建议

- 用 Svelte 组件拆分卡片、列表、统计面板
- 图表优先选轻量库，或先用纯 CSS / SVG 简化
- 不要过早引入重图表框架

---

### 阶段 7：图片资源同步

目标：支持头像/武器图标的本地展示。

#### Rust

实现：

- `resource_sync/service.rs`
  - 下载 `Root_{language}.json`
  - 比较本地版本
  - 下载缺失或 MD5 不一致的资源
  - 保存到本地 `assets/header/...`

#### 建议依赖

- `reqwest`
- `md5` 或等价摘要库
- `base64`（如果资源清单或内容需要）

#### 前端

- 资源同步状态提示
- 图标缺失时显示占位图

#### 产出

- 本地图片链路打通
- 抽卡历史和详情不依赖远程实时图片

---

### 阶段 8：设置页与可维护性补强

目标：把调试态项目变成正常可用应用。

建议加入：

- 设置游戏根目录
- 设置资源源（GitHub / Gitee）
- 设置日志级别
- 设置是否跳过最早 5 星
- 查看当前玩家列表
- 重新同步资源
- 清理缓存/重建索引（如后续需要）

---

## 7. Tauri Command 设计建议

建议不要暴露很多零散 command，而是按业务分组。

### 7.1 设置相关

- `get_app_config`
- `update_app_config`

### 7.2 抽卡输入相关

- `extract_gacha_url_from_log`
- `parse_gacha_url`
- `save_manual_gacha_url`

> `save_manual_gacha_url` 也可以并入 `parse_gacha_url` 后的统一抓取流程，不一定必须单独存在。

### 7.3 抽卡数据相关

- `fetch_gacha_records`
- `refresh_player_gacha_records`
- `load_local_gacha_pool`
- `analyze_local_gacha_pool`
- `list_players`

### 7.4 资源相关

- `sync_gacha_resources`
- `get_local_resource_status`

### 7.5 更推荐的进一步收敛

如果想让接口更稳定，可进一步减少 command 数量：

- `run_gacha_fetch`
- `run_gacha_analysis`
- `run_resource_sync`
- `get_app_state`
- `update_app_config`

即让 command 更偏“用例级”，而不是“函数级”。

---

## 8. Rust 依赖建议

当前 `Cargo.toml` 只有：

- `tauri`
- `serde`
- `serde_json`
- `time`

建议后续按阶段增加：

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"

tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["fmt", "env-filter"] }
thiserror = "1"
anyhow = "1"
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
regex = "1"
url = "2"
directories = "5"
md5 = "0.7"
base64 = "0.22"
```

说明：

- `tracing`：统一日志
- `thiserror`：清晰错误类型
- `anyhow`：上层聚合错误更方便
- `reqwest`：网络请求
- `tokio`：异步运行时
- `regex`：日志 URL 提取
- `url`：解析手动输入 URL
- `directories`：跨平台应用目录
- `md5`：资源校验
- `base64`：资源同步可能需要

如果想进一步追求性能和 JSON 处理体验，可后续评估：

- `simd-json`

但第一阶段不建议为了“新”而引入复杂度。

---

## 9. 前端状态管理建议

### 9.1 状态划分

建议分为：

- `appStore`
  - 配置
  - 全局错误
  - 全局 loading
- `gachaStore`
  - 当前玩家
  - 玩家列表
  - 当前卡池分析结果
  - 获取/刷新状态
- `resourceStore`
  - 资源同步状态
  - 本地图标可用性状态

### 9.2 不要做的事

- 不要把大量业务逻辑塞进 `+page.svelte`
- 不要直接在组件里 `invoke` 一堆 command
- 不要在前端重写一套分析算法

建议：

- `src/lib/api/tauri/*.ts` 统一封装 command
- `src/lib/features/gacha/stores` 管理业务状态
- 组件只负责展示和事件转发

---

## 10. 日志与调试策略

### 10.1 Rust 侧

建议使用 `tracing`，至少记录：

- 配置加载完成
- 数据目录路径
- 日志文件发现结果
- URL 提取成功/失败
- URL 参数解析结果摘要
- 请求目标服区
- 每个卡池的请求开始/结束
- 合并前数量 / 合并后数量
- 分析结果摘要
- 资源同步下载数量 / 失败数量

### 10.2 前端侧

前端不需要铺天盖地 `console.log`，但关键交互要有：

- 用户触发了哪一步
- command 调用是否成功
- 错误提示是否完整

### 10.3 调试优先级

最先保证能查的问题：

1. 日志找不到
2. URL 解析失败
3. 接口请求失败
4. 数据保存失败
5. 合并结果异常
6. 分析结果不符合预期
7. 图片缺失

---

## 11. 推荐开发阶段验收标准

### 阶段 1 验收

- 应用可以启动
- 配置可以读写
- Rust 日志可见
- Hello World 已移除

### 阶段 2 验收

- 给定一份 `pool.json`，能输出正确 `AnalysisData`
- 总览和详情能显示正确数字

### 阶段 3 验收

- 给定旧记录和新记录，合并结果符合文档规则
- `pool.json.bak` 备份规则正确

### 阶段 4 验收

- 能从日志提取 URL
- 能手动输入 URL 并解析成参数
- 自动输入和手动输入都能进入同一抓取流程

### 阶段 5 验收

- 能成功请求至少一个真实玩家数据
- 能保存 `data.json` 和 `pool.json`
- 能刷新已有玩家数据

### 阶段 6 验收

- 总览页和详情页可用
- 切换卡池无明显卡顿

### 阶段 7 验收

- 头像/武器图标可本地显示
- 资源缺失时能优雅降级

---

## 12. 当前最推荐的实际起步顺序

如果现在就开始做，我建议严格按下面顺序推进：

1. 删除 Hello World 页面和 `greet` command
2. 建立 Rust 模块骨架、错误类型、日志初始化
3. 建立配置系统
4. 先只实现 `pool.json -> AnalysisData` 的离线分析
5. 前端做出总览/详情静态展示
6. 再实现新旧数据合并
7. 再实现日志提取与手动 URL 输入
8. 再实现官方接口请求
9. 最后实现资源同步和设置页

这是当前项目最稳、返工最少、最符合 `AGENTS.md` 的路线。
