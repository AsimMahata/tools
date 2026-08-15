use chrono::{DateTime, Local};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::git::GitStatus;
use crate::metadata::ToolMetadata;

#[derive(Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub dir_name: String,
    pub path: PathBuf,
    pub is_self: bool,
    pub metadata: Option<ToolMetadata>,
    pub git: GitStatus,
    pub modified_time: Option<DateTime<Local>>,
    pub modified_relative: String,
}

impl Tool {
    /// Load a Tool from a directory
    pub fn from_dir(path: PathBuf) -> Option<Self> {
        if !path.is_dir() {
            return None;
        }

        let dir_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        if dir_name.is_empty() || dir_name.starts_with('.') {
            return None;
        }

        let is_self = dir_name.eq_ignore_ascii_case("tom");

        // Inspect Git status
        let git = GitStatus::inspect(&path);

        // Check if directory actually contains installed source code (not just a README stub)
        let has_code = path.join("Cargo.toml").exists()
            || path.join("pyproject.toml").exists()
            || path.join("package.json").exists()
            || path.join("src").exists()
            || path.join("setup.py").exists()
            || path.join("go.mod").exists()
            || is_self;

        if !has_code {
            return None;
        }

        // Load optional tom.toml metadata
        let metadata = ToolMetadata::load_from_dir(&path);
        let name = metadata
            .as_ref()
            .and_then(|m| m.name.clone())
            .unwrap_or_else(|| dir_name.clone());

        // Get directory last modified time
        let (modified_time, modified_relative) = get_modified_info(&path);

        Some(Tool {
            name,
            dir_name,
            path,
            is_self,
            metadata,
            git,
            modified_time,
            modified_relative,
        })
    }

    /// Get description if available
    pub fn description(&self) -> Option<&str> {
        self.metadata.as_ref().and_then(|m| m.description.as_deref())
    }

    /// Get version if available
    pub fn version(&self) -> Option<&str> {
        self.metadata.as_ref().and_then(|m| m.version.as_deref())
    }
}

/// Discover all tools in the configured tools directory
pub fn discover_tools(tools_dir: &Path) -> Vec<Tool> {
    let mut tools = Vec::new();

    let entries = match fs::read_dir(tools_dir) {
        Ok(e) => e,
        Err(_) => return tools,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            // Skip hidden folders and common non-tool directories
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }

            if let Some(tool) = Tool::from_dir(path) {
                tools.push(tool);
            }
        }
    }

    tools.sort_by(|a, b| {
        a.name.to_lowercase().cmp(&b.name.to_lowercase())
    });
    tools
}

/// Find a specific tool by name or directory name
pub fn find_tool(tools_dir: &Path, target: &str) -> Option<Tool> {
    let target_lower = target.to_lowercase();
    let tools = discover_tools(tools_dir);
    tools.into_iter().find(|t| {
        t.name.to_lowercase() == target_lower || t.dir_name.to_lowercase() == target_lower
    })
}

/// Helper to get formatted last modified time of a path
fn get_modified_info(path: &Path) -> (Option<DateTime<Local>>, String) {
    if let Ok(meta) = fs::metadata(path) {
        if let Ok(sys_time) = meta.modified() {
            let dt: DateTime<Local> = DateTime::from(sys_time);
            let rel = format_relative_time(sys_time);
            return (Some(dt), rel);
        }
    }
    (None, "unknown".to_string())
}

/// Format relative time
pub fn format_relative_time(time: SystemTime) -> String {
    let now = SystemTime::now();
    let duration = match now.duration_since(time) {
        Ok(d) => d,
        Err(_) => return "just now".to_string(),
    };

    let secs = duration.as_secs();
    if secs < 60 {
        "just now".to_string()
    } else if secs < 3600 {
        let mins = secs / 60;
        if mins == 1 {
            "1 min ago".to_string()
        } else {
            format!("{} mins ago", mins)
        }
    } else if secs < 86400 {
        let hours = secs / 3600;
        if hours == 1 {
            "1 hour ago".to_string()
        } else {
            format!("{} hours ago", hours)
        }
    } else if secs < 172800 {
        "yesterday".to_string()
    } else if secs < 604800 {
        let days = secs / 86400;
        format!("{} days ago", days)
    } else if secs < 2592000 {
        let weeks = secs / 604800;
        if weeks == 1 {
            "1 week ago".to_string()
        } else {
            format!("{} weeks ago", weeks)
        }
    } else {
        let months = secs / 2592000;
        if months == 1 {
            "1 month ago".to_string()
        } else {
            format!("{} months ago", months)
        }
    }
}
