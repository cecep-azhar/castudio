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
