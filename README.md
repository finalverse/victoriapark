# VictoriaPark

**中文优先、英文独立编辑的全球 AI 自主新闻平台。**

VictoriaPark 从政治、国际局势与全球头条出发，逐步覆盖财经、科技、人工智能、科学、健康、文化与体育。整套编辑部由自主代理运行：监看来源、识别突发、聚类同一事件、提取事实主张、交叉核验、写稿、审稿、发布、生成专题与微信候选稿，并持续复核和更正。

传统价值是 VictoriaPark 的公开编辑观察框架：人的尊严、家庭与社区、信仰与良知自由、法治、国家主权、责任、公民秩序、有限而受监督的权力、自由市场与代际传承。它影响选题与评论，不改变证据标准；符合编辑取向的主张同样必须求证，不利于编辑取向的坚实事实同样必须如实发布。

## 核心架构

Rust 全栈，保持来源项目技术架构的端到端设计：

```text
RSS / 合法索引
      │
      ▼
地平线 ─▶ 快讯编辑 ─▶ 聚类编辑
                         │
           ┌─────────────┴─────────────┐
           │ 原创报道                  │ 全球快讯
           ▼                           ▼
        主笔 ─▶ 求证 ─▶ 数据背景      分发台
              ─▶ 标题台 ─▶ 总编          │
                         │               │
                         └──────▶ 发布 ◀─┘
                                    │
                           监察 / 纵深 / 专题 / 微信
```

原子单位不是文章，而是事实主张：

```text
原始材料 ─▶ 事件 ─▶ 主张（来源、立场、置信度） ─▶ 文章与渠道稿件
```

- `bg-core`：领域模型、双语编辑流、版权与发布政策，WASM-safe。
- `bg-db`：PostgreSQL 17 + pgvector、迁移与证据图仓储。
- `bg-ingest`：RSS、robots.txt、条件请求、正文读取、去重与图片来源。
- `bg-llm`：Anthropic、OpenAI-compatible、本地模型与离线 stub，多级路由和成本账本。
- `bg-agents`：自主编辑部、母系统提示词与角色子提示词。
- `bg-api`：REST、RSS、OpenAPI 与 MCP。
- `bg-web`：Leptos SSR + hydration，中文 `/` 与独立英文 `/en`。
- `bg-cli`：迁移、播种、单轮运行、常驻 worker、诊断与更正。

## 编辑提示词

实际运行时直接编译以下文件，不存在文档与生产提示词漂移：

- [`prompts/master-system.md`](prompts/master-system.md)：母系统提示词与不可覆盖原则。
- `prompts/scout.md` 至 `prompts/skein.md`：各编辑角色。
- [`prompts/wechat.md`](prompts/wechat.md)：微信公众号候选稿。

中文和英文不是翻译关系。来源材料在进入事件聚类前即按编辑语言隔离，故事记录冻结 `editorial_language`，首页查询和模型调用都显式传入语言。

## 自动运行

```bash
cp .env.example .env
docker compose up -d
cargo run -p bg-cli -- migrate
cargo run -p bg-cli -- seed
cargo run -p bg-cli -- doctor
cargo run -p bg-cli -- run
cargo run -p bg-cli -- worker --interval 300 --fast-interval 90
```

常驻 worker 每 90 秒运行无模型的快速通道（抓取、突发与专题热度），每 5 分钟运行完整编辑通道；积压时自动缩短间隔，连续失败时指数退避。已发布的中文报道会生成 `wechat_packages` 草稿：标题、长摘要、事实列表、未知事项、VictoriaPark 观点和来源说明。图片优先采用原报道主图；没有合适图片时自动使用 VictoriaPark 为该报道绘制的方形品牌新闻卡。

## 版权与事实边界

- 来源全文只存于私有分析字段，不对外提供。
- 公开文字必须跨来源原创综合；直接引语最多 25 个英文单词或等量短句。
- 任何连续 28 个英文单词以上的来源复现都会阻止发布。
- 每项公开事实必须至少链接一个来源；重大原创稿原则上需要两个独立来源或一手证据。
- 报道、VictoriaPark 观点与预测分别标注；预测必须有期限和可证伪信号。
- 原始主图只有在来源提供且允许展示时才使用，并保留署名和链接；否则标记为需要 VictoriaPark 自有生成图。

## 本地要求

Rust 1.90+、Docker、`wasm32-unknown-unknown` 和 `cargo-leptos`。默认 `BG_LLM_PROVIDER=stub` 可在无 API 密钥情况下验证完整管线；生产环境应配置真实模型提供商、成本限额、数据库凭据与公开域名。

## 品牌

主品牌资产位于 `public/victoriapark-mark.png`，分享卡位于 `public/og-default.png`。域名与部署单元统一为 `victoriapark.io` / `victoriapark-*`。

MIT License。
