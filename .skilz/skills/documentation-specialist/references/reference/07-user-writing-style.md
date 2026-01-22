# User Documentation: Writing Style

---
TOKEN_BUDGET: 220
TIER: 3
LOAD_TRIGGER: On-demand when writing user-facing documentation
DEPENDENCIES: 07-user-kb-approach.md
---

## 7.2 Writing for End-Users

### Focus on Problems, Not Features

**❌ Bad Title**: "The Task Creation Modal"
**✅ Good Title**: "How to Create and Assign a Task"

**Why**: Users think in terms of goals ("I want to create a task"), not UI elements ("I want to use the modal").

---

### Use Plain Language

**Short Sentences**
- Aim for 15-20 words per sentence
- One idea per sentence

**Active Voice**
- ✅ "Click the Save button"
- ❌ "The Save button should be clicked"

**Present Tense**
- ✅ "The system saves your changes"
- ❌ "The system will save your changes"

**Avoid Jargon and Acronyms**
- ✅ "Priority level"
- ❌ "SLA tier"
- If you must use acronyms, define them on first use

---

### Be Visual

**Annotated Screenshots**
- Add red boxes around buttons
- Use arrows to show click paths
- Highlight form fields

**Animated GIFs**
- Show multi-step workflows
- Keep under 10 seconds
- Tools: LICEcap, Kap, ScreenToGif

**Short Videos**
- Maximum 2 minutes
- Add captions for accessibility
- Host on YouTube or Vimeo

---

### Use Sequential Steps

**Format:**
```markdown
## How to Create a Task

1. From your project dashboard, click the **"+ New Task"** button.
   [Screenshot with red box around button]

2. Enter a title for your task (e.g., "Draft Q4 Blog Post").
   [Screenshot of the task form]

3. (Optional) Add a description, due date, and assignee.

4. Click **"Save"**.

Your task will now appear in the "Pending" column.
```

**Key Elements:**
- Numbered steps (not bullets)
- Bold UI elements ("Click **Save**")
- Screenshots after each step
- Expected outcome at the end

---

### Accessibility

- Use descriptive alt text for images
- Ensure sufficient color contrast
- Provide text transcripts for videos
- Use semantic HTML headings (H2, H3, not bold text)

---

**End of User Documentation Writing Style Guide**
