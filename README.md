# Personal Tools Ecosystem

A curated collection of modular, independent command-line utilities and personal tools managed by **TOM** (Tool Manager).

Each tool in this workspace is an independent Git repository. This parent repository serves as an index and landing page for the tool suite.

---

## Tool Index

| Tool | Description | Repository | Role |
| :--- | :--- | :--- | :--- |
| **[`tom`](./tom/)** | Tool Manager & CLI orchestrator for personal tool collections | [AsimMahata/tom](https://github.com/AsimMahata/tom) | Core Manager |
| **[`netman`](./netman/)** | Windows network management and diagnostic CLI | [AsimMahata/netman](https://github.com/AsimMahata/netman) | Active Tool |
| **[`progit`](./progit/)** | Git productivity enhancements and workflow helper | [AsimMahata/progit](https://github.com/AsimMahata/progit) | Active Tool |
| **[`logit`](./logit/)** | Log inspection and management utility | [AsimMahata/logit](https://github.com/AsimMahata/logit) | Active Tool |
| **[`sodo`](./sodo/)** | Fast task management and todo tracker CLI | [AsimMahata/sodo](https://github.com/AsimMahata/sodo) | Active Tool |

---

## Getting Started

### Recommended Setup

> [!TIP]
> **Dedicated Tools Directory**
> It is recommended to clone TOM into a dedicated tools directory (e.g. `C:\tools\tom`, `D:\dev-tools\tom`, or `~/tools/tom`).
>
> During its initial setup, TOM determines where it was cloned, records its parent directory as the managed `tools_directory`, and installs/manages all other tools alongside itself as siblings.

Example:

```text
tools/                     ← Dedicated parent tools directory
├── README.md              ← Index of all tools
├── .gitignore
├── config.toml            ← Recorded tools_directory
├── tom/                   ← TOM tool manager
├── netman/                ← Sibling tool (independent Git repo)
├── progit/                ← Sibling tool (independent Git repo)
├── logit/                 ← Sibling tool (independent Git repo)
└── sodo/                  ← Sibling tool (independent Git repo)
```

---

## Managing Tools with TOM

### Essential Commands

```bash
# List all tools and installation status
tom list

# Inspect detailed metadata and Git state
tom info netman

# Check Git working tree status across all tools
tom status

# Install a tool from registry
tom install netman

# Bootstrap/install all tools defined in registry
tom install --all

# Safely update a tool (pulls from Git and rebuilds)
tom update netman

# Open a tool in your editor (VS Code or system default)
tom open netman
```
