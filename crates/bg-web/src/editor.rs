//! Password-gated human direction desk.
//!
//! Human editors can tell Scout where to look by language and beat. They
//! cannot publish, edit a claim, change an AI score or bypass verification.
//! Credentials live in the service environment; only an HMAC of the password
//! is stored there, and browser sessions are short-lived signed cookies.

use axum::{
    extract::{Form, Path, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use bg_core::domain::{Beat, EditorialLanguage};
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{fmt::Write, str::FromStr};
use subtle::ConstantTimeEq;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;
const COOKIE: &str = "vp_editor";
const SESSION_SECONDS: i64 = 8 * 60 * 60;

#[derive(Clone)]
struct EditorState {
    db: bg_db::Db,
    config: Option<EditorConfig>,
}

#[derive(Clone)]
struct EditorConfig {
    username: String,
    password_sha256: [u8; 32],
    session_secret: Vec<u8>,
}

impl EditorConfig {
    fn from_env() -> Option<Self> {
        let username = std::env::var("BG_EDITOR_USERNAME").ok()?;
        let password_sha256 = decode_32(&std::env::var("BG_EDITOR_PASSWORD_SHA256").ok()?)?;
        let session_secret = decode_hex(&std::env::var("BG_EDITOR_SESSION_SECRET").ok()?)?;
        if username.trim().is_empty() || session_secret.len() < 32 {
            return None;
        }
        Some(Self {
            username,
            password_sha256,
            session_secret,
        })
    }

    fn password_matches(&self, username: &str, password: &str) -> bool {
        if username != self.username {
            return false;
        }
        let digest = Sha256::digest(password.as_bytes());
        bool::from(digest[..].ct_eq(&self.password_sha256))
    }

    fn session_cookie(&self) -> Option<String> {
        let expires = chrono::Utc::now().timestamp() + SESSION_SECONDS;
        let payload = format!("{}|{expires}", self.username);
        let encoded = URL_SAFE_NO_PAD.encode(payload.as_bytes());
        let mut mac = HmacSha256::new_from_slice(&self.session_secret).ok()?;
        mac.update(encoded.as_bytes());
        let signature = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
        Some(format!(
            "{COOKIE}={encoded}.{signature}; Path=/editor; Max-Age={SESSION_SECONDS}; HttpOnly; Secure; SameSite=Strict"
        ))
    }

    fn actor(&self, headers: &HeaderMap) -> Option<String> {
        let cookie = headers.get(header::COOKIE)?.to_str().ok()?;
        let value = cookie.split(';').find_map(|part| {
            part.trim()
                .strip_prefix(&format!("{COOKIE}="))
                .map(str::to_owned)
        })?;
        let (encoded, signature) = value.split_once('.')?;
        let signature = URL_SAFE_NO_PAD.decode(signature).ok()?;
        let mut mac = HmacSha256::new_from_slice(&self.session_secret).ok()?;
        mac.update(encoded.as_bytes());
        mac.verify_slice(&signature).ok()?;
        let payload = String::from_utf8(URL_SAFE_NO_PAD.decode(encoded).ok()?).ok()?;
        let (username, expires) = payload.rsplit_once('|')?;
        let expires = expires.parse::<i64>().ok()?;
        if username == self.username && expires >= chrono::Utc::now().timestamp() {
            Some(username.to_string())
        } else {
            None
        }
    }
}

pub fn router(db: bg_db::Db) -> Router {
    let state = EditorState {
        db,
        config: EditorConfig::from_env(),
    };
    Router::new()
        .route("/editor", get(dashboard))
        .route("/editor/login", get(login_page).post(login))
        .route("/editor/logout", post(logout))
        .route("/editor/directions", post(create_direction))
        .route(
            "/editor/directions/{id}/{status}",
            post(set_direction_status),
        )
        .with_state(state)
}

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String,
}

async fn login_page(State(state): State<EditorState>, headers: HeaderMap) -> Response {
    if let Some(config) = &state.config {
        if config.actor(&headers).is_some() {
            return Redirect::to("/editor").into_response();
        }
    } else {
        return page(
            StatusCode::SERVICE_UNAVAILABLE,
            "编辑台未配置",
            "<main class=login><h1>编辑台未配置</h1><p>请设置 BG_EDITOR_USERNAME、BG_EDITOR_PASSWORD_SHA256 与 BG_EDITOR_SESSION_SECRET。</p></main>".into(),
        );
    }
    page(
        StatusCode::OK,
        "编辑登录",
        r#"<main class="login"><p class="eyebrow">VictoriaPark / 维园网</p><h1>人类编辑台</h1><p>设定发现方向；发布、核验与事实边界仍由独立 AI 编辑流程执行。</p><form method="post" action="/editor/login"><label>用户名<input name="username" autocomplete="username" required></label><label>密码<input name="password" type="password" autocomplete="current-password" required></label><button type="submit">登录</button></form></main>"#.into(),
    )
}

async fn login(State(state): State<EditorState>, Form(form): Form<Login>) -> Response {
    let Some(config) = &state.config else {
        return login_page(State(state), HeaderMap::new()).await;
    };
    if !config.password_matches(form.username.trim(), &form.password) {
        return page(
            StatusCode::UNAUTHORIZED,
            "登录失败",
            "<main class=login><h1>登录失败</h1><p>用户名或密码不正确。</p><p><a href=/editor/login>重试</a></p></main>".into(),
        );
    }
    let Some(cookie) = config.session_cookie() else {
        return page(
            StatusCode::INTERNAL_SERVER_ERROR,
            "登录失败",
            "<main class=login><h1>无法创建会话</h1></main>".into(),
        );
    };
    let mut response = Redirect::to("/editor").into_response();
    if let Ok(value) = HeaderValue::from_str(&cookie) {
        response.headers_mut().insert(header::SET_COOKIE, value);
    }
    response
}

async fn logout() -> Response {
    let mut response = Redirect::to("/editor/login").into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_static(
            "vp_editor=; Path=/editor; Max-Age=0; HttpOnly; Secure; SameSite=Strict",
        ),
    );
    response
}

