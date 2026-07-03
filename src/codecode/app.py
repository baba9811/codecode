from __future__ import annotations

import argparse
from pathlib import Path
import subprocess

from textual.app import App, ComposeResult
from textual.containers import Horizontal, Vertical
from textual.widgets import Button, Footer, Input, Markdown, Static

from codecode.core import (
    LANGUAGES,
    UI_LANGUAGES,
    edit_command,
    ensure_edit_files,
    ensure_submission,
    give_up,
    judge,
    load_bank,
    load_state,
    next_problem,
    problem_by_id,
    record_pass,
    render_problem,
    run_codex_next,
    save_state,
)


VIM_HELP = """Vim quick help
i insert mode
esc normal mode
:w save
:q quit
:wq save and quit
h/j/k/l move left/down/up/right
/text search
dd delete line
u undo
"""


class CodeCodeApp(App[None]):
    CSS = """
    Screen {
        layout: vertical;
        background: #111318;
        color: #eef1f6;
    }
    #status {
        height: 3;
        padding: 1 2;
        background: #161a22;
        color: #70e1c8;
        text-style: bold;
    }
    #body {
        height: 1fr;
        padding: 1 2 0 2;
    }
    #problem {
        width: 58%;
        padding: 1 2;
        border: round #3a4154;
        background: #191d25;
        color: #f3f5f7;
        overflow-y: auto;
    }
    #right {
        width: 42%;
        margin-left: 1;
    }
    #buttons {
        height: 6;
    }
    .button-row {
        height: 3;
    }
    Button {
        min-width: 7;
        margin: 0 1 0 0;
        text-style: bold;
    }
    #output {
        height: 1fr;
        padding: 1 2;
        border: round #3a4154;
        background: #151922;
        color: #d9dee8;
        overflow-y: auto;
    }
    #command {
        height: 3;
        margin: 0 2 1 2;
        border: round #2f3544;
        background: #0d1016;
    }
    Footer {
        background: #111318;
    }
    """
    BINDINGS = [
        ("e", "edit", "Edit"),
        ("r", "run", "Run"),
        ("n", "next", "Next"),
        ("g", "give_up", "Give up"),
        ("l", "cycle_language", "Language"),
        ("u", "toggle_ui_language", "UI"),
        ("slash", "focus_command", "Command"),
        ("q", "quit", "Quit"),
    ]

    def __init__(self, root: Path | None = None) -> None:
        super().__init__()
        self.root = root or Path.cwd()
        self.bank = load_bank()
        self.state = load_state(self.root, self.bank)
        self.problem = problem_by_id(self.bank, self.state.current_problem)

    def compose(self) -> ComposeResult:
        yield Static(id="status")
        with Horizontal(id="body"):
            yield Markdown(id="problem")
            with Vertical(id="right"):
                with Vertical(id="buttons"):
                    with Horizontal(classes="button-row"):
                        yield Button("Edit", id="edit")
                        yield Button("Run", id="run", variant="success")
                        yield Button("Next", id="next")
                    with Horizontal(classes="button-row"):
                        yield Button("Give", id="giveup", variant="error")
                        yield Button("Lang", id="lang")
                        yield Button("UI", id="ui")
                        yield Button("Src", id="source")
                yield Static(id="output")
        yield Input(placeholder="/vim, /lang rust, /ui en, /source codex, /codex <command>", id="command")
        yield Footer()

    def on_mount(self) -> None:
        self.refresh_view("Ready\n\nPress e to edit, r to run, /vim for Vim help.")

    def refresh_view(self, output: str | None = None) -> None:
        self.query_one("#status", Static).update(
            f"CODECODE  {self.problem.id}  {self.problem.difficulty.upper()}  "
            f"{self.state.settings.language}  {self.state.settings.ui_language}  "
            f"next:{self.state.settings.next_source}"
        )
        self.query_one("#problem", Markdown).update(render_problem(self.problem, self.state.settings.ui_language))
        if output is not None:
            self.query_one("#output", Static).update(output)

    def on_button_pressed(self, event: Button.Pressed) -> None:
        actions = {
            "edit": self.action_edit,
            "run": self.action_run,
            "next": self.action_next,
            "giveup": self.action_give_up,
            "lang": self.action_cycle_language,
            "ui": self.action_toggle_ui_language,
            "source": self.action_toggle_next_source,
        }
        action = actions.get(event.button.id or "")
        if action:
            action()

    def action_focus_command(self) -> None:
        self.query_one("#command", Input).focus()

    def action_edit(self) -> None:
        statement, solution = ensure_edit_files(self.root, self.problem, self.state.settings)
        with self.suspend():
            subprocess.run(edit_command(self.state.settings.editor, statement, solution))
        self.refresh_view(f"Edited {solution}")

    def action_run(self) -> None:
        result = judge(self.root, self.problem, self.state.settings)
        if result.passed:
            record_pass(self.root, self.problem, self.state)
        self.refresh_view(result.output)

    def action_next(self) -> None:
        output = ""
        old_problem = self.state.current_problem
        if self.state.settings.next_source == "codex":
            output = run_codex_next(self.root, self.state)
            self.bank = load_bank()
            self.state = load_state(self.root, self.bank)
        self.problem = problem_by_id(self.bank, self.state.current_problem)
        if self.state.settings.next_source != "codex" or self.state.current_problem == old_problem:
            self.problem = next_problem(self.root, self.bank, self.state)
        self.refresh_view(output or f"Loaded {self.problem.id}")

    def action_give_up(self) -> None:
        answer = give_up(self.root, self.problem, self.state)
        self.refresh_view(f"Answer for {self.state.settings.language}:\n\n{answer}")

    def action_cycle_language(self) -> None:
        current = LANGUAGES.index(self.state.settings.language)
        self.state.settings.language = LANGUAGES[(current + 1) % len(LANGUAGES)]
        save_state(self.root, self.state)
        ensure_submission(self.root, self.problem, self.state.settings)
        self.refresh_view(f"Language: {self.state.settings.language}")

    def action_toggle_ui_language(self) -> None:
        current = UI_LANGUAGES.index(self.state.settings.ui_language)
        self.state.settings.ui_language = UI_LANGUAGES[(current + 1) % len(UI_LANGUAGES)]
        save_state(self.root, self.state)
        self.refresh_view(f"UI language: {self.state.settings.ui_language}")

    def action_toggle_next_source(self) -> None:
        self.state.settings.next_source = "codex" if self.state.settings.next_source == "bank" else "bank"
        save_state(self.root, self.state)
        self.refresh_view(f"Next source: {self.state.settings.next_source}")

    def on_input_submitted(self, event: Input.Submitted) -> None:
        value = event.value.strip()
        event.input.value = ""
        if value.startswith("/"):
            value = value[1:].strip()
        self.handle_command(value)

    def handle_command(self, value: str) -> None:
        if not value or value.startswith("vim"):
            self.refresh_view(VIM_HELP)
            return
        parts = value.split(maxsplit=1)
        command, arg = parts[0], parts[1] if len(parts) > 1 else ""
        if command == "lang" and arg in LANGUAGES:
            self.state.settings.language = arg
            save_state(self.root, self.state)
            ensure_submission(self.root, self.problem, self.state.settings)
            self.refresh_view(f"Language: {arg}")
        elif command == "ui" and arg in UI_LANGUAGES:
            self.state.settings.ui_language = arg
            save_state(self.root, self.state)
            self.refresh_view(f"UI language: {arg}")
        elif command == "source" and arg in ("bank", "codex"):
            self.state.settings.next_source = arg
            save_state(self.root, self.state)
            self.refresh_view(f"Next source: {arg}")
        elif command == "codex" and arg:
            self.state.settings.codex_next_command = arg
            self.state.settings.next_source = "codex"
            save_state(self.root, self.state)
            self.refresh_view("Codex next command saved.")
        else:
            self.refresh_view(f"Unknown command: {value}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--smoke", action="store_true")
    args = parser.parse_args()
    if args.smoke:
        bank = load_bank()
        state = load_state(Path.cwd(), bank)
        print(problem_by_id(bank, state.current_problem).title[state.settings.ui_language])
        return
    CodeCodeApp().run()


if __name__ == "__main__":
    main()
