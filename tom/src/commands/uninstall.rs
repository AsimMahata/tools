use colored::*;
use std::path::Path;

use crate::installer::uninstall_tool;
use crate::registry::Registry;
use crate::tool::find_tool;

pub fn execute(tool_name: &str, force: bool, tools_dir: &Path) {
    let registry = Registry::load(tools_dir);

    let tool = match find_tool(tools_dir, tool_name) {
        Some(t) => t,
        None => {
            eprintln!(
                "{} Tool '{}' not found in {}",
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

    let uninstall_cmd = tool
        .metadata
        .as_ref()
        .and_then(|m| m.uninstall_cmd.as_deref())
        .or_else(|| {
            registry
                .get(&tool.name)
                .and_then(|r| r.uninstall_cmd.as_deref())
        });

    println!("Uninstalling {}...", tool.name.cyan().bold());

    match uninstall_tool(&tool.path, &tool.name, force, uninstall_cmd) {
        Ok(_) => {
            println!("{} Successfully uninstalled {}", "✓".green().bold(), tool.name.bold());
        }
        Err(err) => {
            eprintln!("{} {}", "✗".red().bold(), err);
        }
    }
}
