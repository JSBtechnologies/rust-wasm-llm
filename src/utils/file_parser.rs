use anyhow::Result;

/// File parser for different document types
pub struct FileParser;

impl FileParser {
    /// Parse a file based on its type
    pub async fn parse(file_name: &str, content: &[u8]) -> Result<String> {
        let extension = Self::get_extension(file_name);

        match extension.as_str() {
            "txt" | "md" => Self::parse_text(content),
            "pdf" => Self::parse_pdf(content).await,
            "docx" => Self::parse_docx(content).await,
            "html" | "htm" => Self::parse_html(content),
            _ => Err(anyhow::anyhow!("Unsupported file type: {}", extension)),
        }
    }

    /// Get file extension
    fn get_extension(file_name: &str) -> String {
        file_name
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase()
    }

    /// Parse plain text
    fn parse_text(content: &[u8]) -> Result<String> {
        Ok(String::from_utf8(content.to_vec())?)
    }

    /// Parse PDF (TODO: integrate pdf.js or similar)
    async fn parse_pdf(_content: &[u8]) -> Result<String> {
        log::warn!("PDF parsing not yet implemented");
        Err(anyhow::anyhow!("PDF parsing not yet implemented"))
    }

    /// Parse DOCX (TODO: integrate docx parser)
    async fn parse_docx(_content: &[u8]) -> Result<String> {
        log::warn!("DOCX parsing not yet implemented");
        Err(anyhow::anyhow!("DOCX parsing not yet implemented"))
    }

    /// Parse HTML (basic text extraction)
    fn parse_html(content: &[u8]) -> Result<String> {
        let html = String::from_utf8(content.to_vec())?;

        // TODO: Implement proper HTML parsing
        // For now, just remove tags
        let text = html
            .replace("<script", "<\0script")
            .replace("</script>", "</\0script>")
            .split("<\0script")
            .next()
            .unwrap_or("")
            .to_string();

        Ok(text)
    }

    /// Detect file type from content
    pub fn detect_type(content: &[u8]) -> String {
        if content.starts_with(b"%PDF") {
            "pdf".to_string()
        } else if content.starts_with(b"PK") {
            "docx".to_string() // DOCX is a zip file
        } else if content.starts_with(b"<html") || content.starts_with(b"<!DOCTYPE") {
            "html".to_string()
        } else {
            "txt".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text() {
        let content = b"Hello, world!";
        let result = FileParser::parse_text(content).unwrap();
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_get_extension() {
        assert_eq!(FileParser::get_extension("test.txt"), "txt");
        assert_eq!(FileParser::get_extension("document.pdf"), "pdf");
        assert_eq!(FileParser::get_extension("file.DOCX"), "docx");
    }

    #[test]
    fn test_detect_type() {
        assert_eq!(FileParser::detect_type(b"%PDF-1.4"), "pdf");
        assert_eq!(FileParser::detect_type(b"PK\x03\x04"), "docx");
        assert_eq!(FileParser::detect_type(b"<html>"), "html");
        assert_eq!(FileParser::detect_type(b"Plain text"), "txt");
    }
}
