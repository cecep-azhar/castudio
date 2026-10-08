import { invoke } from "@tauri-apps/api/core";

export interface Project {
  id: string;
  title: string;
  description: string;
  target_audience: string;
  output_language: string;
  default_tone: string;
  created_at: number;
  updated_at: number;
}

export interface ProjectBible {
  project_id: string;
  voice_sample: string;
  core_facts: string;
  key_terms: string;
  stories: string;
  guidelines: string;
  updated_at: number;
}

export interface Document {
  id: string;
  project_id: string;
  parent_id: string | null;
  title: string;
  doc_type: string;
  status: string;
  word_count: number;
  meta: string;
  created_at: number;
  updated_at: number;
}

export interface Block {
  id: string;
  document_id: string;
  parent_id: string | null;
  block_type: string; // "heading", "paragraph", "list", "callout", "quote", "stat", "slide_break", "code"
  position: number;
  content: string;
  meta: string;
  updated_at: number;
}

export interface PromptTemplate {
  id: string;
  title: string;
  category: string;
  system_prompt: string;
  user_prompt: string;
  variables: string;
  is_favorite: boolean;
  created_at: number;
}

export interface AiSettings {
  base_url: string;
  api_key: string;
  model: string;
}

export interface AiResponse {
  run_id: string;
  text: string;
  model: string;
  duration_ms: number;
}

// Window controls
export const windowMinimize = () => invoke("window_minimize");
export const windowMaximize = () => invoke("window_maximize");
export const windowClose = () => invoke("window_close");
export const windowStartDragging = () => invoke("window_start_dragging");

// Projects
export const listProjects = () => invoke<Project[]>("cmd_list_projects");
export const createProject = (input: {
  title: string;
  description?: string;
  target_audience?: string;
  output_language?: string;
  default_tone?: string;
}) => invoke<Project>("cmd_create_project", { input });
export const getProject = (id: string) => invoke<Project>("cmd_get_project", { id });
export const updateProject = (id: string, input: Partial<Project>) =>
  invoke<Project>("cmd_update_project", { id, input });
export const deleteProject = (id: string) => invoke<void>("cmd_delete_project", { id });

export const getBible = (projectId: string) =>
  invoke<ProjectBible>("cmd_get_bible", { projectId });
export const saveBible = (bible: ProjectBible) =>
  invoke<ProjectBible>("cmd_save_bible", { bible });

// Documents & Blocks
export const listDocuments = (projectId: string) =>
  invoke<Document[]>("cmd_list_documents", { projectId });
export const createDocument = (input: {
  project_id: string;
  parent_id?: string | null;
  title: string;
  doc_type: string;
  initial_content?: string;
}) => invoke<Document>("cmd_create_document", { input });
export const getDocument = (id: string) => invoke<Document>("cmd_get_document", { id });
export const getBlocks = (documentId: string) =>
  invoke<Block[]>("cmd_get_blocks", { documentId });
export const saveBlocks = (documentId: string, blocks: Block[]) =>
  invoke<Block[]>("cmd_save_blocks", { documentId, blocks });
export const deleteDocument = (id: string) => invoke<void>("cmd_delete_document", { id });

// AI completions
export const generateAi = (req: {
  project_id?: string;
  document_id?: string;
  system_instruction?: string;
  user_prompt: string;
  temperature?: number;
}) => invoke<AiResponse>("cmd_generate_ai", { req });

export const generateEbookOutline = (projectId: string | undefined, topic: string, chaptersCount: number) =>
  invoke<string>("cmd_generate_ebook_outline", { projectId, topic, chaptersCount });

export const generateCarousel = (projectId: string | undefined, topic: string, slidesCount: number) =>
  invoke<string>("cmd_generate_carousel", { projectId, topic, slidesCount });

export const generateReels = (projectId: string | undefined, topic: string) =>
  invoke<string>("cmd_generate_reels", { projectId, topic });

export const generatePitchdeck = (projectId: string | undefined, companyName: string, context: string) =>
  invoke<string>("cmd_generate_pitchdeck", { projectId, companyName, context });

// Exports
export const exportMarkdown = (documentId: string) =>
  invoke<string>("cmd_export_markdown", { documentId });
export const exportHtml = (documentId: string) =>
  invoke<string>("cmd_export_html", { documentId });
export const exportEpub = (documentId: string) =>
  invoke<string>("cmd_export_epub", { documentId });
export const exportProject = (projectId: string) =>
  invoke<string>("cmd_export_project", { projectId });

// Prompts
export const listPrompts = (category?: string) =>
  invoke<PromptTemplate[]>("cmd_list_prompts", { category });
