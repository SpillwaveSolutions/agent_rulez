#!/usr/bin/env python3
"""
Add inline code review comments to a GitHub PR.

Usage:
    python add_inline_comment.py <owner> <repo> <pr_number> <commit_id> <file_path> <line> <comment> [--side RIGHT|LEFT]

Example:
    python add_inline_comment.py owner repo 123 abc123def "src/main.py" 42 "Consider refactoring this logic"
    python add_inline_comment.py owner repo 123 abc123def "src/main.py" 42 "Check edge cases" --side LEFT
"""

import argparse
import json
import subprocess
import sys
from typing import Optional


def add_inline_comment(
    owner: str,
    repo: str,
    pr_number: str,
    commit_id: str,
    path: str,
    line: int,
    body: str,
    side: str = "RIGHT",
    start_line: Optional[int] = None,
    start_side: Optional[str] = None
) -> dict:
    """
    Add an inline comment to a PR using gh CLI.

    Args:
        owner: Repository owner
        repo: Repository name
        pr_number: Pull request number
        commit_id: SHA of the commit to comment on
        path: File path relative to repo root
        line: Line number in the diff
        body: Comment text
        side: "RIGHT" (new version) or "LEFT" (old version)
        start_line: For multi-line comments, the starting line
        start_side: For multi-line comments, the starting side

    Returns:
        API response as dict

    Raises:
        RuntimeError: If gh command fails
    """
    # Build the API request body
    request_body = {
        "body": body,
        "commit_id": commit_id,
        "path": path,
        "side": side,
        "line": line
    }

    # Add multi-line comment fields if provided
    if start_line is not None:
        request_body["start_line"] = start_line
        if start_side is not None:
            request_body["start_side"] = start_side

    # Convert to JSON string for gh CLI
    request_json = json.dumps(request_body)

    # Build gh api command
    cmd = [
        'gh', 'api',
        '-X', 'POST',
        '-H', 'Accept: application/vnd.github+json',
        f'/repos/{owner}/{repo}/pulls/{pr_number}/comments',
        '--input', '-'
    ]

    try:
        result = subprocess.run(
            cmd,
            input=request_json,
            capture_output=True,
            text=True,
            check=True
        )
        return json.loads(result.stdout)
    except subprocess.CalledProcessError as e:
        raise RuntimeError(f"Failed to add comment: {e.stderr}")
    except FileNotFoundError:
        raise RuntimeError("gh CLI not found. Please install: https://cli.github.com/")


def get_latest_commit(owner: str, repo: str, pr_number: str) -> str:
    """Get the latest commit SHA for a PR."""
    try:
        result = subprocess.run([
            'gh', 'api',
            f'/repos/{owner}/{repo}/pulls/{pr_number}/commits',
            '--jq', '.[-1].sha'
        ], capture_output=True, text=True, check=True)
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        raise RuntimeError(f"Failed to get commits: {e.stderr}")


def main():
    parser = argparse.ArgumentParser(
        description='Add inline code review comment to GitHub PR',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    parser.add_argument('owner', help='Repository owner')
    parser.add_argument('repo', help='Repository name')
    parser.add_argument('pr_number', help='Pull request number')
    parser.add_argument('commit_id', help='Commit SHA (use "latest" to auto-fetch)')
    parser.add_argument('path', help='File path relative to repo root')
    parser.add_argument('line', type=int, help='Line number in the diff')
    parser.add_argument('body', help='Comment text')
    parser.add_argument('--side', choices=['RIGHT', 'LEFT'], default='RIGHT',
                       help='Side of the diff (RIGHT=new, LEFT=old)')
    parser.add_argument('--start-line', type=int,
                       help='Starting line for multi-line comment')
    parser.add_argument('--start-side', choices=['RIGHT', 'LEFT'],
                       help='Starting side for multi-line comment')

    args = parser.parse_args()

    try:
        # Get latest commit if requested
        commit_id = args.commit_id
        if commit_id.lower() == 'latest':
            print(f"Fetching latest commit for PR #{args.pr_number}...")
            commit_id = get_latest_commit(args.owner, args.repo, args.pr_number)
            print(f"Latest commit: {commit_id}")

        # Add the inline comment
        print(f"Adding comment to {args.path}:{args.line}...")
        response = add_inline_comment(
            owner=args.owner,
            repo=args.repo,
            pr_number=args.pr_number,
            commit_id=commit_id,
            path=args.path,
            line=args.line,
            body=args.body,
            side=args.side,
            start_line=args.start_line,
            start_side=args.start_side
        )

        print(f"\n✅ Comment added successfully!")
        print(f"Comment ID: {response.get('id')}")
        print(f"URL: {response.get('html_url')}")

    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == '__main__':
    main()
