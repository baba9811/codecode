from pathlib import Path

import pytest
from textual.widgets import Markdown, Static

from codecode.app import CodeCodeApp


@pytest.mark.asyncio
async def test_app_renders_current_problem(tmp_path: Path):
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        problem = app.query_one("#problem", Markdown)
        status = app.query_one("#status", Static)

    assert problem is not None
    assert app.problem.title["ko"] == "누적 합"
    assert "CODECODE" in str(status.content)
    assert "python" in str(status.content)


@pytest.mark.asyncio
async def test_next_button_loads_next_problem(tmp_path: Path):
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.click("#next")
        await pilot.pause()
        problem = app.query_one("#problem", Markdown)

    assert problem is not None
    assert app.problem.title["ko"] == "모음 세기"
