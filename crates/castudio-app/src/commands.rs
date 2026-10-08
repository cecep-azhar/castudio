use crate::window;
use castudio_core::ai::{generate_completion, GenerateAiRequest, GenerateAiResponse};
use castudio_core::document::{
    create_document, delete_document, get_blocks, get_document, list_documents, save_blocks,
    Block, CreateDocumentInput, Document, UpdateDocumentInput,
};
use castudio_core::error::StudioError;
use castudio_core::export::{
    export_epub, export_html, export_markdown, export_project_json,
};
use castudio_core::prefs::{get_ai_settings, save_ai_settings, AiSettings};
use castudio_core::project::{
    create_project, delete_project, get_bible, get_project, list_projects, save_bible,
    update_project, CreateProjectInput, Project, ProjectBible, UpdateProjectInput,
};
use castudio_core::prompt_studio::{
    create_prompt, delete_prompt, list_prompts, toggle_favorite, CreatePromptInput, PromptTemplate,
};
use castudio_core::templates::{
    generate_ebook_outline as tmpl_ebook,
    generate_instagram_carousel as tmpl_carousel,
    generate_pitchdeck as tmpl_pitchdeck,
    generate_reels_script as tmpl_reels,
};
use tauri::Window;

// Window Controls
#[tauri::command]
pub fn window_minimize(window: Window) {
    window::minimize(&window);
}

#[tauri::command]
pub fn window_maximize(window: Window) {
    window::maximize_or_unmaximize(&window);
}

#[tauri::command]
pub fn window_close(window: Window) {
    window::close(&window);
}

#[tauri::command]
pub fn window_start_dragging(window: Window) {
    window::start_dragging(&window);
}

// Projects
#[tauri::command]
pub fn cmd_list_projects() -> Result<Vec<Project>, StudioError> {
    list_projects()
}

#[tauri::command]
pub fn cmd_create_project(input: CreateProjectInput) -> Result<Project, StudioError> {
    create_project(input)
}

#[tauri::command]
pub fn cmd_get_project(id: String) -> Result<Project, StudioError> {
    get_project(&id)
}

#[tauri::command]
pub fn cmd_update_project(id: String, input: UpdateProjectInput) -> Result<Project, StudioError> {
    update_project(&id, input)
}

#[tauri::command]
pub fn cmd_delete_project(id: String) -> Result<(), StudioError> {
    delete_project(&id)
}

#[tauri::command]
pub fn cmd_get_bible(project_id: String) -> Result<ProjectBible, StudioError> {
    get_bible(&project_id)
}

#[tauri::command]
pub fn cmd_save_bible(bible: ProjectBible) -> Result<ProjectBible, StudioError> {
    save_bible(bible)
}

// Documents & Blocks
#[tauri::command]
pub fn cmd_list_documents(project_id: String) -> Result<Vec<Document>, StudioError> {
    list_documents(&project_id)
}

#[tauri::command]
pub fn cmd_create_document(input: CreateDocumentInput) -> Result<Document, StudioError> {
    create_document(input)
}

#[tauri::command]
pub fn cmd_get_document(id: String) -> Result<Document, StudioError> {
    get_document(&id)
}

#[tauri::command]
pub fn cmd_update_document(id: String, input: UpdateDocumentInput) -> Result<Document, StudioError> {
    castudio_core::document::update_document(&id, input)
}

#[tauri::command]
pub fn cmd_get_blocks(document_id: String) -> Result<Vec<Block>, StudioError> {
    get_blocks(&document_id)
}

#[tauri::command]
pub fn cmd_save_blocks(document_id: String, blocks: Vec<Block>) -> Result<Vec<Block>, StudioError> {
    save_blocks(&document_id, blocks)
}

#[tauri::command]
pub fn cmd_delete_document(id: String) -> Result<(), StudioError> {
    delete_document(&id)
}

// AI Completions
#[tauri::command]
pub async fn cmd_generate_ai(req: GenerateAiRequest) -> Result<GenerateAiResponse, StudioError> {
    tokio::task::spawn_blocking(move || generate_completion(req))
        .await
        .map_err(|e| StudioError::Ai(e.to_string()))?
}

