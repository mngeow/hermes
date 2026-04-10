---
name: code-review
description: Perform thorough code reviews using project-specific guidelines and checklists
---

## What this skill does

Guides you through a structured code review process using detailed guidelines
and checklists stored as companion files in this skill's directory.

## How to use companion files

This skill includes reference material split across nested folders. When
performing a code review, read the relevant files below before giving feedback.

### Guidelines (`guidelines/`)

- `guidelines/security.md` -- Security review standards. Read this when
  reviewing authentication, authorization, input handling, or data storage.
- `guidelines/performance.md` -- Performance review standards. Read this when
  reviewing database queries, loops, caching, or resource-intensive code.

### Checklists (`checklists/`)

- `checklists/pull-request.md` -- Step-by-step checklist for every PR. Always
  read this file and work through each item.

## Workflow

1. Read `checklists/pull-request.md` and use it as your review structure.
2. Read any relevant guideline files based on the code being reviewed.
3. For each file in the diff, apply the checklist items and relevant guidelines.
4. Summarize findings grouped by severity: critical, warning, suggestion.
