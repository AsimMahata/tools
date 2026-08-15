use colored::*;
use std::io::{self, Write};
use std::path::Path;

use crate::installer::{clone_repository, run_install_pipeline};
use crate::registry::Registry;
use crate::tool::find_tool;

pub fn execute(target_tool: Option<&str>, all: bool, auto_yes: bool, tools_dir: &Path) {
    let registry = Registry::load(tools_dir);

    let is_all = all || target_tool.map(|t| t.eq_ignore_ascii_case("all")).unwrap_or(false);

    if is_all {
        println!("{}", "Installing all tools...".bold().cyan());
        println!("{}", "=======================".dimmed());

        for (name, _) in &registry.tools {
            if name.eq_ignore_ascii_case("tom") {
                continue; // Skip self
            }
            install_single(name, auto_yes, &registry, tools_dir);
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

    install_single(tool_name, auto_yes, &registry, tools_dir);
}

fn install_single(tool_name: &str, auto_yes: bool, registry: &Registry, tools_dir: &Path) {
    println!("Installing {}...", tool_name.cyan().bold());

    let target_path = tools_dir.join(tool_name);
    let reg_entry = registry.get(tool_name);
    let tool_opt = find_tool(tools_dir, tool_name);

    // 1. Tell user what is required (Rust, Python, etc.) and tips up-front
    let reqs = tool_opt
        .as_ref()
        .and_then(|t| t.metadata.as_ref())
        .and_then(|m| m.requirements.as_deref())
        .or_else(|| reg_entry.and_then(|r| r.requirements.as_deref()));

    let tips = tool_opt
        .as_ref()
        .and_then(|t| t.metadata.as_ref())
        .and_then(|m| m.tips.as_deref())
        .or_else(|| reg_entry.and_then(|r| r.tips.as_deref()));

    if let Some(r_list) = reqs {
        if !r_list.is_empty() {
            println!("  {} Requirements:", "📋".blue());
            for r in r_list {
                println!("    • {}", r.cyan().bold());
            }
        }
    }

    if let Some(t_list) = tips {
        if !t_list.is_empty() {
            println!("  {} Tips:", "💡".yellow());
            for t in t_list {
                println!("    • {}", t.white());
            }
        }
    }

    // 2. If code does not exist locally, prompt to fetch first
    if tool_opt.is_none() {
        if let Some(entry) = reg_entry {
            println!("\n  {} '{}' is not fetched locally.", "ℹ".cyan(), tool_name.bold());

            let should_fetch = if auto_yes {
                true
            } else {
                prompt_confirmation(&format!("  Do you want to fetch '{}' now from {}?", entry.name.bold(), entry.repository.underline()))
            };

            if !should_fetch {
                println!("  {} Installation cancelled.", "✗".yellow());
                return;
            }

            print!("  Cloning repository... ");
            let _ = io::stdout().flush();
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

    // 3. Run installation steps
    let tool = find_tool(tools_dir, tool_name);
    let steps = tool
        .as_ref()
        .and_then(|t| t.metadata.as_ref())
        .and_then(|m| m.install_steps.as_deref())
        .or_else(|| reg_entry.and_then(|r| r.install_steps.as_deref()));

    match run_install_pipeline(&target_path, steps, None, None) {
        Ok(_) => {
            println!("\n{} {} is built and ready to use.", "✓".green().bold(), tool_name.bold());
        }
        Err(err) => {
            eprintln!("\n{} Installation notice: {}", "⚠".yellow().bold(), err);
            if !tool_name.eq_ignore_ascii_case("tom") && !tool_name.eq_ignore_ascii_case("sudow") {
                eprintln!(
                    "  {} If installation failed due to permissions, run with 'sudow' (run 'tom install sudow' to get sudow).",
                    "💡".yellow()
                );
            }
        }
    }
}

fn prompt_confirmation(prompt: &str) -> bool {
    print!("{} [y/N]: ", prompt);
    let _ = io::stdout().flush();
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim().to_lowercase();
        trimmed == "y" || trimmed == "yes"
    } else {
        false
    }
}
