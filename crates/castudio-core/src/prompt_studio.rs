use crate::db::get_connection;
use crate::error::StudioError;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub id: String,
    pub title: String,
    pub category: String, // "content", "media", "software_dev"
    pub system_prompt: String,
    pub user_prompt: String,
    pub variables: String, // JSON array of string variable names
    pub is_favorite: bool,
    pub created_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreatePromptInput {
    pub title: String,
    pub category: String,
    pub system_prompt: String,
    pub user_prompt: String,
    pub variables: Option<String>,
}

pub fn list_prompts(category: Option<&str>) -> Result<Vec<PromptTemplate>, StudioError> {
    ensure_defaults()?;
    let pool = get_connection()?;
    let conn = pool.lock();

    let query = if let Some(cat) = category {
        format!("SELECT id, title, category, system_prompt, user_prompt, variables, is_favorite, created_at FROM prompt_templates WHERE category = '{cat}' ORDER BY is_favorite DESC, title ASC")
    } else {
        "SELECT id, title, category, system_prompt, user_prompt, variables, is_favorite, created_at FROM prompt_templates ORDER BY is_favorite DESC, title ASC".to_string()
    };

    let mut stmt = conn.prepare(&query)?;
    let iter = stmt.query_map([], |row| {
        Ok(PromptTemplate {
            id: row.get(0)?,
            title: row.get(1)?,
            category: row.get(2)?,
            system_prompt: row.get(3)?,
            user_prompt: row.get(4)?,
            variables: row.get(5)?,
            is_favorite: row.get::<_, i64>(6)? != 0,
            created_at: row.get(7)?,
        })
    })?;

    let mut list = Vec::new();
    for p in iter {
        list.push(p?);
    }
    Ok(list)
}

pub fn create_prompt(input: CreatePromptInput) -> Result<PromptTemplate, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp_millis();
    let id = Uuid::new_v4().to_string();

    let template = PromptTemplate {
        id: id.clone(),
        title: input.title,
        category: input.category,
        system_prompt: input.system_prompt,
        user_prompt: input.user_prompt,
        variables: input.variables.unwrap_or_else(|| "[]".to_string()),
        is_favorite: false,
        created_at: now,
    };

    conn.execute(
        "INSERT INTO prompt_templates (id, title, category, system_prompt, user_prompt, variables, is_favorite, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            template.id,
            template.title,
            template.category,
            template.system_prompt,
            template.user_prompt,
            template.variables,
            0,
            template.created_at,
        ],
    )?;

    Ok(template)
}

pub fn toggle_favorite(id: &str) -> Result<bool, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let curr: i64 = conn.query_row(
        "SELECT is_favorite FROM prompt_templates WHERE id = ?1",
        params![id],
        |r| r.get(0),
    )?;
    let new_val = if curr == 0 { 1 } else { 0 };
    conn.execute(
        "UPDATE prompt_templates SET is_favorite = ?1 WHERE id = ?2",
        params![new_val, id],
    )?;
    Ok(new_val == 1)
}

pub fn delete_prompt(id: &str) -> Result<(), StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    conn.execute("DELETE FROM prompt_templates WHERE id = ?1", params![id])?;
    Ok(())
}

fn ensure_defaults() -> Result<(), StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let count: i64 = conn.query_row("SELECT count(*) FROM prompt_templates", [], |r| r.get(0))?;
    if count > 0 {
        return Ok(());
    }

    let defaults = [
        (
            "Software PRD Architecture Architect",
            "software_dev",
            "You are an elite principal software architect. You output precise, minimal, test-driven PRDs without fluff.",
            "Write a PRD for: {{product_name}}.\nStack: {{stack}}.\nFocus: Functional Requirements with verifiable commands, zero fake success.",
            r#"["product_name", "stack"]"#
        ),
        (
            "QA Read-Only Independent Auditor",
            "software_dev",
            "You are an independent, strictly READ-ONLY QA Auditor. You reject stubs and demand reproducible evidence.",
            "Audit stage {{stage_id}} of {{product_name}}.\nCheck: 1) Every command ran with real output, 2) No swallowed errors, 3) Real test pass.",
            r#"["stage_id", "product_name"]"#
        ),
        (
            "Cyberpunk Monoline App Icon Generator",
            "media",
            "You are an AI prompt specialist for Midjourney v6 and Flux.",
            "App icon for {{app_name}}, {{app_function}}. Minimalist geometric wireframe outline logo of {{symbol}}, sharp angular vector strokes, hollow see-through interior, transparent negative space, bold neon {{color}} glowing contour lines, symmetrical tech-brutalist cyber HUD emblem, pitch-black minimalist background, clean flat monoline line art, high contrast, 8k --v 6.0",
            r#"["app_name", "app_function", "symbol", "color"]"#
        ),
        (
            "Viral Tech Instagram Carousel Pack",
            "content",
            "You are an elite Instagram growth strategist for technical founders.",
            "Generate a 7-slide carousel breakdown on: {{topic}}.\nSlide 1: Big bold question hook.\nSlides 2-6: Punchy actionable insight.\nSlide 7: Summary + CTA.",
            r#"["topic"]"#
        )
    ];

    let now = Utc::now().timestamp_millis();
    for (title, cat, sys, user, vars) in defaults {
        let id = Uuid::new_v4().to_string();
        let _ = conn.execute(
            "INSERT INTO prompt_templates (id, title, category, system_prompt, user_prompt, variables, is_favorite, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1, ?7)",
            params![id, title, cat, sys, user, vars, now],
        );
    }
    Ok(())
}
