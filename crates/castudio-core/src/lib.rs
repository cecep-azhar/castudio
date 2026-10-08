pub mod ai;
pub mod automation;
pub mod db;
pub mod document;
pub mod error;
pub mod export;
pub mod paths;
pub mod prefs;
pub mod project;
pub mod prompt_studio;
pub mod repurposing;
pub mod templates;

pub use error::StudioError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_db_and_project_lifecycle() {
        let proj = project::create_project(project::CreateProjectInput {
            title: "Test AI Book".to_string(),
            description: Some("Automated test project".to_string()),
            target_audience: Some("Developers".to_string()),
            output_language: Some("id".to_string()),
            default_tone: Some("technical".to_string()),
        })
        .expect("Create project succeeds");

        assert_eq!(proj.title, "Test AI Book");

        let bible = project::get_bible(&proj.id).expect("Default bible exists");
        assert_eq!(bible.project_id, proj.id);

        let doc = document::create_document(document::CreateDocumentInput {
            project_id: proj.id.clone(),
            parent_id: None,
            title: "Chapter 1: The Sovereign Spark".to_string(),
            doc_type: "ebook".to_string(),
            initial_content: Some("First paragraph of sovereign creation.".to_string()),
        })
        .expect("Create doc succeeds");

        assert_eq!(doc.title, "Chapter 1: The Sovereign Spark");

        let blocks = document::get_blocks(&doc.id).expect("Get blocks succeeds");
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].content, "First paragraph of sovereign creation.");

        let md = document::blocks_to_markdown(&blocks);
        assert!(md.contains("First paragraph of sovereign creation."));

        let prompts = prompt_studio::list_prompts(None).expect("List prompts succeeds");
        assert!(!prompts.is_empty());
    }
}
