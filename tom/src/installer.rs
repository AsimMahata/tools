use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

/// Clone repository using `git clone` into target directory
pub fn clone_repository(repo_url: &str, target_path: &Path) -> Result<(), String> {
    let parent_dir = target_path.parent().unwrap_or_else(|| Path::new("."));
    let tool_name = target_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("tool");
    let temp_clone_path = parent_dir.join(format!(".tom_clone_{}", tool_name));

    // Clean up any stale temp clone directory
    let _ = remove_dir_all_force(&temp_clone_path);

    // 1. Run git clone
    let output = Command::new("git")
        .args(["clone", repo_url, temp_clone_path.to_str().unwrap_or(".")])
        .output()
        .map_err(|e| format!("Failed to execute git clone: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let _ = remove_dir_all_force(&temp_clone_path);
        return Err(format!("git clone failed:\n{}", stderr.trim()));
    }

    // 2. Ensure target directory exists
    let _ = fs::create_dir_all(target_path);
    clear_readonly(target_path);

    // 3. Move all cloned contents into target directory
    let entries = fs::read_dir(&temp_clone_path)
        .map_err(|e| format!("Failed to read cloned files: {}", e))?;

    for entry in entries.flatten() {
        let src = entry.path();
        let name = entry.file_name();
        let dest = target_path.join(&name);

        if dest.exists() {
            clear_readonly(&dest);
            if dest.is_dir() {
                let _ = remove_dir_all_force(&dest);
            } else {
                let _ = fs::remove_file(&dest);
            }
        }

        if let Err(_) = fs::rename(&src, &dest) {
            if src.is_dir() {
                let _ = copy_dir_all(&src, &dest);
            } else {
                let _ = fs::copy(&src, &dest);
            }
        }
    }

    // 4. Remove temp clone directory
    let _ = remove_dir_all_force(&temp_clone_path);

    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Execute sequential install steps or auto-detect build system
pub fn run_install_pipeline(
    tool_path: &Path,
    steps: Option<&[String]>,
    requirements: Option<&[String]>,
    tips: Option<&[String]>,
) -> Result<(), String> {
    // 1. Display Requirements
    if let Some(reqs) = requirements {
        if !reqs.is_empty() {
            println!("  {} Requirements:", "ℹ".blue());
            for r in reqs {
                println!("    • {}", r.cyan());
            }
        }
    }

    // 2. Display Tips
    if let Some(t_list) = tips {
        if !t_list.is_empty() {
            println!("  {} Tips:", "💡".yellow());
            for t in t_list {
                println!("    • {}", t.dimmed());
            }
        }
    }

    // 3. Execute step-by-step install commands
    if let Some(step_list) = steps {
        if !step_list.is_empty() {
            println!("  {} Executing installation steps:", "⚙".bold());
            for (idx, cmd_str) in step_list.iter().enumerate() {
                println!(
                    "    [{}/{}] Running: {}",
                    idx + 1,
                    step_list.len(),
                    cmd_str.bold()
                );

                let status = execute_shell_command(tool_path, cmd_str)?;
                if !status.success() {
                    return Err(format!("Step failed: '{}'", cmd_str));
                }
                println!("    {} Step {} completed.", "✓".green(), idx + 1);
            }
            return Ok(());
        }
    }

    // 4. Auto-detection fallback if no steps specified
    if tool_path.join("Cargo.toml").is_file() {
        println!("  {} Detected Rust project. Running: {}", "→".cyan(), "cargo build --release".bold());
        let status = execute_shell_command(tool_path, "cargo build --release")?;
        if !status.success() {
            return Err("Cargo build failed.".to_string());
        }
    } else if tool_path.join("pyproject.toml").is_file() || tool_path.join("requirements.txt").is_file() {
        println!("  {} Detected Python project. Running: {}", "→".cyan(), "pip install -e .".bold());
        let _ = execute_shell_command(tool_path, "pip install -e .");
    } else if tool_path.join("package.json").is_file() {
        println!("  {} Detected Node.js project. Running: {}", "→".cyan(), "npm install".bold());
        let status = execute_shell_command(tool_path, "npm install")?;
        if !status.success() {
            return Err("npm install failed.".to_string());
        }
    }

    Ok(())
}

pub fn execute_shell_command(cwd: &Path, cmd: &str) -> Result<std::process::ExitStatus, String> {
    #[cfg(target_os = "windows")]
    let status = Command::new("cmd")
        .args(["/C", cmd])
        .current_dir(cwd)
        .status()
        .map_err(|e| format!("Failed to execute command '{}': {}", cmd, e))?;

    #[cfg(not(target_os = "windows"))]
    let status = Command::new("sh")
        .args(["-c", cmd])
        .current_dir(cwd)
        .status()
        .map_err(|e| format!("Failed to execute command '{}': {}", cmd, e))?;

    Ok(status)
}

/// Recursively delete all files and directories in a tool directory EXCEPT README.md
pub fn remove_tool_contents_except_readme(tool_path: &Path) -> std::io::Result<()> {
    if !tool_path.is_dir() {
        return Ok(());
    }

    let mut last_err = None;

    // Retry loop to handle brief Windows file locks from editors/language servers
    for i in 0..5 {
        if i > 0 {
            sleep(Duration::from_millis(250 * i));
        }

        let mut failed = false;

        let entries = match fs::read_dir(tool_path) {
            Ok(e) => e,
            Err(e) => {
                last_err = Some(e);
                continue;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = entry.file_name();
            let name_str = file_name.to_string_lossy();

            // STRICTLY preserve README.md untouched
            if name_str.eq_ignore_ascii_case("readme.md") {
                continue;
            }

            clear_readonly(&path);

            if path.is_dir() {
                if let Err(e) = remove_dir_all_force(&path) {
                    last_err = Some(e);
                    failed = true;
                }
            } else {
                if let Err(e) = fs::remove_file(&path) {
                    last_err = Some(e);
                    failed = true;
                }
            }
        }

        if !failed {
            return Ok(());
        }
    }

    if let Some(e) = last_err {
        Err(e)
    } else {
        Ok(())
    }
}

/// Recursively delete directory tree, explicitly clearing read-only flags on Windows with retry
pub fn remove_dir_all_force(path: &Path) -> std::io::Result<()> {
    let mut last_err = None;
    for i in 0..5 {
        if i > 0 {
            sleep(Duration::from_millis(250 * i));
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
    clear_readonly(path);
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            clear_readonly(&entry_path);
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
