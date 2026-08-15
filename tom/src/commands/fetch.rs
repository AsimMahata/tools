use colored::*;
use std::path::Path;

use crate::installer::clone_repository;
use crate::registry::{Registry, RegistryEntry};
use crate::tool::find_tool;

pub fn execute(target_tool: Option<&str>, all: bool, tools_dir: &Path) {
    let registry = Registry::load(tools_dir);

    let is_all = all || target_tool.map(|t| t.eq_ignore_ascii_case("all")).unwrap_or(false);

    if is_all {
        println!("{}", "Fetching all tools from registry...".bold().cyan());
        println!("{}", "===================================".dimmed());

        for (name, entry) in &registry.tools {
            if name.eq_ignore_ascii_case("tom") {
                continue; // Skip self
            }
            fetch_single(entry, tools_dir);
            println!();
        }
        return;
    }

    let tool_name = match target_tool {
        Some(t) => t,
        None => {
            eprintln!(
                "{} Please specify a tool name to fetch or use --all.",
                "Error:".red().bold()
            );
            eprintln!("Usage: tom fetch <tool> or tom fetch --all");
            return;
        }
    };

    if let Some(entry) = registry.get(tool_name) {
        fetch_single(entry, tools_dir);
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
            requirements: None,
            install_steps: None,
            uninstall_steps: None,
            tips: None,
        };
        fetch_single(&entry, tools_dir);
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

fn fetch_single(entry: &RegistryEntry, tools_dir: &Path) {
    println!("Fetching {}...", entry.name.cyan().bold());

    let target_path = tools_dir.join(&entry.name);

    if let Some(tool) = find_tool(tools_dir, &entry.name) {
        if !tool.is_self {
            println!(
                "  {} {} is already fetched at {}",
                "ℹ".cyan(),
                entry.name.bold(),
                target_path.display()
            );
            println!("  Run 'tom update {}' to pull changes or 'tom install {}' to build.", entry.name, entry.name);
            return;
        }
    }

    // Clone/populate full repository
    print!("  Cloning repository... ");
    match clone_repository(&entry.repository, &target_path) {
        Ok(_) => {
            println!("{}", "✓".green().bold());
            println!("  {} Repository fetched from {}", "✓".green(), entry.repository.dimmed());
            println!("  {} Run '{}' to build and configure.", "→".cyan(), format!("tom install {}", entry.name).bold());
        }
        Err(err) => {
            println!("{}", "✗".red().bold());
            eprintln!("  {} {}", "✗".red(), err);
        }
    }
}
