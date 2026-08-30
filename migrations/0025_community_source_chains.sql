-- Community discovery pages are useful signals, but readers must be able to
-- see whether the crawler found the publisher behind a repost. Source text
-- remains private; this table stores URLs and labels only.
CREATE TABLE IF NOT EXISTS community_source_chains (
    raw_item_id UUID PRIMARY KEY REFERENCES raw_items(id) ON DELETE CASCADE,
    community_name TEXT NOT NULL,
    community_url TEXT NOT NULL,
    origin_name TEXT,
    origin_url TEXT,
    image_url TEXT,
    discovered_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CHECK (origin_url IS NULL OR origin_url ~ '^https?://')
);

CREATE INDEX IF NOT EXISTS community_source_chains_origin_idx
    ON community_source_chains(origin_url) WHERE origin_url IS NOT NULL;

-- Put the existing archive into the zone on first deploy. New items gain the
-- deeper origin link during extraction; historical items retain an honest
-- one-hop chain until revisited rather than pretending the origin was found.
INSERT INTO community_source_chains
  (raw_item_id, community_name, community_url, image_url)
SELECT r.id,
       CASE WHEN r.canonical_url ILIKE '%creaders.net%' THEN '万维读者网' ELSE '文学城' END,
       r.canonical_url,
       r.image_url
  FROM raw_items r
 WHERE r.canonical_url ILIKE '%creaders.net%'
    OR r.canonical_url ILIKE '%wenxuecity.com%'
ON CONFLICT (raw_item_id) DO NOTHING;
