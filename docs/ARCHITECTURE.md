# Architecture

Practicode is local-first: user data stays under `.practicode/`, `problems/`, and `submissions/`.

## Source Layout

- `src/core.rs` owns problem data, state loading/saving, judging, and file generation.
- `src/core/profile.rs` owns practice-profile defaults and normalization.
- `src/tui.rs` owns Ratatui rendering and interaction flow.
- `src/tui/commands.rs` owns the command palette catalog.
- `src/ai.rs` owns provider commands, daemon/model checks, and AI prompts.
- `src/update.rs` owns update checks.
- `src/text.rs` owns terminal text editing and markdown/plain rendering helpers.

## Extension Rules

- Add domain logic under the owning module first; keep `tui.rs` as orchestration and rendering.
- Add user-visible commands in `src/tui/commands.rs`, then route behavior in `PracticodeApp::handle_command`.
- Add persisted profile settings to `Settings`, normalize them in `normalize_settings`, and cover old-state compatibility with tests.
- Keep provider-specific behavior in `src/ai.rs`; TUI should ask for status or start tasks, not know provider internals.
- Keep local user data backwards-compatible. Missing fields should default cleanly.

## Release

See [MAINTAINING.md](MAINTAINING.md) for tag-based releases.
