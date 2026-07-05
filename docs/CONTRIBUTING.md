# Contributing

This repo is a Rust coding-practice workspace with a Ratatui terminal UI.

## Prerequisites

- Rust stable with Cargo, rustfmt, and clippy.
- Node.js 18+ for the npm wrapper and package checks.
- Optional local runtimes for judging: Python, Node.js, JDK, and Rust.

## Development

```bash
cargo run --
cargo run -- --smoke
cargo test
npm run smoke
```

Full local check:

```bash
make test
```

The source is split by boring responsibility:

| Path | Role |
| --- | --- |
| `src/core.rs` | Problem bank, state, rendering, judging |
| `src/tui.rs` | Ratatui app, editor, command parser |
| `src/ai.rs` | Codex/Claude command integration and notes |
| `src/text.rs` | UTF-8 cursor math and Hangul composition |
| `src/process.rs` | Process execution helpers |
| `tests/` | Integration tests split by module |

## Problem Authoring

AI generation reads [problem-authoring-notes.md](problem-authoring-notes.md) every time it creates a problem.

Local generated data stays ignored by git:

| Path | Purpose |
| --- | --- |
| `.practicode/problem_bank.json` | Local/custom/generated problem bank |
| `.practicode/problem_notes.md` | Personal problem-generation notes |
| `.practicode/problem-state.json` | Current problem, history, settings |
| `problems/` | Generated problem markdown/index files |
| `submissions/` | Local answer files |

## Release

`main` runs CI only. Releases are tag-based and publish to crates.io and npm through GitHub Actions.

```bash
make release VERSION=0.1.1
```

The release script checks versions, runs tests, creates the version commit and tag, and pushes `main` plus the tag. Do not print or commit tokens; GitHub Actions uses repository secrets.

## Documentation

Keep the root [README](../README.md) focused on users. Put contributor workflow, implementation notes, release notes, and design references here or in nearby `docs/` files.

Use relative links for repo-local docs and assets. The terminal screenshot is stored at [assets/practicode-terminal.svg](../assets/practicode-terminal.svg).

## UX And Documentation References

- WAI-ARIA combobox keyboard interaction: https://www.w3.org/WAI/ARIA/apg/patterns/combobox/
- Command Line Interface Guidelines: https://clig.dev/
- GitHub README guidance: https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-readmes
- GitHub relative links and images: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#relative-links
- Ratatui terminal UI library: https://ratatui.rs/
- Crossterm terminal backend/events: https://github.com/crossterm-rs/crossterm
- Kattis problem package format: https://www.kattis.com/problem-package-format/
- ICPC judging guidelines: https://icpc.global/regionals/regional-contest-cookbook-judging-guidelines
