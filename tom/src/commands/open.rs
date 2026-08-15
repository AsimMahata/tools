use colored::*;
use std::path::Path;
use std::process::Command;

use crate::tool::find_tool;

pub fn execute(tool_name: &str, tools_dir: &Path, editor_override: Option<&str>) {
    let tool = match find_tool(tools_dir, tool_name) {
        Some(t) => t,
        None => {
            eprintln!("{} Tool '{}' not found in {}", "Error:".red().bold(), tool_name, tools_dir.display());
            return;
        }
    };

    let target_path = tool.path.to_str().unwrap_or(".");

    // 1. If explicit editor requested or configured
    if let Some(editor) = editor_override {
        if open_with_program(editor, target_path) {
            println!("Opened {} in {}", tool.name.cyan(), editor.bold());
            return;
        }
    }

    // 2. Try VS Code (`code`)
    if open_with_program("code", target_path) {
        println!("Opened {} in {}", tool.name.cyan(), "VS Code".bold());
        return;
    }

    // 3. Fallback to system default file manager
    #[cfg(target_os = "windows")]
    {
        if Command::new("explorer").arg(target_path).spawn().is_ok() {
            println!("Opened {} in File Explorer", tool.name.cyan());
            return;
        }
    }

    #[cfg(target_os = "macos")]
    {
        if Command::new("open").arg(target_path).spawn().is_ok() {
            println!("Opened {} in Finder", tool.name.cyan());
            return;
        }
    }

    #[cfg(target_os = "linux")]
    {
        if Command::new("xdg-open").arg(target_path).spawn().is_ok() {
            println!("Opened {} in default file manager", tool.name.cyan());
            return;
        }
    }

    eprintln!("{} Could not open {}", "Error:".red().bold(), target_path);
}

fn open_with_program(prog: &str, path: &str) -> bool {
    Command::new(prog)
        .arg(path)
        .spawn()
        .map(|_| true)
        .unwrap_or(false)
}
