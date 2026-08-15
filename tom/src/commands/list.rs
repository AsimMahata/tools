use colored::*;
use std::path::Path;

use crate::registry::Registry;
use crate::tool::discover_tools;

pub fn execute(tools_dir: &Path) {
    let tools = discover_tools(tools_dir);
    let registry = Registry::load(tools_dir);

    if tools.is_empty() {
        println!("{}", "No tools found in configured directory:".yellow());
        println!("  {}", tools_dir.display());
        return;
    }

    // Determine column widths
    let mut name_width = 4; // "NAME"
    let mut status_width = 6; // "STATUS"
    let mut commit_width = 11; // "LAST COMMIT"
    let mut mod_width = 8; // "MODIFIED"

    for tool in &tools {
        let display_name_len = if tool.is_self {
            tool.name.len() + 7 // " (self)"
        } else {
            tool.name.len()
        };
        if display_name_len > name_width {
            name_width = display_name_len;
        }

        let status_len = tool.git.display_status().len();
        if status_len > status_width {
            status_width = status_len;
        }
        let commit_len = tool.git.last_commit_hash.as_deref().unwrap_or("-").len();
        if commit_len > commit_width {
            commit_width = commit_len;
        }
        if tool.modified_relative.len() > mod_width {
            mod_width = tool.modified_relative.len();
        }
    }

    let install_width = 9; // "INSTALLED"

    // Header
    println!(
        "{:<nw$}  {:<sw$}  {:<iw$}  {:<cw$}  {:<mw$}  {}",
        "NAME".bold(),
        "STATUS".bold(),
        "INSTALLED".bold(),
        "LAST COMMIT".bold(),
        "MODIFIED".bold(),
        "DESCRIPTION".bold(),
        nw = name_width,
        sw = status_width,
        iw = install_width,
        cw = commit_width,
        mw = mod_width
    );

    // Rows
    for tool in &tools {
        let plain_name = if tool.is_self {
            format!("{} [self]", tool.name)
        } else {
            tool.name.clone()
        };

        let raw_status = tool.git.display_status();
        let commit_str = tool
            .git
            .last_commit_hash
            .as_deref()
            .unwrap_or("-");
        let desc_str = tool
            .description()
            .or_else(|| {
                registry
                    .get(&tool.name)
                    .and_then(|r| r.description.as_deref())
            })
            .unwrap_or("");

        let is_inst = tool.is_installed();
        let install_str = if is_inst { "✓ yes" } else { "✗ no" };

        // Pad plain text to required column widths
        let padded_name = format!("{:<nw$}", plain_name, nw = name_width);
        let padded_status = format!("{:<sw$}", raw_status, sw = status_width);
        let padded_inst = format!("{:<iw$}", install_str, iw = install_width);
        let padded_commit = format!("{:<cw$}", commit_str, cw = commit_width);
        let padded_mod = format!("{:<mw$}", tool.modified_relative, mw = mod_width);

        let colored_name = if tool.is_self {
            padded_name.cyan().bold()
        } else {
            padded_name.bold()
        };

        let colored_status = if raw_status == "clean" {
            padded_status.green()
        } else if raw_status.contains("dirty") {
            padded_status.yellow()
        } else if raw_status.contains("untracked") {
            padded_status.magenta()
        } else if raw_status == "no repo" {
            padded_status.dimmed()
        } else {
            padded_status.cyan()
        };

        let colored_inst = if is_inst {
            padded_inst.green()
        } else {
            padded_inst.yellow().dimmed()
        };

        println!(
            "{}  {}  {}  {}  {}  {}",
            colored_name,
            colored_status,
            colored_inst,
            padded_commit.dimmed(),
            padded_mod.dimmed(),
            desc_str
        );
    }

    // Check for any uninstalled tools from the registry
    let uninstalled: Vec<_> = registry
        .tools
        .iter()
        .filter(|(name, _)| !tools.iter().any(|t| t.name.eq_ignore_ascii_case(name)))
        .collect();

    if !uninstalled.is_empty() {
        println!("\n{}", "Available to install from registry:".dimmed());
        for (name, entry) in uninstalled {
            let desc = entry.description.as_deref().unwrap_or("");
            println!("  + {:<12} {} (run 'tom install {}')", name.green(), desc.dimmed(), name);
        }
    }
}
