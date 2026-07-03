import json
from pathlib import Path
import sys

from codecode.core import (
    AppState,
    Settings,
    edit_command,
    ensure_edit_files,
    ensure_submission,
    give_up,
    judge,
    load_bank,
    load_state,
    next_problem,
    record_pass,
    run_codex_next,
    save_state,
)


def test_load_state_uses_first_problem_when_state_file_is_missing(tmp_path: Path):
    bank = load_bank()

    state = load_state(tmp_path, bank)

    assert state.current_problem == "001-running-sum"
    assert state.settings.language == "python"
    assert state.settings.ui_language == "ko"


def test_ensure_submission_creates_language_template(tmp_path: Path):
    bank = load_bank()
    state = AppState(current_problem="001-running-sum", settings=Settings(language="rust"))

    path = ensure_submission(tmp_path, bank[0], state.settings)

    assert path == tmp_path / "submissions" / "001-running-sum" / "solution.rs"
    assert "fn main()" in path.read_text()


def test_ensure_edit_files_creates_problem_statement_and_vim_split_command(tmp_path: Path):
    bank = load_bank()
    state = AppState(current_problem="001-running-sum", settings=Settings(language="python"))

    statement, solution = ensure_edit_files(tmp_path, bank[0], state.settings)

    assert statement == tmp_path / "submissions" / "001-running-sum" / "problem.md"
    assert solution == tmp_path / "submissions" / "001-running-sum" / "solution.py"
    assert "누적 합" in statement.read_text()
    assert edit_command("vim", statement, solution) == ["vim", "-O", str(statement), str(solution)]


def test_judge_runs_python_solution_against_cases(tmp_path: Path):
    bank = load_bank()
    state = AppState(current_problem="001-running-sum", settings=Settings(language="python"))
    path = ensure_submission(tmp_path, bank[0], state.settings)
    path.write_text(
        "import sys\n"
        "nums = list(map(int, sys.stdin.read().split()))\n"
        "out = []\n"
        "total = 0\n"
        "for n in nums:\n"
        "    total += n\n"
        "    out.append(str(total))\n"
        "print(' '.join(out))\n"
    )

    result = judge(tmp_path, bank[0], state.settings)

    assert result.passed
    assert result.passed_cases == result.total_cases


def test_give_up_marks_problem_and_returns_answer(tmp_path: Path):
    bank = load_bank()
    state = AppState(current_problem="001-running-sum")

    answer = give_up(tmp_path, bank[0], state)
    saved = json.loads((tmp_path / ".codex" / "problem-state.json").read_text())

    assert "total" in answer
    assert saved["history"][0]["status"] == "gave_up"


def test_next_problem_skips_history_and_saves_new_current(tmp_path: Path):
    bank = load_bank()
    state = AppState(
        current_problem="001-running-sum",
        history=[{"id": "001-running-sum", "status": "solved"}],
    )
    save_state(tmp_path, state)

    problem = next_problem(tmp_path, bank, state)
    saved = load_state(tmp_path, bank)

    assert problem.id == "002-count-vowels"
    assert saved.current_problem == "002-count-vowels"
    assert "002 | count-vowels" in (tmp_path / "problems" / "INDEX.md").read_text()


def test_record_pass_marks_solved_and_raises_suggested_difficulty_after_two_solves(tmp_path: Path):
    bank = load_bank()
    state = AppState(current_problem="001-running-sum", solved=["000-warmup"])

    record_pass(tmp_path, bank[0], state)
    saved = load_state(tmp_path, bank)

    assert "001-running-sum" in saved.solved
    assert saved.history[0]["status"] == "solved"
    assert saved.suggested_next_difficulty == "medium"


def test_run_codex_next_executes_configured_command_in_repo_root(tmp_path: Path):
    command = (
        f"{sys.executable} -c "
        "\"from pathlib import Path; Path('codex-made.txt').write_text('ok')\""
    )
    state = AppState(
        current_problem="001-running-sum",
        settings=Settings(next_source="codex", codex_next_command=command),
    )

    output = run_codex_next(tmp_path, state)

    assert "finished" in output
    assert (tmp_path / "codex-made.txt").read_text() == "ok"
