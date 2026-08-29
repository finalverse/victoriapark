//! URL slug generation.
//!
//! Slugs are permanent public identifiers — once a story is out, the URL must
//! not drift when a correction rewrites the headline. Generation is therefore
//! deterministic and callers persist the result rather than recomputing it.

/// ASCII-lowercase, hyphen-separated slug, truncated on a word boundary.
pub fn slugify(s: &str) -> String {
    slugify_max(s, 72)
}

pub fn slugify_max(s: &str, max_len: usize) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last_dash = true; // suppresses a leading dash
    let mut has_unrepresented_word = false;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            last_dash = false;
        } else if c == '&' {
            // Spelled out rather than dropped: "Coinbase & Circle" must not
            // collapse to "coinbase-circle", which reads as a single entity.
            if !last_dash {
                out.push('-');
            }
            out.push_str("and-");
            last_dash = true;
        } else if matches!(c, ' ' | '-' | '_' | '/' | '.' | ',' | ':' | ';') || c.is_whitespace() {
            if !last_dash {
                out.push('-');
                last_dash = true;
            }
        } else if !c.is_ascii() {
            // Transliterate the accented Latin we actually see in crypto
            // coverage. CJK, Japanese and Korean words cannot simply be
            // dropped, though: doing so collapsed every Chinese headline to
            // `story`, exhausted the 25 collision suffixes, and stopped the
            // Curator at the first item of every pass. Keep the public path
            // ASCII, but remember that meaningful Unicode was omitted so a
            // stable fingerprint can make the slug unique below.
            if let Some(rep) = transliterate(c) {
                out.push_str(rep);
                last_dash = false;
            } else if c.is_alphanumeric() {
                has_unrepresented_word = true;
            }
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if has_unrepresented_word {
        // FNV-1a is a compact, stable identifier, not a security primitive.
        // DefaultHasher is deliberately avoided because its output is not a
        // permanent-URL contract.
        let fingerprint = format!("{:012x}", stable_hash(s) & 0xffff_ffff_ffff);
        let suffix_len = fingerprint.len() + 1;
        if max_len <= suffix_len {
            return fingerprint[..max_len.min(fingerprint.len())].to_string();
        }
        let prefix_max = max_len - suffix_len;
        truncate_slug(&mut out, prefix_max);
        if out.is_empty() {
            out.push_str("story");
            truncate_slug(&mut out, prefix_max);
        }
        out.push('-');
        out.push_str(&fingerprint);
    } else {
        truncate_slug(&mut out, max_len);
        if out.is_empty() {
            out.push_str("story");
        }
    }
    out
}

fn truncate_slug(out: &mut String, max_len: usize) {
    if out.len() > max_len {
        let cut = out[..max_len].rfind('-').unwrap_or(max_len);
        out.truncate(cut);
    }
    while out.ends_with('-') {
        out.pop();
    }
}

fn stable_hash(s: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in s.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn transliterate(c: char) -> Option<&'static str> {
    Some(match c.to_ascii_lowercase() {
        'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' => "a",
        'é' | 'è' | 'ê' | 'ë' => "e",
        'í' | 'ì' | 'î' | 'ï' => "i",
        'ó' | 'ò' | 'ô' | 'ö' | 'õ' => "o",
        'ú' | 'ù' | 'û' | 'ü' => "u",
        'ñ' => "n",
        'ç' => "c",
        'ß' => "ss",
        'ø' => "o",
        'æ' => "ae",
        _ => return None,
    })
}

/// Appends a short disambiguator when a slug already exists.
pub fn slug_with_suffix(base: &str, n: u32) -> String {
    format!("{base}-{n}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_slugs() {
        assert_eq!(
            slugify("SEC Approves Spot Ether ETFs"),
            "sec-approves-spot-ether-etfs"
        );
        assert_eq!(
            slugify("  Bitcoin's $100,000 Day!  "),
            "bitcoins-100-000-day"
        );
        // Apostrophes close up rather than splitting: "whats", not "what-s".
        assert_eq!(slugify("DeFi / NFT: what's next?"), "defi-nft-whats-next");
    }

    #[test]
    fn never_empty_never_edge_dashes() {
        assert_eq!(slugify("!!!"), "story");
        assert_eq!(slugify(""), "story");
        assert!(!slugify("--hello--").starts_with('-'));
        assert!(!slugify("--hello--").ends_with('-'));
    }

    #[test]
    fn truncates_on_a_word_boundary() {
        let long = "a very long headline that keeps going and going and going past the limit";
        let s = slugify_max(long, 20);
        assert!(s.len() <= 20, "got {} chars: {s}", s.len());
        assert!(!s.ends_with('-'));
    }

    #[test]
    fn transliterates_rather_than_dropping() {
        assert_eq!(slugify("Café & Bär"), "cafe-and-bar");
    }

    #[test]
    fn unicode_headlines_are_stable_and_distinct() {
        let first = slugify("特朗普宣布新的贸易政策");
        let second = slugify("马斯克公布新的太空计划");
        assert!(first.starts_with("story-"), "got {first}");
        assert!(second.starts_with("story-"), "got {second}");
        assert_ne!(first, second);
        assert_eq!(first, slugify("特朗普宣布新的贸易政策"));
        assert!(first.len() <= 72);
    }

    #[test]
    fn mixed_unicode_keeps_the_readable_part_and_a_fingerprint() {
        let slug = slugify_max("Nvidia 黄仁勋 AI 芯片", 24);
        assert!(slug.starts_with("nvidia-ai-"), "got {slug}");
        assert!(slug.len() <= 24);
    }
}
