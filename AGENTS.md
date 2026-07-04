# Codex Problem Session

This repo is a Rust coding-practice workspace with a Ratatui terminal UI.

## Commands

- Run app: `cargo run --`.
- Run smoke check: `cargo run -- --smoke`.
- Run tests: `cargo test`.
- Prefer stdlib. Add crates only when they remove real complexity.

## When User Says `/next` or `/next <request>`

1. Read `docs/problem-authoring-notes.md` if present, `.codecode/problem_notes.md` if present, `problems/INDEX.md` if present, `.codecode/problem_bank.json` if present, and `.codex/problem-state.json`.
2. Pick one new problem not already listed.
3. Keep difficulty gradual:
   - Start with `easy`.
   - Move up only after the user's latest submitted solutions pass and look clean.
   - If the user struggles, keep the same difficulty.
4. If `/next <request>` includes a topic, style, or constraint, honor it unless it conflicts with gradual difficulty or duplicates an existing problem.
5. Treat built-in `001-hello-world` as already used; do not duplicate it.
6. Create exactly one directory: `problems/NNN-short-slug/`.
7. Add `README.md` with Korean problem statement, examples, and constraints.
8. Update `.codecode/problem_bank.json` with stdin/stdout cases and answers for `python`, `ts`, `java`, and `rust`, so the TUI can serve and judge it.
   - If `.codecode/problem_bank.json` does not exist yet, create it and include the built-in `001-hello-world` starter plus the new problem.
9. Update `problems/INDEX.md` and `.codex/problem-state.json`.
10. Do not include the answer unless the user asks.

## TUI Next Source

- Local bank mode uses `.codecode/problem_bank.json` when present; otherwise it uses the built-in `001-hello-world` starter.
- Codex mode runs `settings.codex_next_command` from `.codex/problem-state.json`; when empty, the app uses a default `codex app-server daemon start; codex exec ...` command.
- If Codex creates a problem, it must update `.codecode/problem_bank.json` and `.codex/problem-state.json`.
- `.codecode/`, `.codex/problem-state.json`, `problems/`, and `submissions/` are local user data and are intentionally ignored by git.

## When Grading

1. Run the TUI judge or the local problem test command if one exists.
2. If tests fail, report the first useful failure and the likely cause.
3. If tests pass, review the submitted code briefly for correctness, edge cases, and complexity.
4. Increase future difficulty only when both tests and review are good.
