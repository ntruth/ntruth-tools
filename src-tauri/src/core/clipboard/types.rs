// Clipboard content types
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ClipboardContent {
    /// Plain text content
    Text {
        content: String,
        plain_text: String,
    },
    /// Image content
    Image {
        #[serde(with = "serde_bytes")]
        data: Vec<u8>,
        format: ImageFormat,
        #[serde(with = "serde_bytes")]
        thumbnail: Vec<u8>,
    },
    /// File paths
    Files {
        paths: Vec<PathBuf>,
    },
    /// HTML content
    Html {
        html: String,
        plain_text: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    PNG,
    JPEG,
    GIF,
    BMP,
}

impl ClipboardContent {
    /// Get the plain text representation of the content
    pub fn as_plain_text(&self) -> String {
        match self {
            Self::Text { plain_text, .. } => plain_text.clone(),
            Self::Html { plain_text, .. } => plain_text.clone(),
            Self::Image { .. } => "[Image]".to_string(),
            Self::Files { paths } => {
                paths
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
    }

    /// Get a preview string (limited length)
    pub fn preview(&self, max_len: usize) -> String {
        let text = self.as_plain_text();
        if text.len() <= max_len {
            text
        } else {
            format!("{}...", &text[..max_len])
        }
    }

    /// Get the content type as a string
    pub fn content_type(&self) -> &'static str {
        match self {
            Self::Text { .. } => "text",
            Self::Image { .. } => "image",
            Self::Files { .. } => "files",
            Self::Html { .. } => "html",
        }
    }

    /// Calculate content hash for deduplication
    pub fn hash(&self) -> String {
        let content = match self {
            Self::Text { content, .. } => content.as_bytes(),
            Self::Html { html, .. } => html.as_bytes(),
            Self::Image { data, .. } => data.as_slice(),
            Self::Files { paths } => {
                let paths_str = paths
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join("|");
                return format!("{:x}", md5::compute(paths_str.as_bytes()));
            }
        };
        format!("{:x}", md5::compute(content))
    }
}

// Custom serde module for Vec<u8>
mod serde_bytes {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(bytes)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<u8>::deserialize(deserializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_text() {
        let content = ClipboardContent::Text {
            content: "Hello".to_string(),
            plain_text: "Hello".to_string(),
        };
        assert_eq!(content.as_plain_text(), "Hello");
    }

    #[test]
    fn test_preview() {
        let content = ClipboardContent::Text {
            content: "Hello World!".to_string(),
            plain_text: "Hello World!".to_string(),
        };
        assert_eq!(content.preview(5), "Hello...");
    }

    #[test]
    fn test_content_type() {
        let content = ClipboardContent::Text {
            content: "test".to_string(),
            plain_text: "test".to_string(),
        };
        assert_eq!(content.content_type(), "text");
    }
}
