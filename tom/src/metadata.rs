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
    pub install_cmd: Option<String>,
    pub uninstall_cmd: Option<String>,
    pub requirements: Option<String>,
    pub tips: Option<String>,
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
