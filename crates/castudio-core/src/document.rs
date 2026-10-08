use crate::db::get_connection;
use crate::error::StudioError;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub project_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub doc_type: String, // "ebook", "carousel", "script", "pitchdeck", "custom"
    pub status: String,   // "draft", "in_review", "published"
    pub word_count: i64,
    pub meta: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    pub document_id: String,
    pub parent_id: Option<String>,
    pub block_type: String, // "heading", "paragraph", "list", "callout", "quote", "stat", "slide_break", "image", "code"
    pub position: i64,
    pub content: String,
    pub meta: String,
    pub updated_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateDocumentInput {
    pub project_id: String,
    pub parent_id: Option<String>,
    pub title: String,
    pub doc_type: String,
    pub initial_content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDocumentInput {
    pub title: Option<String>,
    pub doc_type: Option<String>,
    pub status: Option<String>,
    pub meta: Option<String>,
}

pub fn list_documents(project_id: &str) -> Result<Vec<Document>, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_id, title, doc_type, status, word_count, meta, created_at, updated_at 
         FROM documents WHERE project_id = ?1 ORDER BY updated_at DESC",
    )?;

    let iter = stmt.query_map(params![project_id], |row| {
        Ok(Document {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_id: row.get(2)?,
            title: row.get(3)?,
            doc_type: row.get(4)?,
            status: row.get(5)?,
            word_count: row.get(6)?,
            meta: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    let mut list = Vec::new();
    for doc in iter {
        list.push(doc?);
    }
    Ok(list)
}

pub fn create_document(input: CreateDocumentInput) -> Result<Document, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp_millis();
    let doc_id = Uuid::new_v4().to_string();

    let doc = Document {
        id: doc_id.clone(),
        project_id: input.project_id,
        parent_id: input.parent_id,
        title: input.title,
        doc_type: input.doc_type,
        status: "draft".to_string(),
        word_count: 0,
        meta: "{}".to_string(),
        created_at: now,
        updated_at: now,
    };

    conn.execute(
        "INSERT INTO documents (id, project_id, parent_id, title, doc_type, status, word_count, meta, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            doc.id,
            doc.project_id,
            doc.parent_id,
            doc.title,
            doc.doc_type,
            doc.status,
            doc.word_count,
            doc.meta,
            doc.created_at,
            doc.updated_at,
        ],
    )?;

    // If initial text is provided, create a starter paragraph block
    if let Some(content) = input.initial_content {
        let block_id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO blocks (id, document_id, parent_id, block_type, position, content, meta, updated_at)
             VALUES (?1, ?2, NULL, 'paragraph', 0, ?3, '{}', ?4)",
            params![block_id, doc_id, content, now],
        )?;
    }

    Ok(doc)
}

pub fn get_document(id: &str) -> Result<Document, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    conn.query_row(
        "SELECT id, project_id, parent_id, title, doc_type, status, word_count, meta, created_at, updated_at 
         FROM documents WHERE id = ?1",
        params![id],
        |row| {
            Ok(Document {
                id: row.get(0)?,
                project_id: row.get(1)?,
                parent_id: row.get(2)?,
                title: row.get(3)?,
                doc_type: row.get(4)?,
                status: row.get(5)?,
                word_count: row.get(6)?,
                meta: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        },
    ).map_err(|_| StudioError::NotFound(format!("Document {id} not found")))
}

pub fn update_document(id: &str, input: UpdateDocumentInput) -> Result<Document, StudioError> {
    let existing = get_document(id)?;
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp_millis();

    let updated = Document {
        id: existing.id,
        project_id: existing.project_id,
        parent_id: existing.parent_id,
        title: input.title.unwrap_or(existing.title),
        doc_type: input.doc_type.unwrap_or(existing.doc_type),
        status: input.status.unwrap_or(existing.status),
        word_count: existing.word_count,
        meta: input.meta.unwrap_or(existing.meta),
        created_at: existing.created_at,
        updated_at: now,
    };

    conn.execute(
        "UPDATE documents SET title = ?1, doc_type = ?2, status = ?3, meta = ?4, updated_at = ?5 WHERE id = ?6",
        params![
            updated.title,
            updated.doc_type,
            updated.status,
            updated.meta,
            updated.updated_at,
            id,
        ],
    )?;

    Ok(updated)
}

pub fn delete_document(id: &str) -> Result<(), StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    conn.execute("DELETE FROM documents WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_blocks(document_id: &str) -> Result<Vec<Block>, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let mut stmt = conn.prepare(
        "SELECT id, document_id, parent_id, block_type, position, content, meta, updated_at 
         FROM blocks WHERE document_id = ?1 ORDER BY position ASC",
    )?;

    let iter = stmt.query_map(params![document_id], |row| {
        Ok(Block {
            id: row.get(0)?,
            document_id: row.get(1)?,
            parent_id: row.get(2)?,
            block_type: row.get(3)?,
            position: row.get(4)?,
            content: row.get(5)?,
            meta: row.get(6)?,
            updated_at: row.get(7)?,
        })
    })?;

    let mut list = Vec::new();
    for b in iter {
        list.push(b?);
    }
    Ok(list)
}

