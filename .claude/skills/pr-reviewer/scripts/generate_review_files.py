#!/usr/bin/env python3
"""
Generate structured review files from PR analysis.

Creates three review files:
- pr/review.md: Detailed review for internal use
- pr/human.md: Short, clean review for posting (no emojis, em-dashes, line numbers)
- pr/inline.md: List of inline comments with code snippets

Usage:
    python generate_review_files.py <pr_review_dir> --findings <findings_json>

Example:
    python generate_review_files.py /tmp/PRs/myrepo/123 --findings findings.json
"""

import argparse
import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Any


def create_pr_directory(pr_review_dir: Path) -> Path:
    """Create the pr/ subdirectory for review files."""
    pr_dir = pr_review_dir / "pr"
    pr_dir.mkdir(parents=True, exist_ok=True)
    return pr_dir


def load_findings(findings_file: str) -> Dict[str, Any]:
    """
    Load review findings from JSON file.

    Expected structure:
    {
        "summary": "Overall assessment...",
        "blockers": [{
            "category": "Security",
            "issue": "SQL injection vulnerability",
            "file": "src/db/queries.py",
            "line": 45,
            "details": "Using string concatenation...",
            "fix": "Use parameterized queries",
            "code_snippet": "result = db.execute(...)"
        }],
        "important": [...],
        "nits": [...],
        "suggestions": [...],
        "questions": [...],
        "praise": [...],
        "inline_comments": [{
            "file": "src/app.py",
            "line": 42,
            "comment": "Consider edge case handling",
            "code_snippet": "def process(data):\n    return data.strip()",
            "start_line": 41,
            "end_line": 43
        }]
    }
    """
    with open(findings_file, 'r') as f:
        return json.load(f)


def generate_detailed_review(findings: Dict[str, Any], metadata: Dict[str, Any]) -> str:
    """Generate detailed review.md with full analysis."""

    review = f"""# Pull Request Review - Detailed Analysis

## PR Information

**Repository**: {metadata.get('repository', 'N/A')}
**PR Number**: #{metadata.get('number', 'N/A')}
**Title**: {metadata.get('title', 'N/A')}
**Author**: {metadata.get('author', 'N/A')}
**Branch**: {metadata.get('head_branch', 'N/A')} → {metadata.get('base_branch', 'N/A')}

## Summary

{findings.get('summary', 'No summary provided')}

"""

    # Add blockers
    blockers = findings.get('blockers', [])
    if blockers:
        review += "## 🔴 Critical Issues (Blockers)\n\n"
        review += "**These MUST be fixed before merging.**\n\n"
        for i, blocker in enumerate(blockers, 1):
            review += f"### {i}. {blocker.get('category', 'Issue')}: {blocker.get('issue', 'Unknown')}\n\n"
            if blocker.get('file'):
                review += f"**File**: `{blocker['file']}"
                if blocker.get('line'):
                    review += f":{blocker['line']}"
                review += "`\n\n"
            review += f"**Problem**: {blocker.get('details', 'No details')}\n\n"
            if blocker.get('fix'):
                review += f"**Solution**: {blocker['fix']}\n\n"
            if blocker.get('code_snippet'):
                review += f"**Current Code**:\n```\n{blocker['code_snippet']}\n```\n\n"
            review += "---\n\n"

    # Add important issues
    important = findings.get('important', [])
    if important:
        review += "## 🟡 Important Issues\n\n"
        review += "**Should be addressed before merging.**\n\n"
        for i, issue in enumerate(important, 1):
            review += f"### {i}. {issue.get('category', 'Issue')}: {issue.get('issue', 'Unknown')}\n\n"
            if issue.get('file'):
                review += f"**File**: `{issue['file']}"
                if issue.get('line'):
                    review += f":{issue['line']}"
                review += "`\n\n"
            review += f"**Impact**: {issue.get('details', 'No details')}\n\n"
            if issue.get('fix'):
                review += f"**Suggestion**: {issue['fix']}\n\n"
            if issue.get('code_snippet'):
                review += f"**Code**:\n```\n{issue['code_snippet']}\n```\n\n"
            review += "---\n\n"

    # Add nits
    nits = findings.get('nits', [])
    if nits:
        review += "## 🟢 Minor Issues (Nits)\n\n"
        review += "**Nice to have, but not blocking.**\n\n"
        for i, nit in enumerate(nits, 1):
            review += f"{i}. **{nit.get('category', 'Style')}**: {nit.get('issue', 'Unknown')}\n"
            if nit.get('file'):
                review += f"   - File: `{nit['file']}`\n"
            if nit.get('details'):
                review += f"   - {nit['details']}\n"
            review += "\n"

    # Add suggestions
    suggestions = findings.get('suggestions', [])
    if suggestions:
        review += "## 💡 Suggestions for Future\n\n"
        for i, suggestion in enumerate(suggestions, 1):
            review += f"{i}. {suggestion}\n"
        review += "\n"

    # Add questions
    questions = findings.get('questions', [])
    if questions:
        review += "## ❓ Questions / Clarifications Needed\n\n"
        for i, question in enumerate(questions, 1):
            review += f"{i}. {question}\n"
        review += "\n"

    # Add praise
    praise = findings.get('praise', [])
    if praise:
        review += "## ✅ Positive Notes\n\n"
        for item in praise:
            review += f"- {item}\n"
        review += "\n"

    # Add overall recommendation
    review += "## Overall Recommendation\n\n"
    if blockers:
        review += "**Request Changes** - Critical issues must be addressed.\n"
    elif important:
        review += "**Request Changes** - Important issues should be fixed.\n"
    else:
        review += "**Approve** - Looks good! Minor nits can be addressed optionally.\n"

    return review


