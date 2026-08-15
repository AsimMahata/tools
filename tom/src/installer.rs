use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::git::GitStatus;

/// Clone repository into target directory
pub fn clone_repository(repo_url: &str, target_path: &Path) -> Result<(), String> {
    if target_path.exists() {
        return Err(format!("Directory already exists: {}", target_path.display()));
    }

    let target_str = target_path.to_str().unwrap_or(".");
    let output = Command::new("git")
        .args(["clone", repo_url, target_str])
        .output()
        .map_err(|e| format!("Failed to run git clone: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Git clone failed:\n{}", stderr))
    }
}

/// Auto-detect build system and build/install the tool
pub fn build_tool(tool_path: &Path, custom_cmd: Option<&str>) -> Result<Option<String>, String> {
    // 1. Custom build command
    if let Some(cmd) = custom_cmd {
        println!("  {} Running custom build: {}", "→".cyan(), cmd);
        #[cfg(target_os = "windows")]
        let status = Command::new("cmd")
            .args(["/C", cmd])
            .current_dir(tool_path)
            .status();

        #[cfg(not(target_os = "windows"))]
        let status = Command::new("sh")
            .args(["-c", cmd])
            .current_dir(tool_path)
            .status();

        return match status {
            Ok(s) if s.success() => Ok(Some("Custom build succeeded.".to_string())),
            Ok(_) => Err("Custom build command failed.".to_string()),
            Err(e) => Err(format!("Failed to execute build command: {}", e)),
        };
    }

    // 2. Rust / Cargo project
    if tool_path.join("Cargo.toml").is_file() {
        println!("  {} Detected Rust project. Compiling...", "→".cyan());
        let output = Command::new("cargo")
            .args(["build"])
            .current_dir(tool_path)
            .output();

        return match output {
            Ok(out) if out.status.success() => Ok(Some("Cargo build completed.".to_string())),
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                Err(format!("Cargo build failed:\n{}", stderr))
            }
            Err(e) => Err(format!("Failed to execute cargo: {}", e)),
        };
    }

    // 3. Node.js project
    if tool_path.join("package.json").is_file() {
        println!("  {} Detected Node.js project. Installing dependencies...", "→".cyan());
        let output = Command::new("npm")
            .args(["install"])
            .current_dir(tool_path)
            .output();

        return match output {
            Ok(out) if out.status.success() => Ok(Some("npm dependencies installed.".to_string())),
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                Err(format!("npm install failed:\n{}", stderr))
            }
            Err(e) => Err(format!("Failed to execute npm: {}", e)),
        };
    }

    // 4. Python project
    if tool_path.join("pyproject.toml").is_file() || tool_path.join("requirements.txt").is_file() {
        println!("  {} Detected Python project.", "→".cyan());
        return Ok(Some("Python project detected.".to_string()));
    }

    Ok(None)
}

/// Safely uninstall / remove a tool directory
pub fn uninstall_tool(tool_path: &Path, tool_name: &str, force: bool) -> Result<(), String> {
    if !tool_path.exists() {
        return Err(format!("Tool '{}' does not exist at {}", tool_name, tool_path.display()));
    }

    let git_status = GitStatus::inspect(tool_path);
    if !force && git_status.is_repo {
        if !git_status.is_clean {
            return Err(format!(
                "'{}' has uncommitted or untracked changes.\nUninstall aborted to protect your work. Use --force to delete anyway.",
                tool_name
            ));
        }
        if git_status.ahead > 0 {
            return Err(format!(
                "'{}' has {} unpushed commit(s).\nUninstall aborted to prevent loss of commits. Use --force to delete anyway.",
                tool_name, git_status.ahead
            ));
        }
    }

    fs::remove_dir_all(tool_path)
        .map_err(|e| format!("Failed to remove directory {}: {}", tool_path.display(), e))
}
