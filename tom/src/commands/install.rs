use colored::*;
use std::path::Path;

use crate::installer::{clone_repository, run_install_pipeline};
use crate::registry::Registry;
use crate::tool::find_tool;

pub fn execute(target_tool: Option<&str>, all: bool, tools_dir: &Path) {
    let registry = Registry::load(tools_dir);

    if all {
        println!("{}", "Installing all tools...".bold().cyan());
        println!("{}", "=======================".dimmed());

        for (name, _) in &registry.tools {
            if name.eq_ignore_ascii_case("tom") {
                continue; // Skip self
            }
            install_single(name, &registry, tools_dir);
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

    install_single(tool_name, &registry, tools_dir);
}

fn install_single(tool_name: &str, registry: &Registry, tools_dir: &Path) {
    println!("Installing {}...", tool_name.cyan().bold());

    let target_path = tools_dir.join(tool_name);
    let reg_entry = registry.get(tool_name);

    // 1. Ensure repository is fetched
    let tool_opt = find_tool(tools_dir, tool_name);
    if tool_opt.is_none() {
        if let Some(entry) = reg_entry {
            print!("  Tool code not found. Fetching repository... ");
            match clone_repository(&entry.repository, &target_path) {
                Ok(_) => {
                    println!("{}", "✓".green().bold());
                }
                Err(err) => {
                    println!("{}", "✗".red().bold());
                    eprintln!("  {} {}", "✗".red(), err);
                    return;
                }
            }
        } else {
            eprintln!(
                "  {} Tool '{}' is not present in workspace or registry.\n  Run 'tom fetch <repo-url>' first.",
                "Error:".red().bold(),
                tool_name
            );
            return;
        }
    }

    let tool = find_tool(tools_dir, tool_name);
    let steps = tool
        .as_ref()
        .and_then(|t| t.metadata.as_ref())
        .and_then(|m| m.install_steps.as_deref())
        .or_else(|| reg_entry.and_then(|r| r.install_steps.as_deref()));

    let reqs = tool
        .as_ref()
        .and_then(|t| t.metadata.as_ref())
        .and_then(|m| m.requirements.as_deref())
        .or_else(|| reg_entry.and_then(|r| r.requirements.as_deref()));

    let tips = tool
        .as_ref()
        .and_then(|t| t.metadata.as_ref())
        .and_then(|m| m.tips.as_deref())
        .or_else(|| reg_entry.and_then(|r| r.tips.as_deref()));

    // 2. Run step-by-step install pipeline
    match run_install_pipeline(&target_path, steps, reqs, tips) {
        Ok(_) => {
            println!("\n{} {} is built and ready to use.", "✓".green().bold(), tool_name.bold());
        }
        Err(err) => {
            eprintln!("\n{} Installation notice: {}", "⚠".yellow().bold(), err);
        }
    }
}
