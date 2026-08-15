use colored::*;
use std::path::Path;

use crate::registry::Registry;
use crate::tool::find_tool;

pub fn execute(tool_name: &str, tools_dir: &Path) {
    let registry = Registry::load(tools_dir);

    if let Some(tool) = find_tool(tools_dir, tool_name) {
        println!("{}", tool.name.bold().cyan());
        println!("{}", "=".repeat(tool.name.len()).cyan());

        if tool.is_self {
            println!("{:<20} {}", "Role:".bold(), "TOM Manager (Self)".cyan());
        }

        let desc = tool
            .description()
            .or_else(|| registry.get(&tool.name).and_then(|r| r.description.as_deref()));

        if let Some(d) = desc {
            println!("{:<20} {}", "Description:".bold(), d);
        }
        if let Some(ver) = tool.version() {
            println!("{:<20} {}", "Version:".bold(), ver);
        }
        if let Some(author) = tool.metadata.as_ref().and_then(|m| m.author.as_deref()) {
            println!("{:<20} {}", "Author:".bold(), author);
        }
        if let Some(tags) = tool.metadata.as_ref().and_then(|m| m.tags.as_ref()) {
            println!("{:<20} {}", "Tags:".bold(), tags.join(", "));
        }

        println!("{:<20} {}", "Local Path:".bold(), tool.path.display());

        if tool.git.is_repo {
            if let Some(ref remote) = tool.git.remote_url {
                println!("{:<20} {}", "Repository URL:".bold(), remote.underline());
            }
            if let Some(ref branch) = tool.git.branch {
                println!("{:<20} {}", "Branch:".bold(), branch.green());
            }
            if let Some(ref hash) = tool.git.last_commit_hash {
                let summary = tool.git.last_commit_summary.as_deref().unwrap_or("");
                println!("{:<20} {} - {}", "Current Commit:".bold(), hash.yellow(), summary);
            }
            if let Some(ref date) = tool.git.last_commit_date {
                let rel = tool.git.last_commit_relative.as_deref().unwrap_or("");
                if !rel.is_empty() {
                    println!("{:<20} {} ({})", "Last Commit Date:".bold(), date, rel.dimmed());
                } else {
                    println!("{:<20} {}", "Last Commit Date:".bold(), date);
                }
            }

            let raw_status = tool.git.display_status();
            let colored_status = if tool.git.is_clean {
                "clean (working tree clean)".green()
            } else {
                let mut details = Vec::new();
                if tool.git.modified_count > 0 {
                    details.push(format!("{} modified", tool.git.modified_count));
                }
                if tool.git.staged_count > 0 {
                    details.push(format!("{} staged", tool.git.staged_count));
                }
                if tool.git.untracked_count > 0 {
                    details.push(format!("{} untracked", tool.git.untracked_count));
                }
                format!("{} ({})", raw_status.yellow(), details.join(", ")).yellow()
            };
            println!("{:<20} {}", "Git Status:".bold(), colored_status);

            if tool.git.ahead > 0 || tool.git.behind > 0 {
                println!(
                    "{:<20} {} ahead, {} behind remote tracking branch",
                    "Sync Status:".bold(),
                    tool.git.ahead,
                    tool.git.behind
                );
            }
        } else {
            println!("{:<20} {}", "Git:".bold(), "Not a Git repository".dimmed());
        }

        let reg_entry = registry.get(&tool.name);
        let req = tool.metadata.as_ref().and_then(|m| m.requirements.as_deref()).or_else(|| reg_entry.and_then(|r| r.requirements.as_deref()));
        let tip = tool.metadata.as_ref().and_then(|m| m.tips.as_deref()).or_else(|| reg_entry.and_then(|r| r.tips.as_deref()));
        let inst_cmd = tool.metadata.as_ref().and_then(|m| m.install_cmd.as_deref()).or_else(|| reg_entry.and_then(|r| r.install_cmd.as_deref()));
        let uninst_cmd = tool.metadata.as_ref().and_then(|m| m.uninstall_cmd.as_deref()).or_else(|| reg_entry.and_then(|r| r.uninstall_cmd.as_deref()));

        if let Some(r) = req {
            println!("{:<20} {}", "Requirements:".bold(), r.cyan());
        }
        if let Some(t) = tip {
            println!("{:<20} {}", "Tips:".bold(), t.dimmed());
        }
        if let Some(ic) = inst_cmd {
            println!("{:<20} {}", "Install Cmd:".bold(), ic.dimmed());
        }
        if let Some(uc) = uninst_cmd {
            println!("{:<20} {}", "Uninstall Cmd:".bold(), uc.dimmed());
        }

        if let Some(modified) = tool.modified_time {
            println!(
                "{:<20} {} ({})",
                "Last Modified:".bold(),
                modified.format("%Y-%m-%d %H:%M:%S"),
                tool.modified_relative.dimmed()
            );
        }
    } else if let Some(entry) = registry.get(tool_name) {
        println!("{}", entry.name.bold().cyan());
        println!("{}", "=".repeat(entry.name.len()).cyan());
        println!("{:<20} {}", "Status:".bold(), "Not Installed (Available in Registry)".yellow());
        if let Some(ref desc) = entry.description {
            println!("{:<20} {}", "Description:".bold(), desc);
        }
        println!("{:<20} {}", "Repository URL:".bold(), entry.repository.underline());
        if let Some(ref tags) = entry.tags {
            println!("{:<20} {}", "Tags:".bold(), tags.join(", "));
        }
        if let Some(ref req) = entry.requirements {
            println!("{:<20} {}", "Requirements:".bold(), req.cyan());
        }
        if let Some(ref tip) = entry.tips {
            println!("{:<20} {}", "Tips:".bold(), tip.dimmed());
        }
        if let Some(ref ic) = entry.install_cmd {
            println!("{:<20} {}", "Install Cmd:".bold(), ic.dimmed());
        }
        println!("\nRun 'tom install {}' to clone and install this tool.", entry.name);
    } else {
        eprintln!(
            "{} Tool '{}' not found in workspace or registry.",
            "Error:".red().bold(),
            tool_name
        );
    }
}
