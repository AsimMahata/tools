use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::installer::remove_dir_all_force;
use crate::registry::Registry;
use crate::tool::find_tool;

pub fn execute(tool_name: &str, tools_dir: &Path) {
    let registry = Registry::load(tools_dir);

    let tool = match find_tool(tools_dir, tool_name) {
        Some(t) => t,
        None => {
            eprintln!(
                "{} Tool '{}' is not found in {}",
                "Error:".red().bold(),
                tool_name,
                tools_dir.display()
            );
            return;
        }
    };

    if tool.is_self {
        eprintln!(
            "{} Cannot uninstall TOM (self).",
            "Error:".red().bold()
        );
        return;
    }

    let uninstall_steps = tool
        .metadata
        .as_ref()
        .and_then(|m| m.uninstall_steps.as_deref())
        .or_else(|| {
            registry
                .get(&tool.name)
                .and_then(|r| r.uninstall_steps.as_deref())
        });

    println!("Uninstalling {}...", tool.name.cyan().bold());
    if !tool.name.eq_ignore_ascii_case("tom") && !tool.name.eq_ignore_ascii_case("sudow") {
        println!("  {} If uninstallation fails due to permissions, run with '{}' (install via '{}')", "🛡️ Note:".red().bold(), "sudow".red().bold(), "tom install sudow".red());
    }

    let mut ran_script = false;

    // 1. Prioritize local uninstall.bat or uninstall.sh if present
    #[cfg(target_os = "windows")]
    if tool.path.join("uninstall.bat").is_file() {
        println!("  {} Found uninstall.bat. Executing uninstaller script...", "⚙".bold());
        let _ = Command::new("cmd")
            .args(["/C", "uninstall.bat", "-y"])
            .current_dir(&tool.path)
            .status();
        println!("  {} uninstall.bat completed.", "✓".green());
        ran_script = true;
    }

    #[cfg(not(target_os = "windows"))]
    if tool.path.join("uninstall.sh").is_file() {
        println!("  {} Found uninstall.sh. Executing uninstaller script...", "⚙".bold());
        let _ = Command::new("sh")
            .args(["-c", "sh uninstall.sh -y"])
            .current_dir(&tool.path)
            .status();
        println!("  {} uninstall.sh completed.", "✓".green());
        ran_script = true;
    }

    // 2. Run defined uninstallation steps only if no custom script was executed
    if !ran_script {
        if let Some(step_list) = uninstall_steps {
            if !step_list.is_empty() {
                println!("  {} Executing uninstallation steps:", "⚙".bold());
                for (idx, cmd_str) in step_list.iter().enumerate() {
                    println!(
                        "    [{}/{}] Running: {}",
                        idx + 1,
                        step_list.len(),
                        cmd_str.dimmed()
                    );
                    #[cfg(target_os = "windows")]
                    let status = Command::new("powershell")
                        .args(["-NoProfile", "-Command", cmd_str])
                        .current_dir(&tool.path)
                        .status();

                    #[cfg(not(target_os = "windows"))]
                    let status = Command::new("sh")
                        .args(["-c", cmd_str])
                        .current_dir(&tool.path)
                        .status();

                    match status {
                        Ok(s) if s.success() => {
                            println!("    {} Step {} completed.", "✓".green(), idx + 1);
                        }
                        Ok(_) => {
                            eprintln!("    {} Notice: Step completed.", "ℹ".dimmed());
                        }
                        Err(e) => {
                            eprintln!("    {} Failed to run step: {}", "✗".red(), e);
                        }
                    }
                }
            }
        }
    }

    // 2. Remove binary from Cargo bin if present
    let exe_name = if cfg!(target_os = "windows") {
        format!("{}.exe", tool.name)
    } else {
        tool.name.clone()
    };

    if let Some(home) = dirs::home_dir() {
        let cargo_bin = home.join(".cargo").join("bin").join(&exe_name);
        if cargo_bin.exists() {
            let _ = fs::remove_file(&cargo_bin);
            println!("  {} Removed binary from ~/.cargo/bin/{}", "✓".green(), exe_name);
        }
    }

    // 3. Clean target / build directory safely with permission stripping
    let target_dir = tool.path.join("target");
    if target_dir.is_dir() {
        let _ = remove_dir_all_force(&target_dir);
    }

    println!("{} Successfully uninstalled {}.", "✓".green().bold(), tool.name.bold());
    println!("  {} Source files kept. Run '{}' to remove repository files.", "ℹ".dimmed(), format!("tom unfetch {}", tool.name).bold());
}
