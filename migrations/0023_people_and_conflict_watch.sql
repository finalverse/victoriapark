-- Permanent people and conflict watches requested by the VictoriaPark desk.
--
-- These dossiers are search assignments as well as pages. Scout revisits their
-- anchors every twenty minutes; Gaggle attaches matching published stories and
-- only promotes a dossier once it contains at least five articles.

INSERT INTO gaggles (
    id, topic, slug, title, standfirst, source_count, story_count, model,
    editorial_language, pinned, analysis_md, watchpoints, anchor_terms,
    keywords, primary_source_names, primary_source_urls, last_briefed_at
) VALUES
(
    gen_random_uuid(), 'tracked:donald-trump', 'donald-trump',
    '特朗普全景追踪',
    '持续追踪美国总统唐纳德·J·特朗普的白宫决策、行政命令、外交与贸易政策、司法争议、民调及其对美国社会和全球秩序的实际影响。',
    0, 0, 'VictoriaPark People Watch', 'zh', TRUE,
    '## 编辑基线\n\n人物影响力不能替代证据。维园网把政府已经采取的行动、当事人表态、司法或国会程序、数据结果与评论推断分别标示；支持边境安全、国家主权、家庭与社区等传统价值，也持续审查行政权、财政成本、法治和政策兑现。',
    ARRAY['白宫行政命令与执行结果','外交、关税与产业政策','国会及法院对行政权的制衡','选举、民调与共和党内部动向','政策对家庭、就业、物价和社区安全的影响'],
    ARRAY['特朗普','川普','Donald Trump','D.J. Trump'],
    ARRAY['白宫','总统','行政令','关税','外交','国会','法院','民调','共和党'],
    ARRAY['白宫新闻发布','美国国会记录','美国联邦最高法院'],
    ARRAY['https://www.whitehouse.gov/news/','https://www.congress.gov/','https://www.supremecourt.gov/'], now()
),
(
    gen_random_uuid(), 'tracked:elon-musk', 'elon-musk',
    '埃隆·马斯克全景追踪',
    '持续追踪埃隆·马斯克及特斯拉、SpaceX、xAI 和 X 的公司决策、技术进展、监管争议、政治影响与市场结果。',
    0, 0, 'VictoriaPark People Watch', 'zh', TRUE,
    '## 编辑基线\n\n维园网区分企业宣传、监管文件、可复现技术进展、资本市场表现与政治评论。创新和企业家精神值得尊重，但公共合同、产品安全、公司治理、股东利益和平台权力同样必须接受事实审查。',
    ARRAY['特斯拉交付、利润、自动驾驶与监管','SpaceX 发射、星链与政府合同','xAI 模型、融资和基础设施','X 平台治理与政治传播','马斯克的政策角色及利益冲突'],
    ARRAY['马斯克','埃隆·马斯克','Elon Musk'],
    ARRAY['特斯拉','Tesla','SpaceX','星链','Starlink','xAI','X平台','监管','政府合同'],
    ARRAY['特斯拉投资者关系','SpaceX 更新','美国证券交易委员会公司文件'],
    ARRAY['https://ir.tesla.com/','https://www.spacex.com/updates/','https://www.sec.gov/edgar/search/'], now()
),
(
    gen_random_uuid(), 'tracked:jensen-huang', 'jensen-huang',
    '黄仁勋与英伟达全景追踪',
    '持续追踪黄仁勋、英伟达芯片路线、AI 基础设施、出口管制、供应链、竞争格局及估值与产业影响。',
    0, 0, 'VictoriaPark People Watch', 'zh', TRUE,
    '## 编辑基线\n\nAI 热潮中的产品发布、基准测试、订单、监管文件和分析师推断必须分开。维园网关注技术进步，也检验产业集中、能源成本、出口管制、国家安全、股东风险和普通劳动者能否分享生产率收益。',
    ARRAY['GPU 与 AI 系统路线图','财报、订单、产能和客户集中','对华出口管制及合规','台积电与全球半导体供应链','竞争者、自研芯片与 AI 投资周期'],
    ARRAY['黄仁勋','黃仁勳','Jensen Huang','Jensen H. Huang'],
    ARRAY['英伟达','辉达','Nvidia','GPU','AI芯片','出口管制','财报','供应链'],
    ARRAY['NVIDIA 新闻中心','NVIDIA 投资者关系','美国证券交易委员会公司文件'],
    ARRAY['https://nvidianews.nvidia.com/','https://investor.nvidia.com/','https://www.sec.gov/edgar/search/'], now()
),
(
    gen_random_uuid(), 'tracked:donald-trump', 'donald-trump',
    'Donald J. Trump — Presidency Watch',
    'Continuous coverage of President Donald J. Trump’s executive actions, foreign and trade policy, legal disputes, polling and measurable effects on the United States and the world.',
    0, 0, 'VictoriaPark People Watch', 'en', TRUE,
    '## Editorial baseline\n\nPower and popularity are not evidence. VictoriaPark separates enacted policy, personal statements, legislative or judicial process, measured outcomes and commentary. The desk takes sovereignty, secure borders, family and community seriously while scrutinising executive power, fiscal costs, due process and delivery against promises.',
    ARRAY['White House actions and implementation','Foreign, tariff and industrial policy','Congressional and judicial checks','Polling, elections and Republican politics','Effects on families, jobs, prices and public safety'],
    ARRAY['Donald Trump','Donald J. Trump','D.J. Trump','President Trump'],
    ARRAY['White House','executive order','tariff','foreign policy','Congress','court','poll','Republican'],
    ARRAY['White House news','Congress.gov','U.S. Supreme Court'],
    ARRAY['https://www.whitehouse.gov/news/','https://www.congress.gov/','https://www.supremecourt.gov/'], now()
),
(
    gen_random_uuid(), 'tracked:elon-musk', 'elon-musk',
    'Elon Musk — Companies and Influence',
    'Continuous coverage of Elon Musk and the decisions, technology, regulation, political influence and market results surrounding Tesla, SpaceX, xAI and X.',
    0, 0, 'VictoriaPark People Watch', 'en', TRUE,
    '## Editorial baseline\n\nCorporate promotion, regulatory filings, reproducible technical progress, market performance and political opinion are separate forms of evidence. Enterprise deserves room to build; public contracts, product safety, governance, shareholder interests and platform power still require scrutiny.',
    ARRAY['Tesla deliveries, margins, autonomy and safety','SpaceX launches, Starlink and public contracts','xAI models, financing and infrastructure','X governance and political communication','Public roles and conflicts of interest'],
    ARRAY['Elon Musk','Musk'],
    ARRAY['Tesla','SpaceX','Starlink','xAI','X platform','regulation','government contract'],
    ARRAY['Tesla investor relations','SpaceX updates','SEC company filings'],
    ARRAY['https://ir.tesla.com/','https://www.spacex.com/updates/','https://www.sec.gov/edgar/search/'], now()
),
(
    gen_random_uuid(), 'tracked:jensen-huang', 'jensen-huang',
    'Jensen Huang and Nvidia — AI Infrastructure',
    'Continuous coverage of Jensen Huang, Nvidia’s product roadmap, AI infrastructure, export controls, supply chains, competition and the company’s industrial impact.',
    0, 0, 'VictoriaPark People Watch', 'en', TRUE,
    '## Editorial baseline\n\nProduct launches, benchmarks, orders, filings and analyst inference are labelled separately. VictoriaPark welcomes technical progress while testing concentration risk, energy costs, export controls, national-security claims and whether productivity gains reach workers and families.',
    ARRAY['GPU and AI-system roadmap','Earnings, orders, capacity and customer concentration','Export controls and compliance','TSMC and semiconductor supply chains','Competitors, custom silicon and the AI investment cycle'],
    ARRAY['Jensen Huang','Jensen H. Huang'],
    ARRAY['Nvidia','GPU','AI chip','export control','earnings','supply chain'],
    ARRAY['NVIDIA newsroom','NVIDIA investor relations','SEC company filings'],
    ARRAY['https://nvidianews.nvidia.com/','https://investor.nvidia.com/','https://www.sec.gov/edgar/search/'], now()
),
(
    gen_random_uuid(), 'tracked:russia-ukraine-war', 'russia-ukraine-war',
    'Russia–Ukraine War',
    'Continuous, evidence-bounded coverage of battlefield changes, diplomacy, sanctions, military aid, civilian harm and the war’s effects on energy, food and European security.',
    0, 0, 'VictoriaPark Geopolitics Watch', 'en', TRUE,
    '## Editorial baseline\n\nNo belligerent’s communique establishes battlefield fact by itself. Territorial control, casualties and attribution require dated sourcing and explicit uncertainty. VictoriaPark supports national sovereignty, civilian life, the rule of law and an enforceable peace while scrutinising propaganda and the public cost of war.',
    ARRAY['Verified battlefield and territorial changes','Ceasefire talks and security guarantees','Sanctions and military aid','Energy, food, refugees and family costs','War-crimes allegations and investigations'],
    ARRAY['Russia','Ukraine','Russia-Ukraine','Putin','Zelenskyy'],
    ARRAY['war','ceasefire','peace talks','sanctions','military aid','front line','drone','missile'],
    ARRAY['United Nations Ukraine focus','Office of the President of Ukraine','President of Russia'],
    ARRAY['https://news.un.org/en/focus/ukraine','https://www.president.gov.ua/en/news/all','http://en.kremlin.ru/events/president/news'], now()
),
(
    gen_random_uuid(), 'tracked:us-iran-hormuz', 'us-iran-hormuz',
    'U.S.–Iran and Strait of Hormuz Crisis',
    'Continuous coverage of U.S.–Iran military and diplomatic action, Strait of Hormuz shipping, energy markets, regional allies and verifiable escalation signals.',
    0, 0, 'VictoriaPark Geopolitics Watch', 'en', TRUE,
    '## Editorial baseline\n\nOfficial warnings, actual deployments, confirmed incidents and analytical scenarios are different things. Coverage tests policy against freedom of navigation, sovereignty, proportionality, civilian protection and accountable war powers, and follows the costs into energy bills and supply chains.',
    ARRAY['Shipping, seizures and attacks in the strait','Force deployments and rules of engagement','Nuclear diplomacy and sanctions','Regional allies and proxy forces','Oil, insurance and shipping diversions'],
    ARRAY['Iran','United States','Strait of Hormuz','Persian Gulf'],
    ARRAY['shipping','blockade','tanker','sanctions','nuclear talks','U.S. military','IRGC'],
    ARRAY['U.S. Central Command','U.S. State Department Iran page','International Maritime Organization'],
    ARRAY['https://www.centcom.mil/MEDIA/PRESS-RELEASES/','https://www.state.gov/countries-areas/iran/','https://www.imo.org/en/MediaCentre/Pages/Default.aspx'], now()
)
ON CONFLICT (topic, editorial_language) DO UPDATE SET
    slug = EXCLUDED.slug,
    title = EXCLUDED.title,
    standfirst = EXCLUDED.standfirst,
    model = EXCLUDED.model,
    pinned = TRUE,
    analysis_md = EXCLUDED.analysis_md,
    watchpoints = EXCLUDED.watchpoints,
    anchor_terms = EXCLUDED.anchor_terms,
    keywords = EXCLUDED.keywords,
    primary_source_names = EXCLUDED.primary_source_names,
    primary_source_urls = EXCLUDED.primary_source_urls,
    last_hot_at = now(),
    last_searched_at = NULL;