async fn dashboard(State(state): State<EditorState>, headers: HeaderMap) -> Response {
    let Some(actor) = authenticated(&state, &headers) else {
        return Redirect::to("/editor/login").into_response();
    };
    let directions = match bg_db::directions::list(&state.db, 100).await {
        Ok(rows) => rows,
        Err(error) => {
            tracing::error!(%error, "editorial direction dashboard failed");
            return page(
                StatusCode::INTERNAL_SERVER_ERROR,
                "编辑台错误",
                "<main class=login><h1>暂时无法读取编辑方向</h1></main>".into(),
            );
        }
    };
    let mut rows = String::new();
    for direction in directions {
        let next = if direction.status == "active" {
            "paused"
        } else {
            "active"
        };
        let action = if next == "paused" { "暂停" } else { "启用" };
        let searched = direction
            .last_searched_at
            .map(|at| at.format("%Y-%m-%d %H:%M UTC").to_string())
            .unwrap_or_else(|| "等待首次搜索".into());
        let _ = write!(
            rows,
            "<tr><td><strong>{}</strong><small>{}</small></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><form method=post action=\"/editor/directions/{}/{}\"><button class=quiet type=submit>{}</button></form></td></tr>",
            escape(&direction.title),
            escape(&direction.briefing),
            escape(direction.editorial_language.as_str()),
            escape(direction.beat.as_str()),
            direction.priority,
            escape(&searched),
            direction.id,
            next,
            action,
        );
    }
    if rows.is_empty() {
        rows.push_str(
            "<tr><td colspan=6 class=empty>尚无人工编辑方向；AI 编辑部继续独立运行。</td></tr>",
        );
    }
    let body = format!(
        r#"<header><div><p class=eyebrow>VictoriaPark / 维园网</p><h1>编辑方向台</h1><p>登录：{}</p></div><form method=post action=/editor/logout><button class=quiet type=submit>退出</button></form></header><main><section class=notice><strong>双轨编辑：</strong>人工方向只增加搜索优先级；AI 记者与编辑仍独立分诊、核验和发布。专题至少积累 5 篇已发布报道后，才会进入“热门专题”。</section><section><h2>新增发现方向</h2><form class=direction method=post action=/editor/directions><label>方向标题<input name=title minlength=4 maxlength=200 required placeholder="例如：美加关税与汽车供应链"></label><label>语言<select name=language><option value=zh>简体中文</option><option value=zh-hant>繁體中文</option><option value=en>English</option><option value=ja>日本語</option><option value=ko>한국어</option></select></label><label>栏目<select name=beat><option value=world>国际政治</option><option value=markets>财经</option><option value=tech>科技</option><option value=ai>人工智能</option><option value=science>科学健康</option><option value=culture>文化</option><option value=crypto>加密市场</option></select></label><label>优先级<input name=priority type=number min=1 max=100 value=70 required></label><label class=wide>主题实体（逗号分隔）<input name=anchors required placeholder="美国, 加拿大, USMCA"></label><label class=wide>关键议题（逗号分隔）<input name=keywords required placeholder="关税, 贸易战, 供应链"></label><label class=wide>编辑简报<textarea name=briefing rows=4 placeholder="说明希望追踪的角度、地区与后续信号"></textarea></label><button type=submit>交给 Scout 持续追踪</button></form></section><section><h2>现有方向</h2><div class=table-wrap><table><thead><tr><th>方向</th><th>语言</th><th>栏目</th><th>优先级</th><th>最近搜索</th><th>状态</th></tr></thead><tbody>{}</tbody></table></div></section></main>"#,
        escape(&actor),
        rows
    );
    page(StatusCode::OK, "编辑方向台", body)
}