#[tauri::command]
pub async fn cmd_generate_ebook_outline(
    project_id: Option<String>,
    topic: String,
    chapters_count: usize,
) -> Result<String, StudioError> {
    tokio::task::spawn_blocking(move || tmpl_ebook(project_id, &topic, chapters_count))
        .await
        .map_err(|e| StudioError::Ai(e.to_string()))?
}

#[tauri::command]
pub async fn cmd_generate_carousel(
    project_id: Option<String>,
    topic: String,
    slides_count: usize,
) -> Result<String, StudioError> {
    tokio::task::spawn_blocking(move || tmpl_carousel(project_id, &topic, slides_count))
        .await
        .map_err(|e| StudioError::Ai(e.to_string()))?
}

#[tauri::command]
pub async fn cmd_generate_reels(
    project_id: Option<String>,
    topic: String,
) -> Result<String, StudioError> {
    tokio::task::spawn_blocking(move || tmpl_reels(project_id, &topic))
        .await
        .map_err(|e| StudioError::Ai(e.to_string()))?
}

#[tauri::command]
pub async fn cmd_generate_pitchdeck(
    project_id: Option<String>,
    company_name: String,
    context: String,
) -> Result<String, StudioError> {
    tokio::task::spawn_blocking(move || tmpl_pitchdeck(project_id, &company_name, &context))
        .await
        .map_err(|e| StudioError::Ai(e.to_string()))?
}

// Exports
#[tauri::command]
pub fn cmd_export_markdown(document_id: String) -> Result<String, StudioError> {
    let p = export_markdown(&document_id)?;
    Ok(p.to_string_lossy().to_string())
}

#[tauri::command]
pub fn cmd_export_html(document_id: String) -> Result<String, StudioError> {
    let p = export_html(&document_id)?;
    Ok(p.to_string_lossy().to_string())
}

#[tauri::command]
pub fn cmd_export_epub(document_id: String) -> Result<String, StudioError> {
    let p = export_epub(&document_id)?;
    Ok(p.to_string_lossy().to_string())
}

#[tauri::command]
pub fn cmd_export_project(project_id: String) -> Result<String, StudioError> {
    let p = export_project_json(&project_id)?;
    Ok(p.to_string_lossy().to_string())
}

// Prompts
#[tauri::command]
pub fn cmd_list_prompts(category: Option<String>) -> Result<Vec<PromptTemplate>, StudioError> {
    list_prompts(category.as_deref())
}

#[tauri::command]
pub fn cmd_create_prompt(input: CreatePromptInput) -> Result<PromptTemplate, StudioError> {
    create_prompt(input)
}

#[tauri::command]
pub fn cmd_toggle_prompt_favorite(id: String) -> Result<bool, StudioError> {
    toggle_favorite(&id)
}

#[tauri::command]
pub fn cmd_delete_prompt(id: String) -> Result<(), StudioError> {
    delete_prompt(&id)
}

// Settings
#[tauri::command]
pub fn cmd_get_ai_settings() -> Result<AiSettings, StudioError> {
    get_ai_settings()
}

#[tauri::command]
pub fn cmd_save_ai_settings(settings: AiSettings) -> Result<(), StudioError> {
    save_ai_settings(settings)
}

// Omni-Channel Repurposing Engine
#[tauri::command]
pub async fn cmd_fetch_youtube_content(url: String) -> Result<String, StudioError> {
    tokio::task::spawn_blocking(move || castudio_core::repurposing::fetch_youtube_content(&url))
        .await
        .map_err(|e| StudioError::Ai(e.to_string()))?
}

#[tauri::command]
pub fn cmd_create_campaign(
    input: castudio_core::repurposing::CreateCampaignInput,
) -> Result<castudio_core::repurposing::ContentCampaign, StudioError> {
    castudio_core::repurposing::create_campaign(input)
}

#[tauri::command]
pub fn cmd_get_campaign(
    campaign_id: String,
) -> Result<castudio_core::repurposing::ContentCampaign, StudioError> {
    castudio_core::repurposing::get_campaign(&campaign_id)
}

