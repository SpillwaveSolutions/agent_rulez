# Feature Research: RuleZ UI Desktop App

**Domain:** Desktop configuration editor and policy management tool for AI workflows
**Researched:** 2026-02-10
**Confidence:** MEDIUM (WebSearch verified with established patterns)

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist. Missing these = product feels incomplete.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| **YAML autocomplete for RuleZ fields** | Standard in modern code editors (VS Code, Monaco, IntelliJ all provide schema-driven completions) | MEDIUM | Monaco Editor supports `registerCompletionItemProvider` for custom language support. Can use existing hooks.json schema. |
| **Inline validation with error tooltips** | Users expect validation feedback as they type (VS Code, all modern IDEs) | LOW | Already have schema validation; need to wire schema errors to Monaco's `setModelMarkers` API. |
| **Audit log viewer with filtering** | Policy management tools require audit trails (Microsoft Purview, Oracle Audit, AWS CloudTrail all have log viewers) | MEDIUM-HIGH | JSONL parsing, date range filtering, rule correlation. Display ~1K-10K events efficiently. |
| **Import/export configurations** | All config management tools support this (VS Code settings, Visual Studio .vsconfig, Windows Group Policy) | LOW | Export to .yaml, import with merge options. Simple file operations via Tauri commands. |
| **Binary path configuration** | Desktop apps that call CLI tools must let users configure paths (Git GUIspindle, Docker Desktop, etc.) | LOW | Settings panel with file picker. Store in app config (Tauri store API or localStorage). |
| **Error recovery for missing binary** | Users expect graceful degradation when dependencies missing (VS Code extensions, Docker Desktop) | MEDIUM | Detect binary not found, show actionable error with install instructions, offer manual path selection. |
| **Search within files** | All code editors provide this (Ctrl+F / Cmd+F universal pattern) | LOW | Monaco Editor has built-in find widget. Already included in Monaco integration. |
| **File save indicators** | Users expect to know when files have unsaved changes (VS Code dot on tab, asterisk in title) | LOW | Already implemented in M1 (file tab management, unsaved indicators). |
| **Theme persistence** | Users expect theme selection to persist across sessions | LOW | Already have theme toggle; need to persist in localStorage or Tauri store. |

### Differentiators (Competitive Advantage)

Features that set the product apart. Not required, but valuable.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| **Rule correlation in audit viewer** | Click audit log entry -> see which rule fired -> jump to rule definition | MEDIUM | Correlate log entries (rule_id) with YAML AST positions. Requires YAML parser to track line numbers. |
| **Inline documentation on hover** | Hover over `action:` field -> see available actions and examples | MEDIUM | Monaco Editor `registerHoverProvider` API. Fetch docs from inline schema descriptions or embedded markdown. |
| **Template gallery with preview** | Pre-built rule templates (block dangerous ops, inject context, validate inputs) with live preview | MEDIUM | JSON/YAML templates in assets/, preview shows before/after. 10-15 templates covering common use cases. |
| **Config diff view** | Compare global vs project config side-by-side with color-coded changes | MEDIUM-HIGH | Monaco Editor has built-in diff editor (`monaco.editor.createDiffEditor`). Color-code added (green), removed (red), modified (blue). |
| **First-run wizard** | Guided setup for new users (detect/configure binary path, create first rule, test with simulator) | MEDIUM | Multi-step onboarding flow. Skip button for power users. Show once, persist completion state. |
| **Rule evaluation timeline** | Visual timeline showing which rules fired for a given event, with timing | MEDIUM-HIGH | Parse `rulez debug` output for rule evaluation trace. Timeline component with swimlanes. Requires CLI to expose timing data. |
| **Snippet insertion** | Type `trigger` snippet keyword -> insert pre-filled trigger block with placeholders | LOW-MEDIUM | Monaco Editor snippet API. Define snippets for `trigger`, `action`, `condition`, `validation`. |
| **Workspace rule count dashboard** | Show stats: total rules, active vs inactive, rules by type, last modified | LOW | Parse YAML, count rules, display in sidebar or header. Refresh on file change. |
| **Validation error explanations** | Click validation error -> see why it failed and how to fix (not just "invalid YAML") | MEDIUM | Enhance schema validation to include human-readable explanations. Link to docs. |
| **Rule testing without Claude Code** | Inline simulator that lets you test rules in isolation without running Claude | LOW | Already implemented in M1 (debug simulator). Enhance with more event types. |