def generate_human_review(findings: Dict[str, Any], metadata: Dict[str, Any]) -> str:
    """
    Generate short, clean human.md for posting.

    Rules:
    - No emojis
    - No em dashes (use regular hyphens)
    - No code line numbers
    - Concise and professional
    """

    def clean_text(text: str) -> str:
        """Remove em-dashes and replace with regular hyphens."""
        if not text:
            return text
        # Replace em dash (—) with regular hyphen (-)
        # Also replace en dash (–) with regular hyphen
        return text.replace('—', '-').replace('–', '-')

    title = clean_text(metadata.get('title', 'N/A'))
    summary = clean_text(findings.get('summary', 'No summary provided'))

    review = f"""# Code Review

**PR #{metadata.get('number', 'N/A')}**: {title}

## Summary

{summary}

"""

    # Add blockers - no emojis
    blockers = findings.get('blockers', [])
    if blockers:
        review += "## Critical Issues - Must Fix\n\n"
        for i, blocker in enumerate(blockers, 1):
            # No emojis, no em dashes, no line numbers
            issue = clean_text(blocker.get('issue', 'Issue'))
            details = clean_text(blocker.get('details', 'No details'))
            fix = clean_text(blocker.get('fix', ''))

            review += f"{i}. **{issue}**\n"
            if blocker.get('file'):
                # File path without line number
                review += f"   - File: `{blocker['file']}`\n"
            review += f"   - {details}\n"
            if fix:
                review += f"   - Fix: {fix}\n"
            review += "\n"

    # Add important issues
    important = findings.get('important', [])
    if important:
        review += "## Important Issues - Should Fix\n\n"
        for i, issue_item in enumerate(important, 1):
            issue = clean_text(issue_item.get('issue', 'Issue'))
            details = clean_text(issue_item.get('details', 'No details'))
            fix = clean_text(issue_item.get('fix', ''))

            review += f"{i}. **{issue}**\n"
            if issue_item.get('file'):
                review += f"   - File: `{issue_item['file']}`\n"
            review += f"   - {details}\n"
            if fix:
                review += f"   - Suggestion: {fix}\n"
            review += "\n"

    # Add nits - keep brief
    nits = findings.get('nits', [])
    if nits and len(nits) <= 3:  # Only include if few
        review += "## Minor Issues\n\n"
        for i, nit in enumerate(nits, 1):
            issue = clean_text(nit.get('issue', 'Issue'))
            review += f"{i}. {issue}"
            if nit.get('file'):
                review += f" in `{nit['file']}`"
            review += "\n"
        review += "\n"

    # Add praise
    praise = findings.get('praise', [])
    if praise:
        review += "## Positive Notes\n\n"
        for item in praise:
            clean_item = clean_text(item)
            review += f"- {clean_item}\n"
        review += "\n"

    # Add overall recommendation - no emojis
    if blockers:
        review += "## Recommendation\n\nRequest changes - critical issues need to be addressed before merging.\n"
    elif important:
        review += "## Recommendation\n\nRequest changes - please address the important issues listed above.\n"
    else:
        review += "## Recommendation\n\nApprove - the code looks good. Minor items can be addressed optionally.\n"

    return review


