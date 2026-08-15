use colored::*;
use std::path::Path;

use crate::installer::{build_tool, clone_repository};
use crate::registry::Registry;
use crate::tool::find_tool;

pub fn execute(target_tool: Option<&str>, all: bool, tools_dir: &Path) {
    let registry = Registry::load(tools_dir);

    if all {
        println!("{}", "Installing all tools from registry...".bold().cyan());
        println!("{}", "=====================================".dimmed());

        for (name, entry) in &registry.tools {
            if name.eq_ignore_ascii_case("tom") {
                continue; // Skip self
            }
            install_single(name, &entry.repository, entry.build_cmd.as_deref(), tools_dir);
            println!();
        }
        return;
    }

    let tool_name = match target_tool {
        Some(t) => t,
        None => {
            eprintln!("{} Please specify a tool name to install or use --all.", "Error:".red().bold());
            eprintln!("Usage: tom install <tool> or tom install --all");
            return;
        }
    };

    if let Some(entry) = registry.get(tool_name) {
        install_single(&entry.name, &entry.repository, entry.build_cmd.as_deref(), tools_dir);
    } else if tool_name.starts_with("http://")
        || tool_name.starts_with("https://")
        || tool_name.starts_with("git@")
    {
        // Direct git clone URL
        let inferred_name = tool_name
            .trim_end_matches(".git")
            .split('/')
            .last()
            .unwrap_or("unnamed_tool");
        install_single(inferred_name, tool_name, None, tools_dir);
    } else {
        eprintln!(
            "{} Tool '{}' not found in registry.",
            "Error:".red().bold(),
            tool_name
        );
        println!("\nAvailable tools in registry:");
        for (name, entry) in &registry.tools {
            let desc = entry.description.as_deref().unwrap_or("");
            println!("  - {:<12} {}", name.bold(), desc.dimmed());
        }
    }
}

fn install_single(name: &str, repo_url: &str, custom_cmd: Option<&str>, tools_dir: &Path) {
    println!("Installing {}...", name.cyan().bold());

    let target_path = tools_dir.join(name);

    if target_path.exists() {
        if find_tool(tools_dir, name).is_some() {
            println!("  {} {} is already installed at {}", "ℹ".cyan(), name.bold(), target_path.display());
            return;
        }
    }

    // 1. Clone repository
    print!("  Cloning repository... ");
    match clone_repository(repo_url, &target_path) {
        Ok(_) => {
            println!("{}", "✓".green().bold());
            println!("  {} Repository cloned from {}", "✓".green(), repo_url.dimmed());
        }
        Err(err) => {
            println!("{}", "✗".red().bold());
            eprintln!("  {} {}", "✗".red(), err);
            return;
        }
    }

    // 2. Run build / installation procedure
    match build_tool(&target_path, custom_cmd) {
        Ok(Some(msg)) => {
            println!("  {} {}", "✓".green(), msg);
        }
        Ok(None) => {}
        Err(err) => {
            eprintln!("  {} Build error: {}", "⚠".yellow(), err);
        }
    }

    println!("\n{} {} is ready.", "✓".green().bold(), name.bold());
}
