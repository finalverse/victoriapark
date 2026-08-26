-- Five independent editorial editions and durable Chinese geopolitical watches.

ALTER TABLE stories DROP CONSTRAINT IF EXISTS stories_editorial_language_check;
ALTER TABLE stories ADD CONSTRAINT stories_editorial_language_check
    CHECK (editorial_language IN ('zh', 'zh-hant', 'en', 'ja', 'ko'));

ALTER TABLE gaggles DROP CONSTRAINT IF EXISTS gaggles_editorial_language_check;
ALTER TABLE gaggles ADD CONSTRAINT gaggles_editorial_language_check
    CHECK (editorial_language IN ('zh', 'zh-hant', 'en', 'ja', 'ko'));

ALTER TABLE gaggles DROP CONSTRAINT IF EXISTS gaggles_topic_key;
DROP INDEX IF EXISTS gaggles_slug_language;
CREATE UNIQUE INDEX IF NOT EXISTS gaggles_topic_language_unique
    ON gaggles (topic, editorial_language);
CREATE UNIQUE INDEX IF NOT EXISTS gaggles_slug_language_unique
    ON gaggles (slug, editorial_language);
CREATE INDEX IF NOT EXISTS gaggles_language_hot
    ON gaggles (editorial_language, last_hot_at DESC);

INSERT INTO gaggles (
    id, topic, slug, title, standfirst, source_count, story_count, model,
    editorial_language, pinned, analysis_md, watchpoints, anchor_terms,
    keywords, primary_source_names, primary_source_urls, last_briefed_at
) VALUES
(
    gen_random_uuid(), 'tracked:russia-ukraine-war', 'russia-ukraine-war',
    '俄乌战争持续追踪',
    '持续追踪战场变化、外交谈判、制裁与援助、能源粮食影响及战争责任。热度不能替代证据；每次重大更新均区分交战方声明、可独立核验事实与维园网分析。',
    0, 0, 'VictoriaPark Geopolitics Watch', 'zh', TRUE,
    '## 编辑基线\n\n本专题不依据任何一方的战报直接判断战果。领土控制、伤亡和袭击责任须标明时间、来源与不确定性；卫星资料、现场影像、国际组织记录和多家独立报道优先。维园网支持国家主权、平民生命、法治与可执行和平，同时审查所有政府的宣传、权力扩张与战争成本。',
    ARRAY['前线与领土控制的可核验变化','停火、谈判与安全保证','对俄制裁及对乌军援的执行变化','能源、粮食、难民与家庭成本','战争罪指控与司法调查'],
    ARRAY['俄罗斯','乌克兰','俄乌','普京','泽连斯基'],
    ARRAY['战争','停火','和谈','制裁','军援','前线','无人机','导弹'],
    ARRAY['联合国乌克兰专题','乌克兰总统府','俄罗斯总统府'],
    ARRAY['https://news.un.org/en/focus/ukraine','https://www.president.gov.ua/en/news/all','http://en.kremlin.ru/events/president/news'], now()
),
(
    gen_random_uuid(), 'tracked:us-iran-hormuz', 'us-iran-hormuz',
    '美伊与霍尔木兹海峡危机',
    '持续追踪美伊军事与外交行动、霍尔木兹海峡航运安全、能源价格、地区盟友及冲突升级信号。平台热搜只触发跟进，军事与伤亡事实必须交叉核验。',
    0, 0, 'VictoriaPark Geopolitics Watch', 'zh', TRUE,
    '## 编辑基线\n\n霍尔木兹海峡承担关键能源运输，也是军事误判最可能迅速传导至普通家庭物价与全球供应链的地点之一。本专题严格区分官方威慑声明、实际部署、已发生事件与分析推断；从航行自由、国家主权、比例原则、平民保护和受监督的战争权力出发评估政策。',
    ARRAY['海峡通航、扣船与袭船事件','美伊兵力部署及交战规则','核问题谈判与制裁变化','海湾国家、以色列及代理力量动向','油价、保险费率与航运改道'],
    ARRAY['伊朗','美国','霍尔木兹','波斯湾','Iran','Hormuz'],
    ARRAY['海峡','封锁','航运','油轮','制裁','核谈判','美军','革命卫队'],
    ARRAY['美国中央司令部','美国国务院伊朗专题','国际海事组织'],
    ARRAY['https://www.centcom.mil/MEDIA/PRESS-RELEASES/','https://www.state.gov/countries-areas/iran/','https://www.imo.org/en/MediaCentre/Pages/Default.aspx'], now()
),
(
    gen_random_uuid(), 'tracked:russia-ukraine-war', 'russia-ukraine-war',
    '俄烏戰爭持續追蹤',
    '持續追蹤戰場變化、外交談判、制裁與援助、能源糧食影響及戰爭責任；為香港及台灣讀者補充區域安全與經濟脈絡。',
    0, 0, 'VictoriaPark Geopolitics Watch', 'zh-hant', TRUE,
    '## 編輯基線\n\n本專題不依任何一方戰報直接判斷戰果。領土控制、傷亡與襲擊責任均須標明時間、來源及不確定性。維園網以國家主權、平民生命、法治和可執行和平為基準，同時審查所有政府的宣傳與戰爭成本。',
    ARRAY['前線與領土控制變化','停火談判與安全保證','制裁與軍援執行情況','能源糧食與難民影響','戰爭罪調查'],
    ARRAY['俄羅斯','烏克蘭','俄烏','普京','澤連斯基'], ARRAY['戰爭','停火','和談','制裁','軍援','前線'],
    ARRAY['聯合國烏克蘭專題','烏克蘭總統府','俄羅斯總統府'],
    ARRAY['https://news.un.org/en/focus/ukraine','https://www.president.gov.ua/en/news/all','http://en.kremlin.ru/events/president/news'], now()
),
(
    gen_random_uuid(), 'tracked:us-iran-hormuz', 'us-iran-hormuz',
    '美伊與霍爾木茲海峽危機',
    '持續追蹤美伊軍事與外交行動、霍爾木茲海峽航運安全、能源價格、區域盟友及衝突升級訊號。',
    0, 0, 'VictoriaPark Geopolitics Watch', 'zh-hant', TRUE,
    '## 編輯基線\n\n本專題區分官方威懾聲明、實際部署、已發生事件與分析推斷，並從航行自由、國家主權、比例原則、平民保護與受監督的戰爭權力出發評估政策。',
    ARRAY['海峽通航與扣船事件','美伊兵力部署','核談判與制裁','區域盟友動向','油價與航運成本'],
    ARRAY['伊朗','美國','霍爾木茲','波斯灣','Iran','Hormuz'], ARRAY['海峽','封鎖','航運','油輪','制裁','核談判'],
    ARRAY['美國中央司令部','美國國務院伊朗專題','國際海事組織'],
    ARRAY['https://www.centcom.mil/MEDIA/PRESS-RELEASES/','https://www.state.gov/countries-areas/iran/','https://www.imo.org/en/MediaCentre/Pages/Default.aspx'], now()
)
ON CONFLICT (topic, editorial_language) DO UPDATE SET
    slug = EXCLUDED.slug, title = EXCLUDED.title, standfirst = EXCLUDED.standfirst,
    pinned = TRUE, analysis_md = EXCLUDED.analysis_md, watchpoints = EXCLUDED.watchpoints,
    anchor_terms = EXCLUDED.anchor_terms, keywords = EXCLUDED.keywords,
    primary_source_names = EXCLUDED.primary_source_names,
    primary_source_urls = EXCLUDED.primary_source_urls, last_hot_at = now();
