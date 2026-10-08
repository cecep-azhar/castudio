use crate::db::get_connection;
use crate::error::StudioError;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: String,
    pub target_audience: String,
    pub output_language: String,
    pub default_tone: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectBible {
    pub project_id: String,
    pub voice_sample: String,
    pub core_facts: String,
    pub key_terms: String,
    pub stories: String,
    pub guidelines: String,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectInput {
    pub title: String,
    pub description: Option<String>,
    pub target_audience: Option<String>,
    pub output_language: Option<String>,
    pub default_tone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub target_audience: Option<String>,
    pub output_language: Option<String>,
    pub default_tone: Option<String>,
}

pub fn list_projects() -> Result<Vec<Project>, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let mut stmt = conn.prepare(
        "SELECT id, title, description, target_audience, output_language, default_tone, created_at, updated_at 
         FROM projects ORDER BY updated_at DESC",
    )?;

    let iter = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            target_audience: row.get(3)?,
            output_language: row.get(4)?,
            default_tone: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;

    let mut list = Vec::new();
    for p in iter {
        list.push(p?);
    }
    Ok(list)
}

pub fn create_project(input: CreateProjectInput) -> Result<Project, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp_millis();
    let id = Uuid::new_v4().to_string();

    let project = Project {
        id: id.clone(),
        title: input.title,
        description: input.description.unwrap_or_default(),
        target_audience: input.target_audience.unwrap_or_default(),
        output_language: input.output_language.unwrap_or_else(|| "id".to_string()),
        default_tone: input.default_tone.unwrap_or_else(|| "professional".to_string()),
        created_at: now,
        updated_at: now,
    };

    conn.execute(
        "INSERT INTO projects (id, title, description, target_audience, output_language, default_tone, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            project.id,
            project.title,
            project.description,
            project.target_audience,
            project.output_language,
            project.default_tone,
            project.created_at,
            project.updated_at,
        ],
    )?;

    // Create default empty Project Bible
    conn.execute(
        "INSERT INTO project_bibles (project_id, voice_sample, core_facts, key_terms, stories, guidelines, updated_at)
         VALUES (?1, '', '', '', '', '', ?2)",
        params![id, now],
    )?;

    Ok(project)
}

pub fn get_project(id: &str) -> Result<Project, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    conn.query_row(
        "SELECT id, title, description, target_audience, output_language, default_tone, created_at, updated_at 
         FROM projects WHERE id = ?1",
        params![id],
        |row| {
            Ok(Project {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                target_audience: row.get(3)?,
                output_language: row.get(4)?,
                default_tone: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        },
    ).map_err(|_| StudioError::NotFound(format!("Project {id} not found")))
}

pub fn update_project(id: &str, input: UpdateProjectInput) -> Result<Project, StudioError> {
    let existing = get_project(id)?;
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp_millis();

    let updated = Project {
        id: existing.id,
        title: input.title.unwrap_or(existing.title),
        description: input.description.unwrap_or(existing.description),
        target_audience: input.target_audience.unwrap_or(existing.target_audience),
        output_language: input.output_language.unwrap_or(existing.output_language),
        default_tone: input.default_tone.unwrap_or(existing.default_tone),
        created_at: existing.created_at,
        updated_at: now,
    };

    conn.execute(
        "UPDATE projects SET title = ?1, description = ?2, target_audience = ?3, output_language = ?4, default_tone = ?5, updated_at = ?6
         WHERE id = ?7",
        params![
            updated.title,
            updated.description,
            updated.target_audience,
            updated.output_language,
            updated.default_tone,
            updated.updated_at,
            id,
        ],
    )?;

    Ok(updated)
}

pub fn delete_project(id: &str) -> Result<(), StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    conn.execute("DELETE FROM projects WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_bible(project_id: &str) -> Result<ProjectBible, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    conn.query_row(
        "SELECT project_id, voice_sample, core_facts, key_terms, stories, guidelines, updated_at 
         FROM project_bibles WHERE project_id = ?1",
        params![project_id],
        |row| {
            Ok(ProjectBible {
                project_id: row.get(0)?,
                voice_sample: row.get(1)?,
                core_facts: row.get(2)?,
                key_terms: row.get(3)?,
                stories: row.get(4)?,
                guidelines: row.get(5)?,
                updated_at: row.get(6)?,
            })
        },
    ).map_err(|_| StudioError::NotFound(format!("Project Bible for {project_id} not found")))
}

pub fn save_bible(bible: ProjectBible) -> Result<ProjectBible, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp_millis();
    conn.execute(
        "INSERT INTO project_bibles (project_id, voice_sample, core_facts, key_terms, stories, guidelines, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(project_id) DO UPDATE SET
         voice_sample = excluded.voice_sample,
         core_facts = excluded.core_facts,
         key_terms = excluded.key_terms,
         stories = excluded.stories,
         guidelines = excluded.guidelines,
         updated_at = excluded.updated_at",
        params![
            bible.project_id,
            bible.voice_sample,
            bible.core_facts,
            bible.key_terms,
            bible.stories,
            bible.guidelines,
            now,
        ],
    )?;
    let mut updated = bible;
    updated.updated_at = now;
    Ok(updated)
}
