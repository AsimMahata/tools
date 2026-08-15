use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

/// Clone / populate repository into target directory
pub fn clone_repository(repo_url: &str, target_path: &Path) -> Result<(), String> {
    let _ = fs::create_dir_all(target_path);
    clear_readonly(target_path);

    // 1. Initialize git repository in target_path
    let init_res = Command::new("git")
        .arg("init")
        .current_dir(target_path)
        .output()
        .map_err(|e| format!("Failed to execute git init: {}", e))?;

    if !init_res.status.success() {
        let stderr = String::from_utf8_lossy(&init_res.stderr);
        return Err(format!("git init failed: {}", stderr.trim()));
    }

    // 2. Set or update remote origin
    let _ = Command::new("git")
        .args(["remote", "remove", "origin"])
        .current_dir(target_path)
        .output();

    let remote_res = Command::new("git")
        .args(["remote", "add", "origin", repo_url])
        .current_dir(target_path)
        .output()
        .map_err(|e| format!("Failed to set git remote: {}", e))?;

    if !remote_res.status.success() {
        let stderr = String::from_utf8_lossy(&remote_res.stderr);
        return Err(format!("git remote add failed: {}", stderr.trim()));
    }

    // 3. Fetch from remote
    let fetch_res = Command::new("git")
        .args(["fetch", "origin"])
        .current_dir(target_path)
        .output()
        .map_err(|e| format!("Failed to run git fetch: {}", e))?;

    if !fetch_res.status.success() {
        let stderr = String::from_utf8_lossy(&fetch_res.stderr);
        return Err(format!("git fetch failed:\n{}", stderr.trim()));
    }

    // 4. Detect default remote branch
    let branch_out = Command::new("git")
        .args(["remote", "show", "origin"])
        .current_dir(target_path)
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

    // 5. Checkout the branch (overwriting the stub README.md with the real repo files)
    let checkout_res = Command::new("git")
        .args(["checkout", "-f", "-B", &branch, &format!("origin/{}", branch)])
        .current_dir(target_path)
        .output()
        .map_err(|e| format!("Failed to checkout branch: {}", e))?;

    if !checkout_res.status.success() {
        let _ = Command::new("git")
            .args(["checkout", "-f", &branch])
            .current_dir(target_path)
            .output();
    }

    let _ = Command::new("git")
        .args(["branch", "-u", &format!("origin/{}", branch), &branch])
        .current_dir(target_path)
        .output();

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

    // 3. Prioritize local install.bat or install.sh if present
    #[cfg(target_os = "windows")]
    if tool_path.join("install.bat").is_file() {
        println!("  {} Found install.bat. Executing installer script...", "⚙".bold());
        let status = execute_shell_command(tool_path, "cmd /c install.bat -y")?;
        if !status.success() {
            return Err("install.bat execution failed.".to_string());
        }
        println!("  {} install.bat completed successfully.", "✓".green());
        return Ok(());
    }

    #[cfg(not(target_os = "windows"))]
    if tool_path.join("install.sh").is_file() {
        println!("  {} Found install.sh. Executing installer script...", "⚙".bold());
        let status = execute_shell_command(tool_path, "sh install.sh -y")?;
        if !status.success() {
            return Err("install.sh execution failed.".to_string());
        }
        println!("  {} install.sh completed successfully.", "✓".green());
        return Ok(());
    }

    // 4. Execute step-by-step install commands
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
    let cwd_str = cwd.to_str().unwrap_or(".");

    #[cfg(target_os = "windows")]
    let mut command = {
        let mut c = Command::new("powershell");
        let script = format!("Set-Location -LiteralPath '{}'; {}", cwd_str, cmd);
        c.args(["-NoProfile", "-Command", &script]);
        c
    };

    #[cfg(not(target_os = "windows"))]
    let mut command = {
        let mut c = Command::new("sh");
        let script = format!("cd '{}' && {}", cwd_str, cmd);
        c.args(["-c", &script]);
        c
    };

    command.current_dir(cwd);
    let status = command
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
