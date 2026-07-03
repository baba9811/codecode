from dataclasses import replace
from pathlib import Path
import time

import pytest
from textual.widgets import Button, Input, Markdown, Static

from codecode.app import CodeCodeApp
from codecode.core import AppState, Problem, Settings, load_bank, save_bank, save_state


def output_text(app: CodeCodeApp) -> str:
    return app.query_one("#output", Markdown).source or ""


def two_problem_bank(root: Path) -> list[Problem]:
    first = load_bank(root)[0]
    second = replace(
        first,
        id="002-echo",
        slug="echo",
        topics=["io", "string"],
        title={"ko": "그대로 출력", "en": "Echo"},
        statement={"ko": "입력을 그대로 출력하세요.", "en": "Print stdin unchanged."},
        input={"ko": "문자열", "en": "A string"},
        output={"ko": "입력과 같은 문자열", "en": "The same string"},
        examples=[{"input": "code\n", "output": "code\n"}],
        cases=[{"input": "code\n", "output": "code\n"}],
        answers={
            "python": "import sys\nprint(sys.stdin.read(), end='')\n",
            "ts": "const fs = require('fs');\nprocess.stdout.write(fs.readFileSync(0, 'utf8'));\n",
            "java": "class Solution { public static void main(String[] args) throws Exception { System.out.print(new String(System.in.readAllBytes())); } }\n",
            "rust": "use std::io::{self, Read};\nfn main() { let mut s = String::new(); io::stdin().read_to_string(&mut s).unwrap(); print!(\"{}\", s); }\n",
        },
    )
    bank = [first, second]
    save_bank(root, bank)
    return bank


@pytest.mark.asyncio
async def test_app_renders_current_problem(tmp_path: Path):
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        problem = app.query_one("#problem", Markdown)
        status = app.query_one("#status", Static)

    assert problem is not None
    assert app.problem.title["ko"] == "Hello World"
    assert "CODECODE" in str(status.content)
    assert "python" in str(status.content)


@pytest.mark.asyncio
async def test_next_key_loads_next_problem(tmp_path: Path):
    two_problem_bank(tmp_path)
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("n")
        await pilot.pause()
        problem = app.query_one("#problem", Markdown)

    assert problem is not None
    assert app.problem.title["ko"] == "그대로 출력"


@pytest.mark.asyncio
async def test_codex_next_shows_loading_without_blocking(tmp_path: Path, monkeypatch: pytest.MonkeyPatch):
    save_state(
        tmp_path,
        AppState(current_problem="001-hello-world", settings=Settings(next_source="codex")),
    )

    def slow_next(*args, **kwargs):
        time.sleep(0.5)
        return "Codex command finished"

    monkeypatch.setattr("codecode.app.run_codex_next", slow_next)
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("/")
        await pilot.pause()
        await pilot.press("n", "e", "x", "t", "enter")
        await pilot.pause()
        output = app.query_one("#output", Markdown)

        assert not output.loading
        assert "Generating next problem" in output_text(app)
        assert "busy:next" in str(app.query_one("#status", Static).content)


@pytest.mark.asyncio
async def test_bank_next_flows_to_codex_when_bank_is_exhausted(tmp_path: Path, monkeypatch: pytest.MonkeyPatch):
    bank = load_bank(tmp_path)
    save_state(
        tmp_path,
        AppState(
            current_problem=bank[-1].id,
            settings=Settings(next_source="bank"),
            history=[{"id": problem.id, "status": "solved"} for problem in bank],
        ),
    )
    captured = {}

    def slow_next(root, state, force=False):
        captured["force"] = force
        time.sleep(0.5)
        return "Codex command finished"

    monkeypatch.setattr("codecode.app.run_codex_next", slow_next)
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("/")
        await pilot.pause()
        await pilot.press("n", "e", "x", "t", "enter")
        await pilot.pause()
        output = app.query_one("#output", Markdown)

        assert not output.loading
        assert "Generating next problem" in output_text(app)
        assert captured == {"force": True}


@pytest.mark.asyncio
async def test_codex_next_falls_back_to_bank_when_current_problem_does_not_change(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
):
    two_problem_bank(tmp_path)
    save_state(
        tmp_path,
        AppState(
            current_problem="001-hello-world",
            settings=Settings(next_source="codex"),
            history=[{"id": "001-hello-world", "status": "assigned"}],
        ),
    )
    monkeypatch.setattr("codecode.app.run_codex_next", lambda *args, **kwargs: "Codex command finished")
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("/")
        await pilot.pause()
        await pilot.press("n", "e", "x", "t", "enter")
        await pilot.pause(0.1)
        assert "Codex command finished" in output_text(app)

    assert app.problem.title["ko"] == "그대로 출력"


@pytest.mark.asyncio
async def test_previous_key_loads_previous_problem(tmp_path: Path):
    two_problem_bank(tmp_path)
    state = AppState(
        current_problem="002-echo",
        history=[
            {"id": "001-hello-world", "status": "solved"},
            {"id": "002-echo", "status": "assigned"},
        ],
    )
    save_state(tmp_path, state)
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("p")
        await pilot.pause()

    assert app.problem.title["ko"] == "Hello World"


@pytest.mark.asyncio
async def test_escape_exits_command_input(tmp_path: Path):
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("/")
        await pilot.pause()
        command = app.query_one("#command", Input)
        assert app.focused is command

        await pilot.press("h", "e", "l", "p")
        assert command.value == "help"

        await pilot.press("escape")
        await pilot.pause()

        assert command.value == ""
        assert app.focused is not command


