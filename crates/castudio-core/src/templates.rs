use crate::ai::{generate_completion, GenerateAiRequest};
use crate::error::StudioError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarouselSlide {
    pub slide_number: usize,
    pub slide_type: String, // "hook", "point", "summary", "cta"
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReelsScene {
    pub timestamp_sec: u32,
    pub visual_cue: String,
    pub voiceover_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepurposePackResult {
    pub carousel_slides: Vec<CarouselSlide>,
    pub thread_posts: Vec<String>,
    pub reels_script: Vec<ReelsScene>,
    pub email_sequence: String,
}

pub fn generate_ebook_outline(
    project_id: Option<String>,
    topic: &str,
    chapters_count: usize,
) -> Result<String, StudioError> {
    let prompt = format!(
        "Create a comprehensive, production-ready E-Book outline for the topic: '{topic}'.\n\
         Generate exactly {chapters_count} chapters with 3-4 bullet sub-topics per chapter.\n\
         Make each chapter title compelling, actionable, and structured for developers/creators.\n\
         Language: Indonesian (Bahasa Indonesia)."
    );

    let res = generate_completion(GenerateAiRequest {
        project_id,
        document_id: None,
        system_instruction: Some(
            "You are a master technical author and book architect. Output clean markdown without fluff."
                .to_string(),
        ),
        user_prompt: prompt,
        temperature: Some(0.7),
    })?;

    Ok(res.text)
}

pub fn generate_instagram_carousel(
    project_id: Option<String>,
    topic: &str,
    slides_count: usize,
) -> Result<String, StudioError> {
    let count = slides_count.clamp(4, 10);
    let prompt = format!(
        "Create a high-retention Instagram Carousel slide series ({count} slides total) for: '{topic}'.\n\
         Format exactly as:\n\
         Slide 1 (Hook Cover): Big bold hook question/statement + subtitle\n\
         Slide 2 to {}: Deep technical/actionable insight per slide\n\
         Slide {}: Summary checklist + Call To Action (Save, Share, Follow)\n\
         Write in Bahasa Indonesia, concise, punchy.",
        count - 1, count
    );

    let res = generate_completion(GenerateAiRequest {
        project_id,
        document_id: None,
        system_instruction: Some(
            "You are an elite Instagram growth strategist for tech founders. No corporate jargon."
                .to_string(),
        ),
        user_prompt: prompt,
        temperature: Some(0.7),
    })?;

    Ok(res.text)
}

pub fn generate_reels_script(
    project_id: Option<String>,
    topic: &str,
) -> Result<String, StudioError> {
    let prompt = format!(
        "Write a 60-second high-energy vertical video script (Reels/TikTok/Shorts 9:16) for: '{topic}'.\n\
         Structure:\n\
         - 0-3s: Visual Hook (What camera shows + first sentence that stops scrolling)\n\
         - 4-15s: Agitation / The hidden problem\n\
         - 16-45s: 3 Quick solutions / practical steps\n\
         - 46-60s: Strong CTA (Komen/cek bio)\n\
         Include [VISUAL CUE] tags per scene. Language: Bahasa Indonesia."
    );

    let res = generate_completion(GenerateAiRequest {
        project_id,
        document_id: None,
        system_instruction: Some(
            "You are a viral short-form director. High pacing, conversational tone.".to_string(),
        ),
        user_prompt: prompt,
        temperature: Some(0.8),
    })?;

    Ok(res.text)
}

pub fn generate_pitchdeck(
    project_id: Option<String>,
    company_or_product: &str,
    problem_and_solution: &str,
) -> Result<String, StudioError> {
    let prompt = format!(
        "Create a 10-slide VC/Client pitchdeck outline for: '{company_or_product}'.\n\
         Context: {problem_and_solution}\n\n\
         Generate slides:\n\
         1. Cover & One-line vision\n\
         2. The Pain / Problem\n\
         3. The Solution & Value Prop\n\
         4. Product Demo & Architecture\n\
         5. Market Size & Opportunity\n\
         6. Business Model & Pricing\n\
         7. Competitive Edge (Unfair advantage)\n\
         8. Traction & Milestones\n\
         9. Team & Execution Capability\n\
         10. The Ask / Next Step.\n\
         Do not invent fake numbers. Mark missing stats with [ISI: ...].\n\
         Language: Bahasa Indonesia / English technical blend."
    );

    let res = generate_completion(GenerateAiRequest {
        project_id,
        document_id: None,
        system_instruction: Some(
            "You are a YC alumni pitch architect. Direct, evidence-driven, zero fluff.".to_string(),
        ),
        user_prompt: prompt,
        temperature: Some(0.6),
    })?;

    Ok(res.text)
}
