# JSON Extraction from CLI Output

Patterns for parsing JSON from Claude CLI output.

## Contents

- [The Problem](#the-problem)
- [Extraction Function](#extraction-function)
- [Using --output-format json](#using---output-format-json)
- [Extraction Strategies](#extraction-strategies)

---

## The Problem

Claude CLI output may contain JSON in various formats:
1. Pure JSON (when using `--output-format json`)
2. JSON embedded in markdown code blocks
3. JSON mixed with explanatory text

---

## Extraction Function

Robust function to extract JSON from any output format:

```python
import json
import re
from typing import Any


def extract_json(output: str) -> dict[str, Any] | list | None:
    """Extract JSON from Claude CLI output.

    Tries multiple strategies:
    1. Direct parse (pure JSON output)
    2. Markdown code blocks (```json ... ```)
    3. First JSON object/array in text

    Args:
        output: Raw stdout from Claude CLI

    Returns:
        Parsed JSON as dict/list, or None if no JSON found
    """
    if not output:
        return None

    # Strategy 1: Try direct parse (cleanest case)
    try:
        return json.loads(output)
    except json.JSONDecodeError:
        pass

    # Strategy 2: Extract from markdown code blocks
    pattern = r"```(?:json)?\s*\n([\s\S]*?)\n```"
    for match in re.findall(pattern, output):
        try:
            return json.loads(match.strip())
        except json.JSONDecodeError:
            continue

    # Strategy 3: Find first JSON object or array
    for p in [r"(\{[\s\S]*\})", r"(\[[\s\S]*\])"]:
        for match in re.findall(p, output):
            try:
                return json.loads(match)
            except json.JSONDecodeError:
                continue

    return None
```

---

## Using --output-format json

For automation, prefer `--output-format json` to get structured output:

```python
import subprocess
import json


def invoke_claude_json(prompt: str) -> dict:
    """Invoke Claude with JSON output format.

    Args:
        prompt: The prompt to send

    Returns:
        Parsed JSON response

    Raises:
        RuntimeError: If response cannot be parsed
    """
    result = subprocess.run(
        ["claude", "-p", prompt, "--output-format", "json"],
        capture_output=True,
        text=True,
        timeout=600,
    )

    if result.returncode != 0:
        raise RuntimeError(f"Claude failed: {result.stderr}")

    try:
        return json.loads(result.stdout)
    except json.JSONDecodeError as e:
        raise RuntimeError(f"Invalid JSON response: {e}")
```

**Response structure with --output-format json:**
```json
{
  "type": "result",
  "result": "The actual response content...",
  "session_id": "abc123...",
  "cost_usd": 0.0042,
  "is_error": false
}
```

---

## Extraction Strategies

### Strategy 1: Direct Parse

Works when output is pure JSON:

```python
data = json.loads(output)
```

**When it works:**
- Using `--output-format json`
- Claude returns only JSON without explanation

### Strategy 2: Markdown Code Blocks

Extract from ` ```json ... ``` ` blocks:

```python
import re

pattern = r"```(?:json)?\s*\n([\s\S]*?)\n```"
matches = re.findall(pattern, output)
for match in matches:
    try:
        return json.loads(match.strip())
    except json.JSONDecodeError:
        continue
```

**When it works:**
- Claude wraps JSON in code blocks
- Multiple code blocks (returns first valid)

### Strategy 3: Greedy Object/Array Match

Find JSON anywhere in text:

```python
import re

# Find objects
for match in re.findall(r"(\{[\s\S]*\})", output):
    try:
        return json.loads(match)
    except json.JSONDecodeError:
        continue

# Find arrays
for match in re.findall(r"(\[[\s\S]*\])", output):
    try:
        return json.loads(match)
    except json.JSONDecodeError:
        continue
```

**Caveats:**
- May match incomplete JSON if nested
- Falls back through all matches until valid

---

## Using --json-schema for Validation

For strict JSON output matching a schema:

```bash
claude -p "List 3 programming languages" \
  --json-schema '{
    "type": "array",
    "items": {
      "type": "object",
      "properties": {
        "name": {"type": "string"},
        "year": {"type": "integer"}
      },
      "required": ["name", "year"]
    }
  }'
```

**In Python:**
```python
import subprocess
import json

schema = {
    "type": "array",
    "items": {
        "type": "object",
        "properties": {
            "name": {"type": "string"},
            "year": {"type": "integer"}
        },
        "required": ["name", "year"]
    }
}

result = subprocess.run(
    [
        "claude", "-p", "List 3 programming languages",
        "--json-schema", json.dumps(schema)
    ],
    capture_output=True,
    text=True,
)
# Output will be valid JSON matching the schema
```

---

## See Also

- [cli-reference.md](cli-reference.md) - Full CLI flag reference
- [subprocess-patterns.md](subprocess-patterns.md) - Python invocation patterns
- [orchestrator_example.py](orchestrator_example.py) - Complete orchestrator class
