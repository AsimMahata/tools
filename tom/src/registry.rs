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
    pub install_cmd: Option<String>,
    pub uninstall_cmd: Option<String>,
    pub requirements: Option<String>,
    pub tips: Option<String>,
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
                install_cmd: Some("cargo build --release".to_string()),
                uninstall_cmd: None,
                requirements: Some("Rust toolchain (cargo 1.75+)".to_string()),
                tips: Some("Clone to a dedicated parent folder like C:\\tools\\tom".to_string()),
            },
        );

        tools.insert(
            "netman".to_string(),
            RegistryEntry {
                name: "netman".to_string(),
                description: Some("Windows network management CLI".to_string()),
                repository: "https://github.com/AsimMahata/netman.git".to_string(),
                tags: Some(vec!["network".to_string(), "cli".to_string(), "windows".to_string()]),
                install_cmd: Some("pip install -e .".to_string()),
                uninstall_cmd: Some("pip uninstall -y netman".to_string()),
                requirements: Some("Python 3.10+ and pip".to_string()),
                tips: Some("Run inside a virtualenv or install in editable mode with pip install -e .".to_string()),
            },
        );

        tools.insert(
            "progit".to_string(),
            RegistryEntry {
                name: "progit".to_string(),
                description: Some("Git productivity enhancements and workflow helper".to_string()),
                repository: "https://github.com/AsimMahata/progit.git".to_string(),
                tags: Some(vec!["git".to_string(), "workflow".to_string(), "productivity".to_string()]),
                install_cmd: Some("cargo build --release".to_string()),
                uninstall_cmd: Some("cargo clean".to_string()),
                requirements: Some("Rust toolchain (cargo in PATH)".to_string()),
                tips: Some("Compiles to standalone executable in target/release".to_string()),
            },
        );

        tools.insert(
            "logit".to_string(),
            RegistryEntry {
                name: "logit".to_string(),
                description: Some("Log inspection and management utility".to_string()),
                repository: "https://github.com/AsimMahata/logit.git".to_string(),
                tags: Some(vec!["logging".to_string(), "cli".to_string(), "tools".to_string()]),
                install_cmd: Some("cargo build --release".to_string()),
                uninstall_cmd: Some("cargo clean".to_string()),
                requirements: Some("Rust toolchain (cargo in PATH)".to_string()),
                tips: Some("Compiles to standalone binary in target/release/logit.exe".to_string()),
            },
        );

        tools.insert(
            "sodo".to_string(),
            RegistryEntry {
                name: "sodo".to_string(),
                description: Some("Task management and todo tracker CLI".to_string()),
                repository: "https://github.com/AsimMahata/sodo.git".to_string(),
                tags: Some(vec!["todo".to_string(), "productivity".to_string(), "cli".to_string()]),
                install_cmd: Some("pip install -e .".to_string()),
                uninstall_cmd: Some("pip uninstall -y sodo".to_string()),
                requirements: Some("Python 3.10+ and pip".to_string()),
                tips: Some("Requires active Python environment with click/typer".to_string()),
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
