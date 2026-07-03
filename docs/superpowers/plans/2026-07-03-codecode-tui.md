# Codecode TUI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a local TUI coding-test runner with next-problem, give-up, grading, language settings, and Vim-based editing.

**Architecture:** Keep practice logic in `src/codecode/core.py` and terminal UI in `src/codecode/app.py`. Problems come from `src/codecode/problem_bank.json`; user submissions and progress stay in repo-local files.

**Tech Stack:** Python, uv, Textual, pytest.

---

### Task 1: Core Logic

**Files:**
- Create: `src/codecode/core.py`
- Create: `src/codecode/problem_bank.json`
- Create: `tests/test_core.py`
- Modify: `pyproject.toml`

- [ ] Write tests for loading the current problem, creating a language-specific submission file, judging stdin/stdout cases, giving up, and selecting the next unsolved problem.
- [ ] Run `uv run pytest tests/test_core.py -q` and verify failures because `codecode.core` does not exist.
- [ ] Implement only the core functions needed by the tests.
- [ ] Run `uv run pytest tests/test_core.py -q` and verify pass.

### Task 2: TUI Shell

**Files:**
- Create: `src/codecode/app.py`
- Create: `tests/test_app.py`
- Modify: `pyproject.toml`
- Modify: `README.md`
- Modify: `AGENTS.md`

- [ ] Write a Textual headless smoke test that starts the app and checks the current problem is rendered.
- [ ] Run `uv run pytest tests/test_app.py -q` and verify failure because `codecode.app` does not exist.
- [ ] Implement a Textual app with buttons/actions for edit, run, next, give up, settings, and `/vim` help.
- [ ] Run `uv run pytest tests/test_app.py -q` and verify pass.

### Task 3: Verification

**Files:**
- Modify only files touched by Tasks 1 and 2 if verification exposes defects.

- [ ] Run `uv run pytest tests -q`.
- [ ] Run `uv run python -m codecode.app --smoke`.
- [ ] Run `uv run codecode --smoke`.
- [ ] Check `git status --short`.

