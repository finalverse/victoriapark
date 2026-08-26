# Multi-lingual VictoriaPark

Status: **five independent editions are live in the architecture.** Simplified
Chinese is primary (`/`); Traditional Chinese (`/zh-hant`), English (`/en`),
Japanese (`/ja`) and Korean (`/ko`) have separate source pools, stories, trend
baselines and special topics. They are editions, not a translation layer.

## Where we started

The corpus is 100% English: 1,396 items, spelled `en-us`, `en` and `en-US` —
three tags for one language, which is why `text::normalize_lang` now exists. A
per-language surface built on the raw value would have missed most of its own
rows.

`lang` was stored and read by nothing. That is the first thing to change, and
it is cheap.

## The decision that shapes everything else

There are two products here and they are not the same:

**(a) Translate VictoriaPark into other languages.** One newsroom, many renderings.
The Chinese reader gets our English-sourced analysis in Chinese.

**(b) Report in other languages from sources in those languages.** A Chinese
desk reading Chinese sources, whose stories may never exist in English.

(a) is a translation feature. (b) is a second newsroom.

**Recommendation: (b), starting narrow.** (a) is cheaper and worse — it makes
VictoriaPark a machine-translated mirror of an anglophone view, competing with
Google Translate on someone else's reporting. (b) is the only version that
produces something a reader cannot get elsewhere: Chinese and Japanese crypto
and AI coverage that English-language outlets do not carry, run through the same
claim graph and the same corroboration standard.

The Skein's whole argument is that a claim shows its sources. That argument
travels; a translation layer does not.

## Why this is now feasible

Until v0.12.0 a source had to publish RSS. Most non-English outlets that matter
here do not, or publish a truncated one. `bg-ingest::crawl` reads index pages
directly, so the source roster is no longer limited to publishers who chose to
emit XML.

## Implementation record

**1. Make `lang` real — complete**
- Tags are normalised at ingest, including `zh-Hant`, `zh-TW` and `zh-HK`.
- Stories, category trends, entity trends, special topics and public routes are
  filtered by edition.
- The masthead preserves the current section while switching among five editions.
  rather than N duplicate pages

**2. Language-aware relevance — complete for pinned sources**
`bg-ingest::relevance` routes by English keywords. A Chinese headline about
比特币 matches nothing and is dropped before it reaches triage. The term tables
need a per-language set; this is a data problem, not a model one.

**3. Trend detection per language — complete**
`bg-core::trends` retains English named-entity extraction and adds script-aware
CJK/Hangul phrase extraction. Independent-source convergence and the two-week
baseline remain the gate, so one platform cannot manufacture a special topic.

**4. Sources — active roster**
- Simplified Chinese: Google News Chinese sections, VOA Chinese and official
  Weibo/Baidu/NetEase heat signals.
- Traditional Chinese: RTHK and CNA edition feeds.
- Japanese: NHK NEWS WEB and nippon.com.
- Korean: Yonhap News.
Every HTML source is checked against robots.txt. Heat lists are agenda signals,
not factual authorities, and require corroboration before publication.

**5. The Flock, per language — complete**
Gosling, Herald and the Skein all write. Their prompts are English and their
output must match the source language, not the prompt language.
- The house style prompt needs a language directive per run
- The grounding gate and verbatim quote check are language-neutral already —
  `contains_verbatim` normalises typography, not script, so it works on CJK
  unchanged
- **Cost**: this multiplies token spend by the number of active languages. On
  200,000 tokens/day it is not affordable for even one more language without a
  paid tier. This step is gated on that, and should be stated plainly rather
  than discovered halfway.

**6. Cross-language corroboration** (the actually interesting part)
Two outlets in two languages reporting the same event is *stronger* evidence
than two in one — they share no editor, no wire, no press release cycle. The
Curator clusters on lexical similarity, which cannot see across scripts.
Doing this properly needs embeddings (`BG_EMBED_PROVIDER` exists and is off).
This is the feature that would make VictoriaPark genuinely different, and it is
last because it depends on all of the above.

## What not to do

- **Do not machine-translate our own analysis and present it as native.** A
  translated Skein forecast reads as authored in the target language and is not.
  If a translation is shown, it says so.
- **Do not mix languages on one surface.** A reader wants a newsroom, not a
  multilingual pile.
- **Do not let a language desk lower the corroboration bar** because it has
  fewer sources. Three Chinese outlets is three outlets; two is single-sourced
  in any language.

## Remaining work

- Add more primary and regional sources after robots and licensing review.
- Add `hreflang` only where genuinely corresponding pages exist; independent
  editions must not falsely claim that unrelated stories are translations.
- Calibrate CJK/Hangul stop phrases against live traffic without lowering the
  independent-source threshold.