### Anti-Features (Commonly Requested, Often Problematic)

Features that seem good but create problems.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| **Real-time multi-user editing** | "Let teams edit rules together like Google Docs" | Extreme complexity, conflicts with file-based storage, YAML structure breaks with OT/CRDT | Use Git for collaboration. Show file modified warnings. Support import/export for sharing. |
| **Cloud sync/backup** | "I want my rules synced across machines" | Adds cloud infrastructure, privacy concerns, vendor lock-in | Document how to use Git, Dropbox, or iCloud for config sync. File-based storage already supports this. |
| **AI-powered rule generation** | "Generate rules from natural language" | Unpredictable output, requires LLM integration, prompt engineering complexity | Provide comprehensive template gallery instead. Users can use Claude Code itself to help write rules. |
| **Visual drag-and-drop rule builder** | "I don't want to write YAML" | YAML structure is complex (nested conditions, regex, scripts). Visual builder would be massive and inflexible. | Provide autocomplete, snippets, templates, inline docs. YAML is the right abstraction. |
| **Browser extension** | "Let me edit rules in the browser" | Security model conflicts (browser can't access filesystem or run binaries), limited API | Keep desktop app as primary interface. Consider read-only web viewer for sharing. |
| **Integrated terminal** | "Run commands from the app" | Scope creep, security risks, poor UX for CLI power users | Users already have terminals. Focus on GUI-specific value (visual editing, simulation). |
| **Plugin system** | "Let users extend the app" | Maintenance burden, security risks, API surface explosion | Build core features that cover 90% of use cases. Open-source for custom forks. |

## Feature Dependencies

```
[Audit Log Viewer]
    └──requires──> [Binary Path Config]
                       └──requires──> [Error Recovery for Missing Binary]

[Rule Correlation in Audit Viewer]
    └──requires──> [Audit Log Viewer]
    └──requires──> [YAML AST Parser with Line Numbers]

[Config Diff View]
    └──requires──> [Multi-file Support] (already in M1)

[Inline Documentation on Hover]
    └──requires──> [Schema-driven Autocomplete]

[First-run Wizard]
    └──requires──> [Binary Path Config]
    └──requires──> [Template Gallery]
    └──requires──> [Debug Simulator] (already in M1)

[Rule Evaluation Timeline]
    └──requires──> [Audit Log Viewer]
    └──requires──> [CLI Timing Data] (requires rulez binary enhancement)

[Validation Error Explanations]
    └──enhances──> [Inline Validation]
    └──enhances──> [Error Recovery]

[Template Gallery]
    └──enhances──> [First-run Wizard]
    └──enhances──> [Snippet Insertion]
```

### Dependency Notes

- **Audit Log Viewer requires Binary Path Config:** Can't read logs without knowing where rulez binary stores them (~/.claude/logs/rulez.log is default, but may be customized).
- **Rule Correlation requires YAML AST Parser:** Need to map rule IDs in logs to line numbers in YAML files. `yaml-rust2` or `serde-yaml` with custom deserializer.
- **Config Diff requires Multi-file Support:** Already implemented in M1 (global + project configs). Diff is a natural extension.
- **First-run Wizard orchestrates multiple features:** Binary setup, template selection, simulator test. Must be built after prerequisites.
- **Rule Evaluation Timeline requires CLI enhancement:** Current `rulez debug` output doesn't expose timing. Need `--trace` flag or JSON output mode.

## MVP Definition

### Launch With (v1.5)

Minimum viable product for production-quality desktop app.

- [ ] **YAML autocomplete for RuleZ fields** — Core editor feature, expected by all users
- [ ] **Inline validation with error tooltips** — Prevents user frustration, schema already exists
- [ ] **Audit log viewer with filtering** — Primary use case for understanding rule behavior
- [ ] **Import/export configurations** — Essential for sharing, backup, migration
- [ ] **Binary path configuration** — Unblocks users with non-standard installs
- [ ] **Error recovery for missing binary** — Prevents "app doesn't work" first impression
- [ ] **Inline documentation on hover** — Reduces need for docs, improves discoverability
- [ ] **Template gallery with preview** — Fastest way to get started, showcases capabilities
- [ ] **First-run wizard** — Critical for onboarding, reduces support burden

### Add After Validation (v1.6)

Features to add once core is working and user feedback collected.

- [ ] **Rule correlation in audit viewer** — High value but depends on log parsing polish
- [ ] **Config diff view** — Useful for power users, not critical for first release
- [ ] **Snippet insertion** — Nice-to-have, templates + autocomplete cover 80% of use case
- [ ] **Validation error explanations** — Enhances existing validation, can iterate based on feedback
- [ ] **Workspace rule count dashboard** — Useful for monitoring, not critical for editing

### Future Consideration (v2+)

Features to defer until product-market fit is established.

- [ ] **Rule evaluation timeline** — Requires CLI changes, complex visualization, niche use case
- [ ] **Multi-workspace support** — Most users have 1-2 projects, not a pain point yet
- [ ] **Rule version history** — Git already provides this, app integration is duplicative
- [ ] **Performance profiling** — Users can use rulez CLI directly, GUI adds limited value
- [ ] **Custom theme editor** — Dark/light modes cover 95% of needs

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| YAML autocomplete | HIGH | MEDIUM | P1 |
| Inline validation tooltips | HIGH | LOW | P1 |
| Audit log viewer | HIGH | MEDIUM-HIGH | P1 |
| Import/export configs | HIGH | LOW | P1 |
| Binary path config | HIGH | LOW | P1 |
| Error recovery | HIGH | MEDIUM | P1 |
| Inline documentation hover | MEDIUM-HIGH | MEDIUM | P1 |
| Template gallery | HIGH | MEDIUM | P1 |
| First-run wizard | HIGH | MEDIUM | P1 |
| Rule correlation | MEDIUM-HIGH | MEDIUM | P2 |
| Config diff view | MEDIUM | MEDIUM-HIGH | P2 |
| Snippet insertion | MEDIUM | LOW-MEDIUM | P2 |
| Validation error explanations | MEDIUM | MEDIUM | P2 |
| Workspace dashboard | LOW-MEDIUM | LOW | P2 |
| Rule evaluation timeline | MEDIUM | MEDIUM-HIGH | P3 |
| Multi-workspace support | LOW | MEDIUM-HIGH | P3 |
| Rule version history | LOW | HIGH | P3 |
| Performance profiling | LOW | MEDIUM | P3 |
| Custom theme editor | LOW | MEDIUM | P3 |

**Priority key:**
- P1: Must have for v1.5 launch (production-quality)
- P2: Should have in v1.6 (post-feedback iteration)
- P3: Nice to have in v2+ (future consideration)

## Competitor Feature Analysis

| Feature | VS Code Settings Editor | Visual Studio 2026 Settings | Policy Management Tools (Scrut, Oracle HCM) | Our Approach |
|---------|-------------------------|------------------------------|---------------------------------------------|--------------|
| **Schema-driven autocomplete** | Yes (IntelliSense, JSON Schema) | Yes (improved search, JSON tracking) | Limited (form-based) | Monaco + JSON Schema (same as VS Code) |
| **Inline validation** | Yes (red squiggles, hover for details) | Yes (error list panel) | Form validation | Monaco markers + hover tooltips |
| **Settings search** | Yes (fuzzy search, category filter) | Yes (improved search in 2026) | Basic search | Not needed (users search YAML directly with Ctrl+F) |
| **Import/export** | Yes (settings sync, profiles via .code-profile drag-drop) | Yes (.vsconfig JSON files) | Yes (policy templates, 75+ templates in Scrut) | YAML file export/import with merge options |
| **Diff view** | Yes (settings.json vs defaults) | No (but has version control integration) | Yes (config compare, side-by-side, color-coded) | Monaco diff editor (global vs project) |
| **Audit trail** | No (VS Code doesn't track setting changes) | No | Yes (policy management requires audit trails) | JSONL log viewer with filtering |
| **Template gallery** | Extensions marketplace | Project templates | 75+ policy templates (Scrut) | 10-15 rule templates in assets/ |
| **First-run experience** | Welcome page, walkthrough | Welcome dialog, customization wizard | Onboarding workflows | Multi-step wizard (binary setup, template, test) |
| **Error recovery** | Extension errors show in output panel | Build errors in error list | N/A | Modal with actionable steps (install, locate, skip) |
| **Hover documentation** | Yes (IntelliSense hover) | Yes (parameter values inline during debug) | Limited | Schema descriptions + markdown snippets |

**Key Insights:**
- **VS Code/Visual Studio set the bar:** Users expect autocomplete, validation, hover docs. We match this with Monaco.
- **Policy tools focus on audit:** Unlike code editors, policy management tools prioritize audit trails and compliance. We align with this for RuleZ.
- **Config diff is common in network tools:** SolarWinds, ManageEngine use side-by-side diff for comparing device configs. We adopt this pattern.
- **Templates accelerate adoption:** Scrut has 75+ templates, Oracle HCM has built-in templates. We provide focused templates for AI policy use cases.
- **First-run experience is critical:** VS Code, Visual Studio, and dev tools in 2026 all emphasize reducing time to first commit (3 days for SMBs, 2 weeks for enterprise). Our wizard targets <10 minutes to first working rule.

## Complexity Estimates

### LOW Complexity (1-2 days)
- Import/export configs
- Binary path config (settings UI)
- Theme persistence
- Snippet insertion
- Workspace dashboard

### MEDIUM Complexity (3-5 days)
- YAML autocomplete (Monaco API integration)
- Error recovery UI
- Inline documentation hover
- Template gallery
- First-run wizard
- Rule correlation (YAML parsing)
- Validation error explanations

### MEDIUM-HIGH Complexity (5-10 days)
- Audit log viewer (JSONL parsing, filtering, UI)
- Config diff view (Monaco diff editor)

### HIGH Complexity (10+ days)
- Rule evaluation timeline (requires CLI changes + complex visualization)

## Sources

### Desktop Configuration Editor Best Practices
- [Visual Studio 2026 Settings Experience](https://devblogs.microsoft.com/visualstudio/a-first-look-at-the-all%E2%80%91new-ux-in-visual-studio-2026/) - All-new settings UI with transparency, JSON tracking, improved search
- [Microsoft Desktop UX Guide](https://learn.microsoft.com/en-us/windows/win32/uxguide/how-to-design-desktop-ux) - Visual and functional consistency, feature simplicity
- [UI Design Best Practices 2026](https://uxplaybook.org/articles/ui-fundamentals-best-practices-for-ux-designers) - Visual hierarchy, reducing friction

### Policy Management Tools
- [26 Best Policy Management Software 2026](https://peoplemanagingpeople.com/tools/best-policy-management-software/) - Feature comparison, audit requirements
- [Top 5 Policy Management Software 2026](https://www.v-comply.com/blog/top-policy-management-softwares/) - Centralized governance, structured workflows
- [Policy Management Tools Review](https://www.smartsuite.com/blog/policy-management-software) - Built-in editors, version logging, real-time tracking

### YAML Editor Features
- [Boost YAML with Autocompletion and Validation](https://medium.com/@alexmolev/boost-your-yaml-with-autocompletion-and-validation-b74735268ad7) - Schema-driven autocomplete, hover tooltips
- [Oxygen XML YAML Editor](https://www.oxygenxml.com/yaml_editor.html) - Content completion, syntax highlighting, outline view
- [Eclipse YAML Editor](https://marketplace.eclipse.org/content/yaml-editor) - Validation, outline, hierarchical dependencies

### Monaco Editor Integration
- [Monaco Editor Custom Language](https://www.checklyhq.com/blog/customizing-monaco/) - `registerCompletionItemProvider` for custom languages
- [Custom IntelliSense with Monaco](https://mono.software/2017/04/11/custom-intellisense-with-monaco-editor/) - Schema-driven autocompletion
- [4 Steps to Add Custom Language Support](https://ohdarling88.medium.com/4-steps-to-add-custom-language-support-to-monaco-editor-5075eafa156d) - Monaco language infrastructure

### Audit Log Viewer Patterns
- [Microsoft Purview Audit Log Search](https://learn.microsoft.com/en-us/purview/audit-search) - Filtering, date ranges, activity filters
- [Oracle Audit Logs](https://docs.oracle.com/en-us/iaas/Content/Logging/Concepts/audit_logs.htm) - Timeline features, query syntax
- [CrowdStrike Secure Audit Log](https://pangea.cloud/docs/audit/using-secure-audit-log/log-viewer) - Download to CSV, search filters

### Configuration Diff Tools
- [ManageEngine Config Compare](https://www.manageengine.com/network-configuration-manager/compare-config.html) - Side-by-side comparison, color-coded changes
- [SolarWinds Config Compare](https://www.solarwinds.com/network-configuration-manager/use-cases/config-compare) - Multi-vendor support, baseline comparison
- [Best File Comparison Tools Linux 2026](https://thelinuxcode.com/10-best-file-comparison-and-diff-tools-in-linux-2026-developer-guide/) - KDiff3, line-by-line, word-by-word

### Import/Export Patterns
- [Export/Import App Configuration Microsoft](https://learn.microsoft.com/en-us/dynamics365/customer-service/implement/export-import-omnichannel-data) - Configuration Migration tool
- [Visual Studio Import/Export Configurations](https://learn.microsoft.com/en-us/visualstudio/install/import-export-installation-configurations?view=visualstudio) - .vsconfig JSON format
- [Azure App Configuration Import/Export](https://learn.microsoft.com/en-us/azure/azure-app-configuration/howto-import-export-data) - Data exchange patterns

### VS Code Settings & UX
- [VS Code User and Workspace Settings](https://code.visualstudio.com/docs/getstarted/settings) - Settings editor, graphical interface
- [VS Code January 2026 Update](https://code.visualstudio.com/updates/v1_109) - Profile import via drag-and-drop
- [VS Code Extension Settings Guidelines](https://code.visualstudio.com/api/ux-guidelines/settings) - Input boxes, booleans, dropdowns, lists

### Inline Documentation & Hover
- [Visual Studio 2026 Release Notes](https://learn.microsoft.com/en-us/visualstudio/releases/2026/release-notes) - Method parameter values inline, analyze with Copilot
- [VS Code Programmatic Language Features](https://code.visualstudio.com/api/language-extensions/programmatic-language-features) - Hovers show symbol info and description
- [JetBrains Quick Documentation](https://www.jetbrains.com/help/webstorm/viewing-inline-documentation.html) - Show on Mouse Move, Auto-Update, delay settings

### Developer Onboarding
- [Developer Tools That Matter 2026](https://dzone.com/articles/developer-tools-that-actually-matter-in-2026) - DevEx is about flow, reducing friction
- [Developer Onboarding Best Practices](https://www.cortex.io/post/developer-onboarding-guide) - Time to first commit (3 days SMB, 2 weeks for enterprise)
- [Developer Experience 2026](https://tutorialsdojo.com/what-developer-experience-really-means-in-2026/) - Developers expect tools that work with them, not against them

---

*Feature research for: RuleZ UI Desktop App (v1.5)*
*Researched: 2026-02-10*
