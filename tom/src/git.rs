use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Default)]
pub struct GitStatus {
    pub is_repo: bool,
    pub is_clean: bool,
    pub untracked: bool,
    pub uncommitted_changes: bool,
    pub untracked_count: usize,
    pub modified_count: usize,
    pub staged_count: usize,
    pub ahead: usize,
    pub behind: usize,
    pub branch: Option<String>,
    pub remote_url: Option<String>,
    pub last_commit_hash: Option<String>,
    pub last_commit_summary: Option<String>,
    pub last_commit_date: Option<String>,
    pub last_commit_relative: Option<String>,
}

impl GitStatus {
    /// Inspect the given directory to query Git status
    pub fn inspect(path: &Path) -> Self {
        let is_self = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .eq_ignore_ascii_case("tom");

        let mut check_path = path.to_path_buf();
        if !path.join(".git").exists() {
            if is_self {
                if let Some(parent) = path.parent() {
                    if parent.join(".git").exists() {
                        check_path = parent.to_path_buf();
                    } else {
                        return GitStatus::default();
                    }
                } else {
                    return GitStatus::default();
                }
            } else {
                return GitStatus::default();
            }
        }

        let mut status = GitStatus {
            is_repo: true,
            is_clean: true,
            ..Default::default()
        };

        let path_str = check_path.to_str().unwrap_or(".");

        // 1. Current branch
        if let Ok(out) = Command::new("git")
            .args(["-C", path_str, "branch", "--show-current"])
            .output()
        {
            if out.status.success() {
                let branch_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !branch_str.is_empty() {
                    status.branch = Some(branch_str);
                } else {
                    if let Ok(head_out) = Command::new("git")
                        .args(["-C", path_str, "rev-parse", "--short", "HEAD"])
                        .output()
                    {
                        if head_out.status.success() {
                            let head_str = String::from_utf8_lossy(&head_out.stdout).trim().to_string();
                            if !head_str.is_empty() {
                                status.branch = Some(format!("(HEAD detached at {})", head_str));
                            }
                        }
                    }
                }
            }
        }

        // 2. Remote origin URL
        if let Ok(out) = Command::new("git")
            .args(["-C", path_str, "remote", "get-url", "origin"])
            .output()
        {
            if out.status.success() {
                let url = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !url.is_empty() {
                    status.remote_url = Some(url);
                }
            }
        }

        // 3. Last commit info
        if let Ok(out) = Command::new("git")
            .args(["-C", path_str, "log", "-1", "--format=%h%x00%s%x00%cd%x00%cr"])
            .output()
        {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout);
                let parts: Vec<&str> = text.split('\0').collect();
                if parts.len() >= 4 {
                    status.last_commit_hash = Some(parts[0].trim().to_string());
                    status.last_commit_summary = Some(parts[1].trim().to_string());
                    status.last_commit_date = Some(parts[2].trim().to_string());
                    status.last_commit_relative = Some(parts[3].trim().to_string());
                }
            }
        }

        // 4. Working tree porcelain status
        if let Ok(out) = Command::new("git")
            .args(["-C", path_str, "status", "--porcelain"])
            .output()
        {
            if out.status.success() {
                let output_text = String::from_utf8_lossy(&out.stdout);
                for line in output_text.lines() {
                    if line.starts_with("??") {
                        status.untracked = true;
                        status.untracked_count += 1;
                    } else if line.len() >= 2 {
                        let staged = &line[0..1];
                        let unstaged = &line[1..2];
                        if staged != " " && staged != "?" {
                            status.staged_count += 1;
                        }
                        if unstaged != " " && unstaged != "?" {
                            status.modified_count += 1;
                        }
                        status.uncommitted_changes = true;
                    }
                }
            }
        }

        status.is_clean = !status.uncommitted_changes && !status.untracked;

        // 5. Ahead / Behind count
        if let Ok(out) = Command::new("git")
            .args(["-C", path_str, "rev-list", "--left-right", "--count", "HEAD...@{upstream}"])
            .output()
        {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout);
                let parts: Vec<&str> = text.split_whitespace().collect();
                if parts.len() >= 2 {
                    status.ahead = parts[0].parse().unwrap_or(0);
                    status.behind = parts[1].parse().unwrap_or(0);
                }
            }
        }

        status
    }

    /// Pull updates safely from the remote
    pub fn pull_update(path: &Path, tool_name: &str) -> Result<String, String> {
        let status = Self::inspect(path);
        if !status.is_repo {
            return Err(format!("'{}' is not a Git repository.", tool_name));
        }

        if !status.is_clean {
            return Err(format!(
                "'{}' has uncommitted changes.\nUpdate aborted to protect your local work.",
                tool_name
            ));
        }

        let path_str = path.to_str().unwrap_or(".");
        let output = Command::new("git")
            .args(["-C", path_str, "pull"])
            .output()
            .map_err(|e| format!("Failed to execute git pull: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if output.status.success() {
            if !stdout.is_empty() {
                Ok(stdout)
            } else if !stderr.is_empty() {
                Ok(stderr)
            } else {
                Ok("Already up to date.".to_string())
            }
        } else {
            Err(format!("Git pull failed:\n{}\n{}", stdout, stderr).trim().to_string())
        }
    }

    /// Formatted status tag (e.g., "clean", "dirty", "untracked", "dirty + ahead 2", "no repo")
    pub fn display_status(&self) -> String {
        if !self.is_repo {
            return "no repo".to_string();
        }

        let mut badges = Vec::new();
        if self.uncommitted_changes {
            badges.push("dirty");
        }
        if self.untracked {
            badges.push("untracked");
        }
        if badges.is_empty() {
            badges.push("clean");
        }

        let mut status_str = badges.join(" + ");

        if self.ahead > 0 && self.behind > 0 {
            status_str.push_str(&format!(" (ahead {}, behind {})", self.ahead, self.behind));
        } else if self.ahead > 0 {
            status_str.push_str(&format!(" (ahead {})", self.ahead));
        } else if self.behind > 0 {
            status_str.push_str(&format!(" (behind {})", self.behind));
        }

        status_str
    }
}
