use colored::*;
use std::fs;
use std::path::Path;

use crate::git::GitStatus;
use crate::installer::remove_tool_contents_except_readme;

pub fn execute(tool_name: &str, force: bool, tools_dir: &Path) {
    let tool_path = tools_dir.join(tool_name);
    if !tool_path.exists() {
        eprintln!(
            "{} Tool '{}' does not exist in {}",
            "Error:".red().bold(),
            tool_name,
            tools_dir.display()
        );
        return;
    }

    if tool_name.eq_ignore_ascii_case("tom") {
        eprintln!(
            "{} Cannot unfetch/purge TOM (self).",
            "Error:".red().bold()
        );
        return;
    }

    // Check if it's already just a stub README
    let entries: Vec<_> = fs::read_dir(&tool_path)
        .map(|rd| rd.flatten().collect())
        .unwrap_or_default();

    let is_only_readme = entries.iter().all(|e| {
        let name = e.file_name();
        let name_str = name.to_string_lossy();
        name_str.eq_ignore_ascii_case("readme.md")
    });

    if is_only_readme {
        println!("  {} {} is already unfetched (only README.md present).", "ℹ".cyan(), tool_name.bold());
        return;
    }

    let git_status = GitStatus::inspect(&tool_path);
    if !force && git_status.is_repo {
        if !git_status.is_clean {
            eprintln!(
                "{} '{}' has uncommitted or untracked changes.\nUnfetch aborted to protect your work. Use --force to delete anyway.",
                "Error:".red().bold(),
                tool_name
            );
            return;
        }
        if git_status.ahead > 0 {
            eprintln!(
                "{} '{}' has {} unpushed commit(s).\nUnfetch aborted to prevent loss of commits. Use --force to delete anyway.",
                "Error:".red().bold(),
                tool_name, git_status.ahead
            );
            return;
        }
    }

    println!("Unfetching / purging {} repository files...", tool_name.cyan().bold());

    // Delete all repository files and folders WITHOUT touching README.md
    if let Err(e) = remove_tool_contents_except_readme(&tool_path) {
        eprintln!("{} Failed to remove files in {}: {}", "✗".red().bold(), tool_path.display(), e);
        return;
    }

    println!("  {} Preserved {} untouched for parent repository index.", "✓".green(), "README.md".bold());
    println!("{} Successfully unfetched {}. Repository files removed.", "✓".green().bold(), tool_name.bold());
}
