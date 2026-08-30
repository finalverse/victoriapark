-- Legacy Chinese community pages frequently declare GBK/GB18030 only in an
-- HTML meta tag. Before the byte-aware decoder was introduced, reqwest treated
-- those pages as UTF-8 and persisted U+FFFD replacement characters. Withdraw
-- the damaged public cards and remove their raw inputs so the corrected crawler
-- can discover and process the canonical URLs again.
WITH damaged_stories AS (
    SELECT DISTINCT s.id
      FROM stories s
      JOIN story_items si ON si.story_id = s.id
      JOIN raw_items r ON r.id = si.raw_item_id
      JOIN community_source_chains c ON c.raw_item_id = r.id
     WHERE position(chr(65533) in s.title) > 0
        OR position(chr(65533) in coalesce(s.summary, '')) > 0
        OR position(chr(65533) in r.title) > 0
        OR position(chr(65533) in coalesce(r.summary_raw, '')) > 0
        OR position(chr(65533) in coalesce(r.body_raw, '')) > 0
)
UPDATE stories
   SET status = 'killed',
       published_at = NULL,
       editor_note = 'withdrawn: source HTML was decoded with the wrong legacy Chinese charset',
       updated_at = now()
 WHERE id IN (SELECT id FROM damaged_stories);

DELETE FROM raw_items r
 USING community_source_chains c
 WHERE c.raw_item_id = r.id
   AND (
       position(chr(65533) in r.title) > 0
       OR position(chr(65533) in coalesce(r.summary_raw, '')) > 0
       OR position(chr(65533) in coalesce(r.body_raw, '')) > 0
   );
