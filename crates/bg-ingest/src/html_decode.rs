//! Decode publisher HTML before parsing it.
//!
//! `reqwest::Response::text` trusts the HTTP charset and otherwise defaults to
//! UTF-8. A large part of the overseas Chinese web still serves GBK/GB18030,
//! often declaring it only in a `<meta>` tag. Decoding those bytes as UTF-8
//! permanently writes replacement characters into the newsroom database.

use encoding_rs::{Encoding, GBK, UTF_8};

/// Decode an HTML response using BOM, HTTP and in-document charset signals.
///
/// The URL is used only for a conservative GBK fallback on Chinese community
/// hosts whose older pages omit every charset declaration.
pub fn decode(bytes: &[u8], content_type: Option<&str>, url: &str) -> String {
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        return UTF_8.decode(&bytes[3..]).0.into_owned();
    }

    let header = content_type.and_then(charset_from_header);
    let meta = charset_from_meta(bytes);
    let declared = meta.or(header);

    if let Some(enc) = declared.and_then(|label| Encoding::for_label(label.as_bytes())) {
        let (decoded, _, had_errors) = enc.decode(bytes);
        if !had_errors || enc != UTF_8 || !is_legacy_chinese_host(url) {
            return decoded.into_owned();
        }
        // Several legacy sites advertise UTF-8 on an edge response while the
        // origin still emits GBK. Prefer the interpretation that does not turn
        // a headline into rows of U+FFFD replacement glyphs.
        let (gbk, _, gbk_errors) = GBK.decode(bytes);
        if !gbk_errors || replacement_count(&gbk) < replacement_count(&decoded) {
            return gbk.into_owned();
        }
        return decoded.into_owned();
    }

    if let Ok(text) = std::str::from_utf8(bytes) {
        return text.to_owned();
    }

    if is_legacy_chinese_host(url) {
        return GBK.decode(bytes).0.into_owned();
    }

    UTF_8.decode(bytes).0.into_owned()
}

fn charset_from_header(content_type: &str) -> Option<String> {
    content_type.split(';').skip(1).find_map(|part| {
        let (name, value) = part.split_once('=')?;
        name.trim()
            .eq_ignore_ascii_case("charset")
            .then(|| value.trim().trim_matches(['\'', '"']).to_ascii_lowercase())
    })
}

fn charset_from_meta(bytes: &[u8]) -> Option<String> {
    // Charset declarations are ASCII even when the document body is not.
    let prefix = &bytes[..bytes.len().min(16 * 1024)];
    let ascii = String::from_utf8_lossy(prefix).to_ascii_lowercase();
    for marker in ["charset=", "charset ="] {
        if let Some(start) = ascii.find(marker) {
            let tail = ascii[start + marker.len()..].trim_start();
            let tail = tail.trim_start_matches(['\'', '"']);
            let label: String = tail
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
                .collect();
            if !label.is_empty() {
                return Some(label);
            }
        }
    }
    None
}

fn is_legacy_chinese_host(url: &str) -> bool {
    ["creaders.net", "wenxuecity.com"]
        .iter()
        .any(|host| url.to_ascii_lowercase().contains(host))
}

fn replacement_count(text: &str) -> usize {
    text.chars().filter(|&c| c == '\u{fffd}').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_gbk_from_meta_charset() {
        let html = "<meta charset=gb2312><h2>海外华文社区热点追踪</h2>";
        let (bytes, _, _) = GBK.encode(html);
        assert_eq!(
            decode(&bytes, Some("text/html"), "https://www.creaders.net/"),
            html
        );
    }

    #[test]
    fn falls_back_to_gbk_for_legacy_chinese_hosts() {
        let html = "<h2>财经与科技新闻</h2>";
        let (bytes, _, _) = GBK.encode(html);
        assert_eq!(
            decode(
                &bytes,
                Some("text/html"),
                "https://www.wenxuecity.com/news/"
            ),
            html
        );
    }

    #[test]
    fn keeps_utf8_pages_unchanged() {
        let html = "<meta charset=\"utf-8\"><h2>维园网 AI 编辑部</h2>";
        assert_eq!(decode(html.as_bytes(), None, "https://example.com"), html);
    }
}