#[derive(Deserialize)]
struct DirectionForm {
    title: String,
    briefing: String,
    anchors: String,
    keywords: String,
    language: String,
    beat: String,
    priority: i16,
}

async fn create_direction(
    State(state): State<EditorState>,
    headers: HeaderMap,
    Form(form): Form<DirectionForm>,
) -> Response {
    let Some(actor) = authenticated(&state, &headers) else {
        return Redirect::to("/editor/login").into_response();
    };
    let Ok(language) = EditorialLanguage::from_str(&form.language) else {
        return bad_request("未知语言");
    };
    let Ok(beat) = Beat::from_str(&form.beat) else {
        return bad_request("未知栏目");
    };
    let anchors = terms(&form.anchors, 12);
    let keywords = terms(&form.keywords, 20);
    if form.title.trim().len() < 4 || anchors.is_empty() || keywords.is_empty() {
        return bad_request("标题、主题实体和关键议题均为必填项");
    }
    let direction = bg_db::directions::NewEditorialDirection {
        title: form.title.trim(),
        briefing: form.briefing.trim(),
        anchor_terms: &anchors,
        keywords: &keywords,
        editorial_language: language,
        beat,
        priority: form.priority.clamp(1, 100),
        created_by: &actor,
    };
    match bg_db::directions::create(&state.db, &direction).await {
        Ok(_) => Redirect::to("/editor").into_response(),
        Err(error) => {
            tracing::error!(%error, "creating editorial direction failed");
            bad_request("无法保存该方向")
        }
    }
}

async fn set_direction_status(
    State(state): State<EditorState>,
    headers: HeaderMap,
    Path((id, status)): Path<(Uuid, String)>,
) -> Response {
    let Some(actor) = authenticated(&state, &headers) else {
        return Redirect::to("/editor/login").into_response();
    };
    if !matches!(status.as_str(), "active" | "paused" | "completed") {
        return bad_request("未知状态");
    }
    match bg_db::directions::set_status(&state.db, id, &status, &actor).await {
        Ok(true) => Redirect::to("/editor").into_response(),
        Ok(false) => bad_request("方向不存在"),
        Err(error) => {
            tracing::error!(%error, "updating editorial direction failed");
            bad_request("无法更新方向")
        }
    }
}

fn authenticated(state: &EditorState, headers: &HeaderMap) -> Option<String> {
    state.config.as_ref()?.actor(headers)
}

fn terms(value: &str, limit: usize) -> Vec<String> {
    let mut values: Vec<String> = value
        .split([',', '，', '\n'])
        .map(str::trim)
        .filter(|term| !term.is_empty())
        .take(limit)
        .map(str::to_owned)
        .collect();
    values.sort();
    values.dedup();
    values
}

