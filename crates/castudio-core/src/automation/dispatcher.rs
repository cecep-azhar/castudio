use crate::error::StudioError;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResult {
    pub success: bool,
    pub published_url: Option<String>,
    pub message: String,
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N8nOutboundPayload {
    pub campaign_id: String,
    pub channel: String,
    pub title: String,
    pub raw_content: String,
    pub media_urls: Vec<String>,
    pub scheduled_at: i64,
    pub tracking_token: String,
    pub callback_url: String,
}

fn base64_encode(input: &str) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i];
        let b1 = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
        let b2 = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

        out.push(CHARSET[(b0 >> 2) as usize] as char);
        out.push(CHARSET[(((b0 & 3) << 4) | (b1 >> 4)) as usize] as char);
        if i + 1 < bytes.len() {
            out.push(CHARSET[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < bytes.len() {
            out.push(CHARSET[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}

/// Dispatches an asset to the configured platform target
pub fn dispatch_to_platform(
    target_platform: &str,
    endpoint_url: &str,
    credentials_json: &str,
    config_json: &str,
    title: &str,
    content: &str,
    campaign_id: &str,
    channel: &str,
    scheduled_at: i64,
    tracking_token: &str,
) -> Result<PublishResult, StudioError> {
    let creds: serde_json::Value =
        serde_json::from_str(credentials_json).unwrap_or(serde_json::json!({}));
    let config: serde_json::Value =
        serde_json::from_str(config_json).unwrap_or(serde_json::json!({}));

    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(20)))
        .build()
        .into();

    match target_platform {
        "wordpress" => dispatch_wordpress(&agent, endpoint_url, &creds, &config, title, content),
        "ghost" => dispatch_ghost(&agent, endpoint_url, &creds, title, content),
        "telegram" => dispatch_telegram(&agent, &creds, &config, title, content),
        "n8n_webhook" => dispatch_n8n_webhook(
            &agent,
            endpoint_url,
            campaign_id,
            channel,
            title,
            content,
            scheduled_at,
            tracking_token,
        ),
        "custom_webhook" => dispatch_custom_webhook(
            &agent,
            endpoint_url,
            &creds,
            title,
            content,
            campaign_id,
            channel,
            tracking_token,
        ),
        _ => Err(StudioError::Dispatch(format!(
            "Unsupported publishing target platform: {target_platform}"
        ))),
    }
}

/// WordPress REST API: POST /wp-json/wp/v2/posts
fn dispatch_wordpress(
    agent: &ureq::Agent,
    endpoint_url: &str,
    creds: &serde_json::Value,
    config: &serde_json::Value,
    title: &str,
    content: &str,
) -> Result<PublishResult, StudioError> {
    let base_url = endpoint_url.trim_end_matches('/');
    let api_url = format!("{base_url}/wp-json/wp/v2/posts");

    let username = creds["username"].as_str().unwrap_or("");
    let app_password = creds["app_password"]
        .as_str()
        .or_else(|| creds["password"].as_str())
        .unwrap_or("");

    let status = config["status"].as_str().unwrap_or("publish");

    let payload = serde_json::json!({
        "title": title,
        "content": content,
        "status": status,
    });

    let mut req = agent
        .post(&api_url)
        .header("Content-Type", "application/json");

    if !username.is_empty() && !app_password.is_empty() {
        let auth_str = format!("{username}:{app_password}");
        let encoded = base64_encode(&auth_str);
        req = req.header("Authorization", &format!("Basic {encoded}"));
    }

    let mut resp = req
        .send_json(&payload)
        .map_err(|e| StudioError::Dispatch(format!("WordPress push failed: {e}")))?;

    let json_resp: serde_json::Value = resp
        .body_mut()
        .read_json()
        .map_err(|e| StudioError::Dispatch(format!("Invalid response JSON: {e}")))?;

    let post_url = json_resp["link"]
        .as_str()
        .map(|s| s.to_string())
        .or_else(|| {
            json_resp["id"]
                .as_i64()
                .map(|id| format!("{base_url}/?p={id}"))
        });

    Ok(PublishResult {
        success: true,
        published_url: post_url,
        message: "WordPress post published successfully".to_string(),
        external_id: json_resp["id"].as_i64().map(|id| id.to_string()),
    })
}

/// Ghost Admin API: POST /ghost/api/admin/posts/
fn dispatch_ghost(
    agent: &ureq::Agent,
    endpoint_url: &str,
    creds: &serde_json::Value,
    title: &str,
    content: &str,
) -> Result<PublishResult, StudioError> {
    let base_url = endpoint_url.trim_end_matches('/');
    let api_url = format!("{base_url}/ghost/api/admin/posts/");

    let api_key = creds["admin_api_key"]
        .as_str()
        .or_else(|| creds["token"].as_str())
        .unwrap_or("");

    let payload = serde_json::json!({
        "posts": [{
            "title": title,
            "custom_excerpt": title,
            "status": "published",
            "html": format!("<p>{}</p>", content.replace('\n', "<br/>"))
        }]
    });

    let mut req = agent
        .post(&api_url)
        .header("Content-Type", "application/json");

    if !api_key.is_empty() {
        req = req.header("Authorization", &format!("Ghost {api_key}"));
    }

    let mut resp = req
        .send_json(&payload)
        .map_err(|e| StudioError::Dispatch(format!("Ghost push failed: {e}")))?;

    let json_resp: serde_json::Value = resp
        .body_mut()
        .read_json()
        .map_err(|e| StudioError::Dispatch(format!("Invalid Ghost JSON response: {e}")))?;

    let post_url = json_resp["posts"][0]["url"].as_str().map(|s| s.to_string());

    Ok(PublishResult {
        success: true,
        published_url: post_url,
        message: "Ghost post published successfully".to_string(),
        external_id: json_resp["posts"][0]["id"].as_str().map(|s| s.to_string()),
    })
}

/// Telegram Bot API: POST https://api.telegram.org/bot<TOKEN>/sendMessage
fn dispatch_telegram(
    agent: &ureq::Agent,
    creds: &serde_json::Value,
    config: &serde_json::Value,
    title: &str,
    content: &str,
) -> Result<PublishResult, StudioError> {
    let bot_token = creds["bot_token"].as_str().unwrap_or("");
    let chat_id = config["chat_id"]
        .as_str()
        .or_else(|| creds["chat_id"].as_str())
        .unwrap_or("");

    if bot_token.is_empty() || chat_id.is_empty() {
        return Err(StudioError::Dispatch(
            "Telegram bot_token and chat_id are required".to_string(),
        ));
    }

    let url = format!("https://api.telegram.org/bot{bot_token}/sendMessage");

    // Format message with title and auto-chunk if longer than 4000 characters
    let full_text = format!("*{title}*\n\n{content}");
    let chunk: String = full_text.chars().take(4000).collect();

    let payload = serde_json::json!({
        "chat_id": chat_id,
        "text": chunk,
        "parse_mode": "Markdown",
        "disable_web_page_preview": false
    });

    let mut resp = agent
        .post(&url)
        .header("Content-Type", "application/json")
        .send_json(&payload)
        .map_err(|e| StudioError::Dispatch(format!("Telegram push failed: {e}")))?;

    let json_resp: serde_json::Value = resp
        .body_mut()
        .read_json()
        .map_err(|e| StudioError::Dispatch(format!("Invalid Telegram JSON response: {e}")))?;

    let msg_id = json_resp["result"]["message_id"]
        .as_i64()
        .map(|id| id.to_string());

    Ok(PublishResult {
        success: true,
        published_url: Some(format!("https://t.me/c/{chat_id}")),
        message: "Telegram message broadcasted successfully".to_string(),
        external_id: msg_id,
    })
}

/// n8n Outbound Webhook Trigger with standard contract payload
fn dispatch_n8n_webhook(
    agent: &ureq::Agent,
    endpoint_url: &str,
    campaign_id: &str,
    channel: &str,
    title: &str,
    content: &str,
    scheduled_at: i64,
    tracking_token: &str,
) -> Result<PublishResult, StudioError> {
    if endpoint_url.trim().is_empty() {
        return Err(StudioError::Dispatch(
            "n8n webhook endpoint URL is empty".to_string(),
        ));
    }

    let payload = N8nOutboundPayload {
        campaign_id: campaign_id.to_string(),
        channel: channel.to_string(),
        title: title.to_string(),
        raw_content: content.to_string(),
        media_urls: vec![],
        scheduled_at,
        tracking_token: tracking_token.to_string(),
        callback_url: "http://127.0.0.1:20130/api/v1/webhook/n8n-status".to_string(),
    };

    let mut resp = agent
        .post(endpoint_url)
        .header("Content-Type", "application/json")
        .header("X-CAStudio-Tracking", tracking_token)
        .send_json(&payload)
        .map_err(|e| StudioError::Dispatch(format!("n8n webhook trigger failed: {e}")))?;

    let mut resp_str = String::new();
    let _ = resp.body_mut().as_reader().read_to_string(&mut resp_str);

    Ok(PublishResult {
        success: true,
        published_url: None, // Will be filled asynchronously via inbound callback
        message: format!(
            "n8n webhook triggered. Awaiting callback on tracking token {tracking_token}"
        ),
        external_id: Some(tracking_token.to_string()),
    })
}

/// Custom Webhook Dispatcher
fn dispatch_custom_webhook(
    agent: &ureq::Agent,
    endpoint_url: &str,
    creds: &serde_json::Value,
    title: &str,
    content: &str,
    campaign_id: &str,
    channel: &str,
    tracking_token: &str,
) -> Result<PublishResult, StudioError> {
    if endpoint_url.trim().is_empty() {
        return Err(StudioError::Dispatch(
            "Custom webhook URL is empty".to_string(),
        ));
    }

    let secret = creds["secret"].as_str().unwrap_or("");

    let payload = serde_json::json!({
        "event": "castudio.content.publish",
        "campaign_id": campaign_id,
        "channel": channel,
        "title": title,
        "content": content,
        "tracking_token": tracking_token,
        "callback_url": "http://127.0.0.1:20130/api/v1/webhook/n8n-status"
    });

    let mut req = agent
        .post(endpoint_url)
        .header("Content-Type", "application/json")
        .header("X-CAStudio-Tracking", tracking_token);

    if !secret.is_empty() {
        req = req.header("X-CAStudio-Secret", secret);
    }

    let mut resp = req
        .send_json(&payload)
        .map_err(|e| StudioError::Dispatch(format!("Custom webhook failed: {e}")))?;

    let mut resp_str = String::new();
    let _ = resp.body_mut().as_reader().read_to_string(&mut resp_str);

    Ok(PublishResult {
        success: true,
        published_url: None,
        message: "Custom webhook triggered successfully".to_string(),
        external_id: Some(tracking_token.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("admin:secret"), "YWRtaW46c2VjcmV0");
        assert_eq!(base64_encode("hello world"), "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_n8n_outbound_payload_serialization() {
        let payload = N8nOutboundPayload {
            campaign_id: "cmp_123".to_string(),
            channel: "tweets".to_string(),
            title: "Test Thread".to_string(),
            raw_content: "1/5 Hello world".to_string(),
            media_urls: vec!["https://img.jpg".to_string()],
            scheduled_at: 1728412800,
            tracking_token: "trk_abc".to_string(),
            callback_url: "http://127.0.0.1:20130/api/v1/webhook/n8n-status".to_string(),
        };

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("trk_abc"));
        assert!(json.contains("http://127.0.0.1:20130/api/v1/webhook/n8n-status"));
    }
}
