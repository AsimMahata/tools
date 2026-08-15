use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::git::GitStatus;
use crate::installer::remove_dir_all_force;

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

    // 1. Read and preserve README.md content before removing code
    let readme_path = tool_path.join("README.md");
    let readme_content = fs::read_to_string(&readme_path).ok();

    // 2. Remove all files and directories in tool_path
    if let Err(e) = remove_dir_all_force(&tool_path) {
        eprintln!("{} Failed to remove directory {}: {}", "✗".red().bold(), tool_path.display(), e);
        return;
    }

    // 3. Recreate directory with ONLY README.md preserved for parent index repository
    let content_to_write = readme_content.or_else(|| {
        let output = Command::new("git")
            .args(["-C", tools_dir.to_str().unwrap_or("."), "show", &format!("HEAD:{}/README.md", tool_name)])
            .output()
            .ok()?;
        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            None
        }
    });

    if let Some(content) = content_to_write {
        let _ = fs::create_dir_all(&tool_path);
        let _ = fs::write(&readme_path, content);
        println!("  {} Preserved {} for parent repository index.", "✓".green(), "README.md".bold());
    }

    println!("{} Successfully unfetched {}. Repository files removed.", "✓".green().bold(), tool_name.bold());
}
