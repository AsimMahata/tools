# Logit

**Logit** is an extremely lightweight, Git-like CLI application to track all your job and internship applications directly from the terminal. It eliminates the need for manual spreadsheets or web-based dashboards by providing a fast, alias-based workflow with minimal typing and local SQLite storage.

---

## Key Features

* 🚀 **Terminal-First Workflow:** Fast commands designed to minimize keypresses.
* 🏷️ **Alias Binding:** Register email addresses and resumes under short aliases (e.g., `personal` or `main`) and refer to them dynamically.
* 📂 **Local Resume Management:** Automatically copies imported resume files into a dedicated storage directory, ensuring you keep track of which exact copy was sent to each company.
* 🚦 **Status Transitions:** Implements a strict but logical state machine to track applications through Online Assessments (OA), interviews, acceptances, or rejections.
* 🗄️ **Zero Setup Storage:** Stores your applications and notes in a local SQLite database located in your platform's standard configuration directory.
* 🎨 **Nice Output:** Beautifully color-coded command feedback and tables formatted using `comfy-table` and `colored`.

---

## Installation & Compilation

Make sure you have Rust installed. Clone the repository, navigate to the folder, and run:

```bash
# Compile and test the application
cargo test
cargo build --release
```

To make it globally accessible, you can install the binary:

```bash
cargo install --path .
```

---

## Data Storage

Logit uses standard directories to store its sqlite file and copied resumes:

* **Windows:** `C:\Users\<Username>\AppData\Roaming\logit`
* **Linux:** `~/.config/logit` or `~/.local/share/logit`
* **macOS:** `~/Library/Application Support/logit`

Within this folder, you will find:
* `database.db` — Your SQLite database containing applications, email records, resume indexes, and notes.
* `resumes/` — Local repository containing all copied resume files.

---

## Command Reference & Usage

### 1. Emails
```bash
# Add an email alias
logit add-email personal your_email@example.com

# List registered email aliases
logit emails
```

### 2. Resumes
```bash
# Add a resume alias (verifies path and copies it locally)
logit add-resume main "C:\Path\To\resume.pdf"

# List registered resumes
logit resumes
```

### 3. Applications
```bash
# Add a new application entry (Status defaults to "Filled")
logit add Google "Software Engineer Intern" personal main

# You can optionally specify a date (format YYYY-MM-DD):
logit add Meta "ML Engineer" college backend 2026-08-01
```

### 4. Skip a Company
If you decided not to apply to a company but still want to keep record:
```bash
# Record a skipped company
logit skip Netflix "Stipend too low"
```

### 5. Status Flow & State Machine
Update the application status:
```bash
logit status Google "OA Announced"
logit status Google "OA Completed"
logit status Google "Interview Scheduled"
logit status Google "Accepted"
```

The system enforces valid transitions between the following predefined statuses:
* `Filled`
* `OA Announced`
* `OA Completed`
* `OA Selected`
* `OA Rejected`
* `Interview Scheduled`
* `Interview Completed`
* `Accepted`
* `Rejected`
* `Offer Received`
* `Offer Declined`
* `Skipped`

### 6. Set Milestones
```bash
# Set Online Assessment (OA) deadline
logit oa Google 2026-08-14

# Set Interview date
logit interview Google 2026-08-20
```

### 7. Add Notes
Logit supports adding comments or interview logs (appends note to history without overwriting previous logs):
```bash
logit note Google "Asked DSA (Graph BFS) + DBMS Normalization"
```

### 8. View & Search
```bash
# Show full report (including history log of all notes) for a company
logit show Google

# List all applications in a pretty table
logit list

# Filter list by status, email alias, or resume alias:
logit list --status "Interview Scheduled"
logit list --email personal

# Full-text search across company names, aliases, and descriptions
logit search go
```

### 9. Delete
```bash
# Deletes a company application (asks for interactive confirmation)
logit delete Google
```

### 10. Uninstall / Reset Data
```bash
# Deletes all local logit data, database files, and stored resumes (asks for interactive confirmation)
logit uninstall

# Force deletion without confirmation prompt:
logit uninstall --force
```

