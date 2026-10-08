use super::dispatcher::dispatch_to_platform;
use crate::db::get_connection;
use crate::error::StudioError;
use chrono::Utc;
use rand::Rng;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use uuid::Uuid;

static CRON_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSchedule {
    pub id: String,
    pub campaign_id: String,
    pub asset_id: String,
    pub channel: String,
    pub target_platform: String,
    pub scheduled_at: i64,
    pub status: String,
    pub retry_count: i32,
    pub max_retries: i32,
    pub last_error: Option<String>,
    pub published_url: Option<String>,
    pub tracking_token: String,
    pub channel_config_json: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishingChannel {
    pub id: String,
    pub name: String,
    pub channel_type: String,
    pub is_active: bool,
    pub endpoint_url: Option<String>,
    pub credentials_json: String,
    pub config_json: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScheduleInput {
    pub campaign_id: String,
    pub asset_id: String,
    pub channel: String,
    pub target_platform: String,
    pub scheduled_at: i64,
    pub channel_config_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDripBatchInput {
    pub campaign_id: String,
    pub preset: String,          // "aggressive", "balanced", "extended"
    pub start_time: i64,         // unix timestamp in seconds
    pub default_platform: String, // "wordpress", "telegram", "n8n_webhook", etc.
}

/// Calculate drip timestamp based on preset and channel
pub fn calculate_drip_offset_seconds(preset: &str, channel: &str) -> i64 {
    match (preset.to_lowercase().as_str(), channel) {
        // Aggressive (3-Day Cadence)
        ("aggressive", "blog") => 0,                  // D+0 09:00
        ("aggressive", "tweets") => 10 * 3600 + 1800, // D+0 19:30 (+10.5h)
        ("aggressive", "shorts") => 27 * 3600,        // D+1 12:00 (+27h)
        ("aggressive", "carousel") => 33 * 3600,      // D+1 18:00 (+33h)
        ("aggressive", "newsletter") => 47 * 3600,    // D+2 08:00 (+47h)

        // Balanced Evergreen (7-Day Cadence - Default)
        ("balanced", "blog") => 0,                  // D+0 09:00
        ("balanced", "tweets") => 10 * 3600 + 1800, // D+0 19:30 (+10.5h)
        ("balanced", "shorts") => 27 * 3600,        // D+1 12:00 (+27h)
        ("balanced", "carousel") => 56 * 3600,      // D+2 17:00 (+56h)
        ("balanced", "newsletter") => 71 * 3600,    // D+3 08:00 (+71h)

        // Extended Repurposing (14-Day Cadence)
        ("extended", "blog") => 0,
        ("extended", "tweets") => 24 * 3600,
        ("extended", "shorts") => 72 * 3600,
        ("extended", "carousel") => 120 * 3600,
        ("extended", "newsletter") => 168 * 3600,

        // Fallback staggered
        (_, "blog") => 0,
        (_, "tweets") => 12 * 3600,
        (_, "shorts") => 24 * 3600,
        (_, "carousel") => 48 * 3600,
        (_, "newsletter") => 72 * 3600,
        (_, _) => 0,
    }
}

/// Spawns background Tokio worker polling due schedules every 60 seconds
pub fn start_cron_worker_if_needed() {
    if CRON_RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }

    tokio::spawn(async move {
        eprintln!("[CAStudio Cron] Background scheduler worker started (interval: 60s).");
        tokio::time::sleep(Duration::from_secs(5)).await;

        loop {
            if let Err(e) = poll_and_dispatch_due_jobs().await {
                eprintln!("[CAStudio Cron] Error during polling tick: {e}");
            }
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
}

struct DueJob {
    id: String,
    campaign_id: String,
    asset_id: String,
    channel: String,
    target_platform: String,
    scheduled_at: i64,
    retry_count: i32,
    max_retries: i32,
    tracking_token: String,
    channel_config_json: String,
}

/// Polls due jobs and executes dispatch
pub async fn poll_and_dispatch_due_jobs() -> Result<usize, StudioError> {
    let now = Utc::now().timestamp();
    let pool = get_connection()?;

    let jobs: Vec<DueJob> = {
        let conn = pool.lock();
        let mut stmt = conn.prepare(
            r#"SELECT id, campaign_id, asset_id, channel, target_platform, scheduled_at,
                      retry_count, max_retries, tracking_token, channel_config_json
               FROM content_schedules
               WHERE status = 'queued' AND scheduled_at <= ?1
               ORDER BY scheduled_at ASC
               LIMIT 10"#,
        )?;

        let iter = stmt.query_map(params![now], |row| {
            Ok(DueJob {
                id: row.get(0)?,
                campaign_id: row.get(1)?,
                asset_id: row.get(2)?,
                channel: row.get(3)?,
                target_platform: row.get(4)?,
                scheduled_at: row.get(5)?,
                retry_count: row.get(6)?,
                max_retries: row.get(7)?,
                tracking_token: row.get(8)?,
                channel_config_json: row.get(9)?,
            })
        })?;

        let mut res = Vec::new();
        for j in iter {
            res.push(j?);
        }
        res
    };

    let count = jobs.len();
    if count == 0 {
        return Ok(0);
    }

    eprintln!("[CAStudio Cron] Found {count} due schedule(s) to dispatch.");

    for job in jobs {
        let (title, content, endpoint_url, creds_json, config_json) = {
            let conn = pool.lock();

            // Mark as dispatching
            let _ = conn.execute(
                "UPDATE content_schedules SET status = 'dispatching', updated_at = ?1 WHERE id = ?2",
                params![now, job.id],
            );

            // Fetch asset title and content
            let asset_data = conn.query_row(
                "SELECT title, content FROM content_assets WHERE id = ?1",
                params![job.asset_id],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            );

            let (t, c) = match asset_data {
                Ok(data) => data,
                Err(e) => {
                    let _ = conn.execute(
                        "UPDATE content_schedules SET status = 'failed', last_error = ?1, updated_at = ?2 WHERE id = ?3",
                        params![format!("Asset not found: {e}"), now, job.id],
                    );
                    continue;
                }
            };

            // Fetch channel endpoint & credentials if available
            let channel_info = conn
                .query_row(
                    r#"SELECT endpoint_url, credentials_json, config_json
                       FROM publishing_channels
                       WHERE channel_type = ?1 AND is_active = 1
                       LIMIT 1"#,
                    params![job.target_platform],
                    |row| {
                        Ok((
                            row.get::<_, Option<String>>(0)?,
                            row.get::<_, String>(1)?,
                            row.get::<_, String>(2)?,
                        ))
                    },
                )
                .ok();

            let url = channel_info
                .as_ref()
                .and_then(|(u, _, _)| u.clone())
                .unwrap_or_default();
            let creds = channel_info
                .as_ref()
                .map(|(_, cr, _)| cr.clone())
                .unwrap_or_else(|| "{}".to_string());
            let cfg = if !job.channel_config_json.trim().is_empty()
                && job.channel_config_json != "{}"
            {
                job.channel_config_json.clone()
            } else {
                channel_info
                    .as_ref()
                    .map(|(_, _, conf)| conf.clone())
                    .unwrap_or_else(|| "{}".to_string())
            };

            (t, c, url, creds, cfg)
        };

        let dispatch_result = dispatch_to_platform(
            &job.target_platform,
            &endpoint_url,
            &creds_json,
            &config_json,
            &title,
            &content,
            &job.campaign_id,
            &job.channel,
            job.scheduled_at,
            &job.tracking_token,
        );

        let conn = pool.lock();
        match dispatch_result {
            Ok(res) => {
                let pub_url = res.published_url.as_deref();
                let _ = conn.execute(
                    r#"UPDATE content_schedules
                       SET status = 'published',
                           published_url = ?1,
                           last_error = NULL,
                           updated_at = ?2
                       WHERE id = ?3"#,
                    params![pub_url, Utc::now().timestamp(), job.id],
                );
                eprintln!("[CAStudio Cron] Job {} successfully published.", job.id);
            }
            Err(e) => {
                let new_retry = job.retry_count + 1;
                let err_msg = format!("{e}");

                if new_retry < job.max_retries {
                    let mut rng = rand::thread_rng();
                    let jitter = rng.gen_range(5..20);
                    let backoff = (2u64.pow(new_retry as u32) * 60) + jitter;
                    let next_time = Utc::now().timestamp() + backoff as i64;

                    let _ = conn.execute(
                        r#"UPDATE content_schedules
                           SET status = 'queued',
                               retry_count = ?1,
                               scheduled_at = ?2,
                               last_error = ?3,
                               updated_at = ?4
                           WHERE id = ?5"#,
                        params![
                            new_retry,
                            next_time,
                            err_msg,
                            Utc::now().timestamp(),
                            job.id
                        ],
                    );
                    eprintln!(
                        "[CAStudio Cron] Job {} failed, scheduled retry {} in {}s.",
                        job.id, new_retry, backoff
                    );
                } else {
                    let _ = conn.execute(
                        r#"UPDATE content_schedules
                           SET status = 'failed',
                               retry_count = ?1,
                               last_error = ?2,
                               updated_at = ?3
                           WHERE id = ?4"#,
                        params![new_retry, err_msg, Utc::now().timestamp(), job.id],
                    );
                    eprintln!(
                        "[CAStudio Cron] Job {} permanently failed after {} retries.",
                        job.id, new_retry
                    );
                }
            }
        }
    }

    Ok(count)
}

/// Creates a new scheduled item in DB
pub fn create_schedule(input: CreateScheduleInput) -> Result<ContentSchedule, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let id = format!("sch_{}", Uuid::now_v7());
    let token = format!("trk_{}", Uuid::new_v4().simple());
    let now = Utc::now().timestamp();
    let config = input.channel_config_json.unwrap_or_else(|| "{}".to_string());

    conn.execute(
        r#"INSERT INTO content_schedules (
            id, campaign_id, asset_id, channel, target_platform,
            scheduled_at, status, retry_count, max_retries, last_error,
            published_url, tracking_token, channel_config_json, created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'queued', 0, 3, NULL, NULL, ?7, ?8, ?9, ?10)"#,
        params![
            id,
            input.campaign_id,
            input.asset_id,
            input.channel,
            input.target_platform,
            input.scheduled_at,
            token,
            config,
            now,
            now,
        ],
    )?;

    drop(conn);
    get_schedule(&id)
}

/// Retrieves single schedule by ID
pub fn get_schedule(id: &str) -> Result<ContentSchedule, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    conn.query_row(
        r#"SELECT id, campaign_id, asset_id, channel, target_platform,
                  scheduled_at, status, retry_count, max_retries, last_error,
                  published_url, tracking_token, channel_config_json, created_at, updated_at
           FROM content_schedules WHERE id = ?1"#,
        params![id],
        |row| {
            Ok(ContentSchedule {
                id: row.get(0)?,
                campaign_id: row.get(1)?,
                asset_id: row.get(2)?,
                channel: row.get(3)?,
                target_platform: row.get(4)?,
                scheduled_at: row.get(5)?,
                status: row.get(6)?,
                retry_count: row.get(7)?,
                max_retries: row.get(8)?,
                last_error: row.get(9)?,
                published_url: row.get(10)?,
                tracking_token: row.get(11)?,
                channel_config_json: row.get(12)?,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            })
        },
    )
    .map_err(|e| StudioError::Db(e.to_string()))
}

/// Lists all schedules, optionally filtered by campaign_id or status
pub fn list_schedules(
    campaign_id: Option<&str>,
    status_filter: Option<&str>,
) -> Result<Vec<ContentSchedule>, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let mut query = "SELECT id, campaign_id, asset_id, channel, target_platform,
                            scheduled_at, status, retry_count, max_retries, last_error,
                            published_url, tracking_token, channel_config_json, created_at, updated_at
                     FROM content_schedules WHERE 1=1".to_string();

    if campaign_id.is_some() {
        query.push_str(" AND campaign_id = ?1");
    }
    if status_filter.is_some() {
        let param_idx = if campaign_id.is_some() { "?2" } else { "?1" };
        query.push_str(&format!(" AND status = {param_idx}"));
    }
    query.push_str(" ORDER BY scheduled_at ASC");

    let mut stmt = conn.prepare(&query)?;

    let rows = match (campaign_id, status_filter) {
        (Some(c), Some(s)) => stmt.query_map(params![c, s], map_schedule_row)?,
        (Some(c), None) => stmt.query_map(params![c], map_schedule_row)?,
        (None, Some(s)) => stmt.query_map(params![s], map_schedule_row)?,
        (None, None) => stmt.query_map([], map_schedule_row)?,
    };

    let mut result = Vec::new();
    for r in rows {
        result.push(r?);
    }
    Ok(result)
}

fn map_schedule_row(row: &rusqlite::Row) -> rusqlite::Result<ContentSchedule> {
    Ok(ContentSchedule {
        id: row.get(0)?,
        campaign_id: row.get(1)?,
        asset_id: row.get(2)?,
        channel: row.get(3)?,
        target_platform: row.get(4)?,
        scheduled_at: row.get(5)?,
        status: row.get(6)?,
        retry_count: row.get(7)?,
        max_retries: row.get(8)?,
        last_error: row.get(9)?,
        published_url: row.get(10)?,
        tracking_token: row.get(11)?,
        channel_config_json: row.get(12)?,
        created_at: row.get(13)?,
        updated_at: row.get(14)?,
    })
}

/// Automatically builds a drip schedule batch for all assets in a campaign
pub fn create_drip_batch(
    input: CreateDripBatchInput,
) -> Result<Vec<ContentSchedule>, StudioError> {
    let assets = {
        let pool = get_connection()?;
        let conn = pool.lock();
        let mut stmt =
            conn.prepare("SELECT id, channel FROM content_assets WHERE campaign_id = ?1")?;
        let rows = stmt.query_map(params![input.campaign_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        let mut list = Vec::new();
        for r in rows {
            list.push(r?);
        }
        list
    };

    let mut created_list = Vec::new();

    for (asset_id, channel) in assets {
        let offset = calculate_drip_offset_seconds(&input.preset, &channel);
        let scheduled_time = input.start_time + offset;

        // Map default platform if appropriate for channel
        let platform = match channel.as_str() {
            "blog" => "wordpress",
            "tweets" => "n8n_webhook",
            "shorts" => "n8n_webhook",
            "carousel" => "n8n_webhook",
            "newsletter" => "telegram",
            _ => input.default_platform.as_str(),
        };

        let schedule = create_schedule(CreateScheduleInput {
            campaign_id: input.campaign_id.clone(),
            asset_id,
            channel: channel.clone(),
            target_platform: platform.to_string(),
            scheduled_at: scheduled_time,
            channel_config_json: Some("{}".to_string()),
        })?;

        created_list.push(schedule);
    }

    Ok(created_list)
}

/// Cancels a scheduled job
pub fn cancel_schedule(id: &str) -> Result<(), StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp();
    conn.execute(
        "UPDATE content_schedules SET status = 'cancelled', updated_at = ?1 WHERE id = ?2",
        params![now, id],
    )?;
    Ok(())
}

/// Forces instant dispatch of a scheduled job
pub async fn trigger_instant_dispatch(id: &str) -> Result<ContentSchedule, StudioError> {
    let now = Utc::now().timestamp();
    {
        let pool = get_connection()?;
        let conn = pool.lock();
        conn.execute(
            "UPDATE content_schedules SET scheduled_at = ?1, status = 'queued', updated_at = ?1 WHERE id = ?2",
            params![now - 1, id],
        )?;
    }

    poll_and_dispatch_due_jobs().await?;

    get_schedule(id)
}

/// Channel Configuration Management
pub fn save_publishing_channel(
    channel: PublishingChannel,
) -> Result<PublishingChannel, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp();

    conn.execute(
        r#"INSERT INTO publishing_channels (
            id, name, channel_type, is_active, endpoint_url,
            credentials_json, config_json, created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            channel_type = excluded.channel_type,
            is_active = excluded.is_active,
            endpoint_url = excluded.endpoint_url,
            credentials_json = excluded.credentials_json,
            config_json = excluded.config_json,
            updated_at = excluded.updated_at"#,
        params![
            channel.id,
            channel.name,
            channel.channel_type,
            if channel.is_active { 1 } else { 0 },
            channel.endpoint_url,
            channel.credentials_json,
            channel.config_json,
            now,
            now,
        ],
    )?;

    Ok(channel)
}

pub fn list_publishing_channels() -> Result<Vec<PublishingChannel>, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let mut stmt = conn.prepare(
        r#"SELECT id, name, channel_type, is_active, endpoint_url, credentials_json, config_json, created_at, updated_at
           FROM publishing_channels ORDER BY name ASC"#,
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(PublishingChannel {
            id: row.get(0)?,
            name: row.get(1)?,
            channel_type: row.get(2)?,
            is_active: row.get::<_, i32>(3)? != 0,
            endpoint_url: row.get(4)?,
            credentials_json: row.get(5)?,
            config_json: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    })?;

    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drip_cadence_calculation() {
        assert_eq!(calculate_drip_offset_seconds("balanced", "blog"), 0);
        assert_eq!(
            calculate_drip_offset_seconds("balanced", "shorts"),
            27 * 3600
        );
        assert_eq!(
            calculate_drip_offset_seconds("balanced", "carousel"),
            56 * 3600
        );
        assert_eq!(
            calculate_drip_offset_seconds("balanced", "newsletter"),
            71 * 3600
        );

        assert_eq!(calculate_drip_offset_seconds("aggressive", "blog"), 0);
        assert_eq!(
            calculate_drip_offset_seconds("aggressive", "newsletter"),
            47 * 3600
        );
    }
}
