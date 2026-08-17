# Personal Tools Ecosystem

A curated collection of modular, independent personal CLI tools managed and orchestrated through **TOM** (Tool Manager).

The parent `tools` repository tracks the ecosystem index ([`registry.toml`](./registry.toml)), workspace configuration, 1-click installer, and the core **TOM** orchestrator source code. Individual tools exist as independent repositories hosted under the ecosystem directory.

---

## ⚡ Quick Start: Windows Installation

To install **TOM** and add it to your system PATH, run the included batch installer:

```cmd
install.bat
```

Or double-click `install.bat` in File Explorer. You can also pass `-y` to skip the confirmation prompt:

```cmd
install.bat -y
```

> **Prerequisites**: [Rust & Cargo](https://rustup.rs/) installed on your machine (`cargo` in PATH).

The installer compiles TOM in release mode and copies `tom.exe` to `%USERPROFILE%\.cargo\bin\`, making `tom` available globally in your terminal.

---

## 🛠️ Tool Index

| Tool | Description | Repository | Role | Requirements |
| :--- | :--- | :--- | :--- | :--- |
| **[`tom`](./tom/)** | Tool Manager & CLI orchestrator to discover, fetch, install, update, and inspect personal tools | [AsimMahata/tom](https://github.com/AsimMahata/tom) | Core Manager | Rust (Cargo 1.75+) |
| **[`netman`](./netman/)** | Windows network management & diagnostic CLI (Wi-Fi, Mobile Hotspot, LAN, IP, speed & connectivity tests) | [AsimMahata/netman](https://github.com/AsimMahata/netman) | Active Tool | Python 3.8+ |
| **[`progit`](./progit/)** | Personal progress & activity tracker CLI (Codeforces, LeetCode, tasks, daily notes, stats & backups) | [AsimMahata/progit](https://github.com/AsimMahata/progit) | Active Tool | Rust (Cargo) |
| **[`logit`](./logit/)** | Personal log maker & diary CLI (activities, daily logs, job applications & resume tracking) | [AsimMahata/logit](https://github.com/AsimMahata/logit) | Active Tool | Rust (Cargo) |
| **[`sodo`](./sodo/)** | YouTube → MP3 downloader CLI powered by `yt-dlp` (batch queues, history, format selection) | [AsimMahata/sodo](https://github.com/AsimMahata/sodo) | Active Tool | Python 3.10+, `ffmpeg` |
| **[`sudow`](./sudow/)** | Windows Sudo-style command wrapper for seamless Administrator elevation without extra windows | [AsimMahata/sudow](https://github.com/AsimMahata/sudow) | Active Tool | Rust (Cargo) |

### 💡 Naming & Origins

* **`tom`**: **TO**ol **M**anager — Ecosystem orchestrator managing all tools.
* **`logit`**: **Log It** — "Log" as in diary/logbook. Record what you are doing (daily logs, activities, job applications, resume versions used).
* **`progit`**: **Progress It** — Track progress and milestones (coding platforms, tasks, daily notes). Built after `logit` and adopted the `-it` suffix.
* **`sodo`**: **SO**ng **DO**wnloader — YouTube → MP3 downloader CLI.
* **`netman`**: **NET**work **MAN**ager — Windows network diagnostic & management CLI.
* **`sudow`**: **SUDO W**indows — Sudo-style elevation wrapper for Windows terminal.

---

## 📂 Architecture & Directory Model

```text
                 Personal Tools Ecosystem
                            │
                            ▼
                           TOM
                    (Ecosystem Manager)
                            │
       ┌───────────┬────────┴──────────┬───────────┬───────────┐
       ▼           ▼                   ▼           ▼           ▼
    netman      progit               logit       sodo        sudow
   (Network)   (Progress)          (Personal)   (Media)    (Elevation)
```

### Directory Structure

```text
tools/
├── README.md              ← Ecosystem landing page
├── install.bat            ← 1-click Windows installer for TOM
├── registry.toml          ← Master tool metadata & build pipelines
├── config.toml            ← Global configuration (tools path & editor settings)
├── tom/                   ← TOM orchestrator (Rust crate, source tracked in parent repo)
├── netman/                ← Windows network management CLI (Python, independent Git repo)
├── progit/                ← Progress & activity tracker CLI (Rust, independent Git repo)
├── logit/                 ← Personal log & diary tracker CLI (Rust, independent Git repo)
├── sodo/                  ← YouTube → MP3 downloader CLI (Python, independent Git repo)
└── sudow/                 ← Windows sudo elevation wrapper (Rust, independent Git repo)
```

* **TOM Manager**: Source code is located in [`tom/`](./tom/) and tracked directly within the parent workspace.
* **Managed Tools**: Sibling tools (`netman`, `progit`, `logit`, `sodo`, `sudow`) are independent Git repositories managed via TOM.

---

## 🚀 TOM Lifecycle Commands

TOM provides lifecycle commands for managing your tools ecosystem:

### 1. Discovery & Status
```bash
# List all discovered tools in the ecosystem directory
tom list

# Show detailed Git sync state & installation status across all tools (or specific tool)
tom status [tool]

# View detailed tool metadata, build steps, tips, and git info
tom info <tool>
```

### 2. Fetching & Updating (Source Code)
```bash
# Clone/fetch a tool's repository (or a direct Git URL)
tom fetch <tool>

# Fetch all tools defined in the registry
tom fetch all

# Pull latest Git commits in-place for a tool (or all tools)
tom update <tool>
tom update all
```

### 3. Building & Installing (Binaries)
```bash
# Compile and install a specific tool
tom install <tool>

# Build and install all tools defined in the registry (-y skips prompts)
tom install all -y
```

### 4. Uninstallation & Cleanup
```bash
# Run tool-specific uninstallation script/clean step (keeps Git repo)
tom uninstall <tool>

# Purge tool source files & repository (preserves stub README.md)
tom unfetch <tool>
```

### 5. Navigation
```bash
# Open tool directory in VS Code or custom editor (--editor / -e flag)
tom open <tool>
```

---

## ⚙️ Configuration & Registry

* **[`registry.toml`](./registry.toml)**: Defines tool repositories, tags, system requirements, build/install commands, and tips.
* **`config.toml`**: Stores global configuration such as the default `tools_directory` path and preferred text editor.
