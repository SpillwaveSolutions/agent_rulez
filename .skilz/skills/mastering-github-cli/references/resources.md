# Resource Creation Reference

Comprehensive reference for creating and managing PRs, issues, repositories, labels, and milestones.

## Contents

- [Pull Requests](#pull-requests)
- [Issues](#issues)
- [Repositories](#repositories)
- [Labels](#labels)
- [Milestones](#milestones)
- [Projects](#projects)
- [Releases](#releases)
- [Common Recipes](#common-recipes)

---

## Pull Requests

### Creating PRs

```bash
gh pr create [flags]
```

| Flag | Description | Example |
|------|-------------|---------|
| `--title` | PR title | `--title "Add feature"` |
| `--body` | PR description | `--body "Description"` |
| `--body-file` | Read body from file | `--body-file PR.md` |
| `--base` | Target branch | `--base main` |
| `--head` | Source branch | `--head feature` |
| `--reviewer` | Request reviewers | `--reviewer @user,team` |
| `--assignee` | Assign users | `--assignee @me` |
| `--label` | Add labels | `--label "bug,urgent"` |
| `--milestone` | Set milestone | `--milestone "v1.0"` |
| `--project` | Add to project | `--project "Roadmap"` |
| `--draft` | Create as draft | `--draft` |
| `--fill` | Use commit info | `--fill` |
| `--fill-first` | Use first commit | `--fill-first` |
| `--web` | Open in browser | `--web` |
| `--no-maintainer-edit` | Disable maintainer edits | `--no-maintainer-edit` |

### Examples

```bash
# Basic PR
gh pr create --title "Fix bug" --body "Fixes issue #123"

# From commit messages
gh pr create --fill

# Full options
gh pr create \
  --title "Add authentication" \
  --body-file .github/PULL_REQUEST_TEMPLATE.md \
  --base develop \
  --head feature/auth \
  --reviewer security-team,@octocat \
  --assignee @me \
  --label "feature,security" \
  --milestone "v2.0" \
  --draft

# Cross-repo PR (fork to upstream)
gh pr create --repo upstream/repo --head myuser:feature-branch
```

### Managing PRs

```bash
# List PRs
gh pr list
gh pr list --state open --assignee @me
gh pr list --label "needs-review" --json number,title

# View PR
gh pr view 123
gh pr view 123 --json title,body,reviews

# Edit PR
gh pr edit 123 --title "New title"
gh pr edit 123 --add-label "approved" --remove-label "needs-review"
gh pr edit 123 --add-reviewer @user

# Review PR
gh pr review 123 --approve
gh pr review 123 --request-changes --body "Please fix X"
gh pr review 123 --comment --body "Looks good overall"

# Merge PR
gh pr merge 123
gh pr merge 123 --squash
gh pr merge 123 --rebase
gh pr merge 123 --auto --squash  # Enable auto-merge
gh pr merge 123 --delete-branch

# Close without merging
gh pr close 123
gh pr close 123 --comment "Closing: superseded by #456"

# Reopen
gh pr reopen 123
```

### PR checkout and diff

```bash
# Checkout PR locally
gh pr checkout 123

# View diff
gh pr diff 123
gh pr diff 123 --patch > changes.patch
```

---

## Issues

### Creating issues

```bash
gh issue create [flags]
```

| Flag | Description | Example |
|------|-------------|---------|
| `--title` | Issue title | `--title "Bug report"` |
| `--body` | Issue description | `--body "Details..."` |
| `--body-file` | Read body from file | `--body-file issue.md` |
| `--label` | Add labels | `--label "bug,urgent"` |
| `--assignee` | Assign users | `--assignee @me,@user` |
| `--milestone` | Set milestone | `--milestone "v1.0"` |
| `--project` | Add to project | `--project "Backlog"` |
| `--web` | Open in browser | `--web` |

### Examples

```bash
# Basic issue
gh issue create --title "Bug: Login fails" --body "Steps to reproduce..."

# With labels and assignee
gh issue create \
  --title "Feature request" \
  --body "Please add..." \
  --label "enhancement,priority:high" \
  --assignee @me

# From file
gh issue create --title "Release checklist" --body-file checklist.md

# From stdin (automation)
echo "Automated issue body" | gh issue create --title "Auto-created" --body-file -

# Using template
gh issue create --template bug_report.md
```

### Managing issues

```bash
# List issues
gh issue list
gh issue list --state open --assignee @me
gh issue list --label "bug" --limit 50
gh issue list --json number,title,labels

# View issue
gh issue view 123
gh issue view 123 --json title,body,comments

# Edit issue
gh issue edit 123 --title "Updated title"
gh issue edit 123 --add-label "in-progress" --remove-label "triage"
gh issue edit 123 --add-assignee @user

# Close issue
gh issue close 123
gh issue close 123 --comment "Fixed in #456"
gh issue close 123 --reason "not planned"

# Reopen issue
gh issue reopen 123

# Transfer issue
gh issue transfer 123 target-owner/target-repo

# Pin/unpin
gh issue pin 123
gh issue unpin 123

# Delete (requires confirmation)
gh issue delete 123 --yes
```

### Issue comments

```bash
# Add comment
gh issue comment 123 --body "Working on this"

# From file
gh issue comment 123 --body-file update.md

# Edit comment (via API)
gh api repos/{owner}/{repo}/issues/comments/{comment_id} \
  -X PATCH -f body="Updated comment"
```

---

## Repositories

### Creating repositories

```bash
gh repo create [name] [flags]
```

| Flag | Description | Example |
|------|-------------|---------|
| `--public` | Public visibility | `--public` |
| `--private` | Private visibility | `--private` |
| `--internal` | Internal (enterprise) | `--internal` |
| `--clone` | Clone after creating | `--clone` |
| `--description` | Repo description | `--description "My project"` |
| `--homepage` | Homepage URL | `--homepage "https://..."` |
| `--license` | License template | `--license mit` |
| `--gitignore` | Gitignore template | `--gitignore Python` |
| `--template` | Template repository | `--template owner/template` |
| `--add-readme` | Initialize with README | `--add-readme` |
| `--disable-issues` | Disable issues | `--disable-issues` |
| `--disable-wiki` | Disable wiki | `--disable-wiki` |

### Examples

```bash
# Public repo with defaults
gh repo create my-project --public --clone

# Full options
gh repo create my-app \
  --public \
  --clone \
  --description "My awesome app" \
  --license mit \
  --gitignore Node \
  --add-readme

# From template
gh repo create my-project --template org/template-repo --private --clone

# In organization
gh repo create myorg/new-project --private

# Interactive mode
gh repo create
```

### Forking repositories

```bash
gh repo fork [repo] [flags]
```

| Flag | Description | Example |
|------|-------------|---------|
| `--clone` | Clone after forking | `--clone` |
| `--remote` | Add remote | `--remote` |
| `--remote-name` | Remote name | `--remote-name upstream` |
| `--org` | Fork to organization | `--org myorg` |
| `--fork-name` | Custom fork name | `--fork-name my-fork` |

### Examples

```bash
# Fork and clone
gh repo fork owner/repo --clone

# Fork to org
gh repo fork owner/repo --clone --org my-org

# Add upstream remote
gh repo fork owner/repo --clone --remote-name upstream
```

### Syncing forks

```bash
# Sync with upstream
gh repo sync

# Sync specific branch
gh repo sync --branch main

# Force sync (discard local changes)
gh repo sync --force

# Sync specific fork
gh repo sync owner/fork --source upstream/repo
```

### Cloning repositories

```bash
# Clone
gh repo clone owner/repo

# Clone to specific directory
gh repo clone owner/repo my-directory

# Clone with git options
gh repo clone owner/repo -- --depth 1

# Clone specific branch
gh repo clone owner/repo -- -b develop
```

### Repository management

```bash
# View repo
gh repo view
gh repo view owner/repo
gh repo view --json name,description,stargazerCount

# Edit repo
gh repo edit --description "Updated description"
gh repo edit --visibility private
gh repo edit --enable-issues --enable-wiki

# Archive/unarchive
gh repo archive owner/repo
gh repo unarchive owner/repo

# Delete (requires confirmation)
gh repo delete owner/repo --yes

# Rename
gh repo rename new-name
```

---

## Labels

### Managing labels

```bash
# List labels
gh label list
gh label list --json name,color,description

# Create label
gh label create "priority:high" --color FF0000 --description "High priority"
gh label create "bug" --color d73a4a

# Edit label
gh label edit "bug" --name "bug-fix" --color 00FF00

# Delete label
gh label delete "old-label" --yes

# Clone labels from another repo
gh label clone source-owner/source-repo
gh label clone source-owner/source-repo --force  # Overwrite existing
```

### Label colors (common)

| Color | Hex |
|-------|-----|
| Red | `d73a4a` |
| Orange | `f9a825` |
| Yellow | `fbca04` |
| Green | `0e8a16` |
| Blue | `1d76db` |
| Purple | `5319e7` |
| Gray | `7f8c8d` |

---

## Milestones

Milestones require API access:

```bash
# List milestones
gh api repos/{owner}/{repo}/milestones

# Create milestone
gh api repos/{owner}/{repo}/milestones \
  -f title="v1.0" \
  -f description="First release" \
  -f due_on="2025-12-31T23:59:59Z" \
  -f state="open"

# Update milestone
gh api repos/{owner}/{repo}/milestones/{number} \
  -X PATCH \
  -f description="Updated description"

# Close milestone
gh api repos/{owner}/{repo}/milestones/{number} \
  -X PATCH \
  -f state="closed"

# Delete milestone
gh api repos/{owner}/{repo}/milestones/{number} -X DELETE
```

---

## Projects

### Managing projects

```bash
# List projects
gh project list
gh project list --owner myorg

# Create project
gh project create --title "Q1 Roadmap"
gh project create --title "Sprint 1" --owner myorg

# View project
gh project view 1
gh project view 1 --json title,items

# Add item to project
gh project item-add 1 --url https://github.com/owner/repo/issues/123

# List items
gh project item-list 1
gh project item-list 1 --json title,status

# Edit item
gh project item-edit --project-id 1 --id ITEM_ID --field-id FIELD_ID --text "Done"

# Delete project
gh project delete 1

# Close project
gh project close 1
```

---

## Releases

### Managing releases

```bash
# List releases
gh release list
gh release list --limit 10

# Create release
gh release create v1.0.0
gh release create v1.0.0 --title "Version 1.0.0" --notes "Release notes"
gh release create v1.0.0 --notes-file CHANGELOG.md
gh release create v1.0.0 --draft
gh release create v1.0.0 --prerelease
gh release create v1.0.0 ./dist/*  # Upload assets

# View release
gh release view v1.0.0
gh release view v1.0.0 --json tagName,assets

# Edit release
gh release edit v1.0.0 --title "Updated title"
gh release edit v1.0.0 --draft=false  # Publish draft

# Download assets
gh release download v1.0.0
gh release download v1.0.0 -p "*.tar.gz" -D ./downloads

# Upload assets
gh release upload v1.0.0 ./dist/app.zip

# Delete release
gh release delete v1.0.0 --yes
gh release delete v1.0.0 --cleanup-tag  # Also delete tag
```

---

## Common Recipes

### Create PR and auto-merge when ready

```bash
gh pr create --fill
gh pr merge --auto --squash
```

### Create issue from template and assign

```bash
gh issue create \
  --title "Weekly sync" \
  --body-file .github/ISSUE_TEMPLATE/meeting.md \
  --label "meeting" \
  --assignee @me,@teammate
```

### Batch create issues

```bash
while IFS=, read -r title label; do
  gh issue create --title "$title" --label "$label" --body "Auto-generated"
done < issues.csv
```

### Fork, clone, create branch, PR

```bash
gh repo fork upstream/repo --clone
cd repo
git checkout -b my-feature
# ... make changes ...
git add . && git commit -m "Add feature"
git push -u origin my-feature
gh pr create --fill --repo upstream/repo
```

### Sync fork and update PR

```bash
gh repo sync --branch main
git checkout my-feature
git rebase main
git push --force-with-lease
```

### Clone all repos matching criteria

```bash
gh search repos --owner myorg --language python --json name --jq '.[].name' | \
while read repo; do
  gh repo clone "myorg/$repo" "repos/$repo"
done
```

### Create release with changelog

```bash
# Generate changelog from commits
git log v0.9.0..HEAD --pretty=format:"- %s" > /tmp/notes.md

# Create release with assets
gh release create v1.0.0 \
  --title "Version 1.0.0" \
  --notes-file /tmp/notes.md \
  ./dist/*.tar.gz ./dist/*.zip
```

### Transfer issues between repos

```bash
# List and transfer all open bugs
gh issue list --label "bug" --json number --jq '.[].number' | \
while read num; do
  gh issue transfer "$num" target-org/target-repo
done
```
