## 1. TUI Foundation

- [x] 1.1 Add the selected terminal UI dependency and supporting backend dependency for the interactive install flow.
- [x] 1.2 Create a dedicated install-selection UI module with state structures for available skills, available agents, current focus, and selected items.

## 2. Interactive Selection Flow

- [x] 2.1 Implement the terminal UI rendering and keyboard event loop for browsing and toggling skills and agents.
- [x] 2.2 Support confirm and cancel flows so the UI returns selected artifact names or exits without changes.
- [x] 2.3 Add clear handling for non-interactive terminal sessions when `hermes install` is invoked without explicit names.

## 3. Install Integration

- [x] 3.1 Integrate the new terminal UI into `hermes install` as the default interactive path when no explicit names are provided.
- [x] 3.2 Preserve the existing explicit argument and kind-specific install flows for scripting and automation.

## 4. Documentation And Verification

- [x] 4.1 Update Hermes docs to describe the TUI-based install workflow and the selected Rust terminal UI library choice.
- [x] 4.2 Add or update tests around install-selection behavior where practical, and run `cargo build` until it passes.
