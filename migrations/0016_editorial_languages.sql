-- Independent Chinese and English editorial streams.
--
-- Raw items already carry a normalized source language. Stories now freeze the
-- language of their seed item so clustering, drafting, ranking and rendering do
-- not turn the English edition into a translation of the Chinese one (or vice
-- versa).
ALTER TABLE stories
    ADD COLUMN IF NOT EXISTS editorial_language TEXT NOT NULL DEFAULT 'en';

ALTER TABLE stories DROP CONSTRAINT IF EXISTS stories_editorial_language_check;
ALTER TABLE stories ADD CONSTRAINT stories_editorial_language_check
    CHECK (editorial_language IN ('zh','en'));

UPDATE stories s
   SET editorial_language = CASE
       WHEN lower(coalesce((
           SELECT r.lang
             FROM story_items si
             JOIN raw_items r ON r.id = si.raw_item_id
            WHERE si.story_id = s.id
            ORDER BY (si.role = 'seed') DESC, r.published_at ASC
            LIMIT 1
       ), 'en')) LIKE 'zh%' THEN 'zh'
       ELSE 'en'
   END;

CREATE INDEX IF NOT EXISTS stories_language_front_idx
    ON stories (editorial_language, status, published_at DESC);
