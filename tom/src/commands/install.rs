use colored::*;
use std::path::Path;

use crate::installer::{build_tool, clone_repository};
use crate::registry::{Registry, RegistryEntry};
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
            install_single(entry, tools_dir);
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
        install_single(entry, tools_dir);
    } else if tool_name.starts_with("http://")
        || tool_name.starts_with("https://")
        || tool_name.starts_with("git@")
    {
        // Direct git clone URL
        let inferred_name = tool_name
            .trim_end_matches(".git")
            .split('/')
            .last()
            .unwrap_or("unnamed_tool")
            .to_string();

        let entry = RegistryEntry {
            name: inferred_name,
            description: None,
            repository: tool_name.to_string(),
            tags: None,
            install_cmd: None,
            uninstall_cmd: None,
            requirements: None,
            tips: None,
        };
        install_single(&entry, tools_dir);
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

fn install_single(entry: &RegistryEntry, tools_dir: &Path) {
    println!("Installing {}...", entry.name.cyan().bold());

    let target_path = tools_dir.join(&entry.name);

    if target_path.exists() {
        if find_tool(tools_dir, &entry.name).is_some() {
            println!("  {} {} is already installed at {}", "ℹ".cyan(), entry.name.bold(), target_path.display());
            return;
        }
    }

    // 1. Clone repository
    print!("  Cloning repository... ");
    match clone_repository(&entry.repository, &target_path) {
        Ok(_) => {
            println!("{}", "✓".green().bold());
            println!("  {} Repository cloned from {}", "✓".green(), entry.repository.dimmed());
        }
        Err(err) => {
            println!("{}", "✗".red().bold());
            eprintln!("  {} {}", "✗".red(), err);
            return;
        }
    }

    // 2. Run build / installation procedure with requirements & tips
    match build_tool(
        &target_path,
        entry.install_cmd.as_deref(),
        entry.requirements.as_deref(),
        entry.tips.as_deref(),
    ) {
        Ok(Some(msg)) => {
            println!("  {} {}", "✓".green(), msg);
        }
        Ok(None) => {}
        Err(err) => {
            eprintln!("  {} Build notice: {}", "⚠".yellow(), err);
        }
    }

    println!("\n{} {} is ready.", "✓".green().bold(), entry.name.bold());
}
