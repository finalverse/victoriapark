-- Re-file items published before the five-edition split.
--
-- New intake is labelled at the source boundary. This migration handles the
-- historical corpus, where RTHK was previously treated as generic Chinese and
-- would otherwise remain visible on the Simplified Chinese front page.

UPDATE raw_items r
   SET lang = CASE
       WHEN s.slug LIKE 'rthk-%' OR s.slug LIKE 'cna-%' THEN 'zh-hant'
       WHEN s.slug LIKE 'nhk-%' OR s.slug LIKE 'nippon-%' THEN 'ja'
       WHEN s.slug LIKE 'yna-%' OR s.slug LIKE 'kbs-ko-%' THEN 'ko'
       ELSE r.lang
   END
  FROM sources s
 WHERE r.source_id = s.id
   AND (
       s.slug LIKE 'rthk-%' OR s.slug LIKE 'cna-%'
       OR s.slug LIKE 'nhk-%' OR s.slug LIKE 'nippon-%'
       OR s.slug LIKE 'yna-%' OR s.slug LIKE 'kbs-ko-%'
   );

-- Move a story only when every attached item belongs to the target edition.
-- A genuinely cross-language cluster stays in its original edition rather
-- than being assigned according to whichever source happens to be first.
UPDATE stories s
   SET editorial_language = 'zh-hant'
 WHERE EXISTS (
       SELECT 1 FROM story_items si JOIN raw_items r ON r.id = si.raw_item_id
        WHERE si.story_id = s.id AND r.lang = 'zh-hant'
   )
   AND NOT EXISTS (
       SELECT 1 FROM story_items si JOIN raw_items r ON r.id = si.raw_item_id
        WHERE si.story_id = s.id AND r.lang <> 'zh-hant'
   );

UPDATE stories s
   SET editorial_language = 'ja'
 WHERE EXISTS (
       SELECT 1 FROM story_items si JOIN raw_items r ON r.id = si.raw_item_id
        WHERE si.story_id = s.id AND r.lang = 'ja'
   )
   AND NOT EXISTS (
       SELECT 1 FROM story_items si JOIN raw_items r ON r.id = si.raw_item_id
        WHERE si.story_id = s.id AND r.lang <> 'ja'
   );

UPDATE stories s
   SET editorial_language = 'ko'
 WHERE EXISTS (
       SELECT 1 FROM story_items si JOIN raw_items r ON r.id = si.raw_item_id
        WHERE si.story_id = s.id AND r.lang = 'ko'
   )
   AND NOT EXISTS (
       SELECT 1 FROM story_items si JOIN raw_items r ON r.id = si.raw_item_id
        WHERE si.story_id = s.id AND r.lang <> 'ko'
   );

-- Topic membership is edition-specific. The worker will repopulate matching
-- members on its next fast pass.
DELETE FROM gaggle_stories gs
 USING gaggles g, stories s
 WHERE gs.gaggle_id = g.id
   AND gs.story_id = s.id
   AND g.editorial_language <> s.editorial_language;

UPDATE gaggles g
   SET story_count = (
       SELECT count(*)::integer FROM gaggle_stories gs WHERE gs.gaggle_id = g.id
   );
