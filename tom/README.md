# TOM — Tool Manager

**TOM** is a personal tool and package manager for managing a collection of independent Git repositories.

---

## Design Philosophy

* **Sibling Repositories**: TOM and the tools it manages live alongside each other as siblings inside a shared tools directory.
* **Portable Location Memory**: During its initial setup, TOM determines where it was cloned, records the containing parent directory as `tools_directory`, and manages all tools relative to that directory.
* **Independent Repositories**: Each tool maintains its own Git history, remote, and build configuration.

---

## Directory Model

```text
tools/
├── tom/                  ← TOM repository
├── netman/               ← Sibling tool (independent Git repository)
├── progit/               ← Sibling tool (independent Git repository)
├── logit/                ← Sibling tool (independent Git repository)
└── sodo/                 ← Sibling tool (independent Git repository)
```

---

## CLI Usage

```bash
# Discovery & Inspection
tom list                     # List all discovered tools & registry status
tom info <tool>              # Detailed tool report & Git history
tom status [tool]            # Check working tree status across tools

# Lifecycle Management
tom install <tool>           # Clone from registry/URL & build tool
tom install --all            # Install all tools in the registry
tom update <tool>            # Pull latest updates & rebuild safely
tom uninstall <tool>         # Safely remove tool (guards against unpushed changes)

# Navigation
tom open <tool>              # Open tool in VS Code or default file explorer
```
