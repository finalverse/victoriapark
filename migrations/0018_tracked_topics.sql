-- Permanent, bilingual special topics.
--
-- Trend-created gaggles are intentionally temporary. A subject the newsroom
-- has committed to follow needs different semantics: one independent page per
-- edition, explicit search anchors, a durable analysis brief and primary
-- sources readers can inspect without trusting our prose.

ALTER TABLE gaggles
    ADD COLUMN IF NOT EXISTS editorial_language TEXT NOT NULL DEFAULT 'en'
        CHECK (editorial_language IN ('zh', 'en')),
    ADD COLUMN IF NOT EXISTS pinned BOOLEAN NOT NULL DEFAULT FALSE,
    ADD COLUMN IF NOT EXISTS analysis_md TEXT NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS watchpoints TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN IF NOT EXISTS anchor_terms TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN IF NOT EXISTS keywords TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN IF NOT EXISTS primary_source_names TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN IF NOT EXISTS primary_source_urls TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN IF NOT EXISTS last_briefed_at TIMESTAMPTZ;

-- The same public slug can now be a genuinely independent Chinese and English
-- edition. This also makes the masthead's /en switch work naturally.
ALTER TABLE gaggles DROP CONSTRAINT IF EXISTS gaggles_slug_key;
CREATE UNIQUE INDEX IF NOT EXISTS gaggles_slug_language
    ON gaggles (slug, editorial_language);