export const createPrompt = (input: {
  title: string;
  category: string;
  system_prompt: string;
  user_prompt: string;
  variables?: string;
}) => invoke<PromptTemplate>("cmd_create_prompt", { input });
export const togglePromptFavorite = (id: string) =>
  invoke<boolean>("cmd_toggle_prompt_favorite", { id });
export const deletePrompt = (id: string) => invoke<void>("cmd_delete_prompt", { id });

// Settings
export const getAiSettings = () => invoke<AiSettings>("cmd_get_ai_settings");
export const saveAiSettings = (settings: AiSettings) =>
  invoke<void>("cmd_save_ai_settings", { settings });

// Omni-Channel Content Repurposing Engine
export interface ContentAsset {
  id: string;
  campaign_id: string;
  channel: "blog" | "shorts" | "carousel" | "tweets" | "newsletter" | string;
  title: string;
  content: string;
  meta: string;
  status: "draft" | "approved" | "queued" | string;
  created_at: number;
  updated_at: number;
}

export interface ContentCampaign {
  id: string;
  project_id: string | null;
  title: string;
  source_type: "markdown" | "raw_text" | "youtube_url" | string;
  source_content: string;
  source_url: string | null;
  status: "draft" | "queued" | "published" | string;
  created_at: number;
  updated_at: number;
  assets: ContentAsset[];
}

export const fetchYoutubeContent = (url: string) =>
  invoke<string>("cmd_fetch_youtube_content", { url });

export const createCampaign = (input: {
  project_id?: string;
  title: string;
  source_type: string;
  source_content: string;
  source_url?: string;
}) => invoke<ContentCampaign>("cmd_create_campaign", { input });

export const getCampaign = (campaignId: string) =>
  invoke<ContentCampaign>("cmd_get_campaign", { campaignId });

export const listCampaigns = (projectId?: string) =>
  invoke<ContentCampaign[]>("cmd_list_campaigns", { projectId });

export const runRepurpose = (input: {
  campaign_id: string;
  selected_channels?: string[];
}) => invoke<ContentCampaign>("cmd_run_repurpose", { input });

export const updateAsset = (assetId: string, content: string) =>
  invoke<ContentAsset>("cmd_update_asset", { assetId, content });

export const setAssetStatus = (assetId: string, status: string) =>
  invoke<ContentAsset>("cmd_set_asset_status", { assetId, status });

export const queueCampaign = (campaignId: string) =>
  invoke<ContentCampaign>("cmd_queue_campaign", { campaignId });

export const deleteCampaign = (campaignId: string) =>
  invoke<void>("cmd_delete_campaign", { campaignId });

// Content Automation, Drip Scheduling & Webhook Dispatcher
export interface ContentSchedule {
  id: string;
  campaign_id: string;
  asset_id: string;
  channel: string;
  target_platform: string;
  scheduled_at: number;
  status: "queued" | "dispatching" | "published" | "failed" | "cancelled" | string;
  retry_count: number;
  max_retries: number;
  last_error: string | null;
  published_url: string | null;
  tracking_token: string;
  channel_config_json: string;
  created_at: number;
  updated_at: number;
}

export interface PublishingChannel {
  id: string;
  name: string;
  channel_type: string;
  is_active: boolean;
  endpoint_url: string | null;
  credentials_json: string;
  config_json: string;
  created_at: number;
  updated_at: number;
}

export interface PublishResult {
  success: boolean;
  published_url: string | null;
  message: string;
  external_id: string | null;
}

export const createSchedule = (input: {
  campaign_id: string;
  asset_id: string;
  channel: string;
  target_platform: string;
  scheduled_at: number;
  channel_config_json?: string;
}) => invoke<ContentSchedule>("cmd_create_schedule", { input });

export const createDripBatch = (input: {
  campaign_id: string;
  preset: string;
  start_time: number;
  default_platform: string;
}) => invoke<ContentSchedule[]>("cmd_create_drip_batch", { input });

export const listSchedules = (campaignId?: string, statusFilter?: string) =>
  invoke<ContentSchedule[]>("cmd_list_schedules", {
    campaignId,
    statusFilter,
  });

export const cancelSchedule = (id: string) =>
  invoke<void>("cmd_cancel_schedule", { id });

export const triggerInstantDispatch = (id: string) =>
  invoke<ContentSchedule>("cmd_trigger_instant_dispatch", { id });

export const savePublishingChannel = (channel: PublishingChannel) =>
  invoke<PublishingChannel>("cmd_save_publishing_channel", { channel });

export const listPublishingChannels = () =>
  invoke<PublishingChannel[]>("cmd_list_publishing_channels");

export const testChannelConnection = (
  targetPlatform: string,
  endpointUrl: string,
  credentialsJson: string,
  configJson: string
) =>
  invoke<PublishResult>("cmd_test_channel_connection", {
    targetPlatform,
    endpointUrl,
    credentialsJson,
    configJson,
  });


