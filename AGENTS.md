# Codex Problem Session

This repo is a Python coding-practice workspace with a Textual TUI.

## Commands

- Judge current problem: run `uv run pytest problems/<id-slug> -q`.
- Judge all problems: run `uv run pytest problems -q`.
- Run app: `uv run codecode`.
- Run app tests: `uv run pytest tests -q`.
- Use `uv add --dev <pkg>` only when a problem truly needs it. Default to stdlib.

## When User Says `/next`

1. Read `problems/INDEX.md` and `.codex/problem-state.json`.
2. Pick one new problem not already listed.
3. Keep difficulty gradual:
   - Start with `easy`.
   - Move up only after the user's latest submitted solutions pass and look clean.
   - If the user struggles, keep the same difficulty.
4. Create exactly one directory: `problems/NNN-short-slug/`.
5. Add:
   - `README.md` with Korean problem statement, function signature, examples, constraints.
   - `solution.py` with only the required function stub.
   - `test_solution.py` with pytest cases.
6. Also update `src/codecode/problem_bank.json` with stdin/stdout cases and answers for `python`, `ts`, `java`, and `rust`, so the TUI can serve and judge it.
7. Update `problems/INDEX.md` and `.codex/problem-state.json`.
8. Do not include the answer unless the user asks.

## TUI Next Source

- Local bank mode uses `src/codecode/problem_bank.json`.
- Codex mode runs `settings.codex_next_command` from `.codex/problem-state.json`; when empty, the app uses a default `codex app-server daemon start; codex exec ...` command.
- If Codex creates a problem, it must update `src/codecode/problem_bank.json` and `.codex/problem-state.json`.

## When Grading

1. Run the problem's pytest command.
2. If tests fail, report the first useful failure and the likely cause.
3. If tests pass, review the submitted code briefly for correctness, edge cases, and complexity.
4. Increase future difficulty only when both tests and review are good.
