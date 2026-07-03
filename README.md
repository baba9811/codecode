<div align="center">

# codecode

Keyboard-first coding-test practice in your terminal.

![Python](https://img.shields.io/badge/Python-3.13%2B-3776AB?style=flat-square&logo=python&logoColor=white)
![Textual](https://img.shields.io/badge/Textual-8.2%2B-2563EB?style=flat-square)
![uv](https://img.shields.io/badge/uv-ready-111827?style=flat-square)
![Codex](https://img.shields.io/badge/Codex-assisted-0F766E?style=flat-square)

</div>

```text
+------------------------------------------------+-------------------------------+
| 001. Hello World                               | Problems                      |
|                                                | > *  1 001-hello-world easy  |
| Difficulty: easy  Topics: io                   |     2 next-problem     easy  |
|                                                |                               |
| Print exactly Hello, World!                    | up/down or j/k select        |
|                                                | enter open | esc close       |
+------------------------------------------------+-------------------------------+
 CODECODE | 001-hello-world | idle | lang:python | ui:ko | theme:dark | /help
 /run, /edit, /next, /list, /codex hint
```

## Why

`codecode` is a local TUI for practicing stdin/stdout coding problems without leaving the terminal. It keeps the problem statement, current status, command input, and Codex coach output in one place.

- Solve in `vim` split view with the problem on the left and your answer on the right.
- Run local judge cases with `/run`.
- Ask Codex for hints with `/codex <question>`.
- Generate the next problem with Codex when the local bank is exhausted.
- Switch practice language: Python, TypeScript, Java, Rust.
- Switch UI language and theme: Korean/English, dark/light.

## Quick Start

```bash
git clone https://github.com/baba9811/codecode.git
cd codecode
uv run codecode
```

No generated problem bank is committed. A fresh checkout starts with a built-in `001-hello-world` problem.

## Commands

| Command | Action |
| --- | --- |
| `/run` | Judge the current submission |
| `/edit` | Open problem and solution in `vim` split view |
| `/next` | Open the next problem, or ask Codex to create one |
| `/prev` | Go back through problem history |
| `/list` | Select a problem with `up/down` or `j/k`, open with `Enter` |
| `/open 2` | Open by number, id, or slug |
| `/giveup` | Show the reference answer |
| `/codex hint` | Ask Codex about the current problem and submission |
| `/lang python` | Set language: `python`, `ts`, `java`, `rust` |
| `/ui ko` | Set UI language: `ko`, `en` |
| `/theme` | Toggle dark/light theme |
| `/source codex` | Prefer Codex for next-problem generation |
| `/exit` | Quit |

Keyboard shortcuts: `e` edit, `r` run, `n` next, `p` previous, `g` give up, `/` command input, `Esc` leave command input.

## Local Data

These paths are intentionally ignored by git:

| Path | Purpose |
| --- | --- |
| `.codecode/problem_bank.json` | Local/custom/generated problem bank |
| `.codex/problem-state.json` | Current problem, history, settings |
| `problems/` | Generated problem markdown/index files |
| `submissions/` | Your answer files |

That keeps the public repo clean while letting each user build their own problem set.

## Codex Flow

`/next` uses the local bank first. If there is no unseen problem left, or `/source codex` is enabled, the app runs the configured Codex next-problem command.

Default behavior:

```text
codex app-server daemon start
codex exec --sandbox workspace-write "create exactly one new non-duplicate problem"
```

`/codex <question>` is separate. It sends the current problem and current submission to Codex in read-only mode and prints only the final response in the output pane.

Security note: `/next-command <cmd>` is a trusted local hook. Only set it to commands you would run directly in your shell.

## Development

```bash
uv run pytest tests -q
uv run codecode --smoke
```

The app is intentionally small: Textual for the TUI, stdlib for judging/process work, and `uv` for project setup.