def generate_inline_comments_file(findings: Dict[str, Any]) -> str:
    """
    Generate inline.md with list of proposed inline comments.

    Includes code snippets with line number headers.
    """

    inline_comments = findings.get('inline_comments', [])

    if not inline_comments:
        return "# Inline Comments\n\nNo inline comments proposed.\n"

    content = "# Proposed Inline Comments\n\n"
    content += f"**Total Comments**: {len(inline_comments)}\n\n"
    content += "Review these before posting. Edit as needed.\n\n"
    content += "---\n\n"

    for i, comment in enumerate(inline_comments, 1):
        content += f"## Comment {i}\n\n"
        content += f"**File**: `{comment.get('file', 'unknown')}`\n"
        content += f"**Line**: {comment.get('line', 'N/A')}\n"

        if comment.get('start_line') and comment.get('end_line'):
            content += f"**Range**: Lines {comment['start_line']}-{comment['end_line']}\n"

        content += f"\n**Comment**:\n{comment.get('comment', 'No comment')}\n\n"

        if comment.get('code_snippet'):
            # Add line numbers in header
            start = comment.get('start_line', comment.get('line', 1))
            end = comment.get('end_line', comment.get('line', 1))

            if start == end:
                content += f"**Code (Line {start})**:\n"
            else:
                content += f"**Code (Lines {start}-{end})**:\n"

            content += f"```\n{comment['code_snippet']}\n```\n\n"

        # Add command to post this comment
        owner = comment.get('owner', 'OWNER')
        repo = comment.get('repo', 'REPO')
        pr_num = comment.get('pr_number', 'PR_NUM')

        content += "**Command to post**:\n```bash\n"
        content += f"python scripts/add_inline_comment.py {owner} {repo} {pr_num} latest \\\n"
        content += f"  \"{comment.get('file', 'file.py')}\" {comment.get('line', 42)} \\\n"
        content += f"  \"{comment.get('comment', 'comment')}\"\n"
        content += "```\n\n"
        content += "---\n\n"

    return content


