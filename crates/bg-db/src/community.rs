//! Provenance for community-discovered Chinese reporting.

use crate::{Db, Result};
use bg_core::ids::RawItemId;
use sqlx::Row;

#[derive(Debug, Clone)]
pub struct CommunityStats {
    pub stories: i64,
    pub origins: i64,
}

pub async fn record(
    db: &Db,
    item: RawItemId,
    community_name: &str,
    community_url: &str,
    origin_name: Option<&str>,
    origin_url: Option<&str>,
    image_url: Option<&str>,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO community_source_chains
           (raw_item_id, community_name, community_url, origin_name, origin_url, image_url)
         VALUES ($1,$2,$3,$4,$5,$6)
         ON CONFLICT (raw_item_id) DO UPDATE SET
           community_name=EXCLUDED.community_name,
           community_url=EXCLUDED.community_url,
           origin_name=COALESCE(EXCLUDED.origin_name, community_source_chains.origin_name),
           origin_url=COALESCE(EXCLUDED.origin_url, community_source_chains.origin_url),
           image_url=COALESCE(EXCLUDED.image_url, community_source_chains.image_url),
           discovered_at=now()",
    )
    .bind(item.into_uuid())
    .bind(community_name)
    .bind(community_url)
    .bind(origin_name)
    .bind(origin_url)
    .bind(image_url)
    .execute(&db.pool)
    .await?;
    Ok(())
}

pub async fn stats(db: &Db, language: &str) -> Result<CommunityStats> {
    let row = sqlx::query(
        "SELECT count(DISTINCT r.story_id) FILTER (WHERE r.story_id IS NOT NULL) AS stories,
                count(*) FILTER (WHERE c.origin_url IS NOT NULL) AS origins
           FROM community_source_chains c
           JOIN raw_items r ON r.id=c.raw_item_id
           JOIN stories st ON st.id=r.story_id
          WHERE st.status='published' AND st.editorial_language=$1",
    )
    .bind(language)
    .fetch_one(&db.pool)
    .await?;
    Ok(CommunityStats {
        stories: row.try_get("stories")?,
        origins: row.try_get("origins")?,
    })
}
