
# Code-Agent Context Hooks (CCH)

**A deterministic, auditable, local-first AI policy engine for Claude Code.**

CCH provides a robust hook system that replaces fragile JSON configurations with readable YAML, enabling developers to enforce coding standards, inject context dynamically, and block dangerous operations with sub-10ms latency,,.

---

## 📖 Table of Contents
- [Overview](#overview)
- [Why Use CCH?](#why-use-cch)
- [Architecture](#architecture)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration Guide](#configuration-guide)
- [The CCH Skill](#the-cch-skill)
- [Governance & Policy](#governance--policy)
- [Development](#development)
- [CLI User Guide](USER_GUIDE_CLI.md)
- [Agent Skill User Guide](USER_GUIDE_SKILL.md)


---

## 🧐 Overview

Claude Code's native hook system relies on raw JSON configuration and shell commands, which can be opaque, hard to maintain, and lack conditional logic.

**Claude Context Hooks (CCH)** solves this by introducing a two-part system:
1.  **The CCH Binary:** A high-performance Rust executable that acts as the runtime hook handler.
2.  **The CCH Skill:** An intelligent assistant that analyzes your project and generates configurations automatically.

**Core Philosophy:** LLMs do not enforce policy; they are subject to policy. CCH is the policy authority.

---

## 🚀 Why Use CCH?

| Problem with Native Hooks | CCH Solution |
| :--- | :--- |
| **Poor Readability** | Human-friendly YAML configuration with comments. |
| **No Conditionals** | Match rules by file extension, directory, tool, or regex. |
| **Safety Risks** | Block operations like `force push` or edits to production,. |
| **Manual Setup** | Intelligent "Skill" that scans `CLAUDE.md` to auto-generate rules. |
| **Opaque Decisions** | Deterministic logging and "explainable" policy modes. |

---

## 🏗 Architecture

### 1. CCH Binary (`cch`)
*   **Role:** Deterministic runtime engine.
*   **Language:** Rust.
*   **Performance:** <10ms execution time (cold start <5ms),.
*   **Function:** Intercepts Claude Code events (e.g., `PreToolUse`, `PermissionRequest`), evaluates YAML rules, and returns actions (inject context, block, or run scripts).

### 2. CCH Skill
*   **Role:** Configuration assistant.
*   **Function:** Bridges the gap between the binary and the user. It discovers project skills, parses `CLAUDE.md` for rules (MUST/MUST NOT), and generates the `hooks.yaml` file,.

---

## 📥 Installation

### Method 1: Pre-built Binaries
Download the appropriate artifact for your platform (macOS, Linux, Windows) from the releases page.

### Method 2: Cargo (Rust)
```bash
cargo install --git https://github.com/your-org/cch
```

### Method 3: CCH Skill Assistance
If you have the CCH Skill enabled, simply ask Claude:
> "Install CCH"
The skill will detect your OS and handle the installation and verification,.

---

## ⚡ Quick Start

1.  **Initialize Configuration**
    Scaffold a new configuration with example templates:
    ```bash
    cch init
    ```
    This creates `.claude/hooks.yaml`,.

2.  **Install to Claude Code**
    Register CCH as the handler in Claude Code's settings:
    ```bash
    cch install --project
    ```
    *Note: Use `--user` for global installation*.

3.  **Verify**
    Ensure your configuration is valid:
    ```bash
    cch validate
    ```

---

## ⚙️ Configuration Guide

Configuration is stored in `.claude/hooks.yaml`. It supports event-based triggers and additive matching,.

### Example `hooks.yaml`

```yaml
version: "1"

# Event: Before a tool is executed
PreToolUse:
  # Rule: Enforce CDK guidelines
  - match:
      tools: ["Edit", "Write"]
      directories: ["infra/**", "cdk/**"]
    action:
      inject: "docs/cdk-guidelines.md"

  # Rule: Prevent committing "WIP" code
  - match:
      tools: ["Bash"]
      command_match: "git commit.*WIP"
    action:
      block: true
      message: "Commits with 'WIP' are forbidden on this branch."

  # Rule: Run custom Python validator
  - match:
      extensions: [".py"]
    action:
      run: ".claude/validators/security-check.py"
```

### Supported Events
*   `SessionStart` / `SessionEnd`
*   `PreToolUse` / `PostToolUse`
*   `PermissionRequest` (Force Claude to explain commands)
*   `UserPromptSubmit`
*   [See full list in PRD].

### Actions
*   `inject`: Add Markdown context to the conversation.
*   `block`: Stop the operation immediately.
*   `run`: Execute a script (Python, Bun, Bash) for custom logic.

---

## 🤖 The CCH Skill

The CCH Skill allows you to configure hooks using natural language.

**Triggers:**
*   "Set up hooks for this project"
*   "Enforce the 'no console.log' rule with a hook"
*   "Why isn't my hook blocking WIP commits?".

**Capabilities:**
*   **Project Analysis:** Scans `.claude/skills` and `CLAUDE.md` to suggest relevant hooks.
*   **Conflict Detection:** Identifies contradicting rules.
*   **Audit Trail:** Keeps a record of installations in `.claude/cch/install.json`.

---

## 🛡 Governance & Policy (Phase 2)

CCH acts as a policy engine with advanced governance features.

### Policy Modes
Rules can be set to different enforcement levels:
*   `enforce` (Default): Blocks operations if conditions are met.
*   `warn`: Injects a warning context but allows the operation.
*   `audit`: Logs the event without blocking or injecting (useful for silent monitoring).

### Metadata & Provenance
Every rule can track its origin for auditability:
```yaml
metadata:
  author: "cch-skill"
  reason: "Enforce infrastructure coding standards"
  ticket: "PLAT-3421"
  confidence: "high"
```

### Logging
Logs are stored in `~/.claude/logs/cch.log` in JSON Lines format, supporting "Explainable Policy" auditing,.

---

## 💻 Development

### Prerequisites
*   Rust (Cargo)
*   Python 3 (for testing validators)
*   Bun (for testing TypeScript validators).

### Build
```bash
cargo build --release
```

### Testing
CCH includes unit, integration, and end-to-end tests.
```bash
cargo test
```
See `cch_cli_prd.md` for the detailed testing strategy covering cold starts, script timeouts, and error handling,.

---

## 📄 License

MIT License.