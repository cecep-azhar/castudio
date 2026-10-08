use crate::error::StudioError;
use crate::paths;
use rusqlite::Connection;
use std::sync::OnceLock;

static DB_POOL: OnceLock<parking_lot::Mutex<Connection>> = OnceLock::new();

pub fn get_connection() -> Result<&'static parking_lot::Mutex<Connection>, StudioError> {
    if let Some(pool) = DB_POOL.get() {
        return Ok(pool);
    }

    let path = paths::db_path();
    let conn = Connection::open(&path).map_err(|e| StudioError::Db(e.to_string()))?;

    // Enable WAL mode & foreign keys
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA foreign_keys = ON;
        "#,
    )?;

    init_schema(&conn)?;

    let mutex = parking_lot::Mutex::new(conn);
    let _ = DB_POOL.set(mutex);
    Ok(DB_POOL.get().expect("DB pool initialized"))
}

fn init_schema(conn: &Connection) -> Result<(), StudioError> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            target_audience TEXT NOT NULL DEFAULT '',
            output_language TEXT NOT NULL DEFAULT 'id',
            default_tone TEXT NOT NULL DEFAULT 'professional',
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS project_bibles (
            project_id TEXT PRIMARY KEY,
            voice_sample TEXT NOT NULL DEFAULT '',
            core_facts TEXT NOT NULL DEFAULT '',
            key_terms TEXT NOT NULL DEFAULT '',
            stories TEXT NOT NULL DEFAULT '',
            guidelines TEXT NOT NULL DEFAULT '',
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS documents (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            parent_id TEXT,
            title TEXT NOT NULL,
            doc_type TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'draft',
            word_count INTEGER NOT NULL DEFAULT 0,
            meta TEXT NOT NULL DEFAULT '{}',
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS blocks (
            id TEXT PRIMARY KEY,
            document_id TEXT NOT NULL,
            parent_id TEXT,
            block_type TEXT NOT NULL,
            position INTEGER NOT NULL,
            content TEXT NOT NULL,
            meta TEXT NOT NULL DEFAULT '{}',
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS prompt_templates (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            category TEXT NOT NULL,
            system_prompt TEXT NOT NULL,
            user_prompt TEXT NOT NULL,
            variables TEXT NOT NULL DEFAULT '[]',
            is_favorite INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS ai_runs (
            id TEXT PRIMARY KEY,
            project_id TEXT,
            document_id TEXT,
            model TEXT NOT NULL,
            prompt_tokens INTEGER NOT NULL DEFAULT 0,
            completion_tokens INTEGER NOT NULL DEFAULT 0,
            duration_ms INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'done',
            created_at INTEGER NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_documents_project ON documents(project_id);
        CREATE INDEX IF NOT EXISTS idx_blocks_document ON blocks(document_id, position);
        "#,
    )?;

    // Insert default settings if absent
    conn.execute(
        r#"INSERT OR IGNORE INTO settings (key, value) VALUES ('ai_base_url', 'http://localhost:20128/v1')"#,
        [],
    )?;
    conn.execute(
        r#"INSERT OR IGNORE INTO settings (key, value) VALUES ('ai_model', 'gpt-4o')"#,
        [],
    )?;
    conn.execute(
        r#"INSERT OR IGNORE INTO settings (key, value) VALUES ('ai_api_key', '')"#,
        [],
    )?;
    conn.execute(
        r#"INSERT OR IGNORE INTO settings (key, value) VALUES ('theme', 'dark')"#,
        [],
    )?;

    Ok(())
}
