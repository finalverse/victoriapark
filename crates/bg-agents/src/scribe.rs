//! **Scribe** — extracts the claims and writes the draft.
//!
//! Where the inversion happens. Scribe does not write prose and then hunt for
//! citations; it decomposes the source material into discrete claims, each tied
//! to the specific items that support it, and only then writes a body that
//! cites those claims. Everything downstream — verification, the policy engine,
//! the ledger sidebar — reads the claim set, not the prose.

use crate::{stage, Ctx, FlockError, Result, StageOutput};
use bg_core::domain::{AgentRole, ClaimKind, ModelTier, Stance};
use bg_core::ids::{ClaimId, StoryId};
use bg_llm::{schema as sch, Request};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::str::FromStr;
use tracing::info;

pub const SYSTEM: &str = include_str!("../../../prompts/scribe.md");

#[derive(Debug, Deserialize)]
pub struct Draft {
    pub claims: Vec<DraftClaim>,
    pub body_md: String,
    pub working_title: String,
}

#[derive(Debug, Deserialize)]
pub struct DraftClaim {
    pub text: String,
    pub kind: String,
    pub source_indices: Vec<i64>,
    pub excerpt: String,
    pub numeric_value: String,
    pub unit: String,
}

fn schema(n_sources: usize) -> serde_json::Value {
    sch::object(
        vec![
            (
                "working_title",
                sch::string_hinted("plain description of the event", "headline"),
            ),
            (
                "claims",
                sch::array(
                    sch::object(
                        vec![
                            (
                                "text",
                                sch::string_hinted("one self-contained sentence", "claim"),
                            ),
                            (
                                "kind",
                                sch::enumeration(&["fact", "figure", "quote", "forecast"], "kind"),
                            ),
                            (
                                "source_indices",
                                sch::array_n(
                                    sch::integer_index("source index"),
                                    "supporting sources",
                                    n_sources.min(3),
                                ),
                            ),
                            (
                                "excerpt",
                                sch::string_hinted("<=20 words from a source, or empty", "excerpt"),
                            ),
                            (
                                "numeric_value",
                                sch::string_hinted("figures only, else empty", ""),
                            ),
                            ("unit", sch::string_hinted("USD, BTC, %, else empty", "")),
                        ],
                        &[
                            "text",
                            "kind",
                            "source_indices",
                            "excerpt",
                            "numeric_value",
                            "unit",
                        ],
                    ),
                    "3-8 claims",
                ),
            ),
            (
                "body_md",
                sch::string_hinted("markdown body with [^cN] citations", "body_md"),
            ),
        ],
        &["working_title", "claims", "body_md"],
    )
}

/// Draft a Desk story: extract claims, persist them, write the body.
///
/// Returns the claim IDs in citation order, so `[^c1]` resolves to index 0.
pub async fn run(ctx: &Ctx, story: StoryId) -> Result<(Vec<ClaimId>, String)> {
    let items = bg_db::items::by_story(&ctx.db, story).await?;
    if items.is_empty() {
        return Err(FlockError::Other("story has no source items".into()));
    }
    let output_language = crate::output_language(
        bg_core::domain::EditorialLanguage::from_source_lang(&items[0].lang),
    );
    let system = crate::system_prompt(ctx, AgentRole::Scribe).await;

    stage(
        ctx,
        AgentRole::Scribe,
        Some(story),
        "draft",
        |run| async move {
            // Source bodies are the private working copy. They go into the prompt
            // and never anywhere else — see bg-db::items for the accessor boundary.
            let mut prompt = format!("OUTPUT_LANGUAGE={output_language}\n\nSource material:\n\n");
            for (i, it) in items.iter().enumerate() {
                prompt.push_str(&format!(
                    "=== SOURCE [{i}] — {} ===\n",
                    it.published_at.to_rfc3339()
                ));
                prompt.push_str(&format!("Headline: {}\n", it.title));
                if let Some(body) = it.body_raw.as_deref().or(it.summary_raw.as_deref()) {
                    prompt.push_str(&format!(
                        "Text: {}\n",
                        bg_core::text::truncate_words(body, 500)
                    ));
                }
                prompt.push('\n');
            }
            prompt.push_str("\nExtract the claims, then write the story.");

            let req = Request::new("scribe.draft", ModelTier::Mid, system, prompt)
                .with_schema(schema(items.len()))
                .with_max_tokens(8_000);
            let (draft, completion) = ctx.llm.complete_json::<Draft>(&req).await?;

            let mut ids = Vec::with_capacity(draft.claims.len());
            for dc in &draft.claims {
                if dc.text.trim().is_empty() {
                    continue;
                }
                let kind = ClaimKind::from_str(&dc.kind).unwrap_or(ClaimKind::Fact);
                let numeric = Decimal::from_str(dc.numeric_value.trim()).ok();
                let claim_id = bg_db::claims::insert(
                    &ctx.db,
                    story,
                    &bg_db::claims::NewClaim {
                        text: dc.text.trim().to_string(),
                        kind,
                        numeric_value: numeric,
                        unit: Some(dc.unit.trim().to_string()).filter(|u| !u.is_empty()),
                        as_of: Some(chrono::Utc::now()),
                    },
                    Some(run),
                )
                .await?;

                let excerpt = dc.excerpt.trim();
                for idx in &dc.source_indices {
                    let Some(item) = items.get(*idx as usize) else {
                        continue;
                    };
                    bg_db::claims::add_source(
                        &ctx.db,
                        claim_id,
                        item.id,
                        Stance::Supports,
                        (!excerpt.is_empty()).then_some(excerpt),
                    )
                    .await?;
                }
                ids.push(claim_id);
            }

            if ids.is_empty() {
                return Err(FlockError::Other("scribe produced no usable claims".into()));
            }

            bg_db::stories::set_meta(
                &ctx.db,
                story,
                Some(draft.working_title.trim()),
                None,
                &[],
                None,
                None,
            )
            .await?;

            let note = format!(
                "{} claims, {} words",
                ids.len(),
                bg_core::text::word_count(&draft.body_md)
            );
            info!(story = %story, claims = ids.len(), "scribe drafted");
            Ok(StageOutput::with((ids, draft.body_md), completion, note))
        },
    )
    .await
}
