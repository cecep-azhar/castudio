mod commands;
mod window;

use tauri::Builder;

pub fn run() {
    let mut builder = Builder::default();

    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    {
        builder = builder
            .plugin(tauri_plugin_dialog::init())
            .plugin(tauri_plugin_fs::init())
            .plugin(tauri_plugin_process::init());
    }

    builder
        .invoke_handler(tauri::generate_handler![
            // Window controls
            commands::window_minimize,
            commands::window_maximize,
            commands::window_close,
            commands::window_start_dragging,
            // Projects & Bible
            commands::cmd_list_projects,
            commands::cmd_create_project,
            commands::cmd_get_project,
            commands::cmd_update_project,
            commands::cmd_delete_project,
            commands::cmd_get_bible,
            commands::cmd_save_bible,
            // Documents & Blocks
            commands::cmd_list_documents,
            commands::cmd_create_document,
            commands::cmd_get_document,
            commands::cmd_update_document,
            commands::cmd_get_blocks,
            commands::cmd_save_blocks,
            commands::cmd_delete_document,
            // AI Generators
            commands::cmd_generate_ai,
            commands::cmd_generate_ebook_outline,
            commands::cmd_generate_carousel,
            commands::cmd_generate_reels,
            commands::cmd_generate_pitchdeck,
            // Exports
            commands::cmd_export_markdown,
            commands::cmd_export_html,
            commands::cmd_export_epub,
            commands::cmd_export_project,
            // Prompts
            commands::cmd_list_prompts,
            commands::cmd_create_prompt,
            commands::cmd_toggle_prompt_favorite,
            commands::cmd_delete_prompt,
            // Settings
            commands::cmd_get_ai_settings,
            commands::cmd_save_ai_settings,
            // Omni-Channel Repurposing Engine
            commands::cmd_fetch_youtube_content,
            commands::cmd_create_campaign,
            commands::cmd_get_campaign,
            commands::cmd_list_campaigns,
            commands::cmd_run_repurpose,
            commands::cmd_update_asset,
            commands::cmd_set_asset_status,
            commands::cmd_queue_campaign,
            commands::cmd_delete_campaign,
        ])
        .run(tauri::generate_context!())
        .expect("error while running CAStudio application");
}
