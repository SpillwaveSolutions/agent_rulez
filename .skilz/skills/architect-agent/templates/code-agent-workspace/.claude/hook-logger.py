#!/usr/bin/env python3
"""
Enhanced Hook Logger for Claude Code (SPEC.md v4.1 Compliant)
Captures: Tools, Permissions, User Prompts, Notifications, Sessions, Compacts
File Naming: log-YYYY_MM_DD-HH_MM-description.md (SPEC.md Section 2.5)
"""
import json
import sys
from datetime import datetime
import os

try:
    # Read JSON from stdin
    data = json.load(sys.stdin)

    # Get current log file from debugging/current_log_file.txt
    log_file_path = None
    current_log_file = os.path.join(os.environ.get('CLAUDE_PROJECT_DIR', ''), 'debugging', 'current_log_file.txt')

    if os.path.exists(current_log_file):
        with open(current_log_file, 'r') as f:
            log_file_path = f.read().strip()

    # Only log if we have an active session
    if not log_file_path or not os.path.exists(log_file_path):
        sys.exit(0)

    # Determine event type from data
    event_type = data.get('event_type', 'tool')  # tool, permission, prompt, notification, session, compact, stop

    # Extract relevant information based on event type
    tool_name = data.get('tool_name', 'Unknown')
    tool_input = data.get('tool_input', {})

    # Format log entry based on event type
    timestamp = datetime.now().strftime('%H:%M:%SS')
    log_entry = ""

    # ===== USER PROMPT SUBMIT =====
    if event_type == 'user_prompt':
        prompt_text = data.get('prompt', '')
        prompt_type = data.get('prompt_type', 'chat')
        log_entry = f"\n---\n[{timestamp}] 💬 USER_PROMPT ({prompt_type})"
        # Truncate long prompts
        if len(prompt_text) > 300:
            log_entry += f"\nTEXT: {prompt_text[:300]}... (truncated)"
        else:
            log_entry += f"\nTEXT: {prompt_text}"
        log_entry += "\n---"

    # ===== PERMISSION REQUEST =====
    elif event_type == 'permission':
        action = data.get('action', 'unknown')
        tool = data.get('tool', '')
        resource = data.get('resource', '')
        response = data.get('response', 'pending')
        log_entry = f"\n---\n[{timestamp}] 🔒 PERMISSION: {action}"
        if tool:
            log_entry += f"\nTOOL: {tool}"
        if resource:
            log_entry += f"\nRESOURCE: {resource}"
        log_entry += f"\nRESPONSE: {response}"
        log_entry += "\n---"

    # ===== NOTIFICATION =====
    elif event_type == 'notification':
        notification_type = data.get('notification_type', 'unknown')
        message = data.get('message', '')
        level = data.get('level', 'info')
        log_entry = f"\n---\n[{timestamp}] 🔔 NOTIFICATION [{level.upper()}]"
        log_entry += f"\nTYPE: {notification_type}"
        if message:
            log_entry += f"\nMESSAGE: {message[:200]}"
        log_entry += "\n---"

    # ===== SESSION START =====
    elif event_type == 'session_start':
        session_type = data.get('session_type', 'unknown')  # startup, resume, clear, compact
        log_entry = f"\n---\n[{timestamp}] 🎬 SESSION_START: {session_type}"
        log_entry += "\n---"

    # ===== PRE-COMPACT =====
    elif event_type == 'pre_compact':
        compact_type = data.get('compact_type', 'unknown')  # manual, auto
        log_entry = f"\n---\n[{timestamp}] 🗜️  PRE_COMPACT: {compact_type}"
        log_entry += "\n---"

    # ===== STOP (Agent finished) =====
    elif event_type == 'stop':
        stop_reason = data.get('reason', 'completed')
        log_entry = f"\n---\n[{timestamp}] 🛑 AGENT_STOP: {stop_reason}"
        log_entry += "\n---"

    # ===== SUBAGENT STOP =====
    elif event_type == 'subagent_stop':
        subagent_name = data.get('subagent', 'unknown')
        log_entry = f"\n---\n[{timestamp}] 🔻 SUBAGENT_STOP: {subagent_name}"
        log_entry += "\n---"

    # ===== PRE-TOOL USE =====
    elif event_type == 'pre_tool':
        tool_name = data.get('tool_name', 'Unknown')
        log_entry = f"\n---\n[{timestamp}] ⏸️  PRE_TOOL: {tool_name}"
        # Log parameters but don't process yet
        if tool_input:
            log_entry += f"\nPARAMS_READY: Yes"
        log_entry += "\n---"

    # ===== POST-TOOL USE (Original functionality) =====
    else:
        log_entry = f"\n---\n[{timestamp}] TOOL: {tool_name}"

        # Add relevant tool details
        if tool_name == 'Bash':
            command = tool_input.get('command', '')
            description = tool_input.get('description', 'No description')
            log_entry += f"\nCOMMAND: {command[:200]}"  # First 200 chars
            log_entry += f"\nDESC: {description}"

        elif tool_name == 'Read':
            file_path = tool_input.get('file_path', '')
            offset = tool_input.get('offset')
            limit = tool_input.get('limit')
            log_entry += f"\nFILE: {file_path}"
            if offset is not None:
                log_entry += f"\nOFFSET: {offset}"
            if limit is not None:
                log_entry += f"\nLIMIT: {limit}"

        elif tool_name == 'Write':
            file_path = tool_input.get('file_path', '')
            content = tool_input.get('content', '')
            log_entry += f"\nFILE: {file_path}"
            log_entry += f"\nSIZE: {len(content)} chars"

        elif tool_name == 'Edit':
            file_path = tool_input.get('file_path', '')
            old_string = tool_input.get('old_string', '')
            new_string = tool_input.get('new_string', '')
            replace_all = tool_input.get('replace_all', False)
            log_entry += f"\nFILE: {file_path}"
            log_entry += f"\nOLD: {old_string[:100]}..."
            log_entry += f"\nNEW: {new_string[:100]}..."
            if replace_all:
                log_entry += f"\nREPLACE_ALL: True"

        elif tool_name == 'Grep':
            pattern = tool_input.get('pattern', '')
            path = tool_input.get('path', '')
            output_mode = tool_input.get('output_mode', 'files_with_matches')
            glob = tool_input.get('glob', '')
            case_insensitive = tool_input.get('-i', False)
            log_entry += f"\nPATTERN: {pattern}"
            if path:
                log_entry += f"\nPATH: {path}"
            if glob:
                log_entry += f"\nGLOB: {glob}"
            log_entry += f"\nMODE: {output_mode}"
            if case_insensitive:
                log_entry += f"\nCASE_INSENSITIVE: True"

        elif tool_name == 'Glob':
            pattern = tool_input.get('pattern', '')
            path = tool_input.get('path', '')
            log_entry += f"\nPATTERN: {pattern}"
            if path:
                log_entry += f"\nPATH: {path}"

        elif tool_name == 'TodoWrite':
            todos = tool_input.get('todos', [])
            log_entry += f"\nTODOS: {len(todos)} items"
            # Show status summary
            completed = sum(1 for t in todos if t.get('status') == 'completed')
            in_progress = sum(1 for t in todos if t.get('status') == 'in_progress')
            pending = sum(1 for t in todos if t.get('status') == 'pending')
            log_entry += f"\nSTATUS: {completed} done, {in_progress} active, {pending} pending"

        log_entry += "\n---"

    # Write to log
    with open(log_file_path, 'a') as f:
        f.write(log_entry + '\n')

    sys.exit(0)

except Exception as e:
    # Silent failure - don't break hooks
    sys.exit(0)
