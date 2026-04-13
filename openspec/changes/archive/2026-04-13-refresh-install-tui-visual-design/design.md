## Context

Hermes already provides a `ratatui`-based interactive install flow for browsing installable skills, agents, and commands. The current capability spec covers interactive selection behavior and keyboard controls, but it does not yet define a stronger visual structure or branded presentation. This change is intentionally limited to the install TUI and should preserve the existing install flow, focus model, and non-interactive fallback behavior.

## Goals / Non-Goals

**Goals:**
- Rework the install TUI so artifact panes are stacked vertically rather than arranged side by side.
- Add a recognizable Hermes-branded ASCII art banner to the screen.
- Introduce a darker, richer visual theme with clearer use of color and focus styling.
- Keep the current keyboard interaction model intact so the redesign improves appearance without changing the workflow contract.

**Non-Goals:**
- Changing install semantics, selection semantics, or confirmation behavior.
- Adding new keyboard shortcuts, search/filtering, pagination, or mouse support.
- Introducing new TUI dependencies or building a separate standalone Hermes application shell.

## Decisions

### 1. Keep the redesign inside the existing `ratatui` install flow

The install TUI will continue using the current `ratatui` implementation and event loop, with changes limited to layout, visual rendering, and pane presentation.

- **Why:** The request is aesthetic and layout-driven, so the smallest correct change is to update the existing rendering path instead of changing libraries or introducing new UI abstractions.
- **Alternative considered:** Introduce a separate theming or animation framework. Rejected because it would increase implementation scope without changing user-visible functionality enough to justify the added complexity.

### 2. Use a vertically stacked artifact layout beneath a fixed banner area

The screen will reserve a top region for Hermes branding and a lower region for stacked artifact panes, with each configured artifact kind rendered in its own vertical section.

- **Why:** Vertical stacking fits longer artifact descriptions better, scales more cleanly from two to three artifact kinds, and avoids narrow side-by-side panes on smaller terminals.
- **Alternative considered:** Keep multiple columns and only restyle them. Rejected because the user specifically requested vertically stacked panes and the current multi-column layout makes each pane visually cramped.

### 3. Represent focus through layered visual cues instead of changing controls

The active pane will stand out through stronger border/title styling, accent colors, and more visible highlight treatment, while navigation stays on the current Tab-plus-arrow/j/k model.

- **Why:** This makes focus easier to identify at a glance without retraining users or changing the interaction contract already documented in the specs.
- **Alternative considered:** Add new focus controls or automatic pane cycling effects. Rejected because the request is for clearer presentation, not different navigation behavior.

### 4. Treat Hermes ASCII art and color palette as decorative but non-blocking enhancements

The ASCII art banner and darker theme will improve product identity, but the screen must remain legible in typical terminal color environments and degrade gracefully if terminal dimensions are tight.

- **Why:** Branding should enhance the interface without making the install flow harder to use.
- **Alternative considered:** Replace the banner with plain text headings only. Rejected because it would not satisfy the request for Hermes ASCII art or the desire for a less bland interface.

## Risks / Trade-offs

- [A larger banner reduces space for lists on short terminals] → Keep the banner compact and preserve list readability as the higher priority.
- [Heavier use of color can reduce readability in some terminals] → Use contrast-first styling and avoid relying on color alone to indicate selection or focus.
- [Vertical stacking can require more scrolling when many artifacts are available] → Preserve existing keyboard navigation and make pane focus state much clearer so deeper lists remain manageable.
- [Branding can feel inconsistent if docs still describe the older layout] → Update TUI-facing documentation as part of the same change.

## Migration Plan

No migration steps are required. The redesign only affects the interactive install presentation and can ship as an in-place update to the existing TUI.

## Open Questions

None.
