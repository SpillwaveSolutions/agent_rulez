---
created: 2026-02-10T17:50:12.726Z
title: Fix inline script execution bugs
area: core
files:
  - rulez/src/hooks.rs:382-427
  - rulez/src/hooks.rs:361-367
  - rulez/src/hooks.rs:361-386
---

## Problem

Three bugs in `execute_inline_script` (rulez/src/hooks.rs):

1. **High — Timeout doesn't kill child process** (lines 382-427): When `tokio::time::timeout` fires, the child process is not killed. Repeated timeouts accumulate runaway scripts. Must call `child.kill().await` + `wait().await` to reap the process.

2. **Medium — Shebang ignored** (lines 361-367): Scripts are always invoked as `sh <script_path>`, so any shebang (e.g., `#!/usr/bin/env python`) is ignored. Scripts with non-sh shebangs fail silently under sh. Either execute the script directly (after `chmod 700`) to honor shebangs, or document that only POSIX sh is supported and remove the shebang warning from config validation.

3. **Medium — Pipe deadlock risk** (lines 361-386): stdout/stderr are piped but never read. A script writing >64KB can fill the OS pipe buffer and deadlock before exit, which then triggers a timeout (see bug #1). Switch to `wait_with_output()` or spawn tasks to drain pipes.

## Solution

1. On timeout `Err(_)`: call `child.kill().await` then `child.wait().await` to reap.
2. Decision needed: honor shebang (execute directly) vs. constrain to sh (update warnings/docs).
3. Replace `child.wait()` with `child.wait_with_output()` to drain pipes, or spawn drain tasks.
