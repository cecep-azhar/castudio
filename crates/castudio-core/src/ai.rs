use crate::db::get_connection;
use crate::error::StudioError;
use crate::prefs::get_ai_settings;
use crate::project::get_bible;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateAiRequest {
    pub project_id: Option<String>,
    pub document_id: Option<String>,
    pub system_instruction: Option<String>,
    pub user_prompt: String,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateAiResponse {
    pub run_id: String,
    pub text: String,
    pub model: String,
    pub duration_ms: u64,
}

pub fn generate_completion(req: GenerateAiRequest) -> Result<GenerateAiResponse, StudioError> {
    let settings = get_ai_settings()?;
    let base_url = settings.base_url.trim().trim_end_matches('/');
    if base_url.is_empty() {
        return Err(StudioError::Ai(
            "AI Base URL is empty. Configure in Settings (default http://localhost:20128/v1).".to_string(),
        ));
    }

    let mut system_text = req.system_instruction.unwrap_or_else(|| {
        "You are CAStudio AI, an elite technical and creative content engine for sovereign developers and creators.".to_string()
    });

    // Inject Project Bible if project_id is provided
    if let Some(ref pid) = req.project_id {
        if let Ok(bible) = get_bible(pid) {
            system_text.push_str("\n\n### PROJECT BIBLE CONTEXT (STRICT VOICE & FACTS):\n");
            if !bible.core_facts.is_empty() {
                system_text.push_str(&format!("- Core Facts: {}\n", bible.core_facts));
            }
            if !bible.key_terms.is_empty() {
                system_text.push_str(&format!("- Key Terms: {}\n", bible.key_terms));
            }
            if !bible.voice_sample.is_empty() {
                system_text.push_str(&format!("- Tone & Voice: {}\n", bible.voice_sample));
            }
            if !bible.guidelines.is_empty() {
                system_text.push_str(&format!("- Guidelines: {}\n", bible.guidelines));
            }
        }
    }

    let scrubbed_user_prompt = scrub_pii(&req.user_prompt);

    let payload = serde_json::json!({
        "model": settings.model,
        "messages": [
            { "role": "system", "content": system_text },
            { "role": "user", "content": scrubbed_user_prompt }
        ],
        "temperature": req.temperature.unwrap_or(0.7),
        "stream": false
    });

    let url = format!("{base_url}/chat/completions");
    let start = Instant::now();

    let config = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .build();
    let agent: ureq::Agent = config.into();

    let mut request = agent.post(&url).header("Content-Type", "application/json");
    if !settings.api_key.trim().is_empty() {
        request = request.header("Authorization", &format!("Bearer {}", settings.api_key.trim()));
    }

    let mut response = request.send_json(&payload).map_err(|e| {
        StudioError::Ai(format!("Failed to connect to AI provider at {url}: {e}"))
    })?;

    let body = response.body_mut().read_to_string().map_err(|e| {
        StudioError::Ai(format!("Failed to read response from AI provider: {e}"))
    })?;

    let duration_ms = start.elapsed().as_millis() as u64;

    let res_json: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
        StudioError::Ai(format!("Invalid JSON from AI provider: {e}"))
    })?;

    let content = res_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| {
            StudioError::Ai(format!("No completion text found in AI response: {body}"))
        })?
        .to_string();

    let run_id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp_millis();

    // Record in ledger
    if let Ok(pool) = get_connection() {
        let conn = pool.lock();
        let _ = conn.execute(
            "INSERT INTO ai_runs (id, project_id, document_id, model, prompt_tokens, completion_tokens, duration_ms, status, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 'done', ?8)",
            params![
                run_id,
                req.project_id,
                req.document_id,
                settings.model,
                res_json["usage"]["prompt_tokens"].as_i64().unwrap_or(0),
                res_json["usage"]["completion_tokens"].as_i64().unwrap_or(0),
                duration_ms as i64,
                now,
            ],
        );
    }

    Ok(GenerateAiResponse {
        run_id,
        text: content,
        model: settings.model,
        duration_ms,
    })
}

/// Simple PII scrubber for sensitive numbers before sending to LLM
fn scrub_pii(text: &str) -> String {
    let card_re = regex::Regex::new(r"\b\d{4}[ -]?\d{4}[ -]?\d{4}[ -]?\d{4}\b").unwrap();
    let res = card_re.replace_all(text, "[REDACTED_CARD]");
    res.to_string()
}
