use colored::*;
use std::path::Path;
use std::process::Command;

use crate::registry::Registry;
use crate::tool::find_tool;

pub fn execute(tool_name: &str, tools_dir: &Path) {
    let registry = Registry::load(tools_dir);

    let tool = match find_tool(tools_dir, tool_name) {
        Some(t) => t,
        None => {
            eprintln!(
                "{} Tool '{}' is not installed in {}",
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
                let status = Command::new("cmd")
                    .args(["/C", cmd_str])
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
                        eprintln!("    {} Notice: Step failed or returned non-zero.", "⚠".yellow());
                    }
                    Err(e) => {
                        eprintln!("    {} Failed to run step: {}", "✗".red(), e);
                    }
                }
            }
        } else {
            println!("  {} No custom uninstall steps defined.", "ℹ".cyan());
        }
    } else {
        println!("  {} No custom uninstall steps defined.", "ℹ".cyan());
    }

    println!("{} Successfully uninstalled {}.", "✓".green().bold(), tool.name.bold());
    println!("  {} Source files kept. Run '{}' to remove repository files.", "ℹ".dimmed(), format!("tom unfetch {}", tool.name).bold());
}
