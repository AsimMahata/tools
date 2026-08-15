# Personal Tools Ecosystem

A curated collection of modular, independent command-line utilities and personal tools managed by **TOM** (Tool Manager).

Each tool in this workspace is an independent Git repository. This parent repository tracks the ecosystem index, tool documentation stubs, and the TOM orchestrator.

---

## ⚡ Quick Start: 1-Click Windows Installation

To install **TOM** and add it to your system PATH, run the included batch installer:

```cmd
install.bat
```

Or double-click `install.bat` in File Explorer.

> **Prerequisites**: [Rust & Cargo](https://rustup.rs/) installed on your machine.

---

## 🛠️ Tool Index

| Tool | Description | Repository | Role |
| :--- | :--- | :--- | :--- |
| **[`tom`](./tom/)** | Tool Manager & CLI orchestrator for personal tool collections | [AsimMahata/tom](https://github.com/AsimMahata/tom) | Core Manager |
| **[`netman`](./netman/)** | Windows network management and diagnostic CLI | [AsimMahata/netman](https://github.com/AsimMahata/netman) | Active Tool |
| **[`progit`](./progit/)** | Git productivity enhancements and workflow helper | [AsimMahata/progit](https://github.com/AsimMahata/progit) | Active Tool |
| **[`logit`](./logit/)** | Log inspection and management utility | [AsimMahata/logit](https://github.com/AsimMahata/logit) | Active Tool |
| **[`sodo`](./sodo/)** | Fast task management and todo tracker CLI | [AsimMahata/sodo](https://github.com/AsimMahata/sodo) | Active Tool |
| **[`sudow`](./sudow/)** | Windows sudo-style command wrapper for seamless elevation | [AsimMahata/sudow](https://github.com/AsimMahata/sudow) | Active Tool |

---

## 📂 Architecture & Directory Model

```text
tools/                     ← Dedicated parent tools repository
├── README.md              ← Ecosystem landing page
├── install.bat            ← 1-click Windows installer
├── registry.toml          ← Tool metadata, build pipelines & tips
├── tom/                   ← TOM tool manager (full source tracked)
├── netman/                ← Sibling tool (independent Git repo / README stub)
├── progit/                ← Sibling tool (independent Git repo / README stub)
├── logit/                 ← Sibling tool (independent Git repo / README stub)
├── sodo/                  ← Sibling tool (independent Git repo / README stub)
└── sudow/                 ← Sibling tool (independent Git repo / README stub)
```

---

## 🚀 Core Lifecycle Commands

TOM provides a decoupled 4-command lifecycle for managing your tools:

### 1. Discovery & Status
```bash
# Formatted table of all tools with Git & Installation state
tom list

# Detailed Git sync + installation status across all tools
tom status

# Detailed metadata, requirements, and commit history for a tool
tom info <tool>
```

### 2. Fetching & Updating (Source Code)
```bash
# Fetch a tool's source code from GitHub into its directory
tom fetch <tool>

# Fetch all tools defined in the registry
tom fetch all

# Pull latest Git commits in-place and rebuild if updated
tom update <tool>
tom update all
```

### 3. Building & Installing (Binaries)
```bash
# Display requirements & tips, then compile and install
tom install <tool>

# Install all tools in the registry
tom install all
```

### 4. Uninstallation & Unfetching
```bash
# Run uninstall scripts/cleanup while keeping the local repository
tom uninstall <tool>

# Purge source code and .git while preserving README.md for the parent repository
tom unfetch <tool>
```

### 5. Navigation
```bash
# Open tool directory in VS Code or default file manager
tom open <tool>
```
