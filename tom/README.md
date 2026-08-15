# TOM — Tool Manager

**TOM** is a personal tool and package manager for orchestrating a collection of independent Git repositories.

---

## Installation

### Windows 1-Click Installer
From the root of the tools repository:
```cmd
install.bat
```

### Manual Installation
```bash
cargo build --release
copy target\release\tom.exe %USERPROFILE%\.cargo\bin\tom.exe
```

---

## Design Philosophy

* **Sibling Repositories**: TOM and the tools it manages live alongside each other as siblings inside a shared tools directory.
* **Decoupled Lifecycle**: Clean separation between fetching source code, building binaries, uninstalling packages, and purging code.
* **Non-Destructive README Preservation**: Unfetching source code retains the tool's `README.md` stub so the parent repository index remains clean.
* **Portable Location Memory**: During setup, TOM records the containing parent directory as `tools_directory` and manages all tools relative to that directory.

---

## Directory Model

```text
tools/
├── install.bat           ← 1-click Windows installer
├── registry.toml         ← Central registry of tools, steps, and tips
├── tom/                  ← TOM repository
├── netman/               ← Sibling tool (independent Git repository)
├── progit/               ← Sibling tool (independent Git repository)
├── logit/                ← Sibling tool (independent Git repository)
└── sodo/                 ← Sibling tool (independent Git repository)
```

---

## CLI Usage

```bash
# Discovery & Status
tom list                     # Formatted table of tools, status, and installation state
tom status [tool]            # Check working tree and installation status
tom info <tool>              # Deep inspection of tool metadata & requirements

# Source Code Management
tom fetch <tool>             # Fetch repository in-place
tom fetch all                # Fetch all tools from registry
tom update <tool>            # Pull latest changes in-place without removing directory
tom update all               # Update all tools
tom unfetch <tool>           # Purge repository files while preserving README.md

# Build & Installation
tom install <tool>           # Show prerequisites, tips, and run install steps
tom install all              # Install all tools
tom uninstall <tool>         # Run uninstall pipeline while keeping local repository

# Navigation
tom open <tool>              # Open tool in VS Code or default file explorer
```
