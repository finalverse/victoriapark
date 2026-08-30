-- Lake Ontario / “Lake America” is part of the same U.S.–Canada policy and
-- identity conflict, not a second topic. Add multilingual signals to the
-- permanent dossier so the crawler keeps new reporting in one event hub.
UPDATE gaggles SET
  anchor_terms = ARRAY(SELECT DISTINCT x FROM unnest(anchor_terms || ARRAY[
    'Lake Ontario','Lake America','American Lake','Ontario','安大略湖','美国湖','美利坚湖'
  ]) x),
  keywords = ARRAY(SELECT DISTINCT x FROM unnest(keywords || ARRAY[
    'rename','renaming','更名','改名','tariff','关税','關稅','trade war','贸易战','貿易戰'
  ]) x)
WHERE slug='us-canada-trade-war';

-- Existing transient pages about the two permanent conflict dossiers should
-- be reconsidered immediately by the runtime overlap merger after deploy.
UPDATE gaggles SET last_hot_at=now()
WHERE NOT pinned AND editorial_language IN ('zh','zh-hant','en')
  AND (concat_ws(' ', topic,title,standfirst) ILIKE ANY(ARRAY[
    '%Hormuz%','%霍尔木兹%','%霍爾木茲%','%U.S.%Iran%','%美伊%'
  ]));
