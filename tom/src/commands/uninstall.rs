use colored::*;
use std::path::Path;

use crate::installer::uninstall_tool;
use crate::tool::find_tool;

pub fn execute(tool_name: &str, force: bool, tools_dir: &Path) {
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

    println!("Uninstalling {}...", tool.name.cyan().bold());

    match uninstall_tool(&tool.path, &tool.name, force) {
        Ok(_) => {
            println!("{} Successfully removed {}", "✓".green().bold(), tool.name.bold());
        }
        Err(err) => {
            eprintln!("{} {}", "✗".red().bold(), err);
        }
    }
}
