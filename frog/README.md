# FROG — Safe File Organization CLI

A deliberately cautious Windows CLI utility for organizing files inside a user-selected folder into extension-based subdirectories.

---

## Naming & Origin

```text
File OrGanizer
      ↓
     FORG
      ↓
   anagram
      ↓
     FROG
```

`frog` stands for **F**ile **Or**ganizer. It originated from **`FORG`** (**F**ile **Or**ganizer), which was rearranged into **`FROG`** because `FORG` sounded awkward. Built as a safety-first CLI tool for the **Personal Tools Ecosystem**.

---

## 🔒 Core Safety Philosophy

Because reorganizing files in bulk can be destructive if invoked on the wrong folder, `frog` **never immediately modifies files**. It enforces a strict multi-barrier safeguard sequence before touching a single file on disk:

```text
frog <folder>
      │
      ▼
Validate target path (must be inside C:\Users\<user>)
      │
      ├── outside home / forbidden path / symlink → ABORT
      │
      ▼
Check for hidden or dot-style files (.env, .git, FILE_ATTRIBUTE_HIDDEN)
      │
      ├── hidden entries found → ABORT
      │
      ▼
Pre-flight collision check (existing files / duplicate destinations)
      │
      ├── collision detected → ABORT
      │
      ▼
Show interactive operation preview (dry-run plan)
      │
      ▼
Require user to manually inspect the target directory
      │
      ├── offer to open in Windows File Explorer [Y/n]
      ├── require explicit inspection confirmation [y/N]
      │
      ▼
Password verification (double prompt entry using Argon2id)
      │
      ├── input mismatch or wrong password → ABORT
      │
      ▼
Final explicit confirmation prompt [y/N]
      │
      ▼
Perform file reorganization with progress reporting
```

> **Note**: `frog` intentionally **does not provide `--force` or `--yes` flags**. Safety barriers are non-bypassable by design.

---

## 🛠️ Usage

```powershell
# Standard interactive safe organization
frog C:\Users\Asim\Downloads
frog .

# Revert / Undo operations
frog undo                 # Undo the most recent organization operation
frog undo list            # View history log of past organization operations
frog undo op-1723891200   # Undo a specific operation by ID or index

# Preview operations without modifying files or prompting for password
frog --dry-run C:\Users\Asim\Downloads

# Organize without offering to launch File Explorer (manual inspection prompt still required)
frog --no-explorer C:\Users\Asim\Downloads

# Password setup / change
frog passwd

# Forbidden directory management
frog forbid list
frog forbid add C:\Users\Asim\Desktop\Protected
frog forbid remove C:\Users\Asim\Desktop\Protected

# Options & help
frog --help
frog --version
```

---

## 📂 Example Reorganization

Given:

```text
Downloads/
├── image1.jpg
├── photo.png
├── report.pdf
├── notes.txt
├── archive.tar.gz
└── README
```

`frog` reorganizes it into:

```text
Downloads/
├── jpg/
│   └── image1.jpg
├── png/
│   └── photo.png
├── pdf/
│   └── report.pdf
├── txt/
│   └── notes.txt
├── gz/
│   └── archive.tar.gz
└── no-extension/
    └── README
```

---

## 🛡️ Path Restrictions & Rules

1. **Allowed Locations**: Must be a subfolder of your user directory (`C:\Users\<user>\...`).
2. **Forbidden Locations**:
   - Built-in defaults: `C:\`, `C:\Windows`, `C:\Program Files`, `C:\Program Files (x86)`, `C:\ProgramData`, `%USERPROFILE%` (root home), `%USERPROFILE%\AppData`.
   - Custom forbidden list: Manageable via `frog forbid add/remove/list`. Stored in `%APPDATA%\frog\config.toml`.
3. **Hidden / Dot Entries**: Refuses to operate if target contains hidden files/folders (`.git`, `.env`, `.config`, hidden system files).
4. **Collision Protection**: All-or-nothing pre-flight verification ensures no file will ever overwrite an existing destination file.
5. **Direct Children Only**: `frog` only reorganizes direct regular files of the target folder. Existing subdirectories are left completely untouched.

---

## ⚡ Installation & Setup

### Step 1 — Build & Install

Run the included installer:

```cmd
install.bat
```

Or manually:

```powershell
cargo build --release
Copy-Item "target\release\frog.exe" "$HOME\.cargo\bin\frog.exe"
```

### Step 2 — Set Password

Before organizing folders for the first time, configure your FROG password:

```powershell
frog passwd
```

Password credentials are hashed using **Argon2id** and stored securely in `%APPDATA%\frog\passwd`.

---

## 🗑️ Uninstallation

Run:

```cmd
uninstall.bat
```

Or via TOM:

```powershell
tom uninstall frog
```
