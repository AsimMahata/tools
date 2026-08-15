use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl ToolMetadata {
    /// Load metadata from a tool directory if `tom.toml` exists
    pub fn load_from_dir(dir: &Path) -> Option<Self> {
        let metadata_path = dir.join("tom.toml");
        if metadata_path.is_file() {
            if let Ok(content) = fs::read_to_string(&metadata_path) {
                if let Ok(meta) = toml::from_str::<ToolMetadata>(&content) {
                    return Some(meta);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tom_toml() {
        let toml_str = r#"
            name = "netman"
            description = "Windows network management CLI"
            version = "0.1.0"
            author = "Asim Mahata"
            tags = ["network", "cli", "windows"]
        "#;
        let meta: ToolMetadata = toml::from_str(toml_str).unwrap();
        assert_eq!(meta.name.as_deref(), Some("netman"));
        assert_eq!(meta.description.as_deref(), Some("Windows network management CLI"));
        assert_eq!(meta.version.as_deref(), Some("0.1.0"));
    }
}
