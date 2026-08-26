//! Public hot-list adapters.
//!
//! These feeds are agenda signals, not factual authorities. We persist only a
//! title, rank/heat note, timestamp and source link. Downstream agents must
//! corroborate any factual claim before publishing it as news.

use crate::{IngestError, Result};
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use reqwest::header::{COOKIE, REFERER, SET_COOKIE};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct HotItem {
    pub title: String,
    pub url: String,
    pub dek: String,
    pub published_at: DateTime<Utc>,
    pub image_url: Option<String>,
    pub authors: Vec<String>,
}

pub fn is_hotlist(slug: &str) -> bool {
    matches!(slug, "hot-weibo" | "hot-baidu" | "hot-netease")
}

pub async fn fetch(client: &reqwest::Client, slug: &str, url: &str) -> Result<Vec<HotItem>> {
    match slug {
        "hot-weibo" => weibo(client, url).await,
        "hot-baidu" => baidu(client, url).await,
        "hot-netease" => netease(client, url).await,
        _ => Ok(Vec::new()),
    }
}

async fn weibo(client: &reqwest::Client, url: &str) -> Result<Vec<HotItem>> {
    // The official endpoint requires the anonymous session issued by its home
    // page. No login, account or bypass: one bootstrap request and the cookie
    // the site itself returns to every browser.
    let home = client.get("https://weibo.com/").send().await?;
    let cookie = home
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .filter_map(|v| v.split(';').next())
        .collect::<Vec<_>>()
        .join("; ");
    let response = client
        .get(url)
        .header(REFERER, "https://weibo.com/")
        .header(COOKIE, cookie)
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(IngestError::Http {
            status: response.status().as_u16(),
            url: url.to_string(),
        });
    }
    let root: Value = response.json().await?;
    let rows = root
        .pointer("/data/band_list")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    Ok(rows
        .into_iter()
        .take(50)
        .filter_map(|row| {
            let title = row
                .get("note")
                .or_else(|| row.get("word"))?
                .as_str()?
                .trim_matches('#')
                .trim()
                .to_string();
            if title.is_empty() {
                return None;
            }
            let rank = row
                .get("realpos")
                .or_else(|| row.get("rank"))
                .and_then(Value::as_i64)
                .unwrap_or(0);
            let heat = row
                .get("raw_hot")
                .and_then(|v| {
                    v.as_i64()
                        .map(|n| n.to_string())
                        .or_else(|| v.as_str().map(str::to_string))
                })
                .unwrap_or_default();
            let query = url::form_urlencoded::Serializer::new(String::new())
                .append_pair("q", &title)
                .append_pair("Refer", "top")
                .finish();
            Some(HotItem {
                title,
                url: format!("https://s.weibo.com/weibo?{query}"),
                dek: format!(
                    "微博实时热搜第 {rank} 位{}。热度仅代表讨论规模，不代表事实已获证实。",
                    if heat.is_empty() {
                        String::new()
                    } else {
                        format!(" · 热度 {heat}")
                    }
                ),
                published_at: Utc::now(),
                image_url: None,
                authors: Vec::new(),
            })
        })
        .collect())
}

async fn baidu(client: &reqwest::Client, url: &str) -> Result<Vec<HotItem>> {
    let html = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    let start = html
        .find("<!--s-data:")
        .map(|i| i + "<!--s-data:".len())
        .ok_or_else(|| IngestError::Decode("Baidu hot-list payload missing".into()))?;
    let end = html[start..]
        .find("-->")
        .map(|i| start + i)
        .ok_or_else(|| IngestError::Decode("Baidu hot-list payload unterminated".into()))?;
    let root: Value = serde_json::from_str(&html[start..end])
        .map_err(|e| IngestError::Decode(format!("Baidu hot-list JSON: {e}")))?;
    let rows = root
        .pointer("/data/cards/0/content")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    Ok(rows
        .into_iter()
        .take(50)
        .filter_map(|row| {
            let title = row
                .get("word")
                .or_else(|| row.get("query"))?
                .as_str()?
                .trim()
                .to_string();
            let target = row
                .get("url")
                .or_else(|| row.get("rawUrl"))?
                .as_str()?
                .to_string();
            let rank = row.get("index").and_then(Value::as_i64).unwrap_or(0) + 1;
            let heat = row.get("hotScore").and_then(Value::as_str).unwrap_or("");
            Some(HotItem {
                title,
                url: target,
                dek: format!(
                    "百度实时热榜第 {rank} 位 · 热度 {heat}。榜单用于发现选题，事实需另行核验。"
                ),
                published_at: Utc::now(),
                image_url: row.get("img").and_then(Value::as_str).map(str::to_string),
                authors: Vec::new(),
            })
        })
        .collect())
}

async fn netease(client: &reqwest::Client, url: &str) -> Result<Vec<HotItem>> {
    let root: Value = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let rows = root
        .pointer("/data/list")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let china = FixedOffset::east_opt(8 * 3600).expect("valid UTC+8 offset");
    Ok(rows
        .into_iter()
        .take(50)
        .filter_map(|row| {
            let title = row.get("title")?.as_str()?.trim().to_string();
            let target = row.get("url")?.as_str()?.to_string();
            let published_at = row
                .get("publishTime")
                .and_then(Value::as_str)
                .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok())
                .and_then(|n| n.and_local_timezone(china).single())
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);
            Some(HotItem {
                title,
                url: target,
                dek: "网易新闻热点流。该排名是选题信号，报道中的事实由维园网另行交叉核验。".into(),
                published_at,
                image_url: row
                    .get("imgsrc")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                authors: row
                    .get("source")
                    .and_then(Value::as_str)
                    .map(|s| vec![s.to_string()])
                    .unwrap_or_default(),
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    #[test]
    fn identifies_only_explicit_hotlist_sources() {
        assert!(super::is_hotlist("hot-weibo"));
        assert!(!super::is_hotlist("weibo-commentary"));
    }
}