fn bad_request(message: &str) -> Response {
    page(
        StatusCode::BAD_REQUEST,
        "输入有误",
        format!(
            "<main class=login><h1>输入有误</h1><p>{}</p><p><a href=/editor>返回编辑台</a></p></main>",
            escape(message)
        ),
    )
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn decode_32(value: &str) -> Option<[u8; 32]> {
    decode_hex(value)?.try_into().ok()
}

fn decode_hex(value: &str) -> Option<Vec<u8>> {
    if !value.len().is_multiple_of(2) {
        return None;
    }
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let text = std::str::from_utf8(pair).ok()?;
            u8::from_str_radix(text, 16).ok()
        })
        .collect()
}

fn page(status: StatusCode, title: &str, body: String) -> Response {
    let document = format!(
        r#"<!doctype html><html lang=zh-CN><head><meta charset=utf-8><meta name=viewport content="width=device-width,initial-scale=1"><title>{}</title><style>:root{{--ink:#102c27;--paper:#f7f1e3;--line:#c8b991;--accent:#9c6711}}*{{box-sizing:border-box}}body{{margin:0;background:var(--paper);color:var(--ink);font:16px/1.55 system-ui,-apple-system,"Noto Sans SC",sans-serif}}header,main{{width:min(1180px,calc(100% - 32px));margin:auto}}header{{display:flex;justify-content:space-between;align-items:center;padding:34px 0 20px;border-bottom:1px solid var(--line)}}h1{{margin:.1em 0;font:700 clamp(30px,5vw,50px)/1.05 Georgia,"Noto Serif SC",serif}}h2{{font:700 26px/1.2 Georgia,"Noto Serif SC",serif}}.eyebrow{{color:var(--accent);font-weight:800;letter-spacing:.12em;text-transform:uppercase}}section{{margin:28px 0;padding:24px;background:#fffaf0;border:1px solid #ded1ad;border-radius:18px}}.notice{{border-left:5px solid var(--accent)}}form.direction{{display:grid;grid-template-columns:2fr 1fr 1fr 1fr;gap:16px}}label{{display:grid;gap:7px;font-weight:700}}input,select,textarea{{width:100%;padding:12px;border:1px solid var(--line);border-radius:9px;background:white;color:var(--ink);font:inherit}}.wide{{grid-column:1/-1}}button{{padding:12px 18px;border:0;border-radius:999px;background:var(--ink);color:white;font-weight:800;cursor:pointer}}button.quiet{{padding:8px 14px;background:transparent;color:var(--ink);border:1px solid var(--line)}}.table-wrap{{overflow:auto}}table{{width:100%;border-collapse:collapse}}th,td{{padding:13px;text-align:left;border-bottom:1px solid #e4dac0;vertical-align:top}}td small{{display:block;max-width:430px;color:#65736f;margin-top:4px}}.empty{{text-align:center;color:#65736f}}main.login{{width:min(520px,calc(100% - 32px));margin:10vh auto;padding:34px;background:#fffaf0;border:1px solid var(--line);border-radius:20px}}main.login form{{display:grid;gap:18px;margin-top:28px}}a{{color:var(--accent)}}@media(max-width:760px){{form.direction{{grid-template-columns:1fr}}.wide{{grid-column:auto}}header{{align-items:flex-start}}}}</style></head><body>{}</body></html>"#,
        escape(title),
        body
    );
    let mut response = (status, Html(document)).into_response();
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, private"),
    );
    response.headers_mut().insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(
            "default-src 'none'; style-src 'unsafe-inline'; form-action 'self'; base-uri 'none'; frame-ancestors 'none'",
        ),
    );
    response
}

#[cfg(test)]
mod tests {
    use super::{decode_32, escape, terms};

    #[test]
    fn editor_input_is_escaped_and_terms_are_bounded() {
        assert_eq!(escape("<script>\"&"), "&lt;script&gt;&quot;&amp;");
        assert_eq!(
            terms("美国, 加拿大，关税\n美国", 3),
            ["关税", "加拿大", "美国"]
        );
    }

    #[test]
    fn editor_hash_must_be_sha256_width() {
        assert!(decode_32(&"ab".repeat(32)).is_some());
        assert!(decode_32("abcd").is_none());
    }
}