def generate_claude_commands(pr_review_dir: Path, metadata: Dict[str, Any]):
    """Generate .claude directory with custom slash commands."""

    claude_dir = pr_review_dir / ".claude" / "commands"
    claude_dir.mkdir(parents=True, exist_ok=True)

    owner = metadata.get('owner', 'owner')
    repo = metadata.get('repo', 'repo')
    pr_number = metadata.get('number', '123')

    # /send command - approve and post human.md
    send_cmd = f"""Post the human-friendly review and approve the PR.

Steps:
1. Read the file `pr/human.md` in the current directory
2. Post the review content as a PR comment using:
   `gh pr comment {pr_number} --repo {owner}/{repo} --body-file pr/human.md`
3. Approve the PR using:
   `gh pr review {pr_number} --repo {owner}/{repo} --approve`
4. Confirm to the user that the review was posted and PR was approved
"""

    with open(claude_dir / "send.md", 'w') as f:
        f.write(send_cmd)

    # /send-decline command - request changes and post human.md
    send_decline_cmd = f"""Post the human-friendly review and request changes on the PR.

Steps:
1. Read the file `pr/human.md` in the current directory
2. Post the review content as a PR comment using:
   `gh pr comment {pr_number} --repo {owner}/{repo} --body-file pr/human.md`
3. Request changes on the PR using:
   `gh pr review {pr_number} --repo {owner}/{repo} --request-changes`
4. Confirm to the user that the review was posted and changes were requested
"""

    with open(claude_dir / "send-decline.md", 'w') as f:
        f.write(send_decline_cmd)

    # /show command - open in VS Code
    show_cmd = f"""Open the PR review directory in VS Code for editing.

Steps:
1. Run `code .` to open the current directory in VS Code
2. Tell the user they can now edit the review files:
   - pr/review.md (detailed review)
   - pr/human.md (short review for posting)
   - pr/inline.md (inline comments)
3. Remind them to use /send or /send-decline when ready to post
"""

    with open(claude_dir / "show.md", 'w') as f:
        f.write(show_cmd)

    print(f"✅ Created slash commands in {claude_dir}")
    print("   - /send (approve and post)")
    print("   - /send-decline (request changes and post)")
    print("   - /show (open in VS Code)")


def main():
    parser = argparse.ArgumentParser(
        description='Generate structured review files from PR analysis',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    parser.add_argument('pr_review_dir', help='PR review directory path')
    parser.add_argument('--findings', required=True, help='JSON file with review findings')
    parser.add_argument('--metadata', help='JSON file with PR metadata (optional)')

    args = parser.parse_args()

    try:
        # Load findings
        findings = load_findings(args.findings)

        # Load metadata if provided
        metadata = {}
        if args.metadata and os.path.exists(args.metadata):
            with open(args.metadata, 'r') as f:
                metadata = json.load(f)

        # Extract metadata from findings if not provided
        if not metadata:
            metadata = findings.get('metadata', {})

        # Create pr directory
        pr_review_dir = Path(args.pr_review_dir)
        pr_dir = create_pr_directory(pr_review_dir)

        print(f"📝 Generating review files in {pr_dir}...")

        # Generate detailed review
        detailed_review = generate_detailed_review(findings, metadata)
        review_file = pr_dir / "review.md"
        with open(review_file, 'w') as f:
            f.write(detailed_review)
        print(f"✅ Created detailed review: {review_file}")

        # Generate human-friendly review
        human_review = generate_human_review(findings, metadata)
        human_file = pr_dir / "human.md"
        with open(human_file, 'w') as f:
            f.write(human_review)
        print(f"✅ Created human review: {human_file}")

        # Generate inline comments file
        inline_comments = generate_inline_comments_file(findings)
        inline_file = pr_dir / "inline.md"
        with open(inline_file, 'w') as f:
            f.write(inline_comments)
        print(f"✅ Created inline comments: {inline_file}")

        # Generate Claude slash commands
        generate_claude_commands(pr_review_dir, metadata)

        # Create summary file
        summary = f"""PR Review Files Generated
========================

Directory: {pr_review_dir}

Files created:
- pr/review.md      - Detailed analysis for your review
- pr/human.md       - Clean version for posting (no emojis, no line numbers)
- pr/inline.md      - Proposed inline comments with code snippets

Slash commands available:
- /send            - Post human.md and approve PR
- /send-decline    - Post human.md and request changes
- /show            - Open directory in VS Code

Next steps:
1. Review the files (use /show to open in VS Code)
2. Edit as needed
3. Use /send or /send-decline when ready to post

IMPORTANT: Nothing will be posted until you run /send or /send-decline
"""

        summary_file = pr_review_dir / "REVIEW_READY.txt"
        with open(summary_file, 'w') as f:
            f.write(summary)

        print(f"\n{summary}")

    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == '__main__':
    main()
