# Python Subprocess Patterns

Code patterns for invoking Claude CLI from Python orchestrators.

## Contents

- [Synchronous Invocation](#synchronous-invocation)
- [Async Invocation](#async-invocation)
- [Parallel with Semaphore](#parallel-with-semaphore)
- [Sandbox via Stdin](#sandbox-via-stdin)
- [Common Mistakes](#common-mistakes)

---

## Synchronous Invocation

Basic blocking call with timeout and error handling:

```python
import subprocess

def invoke_claude(prompt: str, allowed_tools: list[str] = None,
                  add_dirs: list[str] = None, timeout: int = 600) -> str:
    """Invoke Claude CLI synchronously.

    Args:
        prompt: Prompt text for invocation
        allowed_tools: Tools to pre-approve (eliminates permission prompts)
        add_dirs: Additional directories to grant access
        timeout: Maximum execution time in seconds

    Returns:
        stdout from Claude CLI

    Raises:
        RuntimeError: If Claude returns non-zero exit code
        subprocess.TimeoutExpired: If execution exceeds timeout
    """
    cmd = ["claude"]

    for tool in (allowed_tools or []):
        cmd.extend(["--allowedTools", tool])

    for dir_path in (add_dirs or []):
        cmd.extend(["--add-dir", dir_path])

    cmd.extend(["-p", prompt])

    result = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        timeout=timeout,
    )

    if result.returncode != 0:
        raise RuntimeError(f"Claude failed: {result.stderr}")

    return result.stdout
```

**Usage:**
```python
output = invoke_claude(
    prompt="Generate a Python function to validate emails",
    allowed_tools=["Write", "Read", "Edit"],
    add_dirs=["/path/to/project"],
    timeout=300,
)
```

---

## Async Invocation

Non-blocking call using `asyncio.create_subprocess_exec` (safe, no shell):

```python
import asyncio

async def invoke_claude_async(prompt: str, working_dir: str = None,
                               allowed_tools: list[str] = None,
                               timeout: int = 600) -> str:
    """Invoke Claude CLI asynchronously.

    Uses create_subprocess_exec for safe async invocation without shell.

    Args:
        prompt: Prompt text for invocation
        working_dir: Directory for execution context
        allowed_tools: Tools to pre-approve
        timeout: Maximum wait time in seconds

    Returns:
        stdout from Claude CLI
    """
    cmd = ["claude"]

    for tool in (allowed_tools or []):
        cmd.extend(["--allowedTools", tool])

    cmd.extend(["-p", prompt])

    process = await asyncio.create_subprocess_exec(
        *cmd,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
        cwd=working_dir,
    )

    stdout, stderr = await asyncio.wait_for(
        process.communicate(),
        timeout=timeout,
    )

    return stdout.decode("utf-8")
```

**Usage:**
```python
output = await invoke_claude_async(
    prompt="Explain this error",
    working_dir="/path/to/project",
    allowed_tools=["Read", "Grep"],
)
```

---

## Parallel with Semaphore

Run multiple prompts concurrently with rate limiting:

```python
import asyncio

async def invoke_parallel(prompts: list[str], max_concurrent: int = 3,
                          allowed_tools: list[str] = None) -> list[str]:
    """Run multiple Claude invocations in parallel with concurrency limit.

    Args:
        prompts: List of prompts to process
        max_concurrent: Maximum simultaneous invocations
        allowed_tools: Tools to pre-approve for all invocations

    Returns:
        List of outputs in same order as prompts
    """
    semaphore = asyncio.Semaphore(max_concurrent)

    async def invoke_with_limit(prompt: str) -> str:
        async with semaphore:
            return await invoke_claude_async(
                prompt,
                allowed_tools=allowed_tools,
            )

    tasks = [invoke_with_limit(p) for p in prompts]
    return await asyncio.gather(*tasks)
```

**Usage:**
```python
prompts = [
    "Generate tests for auth module",
    "Generate tests for api module",
    "Generate tests for db module",
]
results = await invoke_parallel(prompts, max_concurrent=2)
```

---

## Sandbox via Stdin

Enable sandbox mode by passing `/sandbox` command before prompt:

```python
import subprocess

def invoke_with_sandbox(prompt: str, timeout: int = 600) -> str:
    """Invoke Claude with sandbox mode via stdin.

    Sandbox mode restricts file operations to safe directories.
    The /sandbox command is passed via stdin before the actual prompt.

    Args:
        prompt: The prompt to run in sandbox mode
        timeout: Maximum time in seconds

    Returns:
        stdout from Claude CLI
    """
    stdin_content = f"/sandbox\n{prompt}"

    result = subprocess.run(
        ["claude", "-p"],
        input=stdin_content,
        capture_output=True,
        text=True,
        timeout=timeout,
    )
    return result.stdout
```

**Usage:**
```python
# Safe mode for untrusted prompts
output = invoke_with_sandbox("Process the uploaded file")
```

---

## Common Mistakes

### Mistake 1: Using shell=True

```python
# DANGEROUS - allows shell injection attacks
subprocess.run(f"claude -p '{prompt}'", shell=True)

# SAFE - pass arguments as a list (no shell)
subprocess.run(["claude", "-p", prompt])
```

### Mistake 2: Missing timeout

```python
# DANGEROUS - can hang indefinitely
subprocess.run(["claude", "-p", prompt])

# SAFE - always specify timeout
subprocess.run(["claude", "-p", prompt], timeout=600)
```

### Mistake 3: Ignoring return code

```python
# BAD - silent failures go unnoticed
result = subprocess.run(["claude", "-p", prompt], capture_output=True)
print(result.stdout)  # May be empty if command failed

# GOOD - check return code
if result.returncode != 0:
    raise RuntimeError(f"Failed: {result.stderr}")
```

---

## See Also

- [orchestrator_example.py](orchestrator_example.py) - Complete orchestrator class
- [hooks-examples.md](hooks-examples.md) - Hook configuration patterns
- [json-extraction.md](json-extraction.md) - Parsing JSON from output
