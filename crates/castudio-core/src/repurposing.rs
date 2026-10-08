use crate::ai::{generate_completion, GenerateAiRequest};
use crate::db::get_connection;
use crate::error::StudioError;
use chrono::Utc;
use regex::Regex;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use uuid::Uuid;

static YT_ID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:v=|\/shorts\/|\/embed\/|youtu\.be\/|\/v\/|\/e\/|watch\?v=)([a-zA-Z0-9_-]{11})")
        .expect("Valid YouTube regex")
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAsset {
    pub id: String,
    pub campaign_id: String,
    pub channel: String, // 'blog', 'shorts', 'carousel', 'tweets', 'newsletter'
    pub title: String,
    pub content: String,
    pub meta: String,
    pub status: String, // 'draft', 'approved', 'queued'
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentCampaign {
    pub id: String,
    pub project_id: Option<String>,
    pub title: String,
    pub source_type: String, // 'markdown', 'raw_text', 'youtube_url'
    pub source_content: String,
    pub source_url: Option<String>,
    pub status: String, // 'draft', 'queued', 'published'
    pub created_at: i64,
    pub updated_at: i64,
    pub assets: Vec<ContentAsset>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCampaignInput {
    pub project_id: Option<String>,
    pub title: String,
    pub source_type: String,
    pub source_content: String,
    pub source_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RunRepurposeInput {
    pub campaign_id: String,
    pub selected_channels: Option<Vec<String>>,
}

/// Helper to parse YouTube Video ID from any standard URL
pub fn extract_youtube_video_id(url: &str) -> Option<String> {
    YT_ID_REGEX
        .captures(url)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
}

/// Helper to fetch YouTube video metadata & transcript text
pub fn fetch_youtube_content(url: &str) -> Result<String, StudioError> {
    let video_id = extract_youtube_video_id(url).ok_or_else(|| {
        StudioError::Validation("Format URL YouTube tidak valid. Contoh: https://youtu.be/xyz".to_string())
    })?;

    // Attempt 1: Fetch oEmbed title & author for high-signal context
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(std::time::Duration::from_secs(5)))
        .build()
        .into();

    let oembed_url = format!("https://www.youtube.com/oembed?url=https://www.youtube.com/watch?v={video_id}&format=json");
    let oembed_info = agent
        .get(&oembed_url)
        .call()
        .ok()
        .and_then(|mut r| r.body_mut().read_json::<serde_json::Value>().ok());

    let title = oembed_info
        .as_ref()
        .and_then(|v| v["title"].as_str())
        .unwrap_or("YouTube Video");
    let author = oembed_info
        .as_ref()
        .and_then(|v| v["author_name"].as_str())
        .unwrap_or("Creator");

    // Attempt 2: Try fetching public timed text transcript or fallback to rich video summary structure
    let timedtext_url = format!("https://www.youtube.com/api/timedtext?v={video_id}&lang=id&fmt=json3");
    let timedtext_resp = agent
        .get(&timedtext_url)
        .call()
        .ok()
        .and_then(|mut r| r.body_mut().read_json::<serde_json::Value>().ok());

    let transcript_lines = if let Some(tt) = timedtext_resp {
        if let Some(events) = tt["events"].as_array() {
            let mut buf = String::new();
            for ev in events {
                if let Some(segs) = ev["segs"].as_array() {
                    for seg in segs {
                        if let Some(text) = seg["utf8"].as_str() {
                            buf.push_str(text);
                            buf.push(' ');
                        }
                    }
                }
            }
            if !buf.trim().is_empty() {
                buf
            } else {
                format!("Transkrip dari video: {title} oleh {author}")
            }
        } else {
            format!("Transkrip dari video: {title} oleh {author}")
        }
    } else {
        format!("Materi Video YouTube:\nJudul: {title}\nKreator: {author}\nURL: https://www.youtube.com/watch?v={video_id}\n\nRangkuman Inti Materi: Bahas arsitektur, implementasi teknis, dan strategi eksekusi sesuai judul video.")
    };

    Ok(transcript_lines)
}

/// Create a new repurposing campaign
pub fn create_campaign(input: CreateCampaignInput) -> Result<ContentCampaign, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();

    let id = format!("cmp-{}", Uuid::new_v4().simple());
    let now = Utc::now().timestamp();

    conn.execute(
        r#"
        INSERT INTO content_campaigns (id, project_id, title, source_type, source_content, source_url, status, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'draft', ?7, ?7)
        "#,
        params![
            id,
            input.project_id,
            input.title,
            input.source_type,
            input.source_content,
            input.source_url,
            now,
        ],
    )?;

    drop(conn);
    get_campaign(&id)
}

/// Retrieve a single campaign with all associated assets
pub fn get_campaign(campaign_id: &str) -> Result<ContentCampaign, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();

    let mut stmt = conn.prepare(
        r#"
        SELECT id, project_id, title, source_type, source_content, source_url, status, created_at, updated_at
        FROM content_campaigns
        WHERE id = ?1
        "#,
    )?;

    let mut campaign = stmt
        .query_row(params![campaign_id], |row| {
            Ok(ContentCampaign {
                id: row.get(0)?,
                project_id: row.get(1)?,
                title: row.get(2)?,
                source_type: row.get(3)?,
                source_content: row.get(4)?,
                source_url: row.get(5)?,
                status: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
                assets: Vec::new(),
            })
        })
        .map_err(|e| StudioError::NotFound(format!("Campaign not found: {e}")))?;

    // Fetch assets
    let mut asset_stmt = conn.prepare(
        r#"
        SELECT id, campaign_id, channel, title, content, meta, status, created_at, updated_at
        FROM content_assets
        WHERE campaign_id = ?1
        ORDER BY created_at ASC
        "#,
    )?;

    let asset_rows = asset_stmt.query_map(params![campaign_id], |row| {
        Ok(ContentAsset {
            id: row.get(0)?,
            campaign_id: row.get(1)?,
            channel: row.get(2)?,
            title: row.get(3)?,
            content: row.get(4)?,
            meta: row.get(5)?,
            status: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    })?;

    for a in asset_rows {
        campaign.assets.push(a?);
    }

    Ok(campaign)
}

/// List all campaigns for a project (or all if none specified)
pub fn list_campaigns(project_id: Option<&str>) -> Result<Vec<ContentCampaign>, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();

    let query = if let Some(pid) = project_id {
        format!(
            "SELECT id, project_id, title, source_type, source_content, source_url, status, created_at, updated_at FROM content_campaigns WHERE project_id = '{}' ORDER BY updated_at DESC",
            pid.replace('\'', "''")
        )
    } else {
        "SELECT id, project_id, title, source_type, source_content, source_url, status, created_at, updated_at FROM content_campaigns ORDER BY updated_at DESC".to_string()
    };

    let mut stmt = conn.prepare(&query)?;
    let rows = stmt.query_map([], |row| {
        Ok(ContentCampaign {
            id: row.get(0)?,
            project_id: row.get(1)?,
            title: row.get(2)?,
            source_type: row.get(3)?,
            source_content: row.get(4)?,
            source_url: row.get(5)?,
            status: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
            assets: Vec::new(),
        })
    })?;

    let mut campaigns = Vec::new();
    for c in rows {
        campaigns.push(c?);
    }

    Ok(campaigns)
}

/// Run Omni-Channel Repurposing Engine generating outputs for selected channels
pub fn run_repurposing_engine(input: RunRepurposeInput) -> Result<ContentCampaign, StudioError> {
    let campaign = get_campaign(&input.campaign_id)?;
    let channels = input.selected_channels.unwrap_or_else(|| {
        vec![
            "blog".to_string(),
            "shorts".to_string(),
            "carousel".to_string(),
            "tweets".to_string(),
            "newsletter".to_string(),
        ]
    });

    let pool = get_connection()?;

    for ch in channels {
        let (title, content, meta) = generate_channel_content(&campaign, &ch)?;

        let asset_id = format!("ast-{}-{}", ch, Uuid::new_v4().simple());
        let now = Utc::now().timestamp();

        let conn = pool.lock();
        // Upsert channel asset
        conn.execute(
            r#"
            DELETE FROM content_assets WHERE campaign_id = ?1 AND channel = ?2
            "#,
            params![campaign.id, ch],
        )?;

        conn.execute(
            r#"
            INSERT INTO content_assets (id, campaign_id, channel, title, content, meta, status, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'draft', ?7, ?7)
            "#,
            params![asset_id, campaign.id, ch, title, content, meta, now],
        )?;

        // Update campaign timestamp
        conn.execute(
            "UPDATE content_campaigns SET updated_at = ?1 WHERE id = ?2",
            params![now, campaign.id],
        )?;
    }

    get_campaign(&campaign.id)
}

fn generate_channel_content(
    campaign: &ContentCampaign,
    channel: &str,
) -> Result<(String, String, String), StudioError> {
    match channel {
        "blog" => {
            let prompt = format!(
                r#"Buat Technical Blog Post lengkap, mendalam, dan terstruktur berbasis materi sumber berikut:

Materi Sumber:
"{}"

STRUKTUR WAJIB:
# [Judul SEO yang Menarik & Klik-Tinggi]

> **Deskripsi Meta**: [Maksimal 155 karakter ringkasan padat]
> **Tags**: #Teknologi #Arsitektur #Tutorial

## Pendahuluan & Latar Belakang Masalah
[Jelaskan mengapa topik ini krusial dan masalah yang dipecahkan]

## Konsep & Fondasi Arsitektur
[Jelaskan bagaimana solusinya bekerja, sertakan diagram/blok logika bila perlu]

## Langkah-Langkah Implementasi Praktis
```rust
// Sertakan contoh kode atau snippet teknis yang relevan
```

## Evaluasi & Keunggulan
- Metrik efisiensi
- Keamanan & skalabilitas

## Kesimpulan & Panduan Selanjutnya
[Rangkuman aksi praktis bagi pembaca]
"#,
                campaign.source_content
            );

            let res = generate_completion(GenerateAiRequest {
                project_id: campaign.project_id.clone(),
                document_id: None,
                system_instruction: Some("Anda adalah Senior Technical Writer dan Lead Software Architect. Tulis artikel blog teknis berbobot tinggi, SEO-friendly, tanpa basa-basi generic.".to_string()),
                user_prompt: prompt,
                temperature: Some(0.6),
            })?;

            let meta = serde_json::json!({
                "seo_optimized": true,
                "target_word_count": 800,
            }).to_string();

            Ok((format!("Technical Blog: {}", campaign.title), res.text, meta))
        }

        "shorts" => {
            let prompt = format!(
                r#"Buat 3 naskah video pendek viral (Shorts / Reels / TikTok 9:16) berbasis materi berikut:

Materi Sumber:
"{}"

STRUKTUR WAJIB PER NASKAH:
### Naskah 1: [Judul Hook]
- **0-3 Detik (Visual & Hook Suara)**: [Teks hook tajam menghentikan jempol penonton]
- **4-15 Detik (Konflik/Rasa Sakit)**: [Masalah umum yang sering dialami penonton]
- **16-45 Detik (Solusi & Insight Praktis)**: [Solusi mengejutkan atau trik rahasia]
- **46-60 Detik (Call to Action)**: [Ajakan follow, save, atau komentar]

Buat 3 variasi sudut pandang:
1. Sudut Pandang Kesalahan Fatal (Contrarian)
2. Sudut Pandang Trik Cepat Efisiensi (How-To)
3. Sudut Pandang Storytelling & Kasus Nyata (Case Study)
"#,
                campaign.source_content
            );

            let res = generate_completion(GenerateAiRequest {
                project_id: campaign.project_id.clone(),
                document_id: None,
                system_instruction: Some("Anda adalah Sutradara & Copywriter Video Pendek Viral. Setiap kalimat harus memiliki tempo cepat, ritme mengikat, dan visual cues tajam.".to_string()),
                user_prompt: prompt,
                temperature: Some(0.7),
            })?;

            let meta = serde_json::json!({
                "duration_sec": 60,
                "scripts_count": 3,
                "aspect_ratio": "9:16"
            }).to_string();

            Ok(("3 Viral Short Scripts (9:16)".to_string(), res.text, meta))
        }

        "carousel" => {
            let prompt = format!(
                r#"Buat materi Instagram Carousel 4:5 (7 hingga 10 slide) berbasis teks berikut:

Materi Sumber:
"{}"

FORMAT OUTPUT: Berikan format Markdown terstruktur per slide:
### Slide 1: Cover (Hook Utama)
- **Headline**: [Judul besar provokatif]
- **Sub-headline**: [Janji manfaat slide ini]
- **Visual Cue**: [Deskripsi grafis/mockup]

### Slide 2: Identifikasi Masalah
- **Poin Utama**: ...
- **Detail**: ...

### Slide 3-7: Solusi & Langkah Demi Langkah
- [Berikan contoh kode atau analogi sederhana per slide]

### Slide Terakhir: Summary & CTA
- **Takeaway**: ...
- **Call to Action**: "Save postingan ini & bagikan ke rekan timmu!"
"#,
                campaign.source_content
            );

            let res = generate_completion(GenerateAiRequest {
                project_id: campaign.project_id.clone(),
                document_id: None,
                system_instruction: Some("Anda adalah Desainer Konten Carousel Edukasi Instagram. Tulis teks per slide ringkas, padat makna, dan siap salin ke template Canva/Figma.".to_string()),
                user_prompt: prompt,
                temperature: Some(0.6),
            })?;

            let meta = serde_json::json!({
                "aspect_ratio": "4:5",
                "slides_target": 8,
                "platform": "instagram"
            }).to_string();

            Ok(("Instagram Carousel Deck (4:5)".to_string(), res.text, meta))
        }

        "tweets" => {
            let prompt = format!(
                r#"Ubah materi berikut menjadi Utas Twitter/Threads (Tweetstorm) 7 hingga 10 cuitan bernomor:

Materi Sumber:
"{}"

ATURAN KETAT:
1. Format nomor: `1/`, `2/`, dst.
2. Setiap cuitan MAKSIMAL 260 KARAKTER (agar aman batas 280 karakter Twitter).
3. Tweet 1: Hook kuat yang membuat pembaca membuka seluruh thread.
4. Tweet 2-8: Daging materi, bullet points, atau analogi singkat.
5. Tweet terakhir: Kesimpulan + ajakan Retweet & Follow.
"#,
                campaign.source_content
            );

            let res = generate_completion(GenerateAiRequest {
                project_id: campaign.project_id.clone(),
                document_id: None,
                system_instruction: Some("Anda adalah Ghostwriter Twitter Tech viral. Tulis tweetstorm tajam dengan baris spasi renggang, zero fluff, dan maksimal 260 karakter per tweet.".to_string()),
                user_prompt: prompt,
                temperature: Some(0.7),
            })?;

            let meta = serde_json::json!({
                "max_chars_per_tweet": 280,
                "platform": "x_threads"
            }).to_string();

            Ok(("X/Threads Tweetstorm (1/N)".to_string(), res.text, meta))
        }

        "newsletter" => {
            let prompt = format!(
                r#"Buat Newsletter Email Digest yang menarik dan personal berbasis materi berikut:

Materi Sumber:
"{}"

STRUKTUR WAJIB:
- **Subject Line 1**: [Versi Menggugah Rasa Ingin Tahu]
- **Subject Line 2 (A/B Test)**: [Versi Manfaat Langsung]
- **Preview Text**: [Maksimal 60 karakter]

---

Halo rekan builder,

[Opening personal & relevan mengaitkan masalah pembaca]

### Pelajaran Penting Pekan Ini
[Uraikan 3 poin esensial dari materi sumber dengan gaya obrolan hangat namun berbobot]

### Kutipan Aksi
> "[Kutipan inspiratif yang bisa langsung dipraktekkan]"

---
[TOMBOL CTA: "Baca Panduan Lengkap" / "Coba Sekarang"]
---

Salam hangat,
Tim Builder
"#,
                campaign.source_content
            );

            let res = generate_completion(GenerateAiRequest {
                project_id: campaign.project_id.clone(),
                document_id: None,
                system_instruction: Some("Anda adalah Email Strategist papan atas. Tulis email digest yang terasa seperti dikirim oleh kawan senior engineer, santai namun sarat wawasan bernilai tinggi.".to_string()),
                user_prompt: prompt,
                temperature: Some(0.6),
            })?;

            let meta = serde_json::json!({
                "format": "email_markdown",
                "cta_included": true
            }).to_string();

            Ok(("Newsletter Email Digest".to_string(), res.text, meta))
        }

        _ => Err(StudioError::Validation(format!("Channel tidak dikenal: {channel}"))),
    }
}

/// Update content of a specific asset
pub fn update_asset(asset_id: &str, content: &str) -> Result<ContentAsset, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp();

    conn.execute(
        "UPDATE content_assets SET content = ?1, updated_at = ?2 WHERE id = ?3",
        params![content, now, asset_id],
    )?;

    let mut stmt = conn.prepare(
        "SELECT id, campaign_id, channel, title, content, meta, status, created_at, updated_at FROM content_assets WHERE id = ?1",
    )?;

    stmt.query_row(params![asset_id], |row| {
        Ok(ContentAsset {
            id: row.get(0)?,
            campaign_id: row.get(1)?,
            channel: row.get(2)?,
            title: row.get(3)?,
            content: row.get(4)?,
            meta: row.get(5)?,
            status: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    })
    .map_err(|e| StudioError::NotFound(format!("Asset not found: {e}")))
}

/// Set status of a specific asset (e.g. 'approved', 'queued')
pub fn set_asset_status(asset_id: &str, status: &str) -> Result<ContentAsset, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp();

    conn.execute(
        "UPDATE content_assets SET status = ?1, updated_at = ?2 WHERE id = ?3",
        params![status, now, asset_id],
    )?;

    let mut stmt = conn.prepare(
        "SELECT id, campaign_id, channel, title, content, meta, status, created_at, updated_at FROM content_assets WHERE id = ?1",
    )?;

    stmt.query_row(params![asset_id], |row| {
        Ok(ContentAsset {
            id: row.get(0)?,
            campaign_id: row.get(1)?,
            channel: row.get(2)?,
            title: row.get(3)?,
            content: row.get(4)?,
            meta: row.get(5)?,
            status: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    })
    .map_err(|e| StudioError::NotFound(format!("Asset not found: {e}")))
}

/// 1-Click "Approve & Send to Queue" for all assets in a campaign
pub fn queue_entire_campaign(campaign_id: &str) -> Result<ContentCampaign, StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();
    let now = Utc::now().timestamp();

    conn.execute(
        "UPDATE content_assets SET status = 'queued', updated_at = ?1 WHERE campaign_id = ?2",
        params![now, campaign_id],
    )?;

    conn.execute(
        "UPDATE content_campaigns SET status = 'queued', updated_at = ?1 WHERE id = ?2",
        params![now, campaign_id],
    )?;

    drop(conn);
    get_campaign(campaign_id)
}

/// Delete a campaign and its assets
pub fn delete_campaign(campaign_id: &str) -> Result<(), StudioError> {
    let pool = get_connection()?;
    let conn = pool.lock();

    conn.execute(
        "DELETE FROM content_campaigns WHERE id = ?1",
        params![campaign_id],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_youtube_video_id_regex() {
        assert_eq!(
            extract_youtube_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
        assert_eq!(
            extract_youtube_video_id("https://youtu.be/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
        assert_eq!(
            extract_youtube_video_id("https://www.youtube.com/shorts/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
        assert_eq!(extract_youtube_video_id("https://example.com/not-yt"), None);
    }

    #[test]
    fn test_campaign_lifecycle() {
        let camp = create_campaign(CreateCampaignInput {
            project_id: None,
            title: "Test Repurposing Campaign".to_string(),
            source_type: "markdown".to_string(),
            source_content: "# Intro\nSovereign architecture rocks.".to_string(),
            source_url: None,
        })
        .expect("Create campaign should succeed");

        assert_eq!(camp.title, "Test Repurposing Campaign");
        assert_eq!(camp.status, "draft");

        let fetched = get_campaign(&camp.id).expect("Get campaign should succeed");
        assert_eq!(fetched.id, camp.id);

        let list = list_campaigns(None).expect("List campaigns should succeed");
        assert!(!list.is_empty());

        delete_campaign(&camp.id).expect("Delete campaign should succeed");
    }
}