pub fn save_blocks(document_id: &str, blocks: Vec<Block>) -> Result<Vec<Block>, StudioError> {
    let pool = get_connection()?;
    let mut conn = pool.lock();
    let now = Utc::now().timestamp_millis();

    let tx = conn.transaction()?;
    tx.execute("DELETE FROM blocks WHERE document_id = ?1", params![document_id])?;

    let mut words = 0;
    for (pos, b) in blocks.iter().enumerate() {
        let block_id = if b.id.is_empty() {
            Uuid::new_v4().to_string()
        } else {
            b.id.clone()
        };
        tx.execute(
            "INSERT INTO blocks (id, document_id, parent_id, block_type, position, content, meta, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                block_id,
                document_id,
                b.parent_id,
                b.block_type,
                pos as i64,
                b.content,
                b.meta,
                now,
            ],
        )?;
        words += b.content.split_whitespace().count();
    }

    tx.execute(
        "UPDATE documents SET word_count = ?1, updated_at = ?2 WHERE id = ?3",
        params![words as i64, now, document_id],
    )?;

    tx.commit()?;
    get_blocks(document_id)
}

/// Detects unfilled placeholders formatted like `[ISI: ...]`
pub fn find_placeholders(blocks: &[Block]) -> Vec<String> {
    let mut out = Vec::new();
    let re = regex::Regex::new(r"\[ISI:\s*([^\]]+)\]").unwrap();
    for b in blocks {
        for cap in re.captures_iter(&b.content) {
            if let Some(m) = cap.get(1) {
                out.push(m.as_str().trim().to_string());
            }
        }
    }
    out
}

/// Converts structured blocks to clean Markdown string
pub fn blocks_to_markdown(blocks: &[Block]) -> String {
    let mut md = String::new();
    for b in blocks {
        match b.block_type.as_str() {
            "heading" => {
                let level = b.meta.parse::<u8>().unwrap_or(2);
                let hashes = "#".repeat(level.clamp(1, 6) as usize);
                md.push_str(&format!("{hashes} {}\n\n", b.content.trim()));
            }
            "paragraph" => {
                md.push_str(&format!("{}\n\n", b.content.trim()));
            }
            "quote" => {
                for line in b.content.lines() {
                    md.push_str(&format!("> {line}\n"));
                }
                md.push('\n');
            }
            "callout" => {
                md.push_str(&format!("> **INFO:** {}\n\n", b.content.trim()));
            }
            "stat" => {
                md.push_str(&format!("**[METRIC: {}]**\n\n", b.content.trim()));
            }
            "slide_break" => {
                md.push_str("---\n\n");
            }
            "code" => {
                md.push_str(&format!("```\n{}\n```\n\n", b.content.trim()));
            }
            _ => {
                md.push_str(&format!("{}\n\n", b.content.trim()));
            }
        }
    }
    md
}
