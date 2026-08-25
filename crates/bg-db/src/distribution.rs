//! Channel-specific editorial packages generated after publication.

use crate::{Db, Result};
use bg_core::ids::StoryId;
use sqlx::Row;
use uuid::Uuid;

pub async fn needing_wechat(db: &Db, limit: i64) -> Result<Vec<StoryId>> {
    let rows = sqlx::query(
        "SELECT s.id FROM stories s
         LEFT JOIN wechat_packages w ON w.story_id = s.id
         WHERE s.status = 'published' AND s.editorial_language = 'zh'
           AND w.story_id IS NULL
         ORDER BY s.newsworthiness DESC, s.published_at DESC LIMIT $1",
    )
    .bind(limit)
    .fetch_all(&db.pool)
    .await?;
    Ok(rows
        .iter()
        .map(|r| StoryId::from_uuid(r.get::<Uuid, _>("id")))
        .collect())
}

pub struct NewWechatPackage<'a> {
    pub title: &'a str,
    pub summary_md: &'a str,
    pub key_facts: &'a [String],
    pub unknowns: &'a [String],
    pub viewpoint: &'a str,
    pub source_note: &'a str,
    pub image_url: Option<&'a str>,
    pub image_origin: &'a str,
}

pub async fn upsert_wechat(db: &Db, story: StoryId, p: &NewWechatPackage<'_>) -> Result<()> {
    sqlx::query(
        "INSERT INTO wechat_packages
         (story_id,title,summary_md,key_facts,unknowns,viewpoint,source_note,image_url,image_origin)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
         ON CONFLICT (story_id) DO UPDATE SET
           title=EXCLUDED.title, summary_md=EXCLUDED.summary_md,
           key_facts=EXCLUDED.key_facts, unknowns=EXCLUDED.unknowns,
           viewpoint=EXCLUDED.viewpoint, source_note=EXCLUDED.source_note,
           image_url=EXCLUDED.image_url, image_origin=EXCLUDED.image_origin, updated_at=now()",
    )
    .bind(story.into_uuid())
    .bind(p.title)
    .bind(p.summary_md)
    .bind(serde_json::to_value(p.key_facts).unwrap_or_default())
    .bind(serde_json::to_value(p.unknowns).unwrap_or_default())
    .bind(p.viewpoint)
    .bind(p.source_note)
    .bind(p.image_url)
    .bind(p.image_origin)
    .execute(&db.pool)
    .await?;
    Ok(())
}