#[tauri::command]
pub fn cmd_list_campaigns(
    project_id: Option<String>,
) -> Result<Vec<castudio_core::repurposing::ContentCampaign>, StudioError> {
    castudio_core::repurposing::list_campaigns(project_id.as_deref())
}

#[tauri::command]
pub async fn cmd_run_repurpose(
    input: castudio_core::repurposing::RunRepurposeInput,
) -> Result<castudio_core::repurposing::ContentCampaign, StudioError> {
    tokio::task::spawn_blocking(move || castudio_core::repurposing::run_repurposing_engine(input))
        .await
        .map_err(|e| StudioError::Ai(e.to_string()))?
}

#[tauri::command]
pub fn cmd_update_asset(
    asset_id: String,
    content: String,
) -> Result<castudio_core::repurposing::ContentAsset, StudioError> {
    castudio_core::repurposing::update_asset(&asset_id, &content)
}

#[tauri::command]
pub fn cmd_set_asset_status(
    asset_id: String,
    status: String,
) -> Result<castudio_core::repurposing::ContentAsset, StudioError> {
    castudio_core::repurposing::set_asset_status(&asset_id, &status)
}

#[tauri::command]
pub fn cmd_queue_campaign(
    campaign_id: String,
) -> Result<castudio_core::repurposing::ContentCampaign, StudioError> {
    castudio_core::repurposing::queue_entire_campaign(&campaign_id)
}

#[tauri::command]
pub fn cmd_delete_campaign(campaign_id: String) -> Result<(), StudioError> {
    castudio_core::repurposing::delete_campaign(&campaign_id)
}

// Automation, Drip Scheduling & Dispatcher Commands
#[tauri::command]
pub fn cmd_create_schedule(
    input: castudio_core::automation::CreateScheduleInput,
) -> Result<castudio_core::automation::ContentSchedule, StudioError> {
    castudio_core::automation::create_schedule(input)
}

#[tauri::command]
pub fn cmd_create_drip_batch(
    input: castudio_core::automation::CreateDripBatchInput,
) -> Result<Vec<castudio_core::automation::ContentSchedule>, StudioError> {
    castudio_core::automation::create_drip_batch(input)
}

#[tauri::command]
pub fn cmd_list_schedules(
    campaign_id: Option<String>,
    status_filter: Option<String>,
) -> Result<Vec<castudio_core::automation::ContentSchedule>, StudioError> {
    castudio_core::automation::list_schedules(campaign_id.as_deref(), status_filter.as_deref())
}

#[tauri::command]
pub fn cmd_cancel_schedule(id: String) -> Result<(), StudioError> {
    castudio_core::automation::cancel_schedule(&id)
}

#[tauri::command]
pub async fn cmd_trigger_instant_dispatch(
    id: String,
) -> Result<castudio_core::automation::ContentSchedule, StudioError> {
    castudio_core::automation::trigger_instant_dispatch(&id).await
}

#[tauri::command]
pub fn cmd_save_publishing_channel(
    channel: castudio_core::automation::PublishingChannel,
) -> Result<castudio_core::automation::PublishingChannel, StudioError> {
    castudio_core::automation::save_publishing_channel(channel)
}

#[tauri::command]
pub fn cmd_list_publishing_channels(
) -> Result<Vec<castudio_core::automation::PublishingChannel>, StudioError> {
    castudio_core::automation::list_publishing_channels()
}

#[tauri::command]
pub fn cmd_test_channel_connection(
    target_platform: String,
    endpoint_url: String,
    credentials_json: String,
    config_json: String,
) -> Result<castudio_core::automation::PublishResult, StudioError> {
    castudio_core::automation::dispatch_to_platform(
        &target_platform,
        &endpoint_url,
        &credentials_json,
        &config_json,
        "Test Ping dari CAStudio Automation",
        "Koneksi teruji sukses dari modul CAStudio Omni-Channel Content Dispatcher.",
        "test_cmp",
        "test_ch",
        chrono::Utc::now().timestamp(),
        "trk_test_ping",
    )
}


