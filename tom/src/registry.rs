use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub name: String,
    pub description: Option<String>,
    pub repository: String,
    pub tags: Option<Vec<String>>,
    pub build_cmd: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Registry {
    #[serde(default)]
    pub tools: BTreeMap<String, RegistryEntry>,
}

impl Registry {
    /// Load registry from disk or return default built-in registry
    pub fn load(tools_dir: &Path) -> Self {
        let candidates = [
            tools_dir.join("tom").join("registry.toml"),
            tools_dir.join("registry.toml"),
        ];

        for path in &candidates {
            if path.is_file() {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(reg) = toml::from_str::<Registry>(&content) {
                        return reg;
                    }
                }
            }
        }

        Self::default_builtin()
    }

    /// Default built-in registry fallback
    pub fn default_builtin() -> Self {
        let mut tools = BTreeMap::new();

        tools.insert(
            "tom".to_string(),
            RegistryEntry {
                name: "tom".to_string(),
                description: Some("Tool Manager — CLI to manage personal tools and repositories".to_string()),
                repository: "https://github.com/AsimMahata/tom.git".to_string(),
                tags: Some(vec!["tools".to_string(), "cli".to_string(), "manager".to_string()]),
                build_cmd: None,
            },
        );

        tools.insert(
            "netman".to_string(),
            RegistryEntry {
                name: "netman".to_string(),
                description: Some("Windows network management CLI".to_string()),
                repository: "https://github.com/AsimMahata/netman.git".to_string(),
                tags: Some(vec!["network".to_string(), "cli".to_string(), "windows".to_string()]),
                build_cmd: None,
            },
        );

        tools.insert(
            "progit".to_string(),
            RegistryEntry {
                name: "progit".to_string(),
                description: Some("Git productivity enhancements and workflow helper".to_string()),
                repository: "https://github.com/AsimMahata/progit.git".to_string(),
                tags: Some(vec!["git".to_string(), "workflow".to_string(), "productivity".to_string()]),
                build_cmd: None,
            },
        );

        tools.insert(
            "logit".to_string(),
            RegistryEntry {
                name: "logit".to_string(),
                description: Some("Log inspection and management utility".to_string()),
                repository: "https://github.com/AsimMahata/logit.git".to_string(),
                tags: Some(vec!["logging".to_string(), "cli".to_string()]),
                build_cmd: None,
            },
        );

        tools.insert(
            "sodo".to_string(),
            RegistryEntry {
                name: "sodo".to_string(),
                description: Some("Task management and todo tracker CLI".to_string()),
                repository: "https://github.com/AsimMahata/sodo.git".to_string(),
                tags: Some(vec!["todo".to_string(), "productivity".to_string(), "cli".to_string()]),
                build_cmd: None,
            },
        );

        Registry { tools }
    }

    /// Lookup a tool in the registry by name (case-insensitive)
    pub fn get(&self, name: &str) -> Option<&RegistryEntry> {
        let lower = name.to_lowercase();
        self.tools
            .iter()
            .find(|(k, _)| k.to_lowercase() == lower)
            .map(|(_, v)| v)
    }
}
