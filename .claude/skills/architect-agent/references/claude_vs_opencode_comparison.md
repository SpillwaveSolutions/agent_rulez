# Claude Code vs OpenCode: Comprehensive Comparison

**Purpose:** Help users decide which AI coding tool to use for architect and code agent workflows
**Last Updated:** 2025-01-17

---

## Executive Summary

Both **Claude Code** (Anthropic's official CLI) and **OpenCode** (open-source alternative) work excellently with the architect-agent skill. The choice depends on your priorities:

- **Claude Code** → Official support, built-in features, simpler setup
- **OpenCode** → Open source, customizable, community-driven
- **Hybrid** → Mix and match! A single project can have code agents using Claude Code alongside code agents using OpenCode

**Bottom Line:** The architect-agent skill provides full compatibility with both tools through the hybrid logging protocol v2.0. Both tools now have slash command parity (same format, different directory locations).

---

## Quick Comparison Matrix

| Feature | Claude Code | OpenCode | Winner |
|---------|------------|----------|--------|
| **Open Source** | ❌ Proprietary | ✅ Fully open | OpenCode |
| **Official Support** | ✅ Anthropic-backed | ⚠️ Community | Claude Code |
| **Automated Logging** | ✅ hooks.json | ✅ Plugin/wrappers | Tie |
| **Slash Commands** | ✅ `.claude/commands/` | ✅ `.opencode/command/` | Tie |
| **Customizability** | ⚠️ Limited | ✅ Full control | OpenCode |
| **Setup Complexity** | ✅ Simpler | ⚠️ More steps | Claude Code |
| **Performance (logging)** | ✅ Fast hooks | ✅ Fast plugin | Tie |
| **Cost** | Varies | Varies | Depends |
| **MCP Servers** | ✅ Extensive | ⚠️ Growing | Claude Code |
| **Documentation** | ✅ Official docs | ⚠️ Community docs | Claude Code |
| **Token Efficiency** | ✅ 60-70% savings | ✅ 60-70% savings | Tie |
| **Grading Compatible** | ✅ Yes | ✅ Yes | Tie |

---

## Detailed Feature Comparison

### 1. Automated Logging

#### Claude Code

**Implementation:** `.claude/hooks.json`

```json
{
  "hooks": {
    "tool-call-hook": {
      "command": "bash -c 'LOG_FILE=$(cat debugging/current_log_file.txt) && echo \"TOOL: $TOOL_NAME\" >> \"$LOG_FILE\"'"
    }
  }
}
```

**Pros:**
- Native integration, no compilation
- Simple JSON configuration
- Executes via shell commands
- Well-documented

**Cons:**
- Bash-only (no TypeScript/Python hooks)
- Less type-safe
- Shell overhead (~5-10ms per hook)

#### OpenCode

**Implementation:** `.opencode/plugins/logger/index.ts` OR `debugging/wrapper-scripts/*.sh`

**Plugin Approach (TypeScript):**
```typescript
export default definePlugin({
  name: "logger",
  async onToolCalled({ tool, params }) {
    // Log tool call
  }
});
```

**Wrapper Approach (Bash):**
```bash
./debugging/wrapper-scripts/run-with-logging.sh task test
```

**Pros:**
- **Plugin:** Type-safe, faster (<1ms overhead), native to OpenCode
- **Wrapper:** Transparent, no compilation, works with any OpenCode version
- Choice of implementation

**Cons:**
- **Plugin:** Requires TypeScript knowledge, compilation
- **Wrapper:** Manual wrapping required, slightly slower

**Verdict:** **Tie** - Both achieve same result (60-70% token savings, identical log format)

---

### 2. Session Management

#### Claude Code

**Slash Commands (built-in):**

```bash
/log-start          # Start new log session
/log-checkpoint     # Manual milestone
/log-complete       # Complete session
```

**Command Directory:** `.claude/commands/`

**Pros:**
- Zero setup required
- Consistent UX across projects
- No scripts to maintain
- Official support

**Cons:**
- Can't customize behavior
- Black box (can't see implementation)

#### OpenCode

**Slash Commands (now supported!):**

```bash
/log-start          # Start new log session
/log-checkpoint     # Manual milestone
/log-complete       # Complete session
```

**Command Directory:** `.opencode/command/`

**Alternative - Bash Scripts:**

```bash
./debugging/scripts/log-start.sh "task-description"
./debugging/scripts/log-complete.sh
```

**Pros:**
- **Slash commands now work identically to Claude Code**
- Fully customizable (edit markdown files or bash scripts)
- Transparent (readable implementation)
- Can add custom logic (notifications, integrations, etc.)

**Cons:**
- Requires copying command files (templates provided)

**Verdict:** **Tie** - Both now have slash command parity. OpenCode still wins on customization.

---

### 3. Open Source vs Proprietary

#### Claude Code

- **License:** Proprietary (Anthropic)
- **Source Code:** Not publicly available
- **Modifications:** Not allowed
- **Community:** Limited (official support only)

#### OpenCode

- **License:** Open source (check OpenCode repo for exact license)
- **Source Code:** Fully available on GitHub
- **Modifications:** Allowed and encouraged
- **Community:** Growing open-source community

**Verdict:** **OpenCode wins** for transparency, customizability, and community control

---

### 4. Setup & Configuration

#### Claude Code (Simpler)

**Setup Steps:**
1. Install Claude Code CLI
2. Copy `hooks.json` to workspace
3. Add permissions to `settings.local.json`
4. Done

**Estimated Time:** 5 minutes

#### OpenCode (More Complex)

**Setup Steps (Plugin Approach):**
1. Install OpenCode
2. Copy plugin directory
3. Create `opencode.json`
4. Create session management scripts
5. Configure permissions (OpenCode-specific syntax)
6. Make scripts executable

**Estimated Time:** 15 minutes

**Setup Steps (Wrapper Approach):**
1. Install OpenCode
2. Copy wrapper scripts
3. Create session management scripts
4. Make scripts executable
5. Configure permissions

**Estimated Time:** 10 minutes

**Verdict:** **Claude Code wins** - Simpler, faster setup

---

### 5. Performance

| Operation | Claude Code | OpenCode (Plugin) | OpenCode (Wrapper) |
|-----------|------------|------------------|-------------------|
| **Hook Overhead** | ~5-10ms | <1ms | ~5-10ms |
| **Log Write** | Fast | Fast | Fast |
| **Session Start** | Instant (slash) | Script execution | Script execution |
| **Total Impact** | Negligible | Negligible | Negligible |

**Verdict:** **OpenCode Plugin wins marginally**, but all approaches are fast enough that performance is not a deciding factor

---

### 6. Customization & Extensibility

#### Claude Code

**Customization Options:**
- ✅ Hook commands (bash only)
- ✅ Permission rules
- ❌ Cannot modify slash command behavior
- ❌ Cannot add custom lifecycle hooks
- ❌ Cannot change log file format

**Extension Points:**
- Hooks can call external scripts
- Settings.json for permissions
- MCP servers (if supported)

#### OpenCode

**Customization Options:**
- ✅ Plugin lifecycle hooks (TypeScript)
- ✅ Wrapper scripts (bash, Python, anything)
- ✅ Session management (fully customizable)
- ✅ Log format (can modify scripts)
- ✅ Custom lifecycle events
- ✅ Extend plugin API

**Extension Points:**
- Full TypeScript plugin API
- Any bash script integration
- Complete control over workflow
- Can fork and modify OpenCode itself

**Verdict:** **OpenCode wins decisively** - Unlimited customization potential

---

### 7. Documentation & Support

#### Claude Code

**Documentation:**
- ✅ Official docs at code.claude.com
- ✅ Maintained by Anthropic
- ✅ Comprehensive guides
- ✅ Examples and tutorials

**Support:**
- ✅ Official support channels
- ✅ GitHub issues (official repo)
- ⚠️ Community Discord (limited)

#### OpenCode

**Documentation:**
- ⚠️ Community-driven docs
- ⚠️ May lag behind features
- ⚠️ Variable quality
- ✅ Open contribution model

**Support:**
- ✅ GitHub issues (community)
- ✅ Community Discord/forums
- ⚠️ No official support
- ✅ Can debug source code yourself

**Verdict:** **Claude Code wins** - Official docs and support

---

### 8. Cost Considerations

**Note:** Pricing may change - verify current pricing

#### Claude Code

- License cost (if any)
- API token costs (Claude usage)
- MCP server costs (third-party services)

#### OpenCode

- Free (open source)
- API token costs (Claude usage via API)
- MCP server costs (third-party services)

**Key Difference:** OpenCode itself is free, but you still pay for Claude API usage

**Verdict:** **OpenCode potentially cheaper**, depending on licensing model

---

### 9. MCP (Model Context Protocol) Servers

#### Claude Code

**MCP Support:**
- ✅ Extensive built-in MCP servers
- ✅ Official MCP server directory
- ✅ Easy installation via Claude Code CLI
- ✅ Well-tested integrations

#### OpenCode

**MCP Support:**
- ⚠️ Growing MCP ecosystem
- ⚠️ May require manual configuration
- ⚠️ Community-driven MCP servers
- ✅ Can use Claude Code MCP servers (compatibility)

**Verdict:** **Claude Code wins currently** - More mature MCP ecosystem

---

### 10. Architect-Agent Skill Compatibility

#### Both Claude Code and OpenCode

**Full Compatibility:**
- ✅ Hybrid logging protocol v2.0
- ✅ Same log file format (markdown)
- ✅ Same manual decision logging
- ✅ Same 60-70% token reduction
- ✅ Same grading rubric (10 points)
- ✅ Cross-workspace permissions work
- ✅ Same instruction format
- ✅ Same workflow (instruct → implement → grade)

**No Compatibility Issues:** The architect-agent skill was designed to work with both tools from the start.

**Verdict:** **Perfect Tie** - Architect-agent skill treats both identically

---

## Use Case Recommendations

### When to Choose Claude Code

✅ **You want official support and documentation**
✅ **You prefer simpler setup (just hooks.json)**
✅ **You value built-in slash commands**
✅ **You need extensive MCP server support**
✅ **You want "it just works" experience**
✅ **You're new to AI coding tools**

**Example User:**
> "I want to get started quickly with architect-agent. I don't need customization, just reliable logging and grading. Official support is important to me."

### When to Choose OpenCode

✅ **You value open source and transparency**
✅ **You want full customization capability**
✅ **You're comfortable with TypeScript or bash scripting**
✅ **You want to avoid vendor lock-in**
✅ **You want to contribute to or fork the tool**
✅ **Cost is a primary concern (if OpenCode is cheaper)**

**Example User:**
> "I want to customize my logging workflow, add custom integrations, and have full control. I'm comfortable writing TypeScript plugins or bash scripts."

### When to Use Hybrid Approach

✅ **Architect agent** → Claude Code (benefits from slash commands)
✅ **Code agent** → OpenCode (open source, customizable)
✅ **Multiple code agents** → Mix of Claude Code and OpenCode in the same project!

**Benefits:**
- Architect gets simple UX with built-in commands
- Code agent gets customization and open-source benefits
- Both use same log format (grading compatible)
- Best of both worlds
- **A single project can use both types of code agents** - some team members can use Claude Code while others use OpenCode

**Example Workflow:**
```
Architect (Claude Code):
/log-start
[Write instructions to code agent]
[Read code agent logs]
[Grade work]

Code Agent (OpenCode):
./debugging/scripts/log-start.sh "implement-api"
[Implement instructions]
[Plugin logs automatically]
./debugging/scripts/log-complete.sh
```

**Verdict:** **Hybrid approach recommended for many users** - Combines strengths of both

---

## Migration Considerations

### Claude Code → OpenCode

**Difficulty:** Medium
**Time:** 15-20 minutes
**Data Loss:** None (logs preserved)

**Key Steps:**
1. Remove `.claude/hooks.json`
2. Add OpenCode plugin or wrappers
3. Create session management scripts
4. Test workflow

**See:** [OpenCode Migration Guide](./opencode_migration_guide.md)

### OpenCode → Claude Code

**Difficulty:** Easy
**Time:** 10 minutes
**Data Loss:** None (logs preserved)

**Key Steps:**
1. Remove `.opencode/` directory
2. Add `.claude/hooks.json`
3. Update permissions to Claude Code syntax
4. Remove session management scripts (use slash commands)

---

## Feature Roadmap Comparison

### Claude Code (Anthropic Roadmap)

**Expected Features:**
- More MCP servers
- Enhanced slash commands
- Improved permissions model
- Better error handling
- Potential AI improvements

**Update Frequency:** Regular (Anthropic-driven)

### OpenCode (Community Roadmap)

**Expected Features:**
- Community-driven priorities
- Plugin API enhancements
- More MCP servers (community)
- Custom UI extensions
- Whatever contributors add

**Update Frequency:** Variable (depends on community)

---

## Decision Framework

### Questions to Ask Yourself

1. **Do I need official support?**
   - Yes → Claude Code
   - No → OpenCode

2. **Do I value customization over convenience?**
   - Yes → OpenCode
   - No → Claude Code

3. **Am I comfortable with TypeScript or advanced bash?**
   - Yes → OpenCode (plugin approach)
   - No → Claude Code

4. **Is open source important to me?**
   - Yes → OpenCode
   - No → Either

5. **Do I want the simplest possible setup?**
   - Yes → Claude Code
   - No → OpenCode

6. **Will I use architect + code agent pattern?**
   - Yes → Consider hybrid (architect=Claude, code=OpenCode)
   - No → Either

---

## Summary Table

| Criterion | Claude Code | OpenCode | Hybrid |
|-----------|------------|----------|--------|
| **Best For** | Simplicity, official support | Customization, open source | Both architect & code agent |
| **Setup Time** | 5 min | 10-15 min | 10-15 min (code agent side) |
| **Customization** | Limited | Unlimited | Code agent: unlimited |
| **Support** | Official | Community | Both |
| **Cost** | Varies | Potentially lower | Mixed |
| **Token Efficiency** | 60-70% | 60-70% | 60-70% |
| **Grading Compatible** | ✅ Yes | ✅ Yes | ✅ Yes |
| **Learning Curve** | Low | Medium | Medium |
| **Recommended For** | Beginners, simple workflows | Advanced users, custom needs | Most users |

---

## Final Recommendation

**For Most Users:**
- **Architect Agent:** Use **Claude Code** (slash commands, official support)
- **Code Agent:** Use **OpenCode** (open source, customizable)
- **Result:** Best of both worlds, fully compatible via hybrid logging protocol

**For Beginners:**
- **Both Agents:** Use **Claude Code** (simpler setup, official docs)

**For Advanced Users:**
- **Both Agents:** Use **OpenCode** (full customization, open source)

**For Organizations:**
- **Evaluate:** Licensing, support requirements, customization needs
- **Test:** Both tools in parallel before committing
- **Consider:** Hybrid approach for flexibility

---

## Related Documentation

- [OpenCode Logging Protocol](./opencode_logging_protocol.md) - Complete OpenCode protocol
- [OpenCode Setup Guide](./opencode_setup_guide.md) - Setup instructions
- [OpenCode Migration Guide](./opencode_migration_guide.md) - Migration from Claude Code
- [Hybrid Logging Protocol (Claude Code)](./hybrid_logging_protocol.md) - Claude Code protocol
- [Permissions Setup Protocol](./permissions_setup_protocol.md) - Cross-workspace permissions

---

**Version:** 1.0
**Last Updated:** 2025-01-17
**Contributors:** architect-agent maintainers
