use crate::db::get_connection;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};

static LISTENER_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N8nCallbackPayload {
    pub tracking_token: String,
    pub status: String,
    pub live_url: Option<String>,
    pub error_message: Option<String>,
    pub execution_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CallbackResponse {
    pub ok: bool,
    pub message: String,
    pub tracking_token: String,
}

/// Spawns the local Axum HTTP server on 127.0.0.1:20130 in a background Tokio task.
pub fn start_inbound_listener_if_needed() {
    if LISTENER_RUNNING.swap(true, Ordering::SeqCst) {
        return; // Already running
    }

    tokio::spawn(async move {
        let app = Router::new()
            .route("/health", get(health_handler))
            .route("/api/v1/webhook/n8n-status", post(n8n_status_handler))
            .route("/webhook/n8n-callback", post(n8n_status_handler))
            .route(
                "/api/v1/schedules/status/{token}",
                get(get_status_by_token_handler),
            );

        let addr = SocketAddr::from(([127, 0, 0, 1], 20130));
        eprintln!("[CAStudio Inbound] Starting local n8n callback listener on {addr}...");

        match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => {
                if let Err(e) = axum::serve(listener, app).await {
                    eprintln!("[CAStudio Inbound] Server error: {e}");
                    LISTENER_RUNNING.store(false, Ordering::SeqCst);
                }
            }
            Err(e) => {
                eprintln!("[CAStudio Inbound] Failed to bind to {addr}: {e}");
                LISTENER_RUNNING.store(false, Ordering::SeqCst);
            }
        }
    });
}

async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "ok",
            "service": "CAStudio Automation & n8n Callback Listener",
            "port": 20130,
            "version": "0.1.0"
        })),
    )
}

async fn n8n_status_handler(Json(payload): Json<N8nCallbackPayload>) -> impl IntoResponse {
    let now = Utc::now().timestamp();
    let is_success = payload.status.eq_ignore_ascii_case("COMPLETED")
        || payload.status.eq_ignore_ascii_case("SUCCESS");

    let pool = match get_connection() {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CallbackResponse {
                    ok: false,
                    message: format!("DB connection error: {e}"),
                    tracking_token: payload.tracking_token,
                }),
            );
        }
    };
    let conn = pool.lock();

    let new_status = if is_success { "published" } else { "failed" };
    let live_url = payload.live_url.as_deref();
    let err_msg = payload.error_message.as_deref();

    let updated = conn.execute(
        r#"UPDATE content_schedules
           SET status = ?1,
               published_url = coalesce(?2, published_url),
               last_error = ?3,
               updated_at = ?4
           WHERE tracking_token = ?5"#,
        params![new_status, live_url, err_msg, now, payload.tracking_token],
    );

    match updated {
        Ok(rows) if rows > 0 => {
            eprintln!(
                "[CAStudio Inbound] Updated schedule for token {} to status {new_status}",
                payload.tracking_token
            );
            (
                StatusCode::OK,
                Json(CallbackResponse {
                    ok: true,
                    message: format!("Schedule status successfully updated to {new_status}"),
                    tracking_token: payload.tracking_token,
                }),
            )
        }
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Json(CallbackResponse {
                ok: false,
                message: format!("No scheduled job found with token {}", payload.tracking_token),
                tracking_token: payload.tracking_token,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CallbackResponse {
                ok: false,
                message: format!("DB update failed: {e}"),
                tracking_token: payload.tracking_token,
            }),
        ),
    }
}

async fn get_status_by_token_handler(Path(token): Path<String>) -> impl IntoResponse {
    let pool = match get_connection() {
        Ok(c) => c,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("{e}") })),
            );
        }
    };
    let conn = pool.lock();

    let result = conn.query_row(
        r#"SELECT id, campaign_id, asset_id, channel, target_platform, scheduled_at, status, retry_count, published_url, last_error
           FROM content_schedules WHERE tracking_token = ?1"#,
        params![token],
        |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, String>(0)?,
                "campaign_id": row.get::<_, String>(1)?,
                "asset_id": row.get::<_, String>(2)?,
                "channel": row.get::<_, String>(3)?,
                "target_platform": row.get::<_, String>(4)?,
                "scheduled_at": row.get::<_, i64>(5)?,
                "status": row.get::<_, String>(6)?,
                "retry_count": row.get::<_, i32>(7)?,
                "published_url": row.get::<_, Option<String>>(8)?,
                "last_error": row.get::<_, Option<String>>(9)?,
            }))
        },
    );

    match result {
        Ok(data) => (StatusCode::OK, Json(data)),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Token not found" })),
        ),
    }
}
