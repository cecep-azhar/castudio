use crate::document::{blocks_to_markdown, get_blocks, get_document};
use crate::error::StudioError;
use crate::paths;
use crate::project::{get_bible, get_project};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

pub fn export_markdown(document_id: &str) -> Result<PathBuf, StudioError> {
    let doc = get_document(document_id)?;
    let blocks = get_blocks(document_id)?;
    let md = blocks_to_markdown(&blocks);

    let filename = format!("{}_{}.md", sanitize_filename(&doc.title), doc.id);
    let out_path = paths::export_dir().join(&filename);

    let mut file = File::create(&out_path)?;
    file.write_all(format!("# {}\n\n{}", doc.title, md).as_bytes())?;

    Ok(out_path)
}

pub fn export_html(document_id: &str) -> Result<PathBuf, StudioError> {
    let doc = get_document(document_id)?;
    let blocks = get_blocks(document_id)?;
    let md = blocks_to_markdown(&blocks);

    let html_content = format!(
        r#"<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <title>{}</title>
  <style>
    body {{
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      line-height: 1.6;
      max-width: 800px;
      margin: 40px auto;
      padding: 0 20px;
      color: #1e293b;
      background: #ffffff;
    }}
    h1, h2, h3 {{ color: #0f172a; }}
    pre {{ background: #0f172a; color: #f8fafc; padding: 16px; border-radius: 8px; overflow-x: auto; }}
    blockquote {{ border-left: 4px solid #8b5cf6; margin: 0; padding-left: 16px; color: #475569; }}
    @media print {{
      body {{ max-width: 100%; margin: 0; }}
      .page-break {{ page-break-after: always; }}
    }}
  </style>
</head>
<body>
  <h1>{}</h1>
  <div>{}</div>
</body>
</html>"#,
        doc.title,
        doc.title,
        simple_md_to_html(&md)
    );

    let filename = format!("{}_{}.html", sanitize_filename(&doc.title), doc.id);
    let out_path = paths::export_dir().join(&filename);

    let mut file = File::create(&out_path)?;
    file.write_all(html_content.as_bytes())?;

    Ok(out_path)
}

pub fn export_epub(document_id: &str) -> Result<PathBuf, StudioError> {
    let doc = get_document(document_id)?;
    let blocks = get_blocks(document_id)?;
    let md = blocks_to_markdown(&blocks);

    let filename = format!("{}_{}.epub", sanitize_filename(&doc.title), doc.id);
    let out_path = paths::export_dir().join(&filename);
    let file = File::create(&out_path)?;
    let mut zip = ZipWriter::new(file);

    let uncompressed = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    let compressed = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // 1. mimetype (MUST be first, uncompressed)
    zip.start_file("mimetype", uncompressed)
        .map_err(|e| StudioError::Export(e.to_string()))?;
    zip.write_all(b"application/epub+zip")?;

    // 2. META-INF/container.xml
    zip.start_file("META-INF/container.xml", compressed)
        .map_err(|e| StudioError::Export(e.to_string()))?;
    zip.write_all(
        br#"<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>"#,
    )?;

    // 3. OEBPS/content.opf
    zip.start_file("OEBPS/content.opf", compressed)
        .map_err(|e| StudioError::Export(e.to_string()))?;
    let opf = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="pub-id">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:identifier id="pub-id">urn:uuid:{}</dc:identifier>
    <dc:title>{}</dc:title>
    <dc:language>id</dc:language>
    <meta property="dcterms:modified">2026-10-08T00:00:00Z</meta>
  </metadata>
  <manifest>
    <item id="chapter1" href="chapter1.xhtml" media-type="application/xhtml+xml"/>
  </manifest>
  <spine>
    <itemref idref="chapter1"/>
  </spine>
</package>"#,
        doc.id, doc.title
    );
    zip.write_all(opf.as_bytes())?;

    // 4. OEBPS/chapter1.xhtml
    zip.start_file("OEBPS/chapter1.xhtml", compressed)
        .map_err(|e| StudioError::Export(e.to_string()))?;
    let xhtml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" lang="id">
<head>
  <title>{}</title>
</head>
<body>
  <h1>{}</h1>
  <div>{}</div>
</body>
</html>"#,
        doc.title,
        doc.title,
        simple_md_to_html(&md)
    );
    zip.write_all(xhtml.as_bytes())?;

    zip.finish().map_err(|e| StudioError::Export(e.to_string()))?;
    Ok(out_path)
}

pub fn export_project_json(project_id: &str) -> Result<PathBuf, StudioError> {
    let proj = get_project(project_id)?;
    let bible = get_bible(project_id).ok();

    let backup = serde_json::json!({
        "project": proj,
        "bible": bible,
        "exported_at": chrono::Utc::now().to_rfc3339()
    });

    let filename = format!("project_{}_{}.json", sanitize_filename(&proj.title), proj.id);
    let out_path = paths::export_dir().join(&filename);

    let mut file = File::create(&out_path)?;
    file.write_all(serde_json::to_string_pretty(&backup).unwrap().as_bytes())?;

    Ok(out_path)
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .trim_matches('_')
        .to_lowercase()
}

fn simple_md_to_html(md: &str) -> String {
    let mut out = String::new();
    for line in md.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            out.push_str(&format!("<h1>{}</h1>\n", &trimmed[2..]));
        } else if trimmed.starts_with("## ") {
            out.push_str(&format!("<h2>{}</h2>\n", &trimmed[3..]));
        } else if trimmed.starts_with("### ") {
            out.push_str(&format!("<h3>{}</h3>\n", &trimmed[4..]));
        } else if trimmed.starts_with("> ") {
            out.push_str(&format!("<blockquote>{}</blockquote>\n", &trimmed[2..]));
        } else if !trimmed.is_empty() {
            out.push_str(&format!("<p>{}</p>\n", trimmed));
        }
    }
    out
}
