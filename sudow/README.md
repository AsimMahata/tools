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

### Using installer script
Run the provided `install.bat`:
```cmd
install.bat -y
```

### Using Cargo directly
```powershell
cargo build --release
Copy-Item "target\release\sudow.exe" "$HOME\.cargo\bin\sudow.exe"
```

### Via `tom`
```powershell
tom install sudow
```

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
- Does **NOT** store or cache user credentials or passwords.
- Does **NOT** modify Windows security policies or create permanent elevated background services.
- Prompts for standard Windows UAC confirmation when elevation is needed.
