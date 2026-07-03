from pathlib import Path

import pytest
from textual.widgets import Static

from codecode.app import CodeCodeApp


@pytest.mark.asyncio
async def test_app_renders_current_problem(tmp_path: Path):
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.pause()
        problem = app.query_one("#problem", Static)

    assert "누적 합" in str(problem.content)


@pytest.mark.asyncio
async def test_next_button_loads_next_problem(tmp_path: Path):
    app = CodeCodeApp(root=tmp_path)

    async with app.run_test(size=(100, 35)) as pilot:
        await pilot.click("#next")
        await pilot.pause()
        problem = app.query_one("#problem", Static)

    assert "모음 세기" in str(problem.content)
