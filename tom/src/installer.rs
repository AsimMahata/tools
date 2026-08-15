use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use crate::git::GitStatus;

/// Clone / populate repository into target directory reliably, even if directory or stub README exists
pub fn clone_repository(repo_url: &str, target_path: &Path) -> Result<(), String> {
    let _ = fs::create_dir_all(target_path);
    let target_str = target_path.to_str().unwrap_or(".");

    // Check if target directory already has a git repo
    if target_path.join(".git").exists() {
        return Err(format!(
            "Directory already contains a Git repository: {}",
            target_path.display()
        ));
    }

    // 1. Initialize git in target directory
    let init_out = Command::new("git")
        .args(["-C", target_str, "init"])
        .output()
        .map_err(|e| format!("Failed to run git init: {}", e))?;

    if !init_out.status.success() {
        let stderr = String::from_utf8_lossy(&init_out.stderr);
        return Err(format!("git init failed: {}", stderr));
    }

    // 2. Add remote origin
    let _ = Command::new("git")
        .args(["-C", target_str, "remote", "remove", "origin"])
        .output();

    let remote_out = Command::new("git")
        .args(["-C", target_str, "remote", "add", "origin", repo_url])
        .output()
        .map_err(|e| format!("Failed to set git remote: {}", e))?;

    if !remote_out.status.success() {
        let stderr = String::from_utf8_lossy(&remote_out.stderr);
        return Err(format!("git remote add failed: {}", stderr));
    }

    // 3. Fetch all objects from origin
    let fetch_out = Command::new("git")
        .args(["-C", target_str, "fetch", "origin"])
        .output()
        .map_err(|e| format!("Failed to run git fetch: {}", e))?;

    if !fetch_out.status.success() {
        let stderr = String::from_utf8_lossy(&fetch_out.stderr);
        return Err(format!("git fetch failed:\n{}", stderr));
    }

    // 4. Detect default remote branch (main or master)
    let branch_out = Command::new("git")
        .args(["-C", target_str, "remote", "show", "origin"])
        .output();

    let mut branch = "main".to_string();
    if let Ok(b_out) = branch_out {
        let text = String::from_utf8_lossy(&b_out.stdout);
        for line in text.lines() {
            if line.contains("HEAD branch:") {
                if let Some(b) = line.split(':').nth(1) {
                    let trimmed = b.trim();
                    if !trimmed.is_empty() {
                        branch = trimmed.to_string();
                    }
                }
            }
        }
    }

    // 5. Checkout the branch cleanly and force overwrite any stub files
    let checkout_out = Command::new("git")
        .args([
            "-C",
            target_str,
            "checkout",
            "-f",
            "-B",
            &branch,
            &format!("origin/{}", branch),
        ])
        .output()
        .map_err(|e| format!("Failed to checkout branch: {}", e))?;

    if !checkout_out.status.success() {
        // Fallback checkout attempt
        let _ = Command::new("git")
            .args(["-C", target_str, "checkout", "-f", &branch])
            .output();
    }

    let _ = Command::new("git")
        .args([
            "-C",
            target_str,
            "branch",
            "-u",
            &format!("origin/{}", branch),
            &branch,
        ])
        .output();

    Ok(())
}

/// Auto-detect build system or run custom install command
pub fn build_tool(
    tool_path: &Path,
    custom_cmd: Option<&str>,
    requirements: Option<&str>,
    tips: Option<&str>,
) -> Result<Option<String>, String> {
    if let Some(req) = requirements {
        println!("  {} Requirements: {}", "ℹ".blue(), req.cyan());
    }
    if let Some(tip) = tips {
        println!("  {} Tip: {}", "💡".yellow(), tip.dimmed());
    }

    // 1. Custom install command
    if let Some(cmd) = custom_cmd {
        println!("  {} Running install command: {}", "→".cyan(), cmd.bold());
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
            Ok(s) if s.success() => Ok(Some("Install command completed successfully.".to_string())),
            Ok(_) => Err("Install command failed.".to_string()),
            Err(e) => Err(format!("Failed to execute install command: {}", e)),
        };
    }

    // 2. Rust / Cargo project
    if tool_path.join("Cargo.toml").is_file() {
        println!("  {} Detected Rust project. Compiling with cargo...", "→".cyan());
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

    // 3. Python project
    if tool_path.join("pyproject.toml").is_file() || tool_path.join("requirements.txt").is_file() {
        println!("  {} Detected Python project.", "→".cyan());
        return Ok(Some("Python project ready.".to_string()));
    }

    // 4. Node.js project
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

    Ok(None)
}

/// Safely uninstall tool code/git repository while preserving its README.md index file
pub fn uninstall_tool(
    tool_path: &Path,
    tool_name: &str,
    force: bool,
    uninstall_cmd: Option<&str>,
) -> Result<(), String> {
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

    // 1. Run custom uninstall command if specified
    if let Some(cmd) = uninstall_cmd {
        println!("  {} Running uninstall command: {}", "→".cyan(), cmd.dimmed());
        #[cfg(target_os = "windows")]
        let _ = Command::new("cmd")
            .args(["/C", cmd])
            .current_dir(tool_path)
            .status();

        #[cfg(not(target_os = "windows"))]
        let _ = Command::new("sh")
            .args(["-c", cmd])
            .current_dir(tool_path)
            .status();

        sleep(Duration::from_millis(300));
    }

    // 2. Read and preserve README.md content before removing code
    let readme_path = tool_path.join("README.md");
    let readme_content = fs::read_to_string(&readme_path).ok();

    // 3. Remove all files and directories in tool_path
    remove_dir_all_force(tool_path)
        .map_err(|e| format!("Failed to remove directory {}: {}", tool_path.display(), e))?;

    // 4. Recreate directory with ONLY README.md preserved for parent index repository
    if let Some(content) = readme_content {
        let _ = fs::create_dir_all(tool_path);
        let _ = fs::write(&readme_path, content);
        println!("  {} Preserved {} for parent repository index", "✓".green(), "README.md".bold());
    }

    Ok(())
}

/// Recursively delete directory tree, explicitly clearing read-only flags on Windows with retry
fn remove_dir_all_force(path: &Path) -> std::io::Result<()> {
    let mut last_err = None;
    for i in 0..5 {
        if i > 0 {
            sleep(Duration::from_millis(300 * i));
        }
        match try_remove_dir_all_force(path) {
            Ok(_) => return Ok(()),
            Err(e) => {
                last_err = Some(e);
            }
        }
    }
    Err(last_err.unwrap_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to remove directory")
    }))
}

fn try_remove_dir_all_force(path: &Path) -> std::io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                try_remove_dir_all_force(&entry_path)?;
            } else {
                clear_readonly(&entry_path);
                let _ = fs::remove_file(&entry_path);
            }
        }
        clear_readonly(path);
        fs::remove_dir(path)?;
    } else if path.exists() {
        clear_readonly(path);
        fs::remove_file(path)?;
    }
    Ok(())
}

fn clear_readonly(path: &Path) {
    if let Ok(metadata) = fs::symlink_metadata(path) {
        let mut perms = metadata.permissions();
        if perms.readonly() {
            perms.set_readonly(false);
            let _ = fs::set_permissions(path, perms);
        }
    }
}