@pytest.mark.asyncio
async def test_exit_command_quits_app(tmp_path: Path, monkeypatch: pytest.MonkeyPatch):
    app = CodeCodeApp(root=tmp_path)
    called = {}

    def fake_exit(*args, **kwargs):
        called["exit"] = True

    monkeypatch.setattr(app, "exit", fake_exit)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("/")
        await pilot.pause()
        await pilot.press("e", "x", "i", "t", "enter")
        await pilot.pause()

    assert called == {"exit": True}


@pytest.mark.asyncio
async def test_slash_commands_run_actions(tmp_path: Path):
    two_problem_bank(tmp_path)
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("/")
        await pilot.pause()
        await pilot.press("h", "e", "l", "p", "enter")
        await pilot.pause()
        assert "Commands" in output_text(app)

        await pilot.press("/")
        await pilot.pause()
        await pilot.press("r", "u", "n", "enter")
        await pilot.pause()
        assert "case 1:" in output_text(app)

        await pilot.press("/")
        await pilot.pause()
        await pilot.press("n", "e", "x", "t", "enter")
        await pilot.pause()
        assert app.problem.title["ko"] == "그대로 출력"

        await pilot.press("/")
        await pilot.pause()
        await pilot.press("p", "r", "e", "v", "enter")
        await pilot.pause()

    assert app.problem.title["ko"] == "Hello World"


@pytest.mark.asyncio
async def test_list_and_open_commands_show_and_load_problems(tmp_path: Path):
    two_problem_bank(tmp_path)
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("/")
        await pilot.pause()
        await pilot.press("l", "i", "s", "t", "enter")
        await pilot.pause()
        assert "001-hello-world" in output_text(app)
        assert "002-echo" in output_text(app)

        await pilot.press("/")
        await pilot.pause()
        await pilot.press("o", "p", "e", "n", " ", "2", "enter")
        await pilot.pause()

    assert app.problem.title["ko"] == "그대로 출력"


@pytest.mark.asyncio
async def test_open_command_shows_problem_status_and_submission_state(tmp_path: Path):
    two_problem_bank(tmp_path)
    state = AppState(
        current_problem="002-echo",
        history=[
            {"id": "001-hello-world", "status": "solved"},
            {"id": "002-echo", "status": "assigned"},
        ],
    )
    save_state(tmp_path, state)
    submission = tmp_path / "submissions" / "001-hello-world" / "solution.py"
    submission.parent.mkdir(parents=True)
    submission.write_text("print('done')\n")
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("/")
        await pilot.pause()
        await pilot.press("o", "p", "e", "n", " ", "1", "enter")
        await pilot.pause()
        status = app.query_one("#status", Static)
        assert "Status: solved" in output_text(app)
        assert "Submission: written" in output_text(app)

    assert app.problem.title["ko"] == "Hello World"
    assert "status:solved" in str(status.content)
    assert "code:written" in str(status.content)


@pytest.mark.asyncio
async def test_open_command_reports_missing_submission_without_creating_it(tmp_path: Path):
    two_problem_bank(tmp_path)
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("/")
        await pilot.pause()
        await pilot.press("o", "p", "e", "n", " ", "2", "enter")
        await pilot.pause()
        assert "Status: assigned" in output_text(app)
        assert "Submission: missing" in output_text(app)

    assert not (tmp_path / "submissions" / "002-echo" / "solution.py").exists()


@pytest.mark.asyncio
async def test_codex_command_prints_response_without_changing_next_settings(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
):
    captured = {}

    def fake_codex(root, problem, settings, prompt):
        captured["problem"] = problem.id
        captured["language"] = settings.language
        captured["prompt"] = prompt
        return f"Codex says: {prompt}"

    monkeypatch.setattr("codecode.app.run_codex_prompt", fake_codex)
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("/")
        await pilot.pause()
        await pilot.press("c", "o", "d", "e", "x", " ", "h", "e", "l", "l", "o", "enter")
        await pilot.pause(0.1)
        assert "Codex says: hello" in output_text(app)

    assert captured == {"problem": "001-hello-world", "language": "python", "prompt": "hello"}
    assert app.state.settings.next_source == "bank"
    assert app.state.settings.codex_next_command == ""


@pytest.mark.asyncio
async def test_codex_command_shows_loading_in_scrollable_output(tmp_path: Path, monkeypatch: pytest.MonkeyPatch):
    def slow_codex(*args):
        time.sleep(0.5)
        return "later"

    monkeypatch.setattr("codecode.app.run_codex_prompt", slow_codex)
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        await pilot.press("/")
        await pilot.pause()
        await pilot.press("c", "o", "d", "e", "x", " ", "h", "i", "enter")
        await pilot.pause()
        output = app.query_one("#output", Markdown)
        assert output.can_focus
        assert not output.loading
        assert "Codex is thinking" in output_text(app)
        assert "busy:codex" in str(app.query_one("#status", Static).content)


@pytest.mark.asyncio
async def test_output_renders_markdown_for_codex_answers(tmp_path: Path):
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        app.write_output("Use `stdin`:\n\n```python\nseq = sys.stdin.read().split()\n```")
        await pilot.pause()
        output = app.query_one("#output", Markdown)

    assert output.can_focus
    assert "```python" in (output.source or "")


@pytest.mark.asyncio
async def test_tui_uses_statusline_instead_of_buttons(tmp_path: Path):
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        status = app.query_one("#status", Static)
        buttons = app.query(Button)

    assert "lang:python" in str(status.content)
    assert "ui:ko" in str(status.content)
    assert len(buttons) == 0
