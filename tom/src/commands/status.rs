use colored::*;
use std::path::Path;
use std::process::Command;

use crate::tool::{discover_tools, find_tool, Tool};

pub fn execute(tool_name: Option<&str>, tools_dir: &Path) {
    if let Some(target) = tool_name {
        let tool = match find_tool(tools_dir, target) {
            Some(t) => t,
            None => {
                eprintln!("{} Tool '{}' not found in {}", "Error:".red().bold(), target, tools_dir.display());
                return;
            }
        };

        print_single_tool_status(&tool);
    } else {
        let tools = discover_tools(tools_dir);
        if tools.is_empty() {
            println!("{}", "No tools found in configured directory:".yellow());
            println!("  {}", tools_dir.display());
            return;
        }

        println!("{}", "Git Status Summary".bold());
        println!("{}", "==================".dimmed());

        for tool in &tools {
            if !tool.git.is_repo {
                println!("{:<20} {}", tool.name.bold(), "Not a Git repository".dimmed());
                continue;
            }

            let status_badge = if tool.git.is_clean {
                "clean".green()
            } else if tool.git.uncommitted_changes {
                "dirty".yellow()
            } else {
                "untracked".magenta()
            };

            let branch = tool.git.branch.as_deref().unwrap_or("unknown");
            let mut sync_info = Vec::new();
            if tool.git.ahead > 0 {
                sync_info.push(format!("↑{}", tool.git.ahead));
            }
            if tool.git.behind > 0 {
                sync_info.push(format!("↓{}", tool.git.behind));
            }
            let sync_str = if sync_info.is_empty() {
                "".to_string()
            } else {
                format!(" [{}]", sync_info.join(" "))
            };

            let mut change_summary = Vec::new();
            if tool.git.modified_count > 0 {
                change_summary.push(format!("{} modified", tool.git.modified_count));
            }
            if tool.git.staged_count > 0 {
                change_summary.push(format!("{} staged", tool.git.staged_count));
            }
            if tool.git.untracked_count > 0 {
                change_summary.push(format!("{} untracked", tool.git.untracked_count));
            }

            let changes_str = if change_summary.is_empty() {
                "".to_string()
            } else {
                format!(" ({})", change_summary.join(", "))
            };

            println!(
                "{:<18} [{}] on {}{}{}",
                tool.name.bold(),
                status_badge,
                branch.cyan(),
                sync_str.cyan(),
                changes_str.yellow()
            );
        }
    }
}

fn print_single_tool_status(tool: &Tool) {
    println!("Tool: {}", tool.name.bold().cyan());
    println!("Path: {}", tool.path.display().to_string().dimmed());

    if !tool.git.is_repo {
        println!("{}", "Not a Git repository.".dimmed());
        return;
    }

    if let Some(ref branch) = tool.git.branch {
        println!("Branch: {}", branch.green());
    }

    if tool.git.ahead > 0 || tool.git.behind > 0 {
        println!(
            "Sync: {} unpushed commit(s) ahead, {} commit(s) behind remote",
            tool.git.ahead, tool.git.behind
        );
    }

    let path_str = tool.path.to_str().unwrap_or(".");
    if let Ok(out) = Command::new("git")
        .args(["-C", path_str, "status", "-s"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if stdout.is_empty() {
            println!("{}", "Working tree is clean.".green());
        } else {
            println!("{}", "\nChanges:".yellow().bold());
            for line in stdout.lines() {
                if line.starts_with("??") {
                    println!("  {} {}", "?".magenta(), &line[3..]);
                } else if line.starts_with('M') || line.starts_with(" M") {
                    println!("  {} {}", "M".yellow(), &line[2..].trim());
                } else if line.starts_with('A') || line.starts_with(" A") {
                    println!("  {} {}", "A".green(), &line[2..].trim());
                } else if line.starts_with('D') || line.starts_with(" D") {
                    println!("  {} {}", "D".red(), &line[2..].trim());
                } else {
                    println!("  {}", line);
                }
            }
        }
    }
}
