# Architecture

Practicode is local-first: user data stays under `.practicode/`, `problems/`, and `submissions/`.

## Source Layout

- `src/core.rs` owns problem data, state loading/saving, judging, and file generation.
- `src/core/profile.rs` owns user-profile defaults and normalization.
- `src/tui.rs` owns the Ratatui app shell, event routing, and workflow orchestration.
- `src/tui/commands.rs` owns the command palette catalog.
- `src/tui/editor.rs` owns the in-terminal code editor state.
- `src/tui/problem_view.rs` owns problem-statement rendering.
- `src/tui/settings_panel.rs` owns `/profile` setup-panel rendering and keyboard toggles.
- `src/ai.rs` owns provider commands, daemon/model checks, and AI prompts for foreground `/next` generation and background `/generate` prefetch.
- `src/update.rs` owns update checks.
- `src/text.rs` owns terminal text editing and markdown/plain rendering helpers.

## Extension Rules

- Add domain logic under the owning module first; keep `tui.rs` as orchestration, not a catch-all.
- Add user-visible commands in `src/tui/commands.rs`, then route behavior in `PracticodeApp::handle_command`.
- Add persisted user profile settings to `Settings`, normalize them in `normalize_settings`, and cover old-state compatibility with tests.
- Keep provider-specific behavior in `src/ai.rs`; TUI should ask for status or start tasks, not know provider internals.
- Keep foreground and background generation flows separate: `/next` may block when no local problem exists, while `/generate` must preserve the current problem and user profile state.
- Keep output panes copy-friendly. Mouse capture should be enabled for the visible code editor, but disabled while output, hints, answers, lists, or settings panels are shown so terminal drag selection keeps working.
- Keep local user data backwards-compatible. Missing fields should default cleanly.

## Release

See [MAINTAINING.md](MAINTAINING.md) for tag-based releases.
