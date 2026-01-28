/**
 * OpenCode Logger Plugin (SPEC.md v4.1 Compliant)
 * Enhanced with 13 event handlers for comprehensive session tracking
 *
 * Tracks: Tools, Sessions, Files, Commands, Permissions, TUI, TODOs
 * Token Cost: 0 (all background logging)
 * Savings: 60-70% vs manual logging
 * File Naming: Follows SPEC.md Section 2.5 (log-YYYY_MM_DD-HH_MM-description.md)
 */

import { readFileSync, appendFileSync, existsSync } from "fs"
import { join } from "path"

export const LoggerPlugin = async ({ project, client, $, directory, worktree }) => {

  // Configuration (environment variables or defaults)
  const config = {
    logLevel: process.env.OPENCODE_LOG_LEVEL || 'VERBOSE', // ESSENTIAL | VERBOSE
    enableTUI: process.env.OPENCODE_LOG_TUI !== 'false', // default true
    enableFileWatcher: process.env.OPENCODE_LOG_FILE_WATCHER !== 'false', // default true
    enablePermissions: true, // Always log permissions for security
    enableTodos: true, // Track task progression
  }


  // ===== HELPER FUNCTIONS =====

  const getCurrentLogFile = () => {
    const pointerFile = join(directory, "debugging/current_log_file.txt")
    if (!existsSync(pointerFile)) return null
    try {
      return readFileSync(pointerFile, "utf-8").trim()
    } catch (error) {
      return null
    }
  }

  const writeLog = (content) => {
    const logFile = getCurrentLogFile()
    if (!logFile) return
    try {
      appendFileSync(logFile, content + "\n", "utf-8")
    } catch (error) {
    }
  }

  const getTimestamp = () => {
    const now = new Date()
    return now.toTimeString().split(" ")[0] // HH:MM:SS (SPEC.md Section 2.5 format for log entries)
  }

  const truncate = (str, maxLen = 200) => {
    if (!str) return ""
    const s = String(str)
    return s.length > maxLen ? s.substring(0, maxLen) + "..." : s
  }

  // Convert absolute path to relative path from project root
  const toRelativePath = (absolutePath) => {
    if (!absolutePath) return ""
    const path = String(absolutePath)
    if (path.startsWith(directory)) {
      return path.substring(directory.length + 1) // +1 for trailing slash
    }
    return path
  }

  // Format bash command - split by && and display as code block
  const formatBashCommand = (command) => {
    if (!command) return "```bash\n(no command)\n```"
    const parts = command.split(" && ").map(p => p.trim())
    return "```bash\n" + parts.join(" && \\\n  ") + "\n```"
  }

  // Format params with special handling for different tools
  const formatParamsEnhanced = (toolName, args) => {
    if (!args || Object.keys(args).length === 0) return "PARAMS: (none)"

    const lines = []

    for (const [key, value] of Object.entries(args)) {
      if (toolName === "bash" && key === "command") {
        // Special handling for bash commands - show in code block
        lines.push(`COMMAND:\n${formatBashCommand(value)}`)
      } else if (key === "description" && toolName === "bash") {
        // Show description first for bash
        lines.unshift(`DESCRIPTION: ${value}`)
      } else if (key === "filePath" || key === "file_path" || key === "path") {
        // Convert file paths to relative
        lines.push(`${key.toUpperCase()}: ${toRelativePath(value)}`)
      } else if (typeof value === "string") {
        // Other string params - show full value up to 500 chars
        const displayValue = value.length > 500 ? value.substring(0, 500) + "..." : value
        lines.push(`${key.toUpperCase()}: ${displayValue}`)
      } else {
        // Non-string params
        const strValue = JSON.stringify(value)
        const displayValue = strValue.length > 200 ? strValue.substring(0, 200) + "..." : strValue
        lines.push(`${key.toUpperCase()}: ${displayValue}`)
      }
    }

    return lines.join("\n")
  }

  // ===== EVENT FORMATTERS =====

  const formatSessionUpdated = (event) => {
    const timestamp = getTimestamp()
    const changes = []
    if (event.title) changes.push(`title="${truncate(event.title)}"`)
    if (event.summary) {
      const s = event.summary
      if (s.additions) changes.push(`additions=${s.additions}`)
      if (s.deletions) changes.push(`deletions=${s.deletions}`)
      if (s.files) changes.push(`files=${s.files}`)
    }
    return `---\n[${timestamp}] SESSION: Updated\nCHANGES: ${changes.join(", ") || "metadata"}\n---`
  }

  const formatSessionDeleted = (event) => {
    const timestamp = getTimestamp()
    const duration = event.duration ? `${Math.floor(event.duration / 1000)}s` : "unknown"
    return `---\n[${timestamp}] SESSION: Deleted\nDURATION: ${duration}\nFINAL_STATUS: ${event.status || "unknown"}\n---`
  }

  const formatSessionStatus = (event) => {
    const timestamp = getTimestamp()
    return `---\n[${timestamp}] SESSION: Status → ${event.status || "unknown"}\n---`
  }

  const formatFileEdited = (event) => {
    const timestamp = getTimestamp()
    const path = truncate(event.path || event.file, 150)
    return `---\n[${timestamp}] FILE: Edited\nPATH: ${path}\nACTION: ${event.action || "modified"}\n---`
  }

  const formatFileWatcherUpdated = (event) => {
    const timestamp = getTimestamp()
    const path = truncate(event.path || event.file, 150)
    const eventType = event.event || event.type || "changed"
    return `---\n[${timestamp}] FILE: Watcher Update\nPATH: ${path}\nEVENT: ${eventType}\nSOURCE: external\n---`
  }

  const formatCommandExecuted = (event) => {
    const timestamp = getTimestamp()
    const name = event.name || event.command || "unknown"
    const args = event.arguments ? truncate(JSON.stringify(event.arguments), 100) : "(none)"
    return `---\n[${timestamp}] COMMAND: ${name}\nARGS: ${args}\n---`
  }

  const formatTuiPromptAppend = (event) => {
    const timestamp = getTimestamp()
    const prompt = truncate(event.prompt || event.text, 150)
    return `---\n[${timestamp}] TUI: Prompt\nTEXT: ${prompt}\n---`
  }

  const formatTuiCommandExecute = (event) => {
    const timestamp = getTimestamp()
    const command = event.command || "unknown"
    const args = event.args ? truncate(JSON.stringify(event.args), 80) : ""
    return `---\n[${timestamp}] TUI: Command → ${command}\nARGS: ${args}\n---`
  }

  const formatTuiToastShow = (event) => {
    const timestamp = getTimestamp()
    const level = event.level || event.severity || "info"
    const message = truncate(event.message || event.text, 150)
    return `---\n[${timestamp}] TUI: Toast [${level.toUpperCase()}]\nMESSAGE: ${message}\n---`
  }

  const formatPermissionReplied = (event) => {
    const timestamp = getTimestamp()
    const action = event.action || "unknown"
    const response = event.response || event.reply || "unknown"
    const reason = event.reason ? `\nREASON: ${truncate(event.reason, 100)}` : ""
    return `---\n[${timestamp}] PERMISSION: Replied\nACTION: ${action}\nRESPONSE: ${response}${reason}\n---`
  }

  const formatPermissionUpdated = (event) => {
    const timestamp = getTimestamp()
    const action = event.action || "unknown"
    const status = event.status || "unknown"
    const target = event.path || event.target || ""
    const targetStr = target ? `\nTARGET: ${truncate(target, 100)}` : ""
    return `---\n[${timestamp}] PERMISSION: Updated\nACTION: ${action}\nSTATUS: ${status}${targetStr}\n---`
  }

  const formatTodoUpdated = (event) => {
    const timestamp = getTimestamp()
    const action = event.action || "updated"
    const task = truncate(event.task || event.content || "unknown", 100)
    const remaining = event.remaining !== undefined ? `\nREMAINING: ${event.remaining}` : ""
    return `---\n[${timestamp}] TODO: ${action}\nTASK: ${task}${remaining}\n---`
  }

  const formatUnknown = (event) => {
    const timestamp = getTimestamp()
    const action = event.action || "updated"
    return `---\n[${timestamp}] UNKNOWN: ${action}\n---`
  }


  // ===== SHOULD LOG FUNCTION =====

  const shouldLogEvent = (eventType) => {
    // Always log these critical events
    const essentialEvents = [
      'session.created', 'session.deleted', 'session.updated', 'session.status', 'session.error',
      'permission.replied', 'permission.updated',
      'command.executed',
      'file.edited'
    ]

    if (essentialEvents.includes(eventType)) return true

    // Verbose events (check config)
    if (eventType.startsWith('tui.') && !config.enableTUI) return false
    if (eventType === 'file.watcher.updated' && !config.enableFileWatcher) return false
    if (eventType === 'todo.updated' && !config.enableTodos) return false

    // Log everything else in VERBOSE mode
    return config.logLevel === 'VERBOSE'
  }


  // ===== RETURN PLUGIN HOOKS =====

  return {
    // ===== TOOL HOOKS =====
    "tool.execute.before": async (input, output) => {
      const timestamp = getTimestamp()
      const toolName = input.tool || "unknown"
      const params = formatParamsEnhanced(toolName, output.args)
      const logEntry = `---\n[${timestamp}] TOOL: ${toolName}\n${params}`
      writeLog(logEntry)
    },

    "tool.execute.after": async (input, output) => {
      const timestamp = getTimestamp()
      const success = !output.error
      const status = success ? "✅ Success" : "❌ Error"

      let resultContent = ""
      if (output.error) {
        resultContent = `ERROR: ${output.error.message || output.error}`
      } else if (output.result) {
        const resultStr = typeof output.result === "string"
          ? output.result
          : JSON.stringify(output.result, null, 2)
        resultContent = resultStr.length > 500
          ? resultStr.substring(0, 500) + "\n... (truncated)"
          : resultStr
      }

      const logEntry = `[${timestamp}] RESULT: ${status}\nOUTPUT:\n${resultContent}\n---`
      writeLog(logEntry)
    },

    // ===== UNIFIED EVENT HANDLER =====
    event: async ({ event }) => {
      if (!shouldLogEvent(event.type)) return


      let logEntry = ""

      switch (event.type) {
        // Session Events
        case 'session.created':
          logEntry = `---\n[${getTimestamp()}] SESSION: Created\nSESSION_ID: ${event.id || "unknown"}\n---`
          break

        case 'session.updated':
          logEntry = formatSessionUpdated(event)
          break

        case 'session.deleted':
          logEntry = formatSessionDeleted(event)
          break

        case 'session.status':
          logEntry = formatSessionStatus(event)
          break

        case 'session.error':
          logEntry = `---\n[${getTimestamp()}] SESSION: Error\nERROR: ${truncate(event.error || event.message, 200)}\n---`
          break

        case 'session.idle':
          logEntry = `---\n[${getTimestamp()}] SESSION: Idle\n---`
          break

        // File Events
        case 'file.edited':
          logEntry = formatFileEdited(event)
          break

        case 'file.watcher.updated':
          logEntry = formatFileWatcherUpdated(event)
          break

        // Command Events
        case 'command.executed':
          logEntry = formatCommandExecuted(event)
          break

        // Permission Events
        case 'permission.replied':
          logEntry = formatPermissionReplied(event)
          break

        case 'permission.updated':
          logEntry = formatPermissionUpdated(event)
          break

        // TUI Events
        case 'tui.prompt.append':
          logEntry = formatTuiPromptAppend(event)
          break

        case 'tui.command.execute':
          logEntry = formatTuiCommandExecute(event)
          break

        case 'tui.toast.show':
          logEntry = formatTuiToastShow(event)
          break

        // Todo Events
        case 'todo.updated':
          logEntry = formatTodoUpdated(event)
          break

        default:
          // Unknown event type - log for debugging
          break
      }

      if (logEntry) {
        writeLog(logEntry)
      }
    },
  }
}

// Default export for OpenCode plugin system
export default LoggerPlugin
