# SUDOW — Windows Sudo-Style Command Wrapper

A lightweight, high-performance CLI tool for Windows that enables seamless command elevation with Administrator privileges directly from any standard terminal.

---

## Features

- **Standard Linux `sudo`-like Experience**: Run `sudow <command>` from PowerShell, CMD, Windows Terminal, Git Bash, or VS Code integrated terminal.
- **Smart Elevation Detection**: If the terminal is already elevated as Administrator, `sudow` executes the command immediately without redundant UAC prompts.
- **Hidden Background Worker**: Spawns elevated tasks invisibly (`SW_HIDE`) without flickering or opening extra console windows.
- **Real-Time Stream Forwarding**: Real-time multiplexing of `stdout` and `stderr` through a local Named Pipe directly into your active console.
- **Accurate Exit Code Relay**: Perfectly captures and forwards the underlying process exit status or UAC rejection code.
- **Smart Command Resolution**:
  - Standalone binaries & executables (`.exe`) in PATH or specific file paths.
  - CMD shell builtins (`mkdir`, `dir`, `del`, `ren`, `rmdir`, `copy`, etc.) via `cmd.exe /d /c`.
  - PowerShell scripts (`.ps1`) via `powershell.exe -ExecutionPolicy Bypass -File`.
  - Batch scripts (`.bat`, `.cmd`).
- **Safe Argument Escaping**: Follows standard Windows `CommandLineToArgvW` quoting semantics to handle spaces, quotes, flags, and paths.
- **System Doctor**: Built-in `sudow doctor` diagnostic command to check environment health and privileges.

---

## Usage

```powershell
# Basic usage
sudow <command> [arguments...]

# Examples
sudow ipconfig /flushdns
sudow netsh interface set interface name="Wi-Fi" admin=disabled
sudow netsh interface set interface name="Wi-Fi" admin=enabled
sudow sc stop SomeService
sudow sc start SomeService
sudow mkdir C:\SomeProtectedDirectory
sudow powershell -Command "Get-Service | Where-Object Status -eq Running"

# Diagnostic check
sudow doctor

# Help and version
sudow --help
sudow --version
```

---

## Installation

> **⚠ Must be installed and uninstalled manually** — `tom install sudow` and
> `tom uninstall sudow` do not work because the password setup step (`sudow passwd`)
> requires an interactive terminal that `tom` does not provide.

### Step 1 — Build & install the binary

Open a terminal in the project directory and run:

```cmd
install.bat
```

Or manually with Cargo:

```powershell
cargo build --release
Copy-Item "target\release\sudow.exe" "$HOME\.cargo\bin\sudow.exe"
```

### Step 2 — Set your sudo password

After installation, run once to configure your password:

```powershell
sudow passwd
```

You will be prompted twice (no echo). The password is stored as a SHA-256 hash
in `%APPDATA%\sudow\passwd` — never in plaintext.

### Step 3 — Verify

```powershell
sudow doctor
sudow ipconfig /flushdns
```

---

## Uninstallation

> **⚠ Must be uninstalled manually** — `tom uninstall sudow` does not work
> because Windows locks the binary when it is in use.

Open a terminal in the project directory and run:

```cmd
uninstall.bat
```

This removes:
- The `sudow.exe` binary from `%USERPROFILE%\.cargo\bin\`
- The password hash from `%APPDATA%\sudow\`
- The `target\` build directory

---

## Architecture

```text
sudow <command> <args>
        │
        ▼
Check current elevation (TokenElevation)
        │
   ┌────┴────┐
   │         │
Elevated   Normal
   │         │
   │         ▼
   │    Create IPC Named Pipe
   │    Trigger UAC via ShellExecuteExW("runas", SW_HIDE)
   │         │
   │         ▼
   └────► Launch command
              │
              ▼
        Stream stdout / stderr in real-time
              │
              ▼
        Capture & return exit code
```

---

## Security Model

`sudow` complies with Windows security standards:
- Does **NOT** bypass or disable UAC.
- Does **NOT** store plaintext passwords — only a SHA-256 hash in `%APPDATA%\sudow\passwd`.
- Does **NOT** modify Windows security policies or create permanent elevated background services.
- Requires a **sudo password** on every invocation (set via `sudow passwd`), just like Linux `sudo`.
- Prompts for standard Windows UAC confirmation when elevation is needed.
