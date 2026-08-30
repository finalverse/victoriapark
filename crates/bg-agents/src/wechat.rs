//! Post-publication WeChat packages for the Chinese edition.

use crate::{stage, Ctx, Result, StageOutput};
use bg_core::domain::{AgentRole, ModelTier};
use bg_core::ids::StoryId;
use bg_llm::{schema as sch, Request};
use serde::Deserialize;

pub const SYSTEM: &str = include_str!("../../../prompts/wechat.md");

#[derive(Debug, Deserialize)]
struct WechatCopy {
    title: String,
    summary: String,
    key_facts: Vec<String>,
    unknowns: Vec<String>,
    victoriapark_view: String,
    source_note: String,
}

fn schema() -> serde_json::Value {
    sch::object(
        vec![
            (
                "title",
                sch::string_hinted("12-30 Chinese characters", "title"),
            ),
            (
                "summary",
                sch::string_hinted("600-1200 Chinese characters in Markdown", "summary"),
            ),
            (
                "key_facts",
                sch::array(sch::string("one verified fact"), "3-6 key facts"),
            ),
            (
                "unknowns",
                sch::array(sch::string("one disputed or unknown point"), "0-6 unknowns"),
            ),
            (
                "victoriapark_view",
                sch::string_hinted("clearly labelled editorial view", "viewpoint"),
            ),
            (
                "source_note",
                sch::string_hinted("one sentence", "source note"),
            ),
        ],
        &[
            "title",
            "summary",
            "key_facts",
            "unknowns",
            "victoriapark_view",
            "source_note",
        ],
    )
}

pub async fn run_pending(ctx: &Ctx, limit: i64) -> Result<usize> {
    let ids = bg_db::distribution::needing_wechat(&ctx.db, limit).await?;
    let mut made = 0;
    for id in ids {
        if run_one(ctx, id).await.is_ok() {
            made += 1;
        }
    }
    Ok(made)
}

async fn run_one(ctx: &Ctx, story: StoryId) -> Result<()> {
    let s = bg_db::stories::by_id(&ctx.db, story).await?;
    let article = bg_db::articles::latest_for_story(&ctx.db, story).await?;
    let claims = bg_db::claims::with_sources(&ctx.db, story).await?;
    let sources = bg_db::stories::source_refs(&ctx.db, story).await?;
    let fallback_image = format!(
        "{}/og/{}.png?sq=1",
        std::env::var("BG_PUBLIC_BASE_URL")
            .unwrap_or_else(|_| "https://victoriapark.io".into())
            .trim_end_matches('/'),
        s.slug
    );
    // Publisher images are mirrored by the public image route before they are
    // exposed to readers. Use that stable, credited copy for WeChat rather
    // than hotlinking a CDN URL that may expire or block Chinese clients.
    let mirrored_source_image = format!(
        "{}/img/{}",
        std::env::var("BG_PUBLIC_BASE_URL")
            .unwrap_or_else(|_| "https://victoriapark.io".into())
            .trim_end_matches('/'),
        s.slug
    );
    let system = format!("{}\n\n---\n\n{}", crate::HOUSE_STYLE, SYSTEM);
    stage(ctx, AgentRole::Herald, Some(story), "wechat", |_run| async move {
        let mut prompt = format!(
            "OUTPUT_LANGUAGE=zh\n\nHeadline: {}\nSummary: {}\n\nPublished article:\n{}\n\nVerified claims:\n",
            s.title, s.summary.as_deref().unwrap_or(""),
            article.as_ref().map(|a| a.body_md.as_str()).unwrap_or("")
        );
        for c in &claims {
            prompt.push_str(&format!("- [{} · {:.0}%] {}\n", c.claim.verification.label(), c.claim.confidence * 100.0, c.claim.text));
        }
        prompt.push_str("\nOriginal source links:\n");
        for source in &sources { prompt.push_str(&format!("- {}: {}\n", source.name, source.url)); }
        let req = Request::new("herald.wechat", ModelTier::Mid, system, prompt)
            .with_schema(schema()).with_max_tokens(4_500);
        let (copy, completion) = ctx.llm.complete_json::<WechatCopy>(&req).await?;
        let (image_url, image_origin) = s.image_url.as_deref()
            .filter(|url| !url.is_empty())
            .map(|_| (mirrored_source_image.as_str(), "source-mirrored"))
            .unwrap_or((fallback_image.as_str(), "victoriapark-generated"));
        bg_db::distribution::upsert_wechat(&ctx.db, story, &bg_db::distribution::NewWechatPackage {
            title: copy.title.trim(), summary_md: copy.summary.trim(),
            key_facts: &copy.key_facts, unknowns: &copy.unknowns,
            viewpoint: copy.victoriapark_view.trim(), source_note: copy.source_note.trim(),
            image_url: Some(image_url), image_origin,
        }).await?;
        Ok(StageOutput::with((), completion, "WeChat draft generated"))
    }).await
}
