use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub tools_directory: Option<String>,
    pub editor: Option<String>,
}

impl Config {
    /// Get the default user config file path (~/.config/tom/config.toml or %APPDATA%\tom\config.toml)
    pub fn user_config_file_path() -> Option<PathBuf> {
        if let Some(config_dir) = dirs::config_dir() {
            Some(config_dir.join("tom").join("config.toml"))
        } else if let Some(home_dir) = dirs::home_dir() {
            Some(home_dir.join(".config").join("tom").join("config.toml"))
        } else {
            None
        }
    }

    /// Load config from disk, checking user config dir, home dir, workspace or exe dir.
    /// If no tools_directory was previously saved, detect the initial clone location,
    /// set tools_directory = parent(TOM_DIR), and persist it for future runs.
    pub fn load_and_persist() -> (Self, Option<PathBuf>) {
        let mut candidates = Vec::new();

        if let Some(path) = Self::user_config_file_path() {
            candidates.push(path);
        }
        if let Some(home_dir) = dirs::home_dir() {
            candidates.push(home_dir.join(".tom.toml"));
            candidates.push(home_dir.join(".config").join("tom").join("config.toml"));
        }
        if let Ok(cwd) = std::env::current_dir() {
            candidates.push(cwd.join(".tom.toml"));
            candidates.push(cwd.join("tom.config.toml"));
            candidates.push(cwd.join("config.toml"));
            if let Some(parent) = cwd.parent() {
                candidates.push(parent.join(".tom.toml"));
                candidates.push(parent.join("config.toml"));
            }
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                candidates.push(dir.join("config.toml"));
            }
        }

        // 1. Try loading from existing config
        for path in &candidates {
            if path.is_file() {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(mut cfg) = toml::from_str::<Config>(&content) {
                        if cfg.tools_directory.is_some() {
                            return (cfg, Some(path.clone()));
                        } else {
                            // If tools_directory was missing in the file, infer and save it
                            let inferred = Self::infer_initial_tools_directory();
                            cfg.tools_directory = Some(inferred.to_string_lossy().to_string());
                            let _ = cfg.save(path);
                            return (cfg, Some(path.clone()));
                        }
                    }
                }
            }
        }

        // 2. Initial Setup: determine initial clone location
        let inferred = Self::infer_initial_tools_directory();
        let cfg = Config {
            tools_directory: Some(inferred.to_string_lossy().to_string()),
            editor: None,
        };

        // Persist to user config or local workspace config
        let save_target = Self::user_config_file_path()
            .unwrap_or_else(|| PathBuf::from("config.toml"));

        let _ = cfg.save(&save_target);

        (cfg, Some(save_target))
    }

    /// Infer initial tools directory from TOM's repository / clone location
    pub fn infer_initial_tools_directory() -> PathBuf {
        // Check current working directory
        if let Ok(cwd) = std::env::current_dir() {
            let folder_name = cwd.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if folder_name.eq_ignore_ascii_case("tom") {
                if let Some(parent) = cwd.parent() {
                    return parent.to_path_buf();
                }
            }
            if cwd.join("tom").exists() {
                return cwd;
            }
        }

        // Check executable path (e.g. if running target/debug/tom.exe inside tom)
        if let Ok(current_exe) = std::env::current_exe() {
            if let Some(parent) = current_exe.parent() {
                let mut check = parent;
                while let Some(p) = check.parent() {
                    let folder_name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if folder_name.eq_ignore_ascii_case("tom") {
                        if let Some(grandparent) = p.parent() {
                            if grandparent.exists() {
                                return grandparent.to_path_buf();
                            }
                        }
                    }
                    check = p;
                }
            }
        }

        if let Ok(cwd) = std::env::current_dir() {
            cwd
        } else {
            PathBuf::from(".")
        }
    }

    /// Resolve tools directory:
    /// 1. CLI flag override (if supplied)
    /// 2. Saved `tools_directory` in config
    /// 3. Inferred fallback
    pub fn resolve_tools_directory(&self, cli_dir: Option<&Path>) -> PathBuf {
        if let Some(dir) = cli_dir {
            return dir.to_path_buf();
        }

        if let Some(ref dir_str) = self.tools_directory {
            let expanded = expand_tilde(dir_str);
            let path = PathBuf::from(expanded);
            if path.exists() {
                return path;
            }
        }

        Self::infer_initial_tools_directory()
    }

    /// Save configuration to a file
    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(path, content)
            .map_err(|e| format!("Failed to write config file to {}: {}", path.display(), e))
    }
}

/// Expands leading `~` to the user's home directory
pub fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") || path.starts_with("~\\") {
        if let Some(home) = dirs::home_dir() {
            let without_tilde = &path[2..];
            return home.join(without_tilde).to_string_lossy().to_string();
        }
    } else if path == "~" {
        if let Some(home) = dirs::home_dir() {
            return home.to_string_lossy().to_string();
        }
    }
    path.to_string()
}