INSERT INTO gaggles (
    id, topic, slug, title, standfirst, source_count, story_count, model,
    editorial_language, pinned, analysis_md, watchpoints, anchor_terms,
    keywords, primary_source_names, primary_source_urls, last_briefed_at
) VALUES (
    gen_random_uuid(),
    'tracked:us-canada-trade-war:zh',
    'us-canada-trade-war',
    '美加贸易战全景追踪',
    '美国针对加拿大酒类、乳制品与汽车的新一轮附加关税在短暂延后后于 8 月 22 日生效；加拿大宣布暂停谈判，并将从 9 月 8 日起按金额与税率对等反制。本专题独立追踪关税清单、CUSMA 审查、供应链、就业、物价与重启谈判的信号。',
    4, 0, 'VictoriaPark Trade Watch', 'zh', TRUE,
    $zh$
## 局势速览

**截至 2026 年 8 月 25 日：** 美国政府称，针对加拿大酒类、乳制品和汽车的 Section 338 附加关税在延后 3 天后，于 8 月 22 日生效。加拿大财政部随后称，美国对约 276 亿加元加拿大商品加征 50% 关税；加拿大将从 9 月 8 日起，对美国商品实施 15%、25% 和 50% 的对应关税，并推出 75 亿加元的企业与就业支持方案。

## 这场冲突为何重要

这不是单一商品的争端，而是北美一体化供应链与国家政策自主权之间的再平衡。汽车零部件、钢铝、能源、农产品和消费品多次跨境，名义上由进口商缴纳的关税，最终可能通过价格、订单、工资和投资传导给家庭与社区。2026 年 CUSMA 联合审查又把原产地规则、乳制品市场准入和产业安全放在同一张谈判桌上。

## VictoriaPark 观点

国家有权维护主权、互惠准入和本国劳动者，但关税本质上也是税，成本不能被口号掩盖。符合传统价值的贸易政策，应同时维护法治、契约可预期性、家庭购买力、社区就业和政府权力的可监督性。短期反制可以形成谈判筹码；若演变为永久补贴、任意行政裁量或没有退出条件的壁垒，就会把代价转嫁给普通家庭和中小企业。最稳健的终局仍是可执行、可审查、对双方同等适用的北美贸易规则。

## 证据边界

美国与加拿大官方文件分别陈述各自的法律依据和政策立场，不应被当作中立裁判。本页把官方文本作为“政府采取了什么行动”的一手证据；对就业、价格和贸易流量的实际影响，则持续以统计数据和多家独立报道交叉核验。
$zh$,
    ARRAY[
        '加拿大 9 月 8 日反制清单的实际征收与豁免',
        '美加谈判是否恢复，以及 Section 338 措施的延长、扩大或撤销',
        'CUSMA 联合审查中的原产地、乳制品与争端解决条款',
        '汽车、钢铝、木材、能源和农业供应链的停工与投资信号',
        '两国通胀、就业、贸易量与加元汇率的可验证变化',
        '法院、国会与议会对行政关税权的审查'
    ],
    ARRAY['加拿大', '加美', '美加', 'Canada', 'Canadian'],
    ARRAY['关税', '贸易战', '反制', '贸易谈判', 'CUSMA', 'USMCA', 'Section 338', '钢铝', '汽车', '乳制品'],
    ARRAY[
        '加拿大财政部：8 月 25 日反制与支持方案',
        '加拿大财政部：9 月 8 日反制商品清单',
        '白宫：8 月 18 日 Section 338 临时延后令',
        '美国贸易代表办公室：2026 年贸易政策议程',
        '加拿大统计局：2026 年春季经济进展'
    ],
    ARRAY[
        'https://www.canada.ca/en/department-finance/news/2026/08/canada-announces-targeted-countermeasures-and-substantive-support-for-workers-and-businesses-in-response-to-us-tariffs.html',
        'https://www.canada.ca/en/department-finance/news/2026/08/list-of-products-from-the-united-states-subject-to-counter-tariffs-effective-september-8-2026.html',
        'https://www.whitehouse.gov/presidential-actions/2026/08/temporary-suspension-of-additional-duties-to-offset-canadian-discrimination-against-the-commerce-of-the-united-states-with-respect-to-alcoholic-beverages-dairy-and-motor-vehicles/',
        'https://ustr.gov/sites/default/files/files/Press/Releases/2026/2026%20Trade%20Policy%20Agenda.pdf',
        'https://www150.statcan.gc.ca/n1/pub/36-28-0001/2026004/article/00005-eng.htm'
    ],
    now()
), (
    gen_random_uuid(),
    'tracked:us-canada-trade-war:en',
    'us-canada-trade-war',
    'U.S.–Canada Trade War',
    'New U.S. duties on selected Canadian alcohol, dairy and motor-vehicle goods took effect August 22 after a three-day delay. Canada has suspended negotiations and says matched counter-tariffs will begin September 8; this page tracks the measures, the USMCA review and the effects on workers, consumers and North American supply chains.',
    4, 0, 'VictoriaPark Trade Watch', 'en', TRUE,
    $en$
## Where matters stand

**As of August 25, 2026:** The White House says additional Section 338 duties covering selected Canadian alcoholic beverage, dairy and motor-vehicle goods took effect August 22 after a three-day suspension. Canada says the U.S. action applies a 50% tariff to C$27.6 billion of Canadian goods. Ottawa plans matching tariffs of 15%, 25% and 50% on selected U.S. products from September 8 and has announced C$7.5 billion in worker and business support.

## Why this is larger than a tariff list

The dispute tests whether an integrated North American production system can coexist with increasingly assertive national industrial policy. Autos and parts, metals, energy, farm goods and consumer products cross the border repeatedly; tariffs formally collected from importers can travel through prices, orders, wages and investment into household and community life. The 2026 USMCA joint review puts rules of origin, dairy access, industrial security and dispute settlement into the same negotiation.

## VictoriaPark view

Reciprocal market access and national sovereignty are legitimate public aims. Tariffs are also taxes, and their cost should not be hidden behind patriotic language. A conservative trade policy should defend the rule of law, predictable contracts, family purchasing power, productive work and accountable executive power at the same time. A proportionate response may create bargaining leverage; permanent subsidies, open-ended discretion and barriers with no exit test would transfer too much of the bill to families and smaller firms. The durable end state is an enforceable North American bargain whose rules bind both governments.

## Evidence boundary

U.S. and Canadian releases are primary evidence of what each government did and how it justifies the action; neither is a neutral assessment of economic impact. VictoriaPark will test claims about jobs, prices and trade flows against official statistics and independent reporting as data arrive.
$en$,
    ARRAY[
        'Implementation, exclusions and remission under Canada’s September 8 counter-tariff list',
        'Whether talks restart and whether Section 338 duties expand, lapse or are withdrawn',
        'Rules of origin, dairy access and dispute settlement in the USMCA joint review',
        'Shutdown, hiring and investment signals in autos, metals, lumber, energy and agriculture',
        'Measurable changes in inflation, employment, bilateral trade and the Canadian dollar',
        'Legislative and judicial scrutiny of executive tariff authority'
    ],
    ARRAY['Canada', 'Canadian', 'Ottawa', 'U.S.-Canada', 'US-Canada'],
    ARRAY['tariff', 'trade war', 'counter-tariff', 'trade negotiation', 'USMCA', 'CUSMA', 'Section 338', 'steel', 'aluminum', 'automobile', 'dairy'],
    ARRAY[
        'Finance Canada: August 25 countermeasures and support',
        'Finance Canada: September 8 counter-tariff product list',
        'White House: August 18 Section 338 suspension proclamation',
        'USTR: 2026 Trade Policy Agenda',
        'Statistics Canada: Recent economic developments, spring 2026'
    ],
    ARRAY[
        'https://www.canada.ca/en/department-finance/news/2026/08/canada-announces-targeted-countermeasures-and-substantive-support-for-workers-and-businesses-in-response-to-us-tariffs.html',
        'https://www.canada.ca/en/department-finance/news/2026/08/list-of-products-from-the-united-states-subject-to-counter-tariffs-effective-september-8-2026.html',
        'https://www.whitehouse.gov/presidential-actions/2026/08/temporary-suspension-of-additional-duties-to-offset-canadian-discrimination-against-the-commerce-of-the-united-states-with-respect-to-alcoholic-beverages-dairy-and-motor-vehicles/',
        'https://ustr.gov/sites/default/files/files/Press/Releases/2026/2026%20Trade%20Policy%20Agenda.pdf',
        'https://www150.statcan.gc.ca/n1/pub/36-28-0001/2026004/article/00005-eng.htm'
    ],
    now()
)
ON CONFLICT (topic) DO UPDATE SET
    slug = EXCLUDED.slug,
    title = EXCLUDED.title,
    standfirst = EXCLUDED.standfirst,
    model = EXCLUDED.model,
    editorial_language = EXCLUDED.editorial_language,
    pinned = TRUE,
    analysis_md = EXCLUDED.analysis_md,
    watchpoints = EXCLUDED.watchpoints,
    anchor_terms = EXCLUDED.anchor_terms,
    keywords = EXCLUDED.keywords,
    primary_source_names = EXCLUDED.primary_source_names,
    primary_source_urls = EXCLUDED.primary_source_urls,
    last_briefed_at = EXCLUDED.last_briefed_at,
    last_hot_at = now();
