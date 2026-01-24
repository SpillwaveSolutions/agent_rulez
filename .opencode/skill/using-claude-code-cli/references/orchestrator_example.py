#!/usr/bin/env python3
"""Reference implementation of CLIOrchestrator class with all invocation patterns."""

import asyncio
import json
import logging
import re
import subprocess
import time
from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
from typing import Any

logger = logging.getLogger(__name__)


class InvocationStatus(str, Enum):
    """Status of a CLI invocation."""
    SUCCESS = "success"
    FAILED = "failed"
    TIMEOUT = "timeout"
    FALLBACK = "fallback"


@dataclass
class CLIInvocationResult:
    """Result of a CLI tool invocation."""
    status: InvocationStatus
    tool: str
    output: str
    parsed_json: dict[str, Any] | None = None
    duration_ms: int = 0
    returncode: int = 0
    error: str | None = None
    fallback_used: bool = False
    command: list[str] = field(default_factory=list)


class CLIOrchestrator:
    """Orchestrator for invoking Claude and OpenCode CLI tools."""

    # Default tools to pre-approve for automation
    DEFAULT_ALLOWED_TOOLS = [
        "Write", "Read", "Edit", "Glob", "Grep", "LS",
        "Bash", "Task", "Skill",
        "WebFetch", "WebSearch",
        "mcp__perplexity-ask__perplexity_ask",
        "mcp__context7__resolve-library-id",
        "mcp__context7__get-library-docs",
        "mcp__brave-search__brave_web_search",
    ]

    def __init__(
        self,
        primary_cli: str = "claude",
        fallback_cli: str | None = "opencode",
        default_timeout: int = 600,
        project_dir: Path | None = None,
    ):
        self.primary_cli = primary_cli
        self.fallback_cli = fallback_cli if fallback_cli != "none" else None
        self.default_timeout = default_timeout
        self.project_dir = project_dir or Path.cwd()

    def build_command(
        self,
        tool: str,
        prompt: str,
        model: str | None = None,
        allowed_tools: list[str] | None = None,
        add_dirs: list[str | Path] | None = None,
        settings_json: str | None = None,
        enable_sandbox: bool = False,
    ) -> tuple[list[str], str | None]:
        """Build the CLI command with all options."""
        stdin_content: str | None = None

        if tool == "opencode":
            cmd = ["opencode", "run"]
            if model:
                cmd.extend(["--model", model])
            cmd.append(prompt)
            return cmd, stdin_content

        cmd = ["claude"]
        if model:
            cmd.extend(["--model", model])
        if settings_json:
            cmd.extend(["--settings", settings_json])
        if add_dirs:
            for dir_path in add_dirs:
                cmd.extend(["--add-dir", str(dir_path)])
        if allowed_tools:
            for tool_name in allowed_tools:
                cmd.extend(["--allowedTools", tool_name])

        if enable_sandbox:
            cmd.append("-p")
            stdin_content = f"/sandbox\n{prompt}"
        else:
            cmd.extend(["-p", prompt])

        return cmd, stdin_content

    def generate_settings_json(
        self,
        hook_script: Path | None = None,
        additional_dirs: list[str | Path] | None = None,
    ) -> str:
        """Generate settings JSON for hooks and permissions."""
        settings: dict[str, Any] = {}

        if hook_script and hook_script.exists():
            hook_path = str(hook_script.absolute())
            hook_config = [{"type": "command", "command": hook_path, "timeout": 5}]
            settings["hooks"] = {
                "PostToolUse": [{"matcher": "*", "hooks": hook_config}],
                "PreToolUse": [{"matcher": "*", "hooks": hook_config}],
            }

        if additional_dirs:
            settings["permissions"] = {
                "additionalDirectories": [str(d) for d in additional_dirs]
            }

        return json.dumps(settings)

    def extract_json_from_output(self, output: str) -> dict[str, Any] | None:
        """Extract JSON from CLI output."""
        if not output:
            return None

        try:
            return json.loads(output)
        except json.JSONDecodeError:
            pass

        pattern = r"```(?:json)?\s*\n([\s\S]*?)\n```"
        for match in re.findall(pattern, output):
            try:
                return json.loads(match.strip())
            except json.JSONDecodeError:
                continue

        for p in [r"(\{[\s\S]*\})", r"(\[[\s\S]*\])"]:
            for match in re.findall(p, output):
                try:
                    return json.loads(match)
                except json.JSONDecodeError:
                    continue

        return None

    def invoke_sync(
        self,
        prompt: str,
        tool: str | None = None,
        timeout: int | None = None,
        working_dir: Path | str | None = None,
        allowed_tools: list[str] | None = None,
    ) -> CLIInvocationResult:
        """Invoke CLI tool synchronously."""
        tool = tool or self.primary_cli
        timeout = timeout or self.default_timeout
        allowed_tools = allowed_tools or self.DEFAULT_ALLOWED_TOOLS

        start_time = time.time()
        cmd, stdin_content = self.build_command(
            tool=tool,
            prompt=prompt,
            allowed_tools=allowed_tools if tool == "claude" else None,
        )

        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=timeout,
                cwd=working_dir,
                input=stdin_content,
            )

            duration_ms = int((time.time() - start_time) * 1000)

            if result.returncode == 0:
                return CLIInvocationResult(
                    status=InvocationStatus.SUCCESS,
                    tool=tool,
                    output=result.stdout,
                    parsed_json=self.extract_json_from_output(result.stdout),
                    duration_ms=duration_ms,
                    returncode=result.returncode,
                    command=cmd,
                )
            else:
                return CLIInvocationResult(
                    status=InvocationStatus.FAILED,
                    tool=tool,
                    output=result.stdout,
                    error=result.stderr,
                    duration_ms=duration_ms,
                    returncode=result.returncode,
                    command=cmd,
                )

        except subprocess.TimeoutExpired:
            return CLIInvocationResult(
                status=InvocationStatus.TIMEOUT,
                tool=tool,
                output="",
                error=f"Timeout after {timeout}s",
                duration_ms=int((time.time() - start_time) * 1000),
                returncode=-1,
                command=cmd,
            )

    async def invoke_async(
        self,
        prompt: str,
        tool: str | None = None,
        timeout: int | None = None,
        working_dir: Path | str | None = None,
        allowed_tools: list[str] | None = None,
    ) -> CLIInvocationResult:
        """Invoke CLI tool asynchronously using safe subprocess (no shell)."""
        tool = tool or self.primary_cli
        timeout = timeout or self.default_timeout
        allowed_tools = allowed_tools or self.DEFAULT_ALLOWED_TOOLS

        start_time = time.time()
        cmd, stdin_content = self.build_command(
            tool=tool,
            prompt=prompt,
            allowed_tools=allowed_tools if tool == "claude" else None,
        )

        try:
            # Safe async subprocess - passes args directly, no shell
            process = await asyncio.create_subprocess_exec(
                *cmd,
                stdin=asyncio.subprocess.PIPE if stdin_content else None,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
                cwd=str(working_dir) if working_dir else None,
            )

            if stdin_content and process.stdin:
                process.stdin.write(stdin_content.encode("utf-8"))
                await process.stdin.drain()
                process.stdin.close()

            stdout, stderr = await asyncio.wait_for(
                process.communicate(),
                timeout=timeout,
            )

            duration_ms = int((time.time() - start_time) * 1000)
            stdout_str = stdout.decode("utf-8") if stdout else ""

            if process.returncode == 0:
                return CLIInvocationResult(
                    status=InvocationStatus.SUCCESS,
                    tool=tool,
                    output=stdout_str,
                    parsed_json=self.extract_json_from_output(stdout_str),
                    duration_ms=duration_ms,
                    returncode=process.returncode or 0,
                    command=cmd,
                )
            else:
                return CLIInvocationResult(
                    status=InvocationStatus.FAILED,
                    tool=tool,
                    output=stdout_str,
                    error=stderr.decode("utf-8") if stderr else "",
                    duration_ms=duration_ms,
                    returncode=process.returncode or -1,
                    command=cmd,
                )

        except TimeoutError:
            return CLIInvocationResult(
                status=InvocationStatus.TIMEOUT,
                tool=tool,
                output="",
                error=f"Timeout after {timeout}s",
                duration_ms=int((time.time() - start_time) * 1000),
                returncode=-1,
                command=cmd,
            )

    async def invoke_parallel(
        self,
        prompts: list[str],
        max_concurrent: int = 3,
    ) -> list[CLIInvocationResult]:
        """Run multiple prompts in parallel with concurrency limit."""
        semaphore = asyncio.Semaphore(max_concurrent)

        async def invoke_with_limit(prompt: str) -> CLIInvocationResult:
            async with semaphore:
                return await self.invoke_async(prompt)

        tasks = [invoke_with_limit(p) for p in prompts]
        return await asyncio.gather(*tasks)


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    
    orchestrator = CLIOrchestrator()
    result = orchestrator.invoke_sync("What is 2 + 2?")
    print(f"Status: {result.status}, Output: {result.output[:100]}...")
