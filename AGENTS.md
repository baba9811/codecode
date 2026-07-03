# Codex Problem Session

This repo is a Python coding-practice workspace with a Textual TUI.

## Commands

- Run app: `uv run codecode`.
- Run app tests: `uv run pytest tests -q`.
- Use `uv add --dev <pkg>` only when a problem truly needs it. Default to stdlib.

## When User Says `/next`

1. Read `problems/INDEX.md` if present, `.codecode/problem_bank.json` if present, and `.codex/problem-state.json`.
2. Pick one new problem not already listed.
3. Keep difficulty gradual:
   - Start with `easy`.
   - Move up only after the user's latest submitted solutions pass and look clean.
   - If the user struggles, keep the same difficulty.
4. Treat built-in `001-hello-world` as already used; do not duplicate it.
5. Create exactly one directory: `problems/NNN-short-slug/`.
6. Add `README.md` with Korean problem statement, examples, and constraints.
7. Update `.codecode/problem_bank.json` with stdin/stdout cases and answers for `python`, `ts`, `java`, and `rust`, so the TUI can serve and judge it.
   - If `.codecode/problem_bank.json` does not exist yet, create it and include the built-in `001-hello-world` starter plus the new problem.
8. Update `problems/INDEX.md` and `.codex/problem-state.json`.
9. Do not include the answer unless the user asks.

## TUI Next Source

- Local bank mode uses `.codecode/problem_bank.json` when present; otherwise it uses the built-in `001-hello-world` starter.
- Codex mode runs `settings.codex_next_command` from `.codex/problem-state.json`; when empty, the app uses a default `codex app-server daemon start; codex exec ...` command.
- If Codex creates a problem, it must update `.codecode/problem_bank.json` and `.codex/problem-state.json`.
- `.codecode/`, `.codex/problem-state.json`, `problems/`, and `submissions/` are local user data and are intentionally ignored by git.

## When Grading

1. Run the TUI judge or the local problem pytest command if one exists.
2. If tests fail, report the first useful failure and the likely cause.
3. If tests pass, review the submitted code briefly for correctness, edge cases, and complexity.
4. Increase future difficulty only when both tests and review are good.
