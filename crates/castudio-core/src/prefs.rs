use crate::db::get_connection;
use crate::error::StudioError;
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSettings {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

pub fn get_setting(key: &str) -> Result<Option<String>, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
    let mut rows = stmt.query(params![key])?;
    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

pub fn save_setting(key: &str, value: &str) -> Result<(), StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

pub fn get_ai_settings() -> Result<AiSettings, StudioError> {
    let base_url = get_setting("ai_base_url")?
        .unwrap_or_else(|| "http://localhost:20128/v1".to_string());
    let api_key = get_setting("ai_api_key")?.unwrap_or_default();
    let model = get_setting("ai_model")?.unwrap_or_else(|| "gpt-4o".to_string());

    Ok(AiSettings {
        base_url,
        api_key,
        model,
    })
}

pub fn save_ai_settings(settings: AiSettings) -> Result<(), StudioError> {
    save_setting("ai_base_url", &settings.base_url)?;
    save_setting("ai_api_key", &settings.api_key)?;
    save_setting("ai_model", &settings.model)?;
    Ok(())
}
