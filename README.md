# codecode

Local coding-test practice TUI.

## Run

```bash
uv run codecode
```

Keys:

- `e`: open the current submission in `vim`
- `r`: run hidden-style stdin/stdout cases
- `n`: load the next problem
- `g`: give up and show the answer
- `l`: cycle language: Python, TS, Java, Rust
- `u`: toggle Korean/English UI text
- `/`: command input

Commands:

```text
/vim
/lang python
/lang ts
/lang java
/lang rust
/ui ko
/ui en
/source bank
/source codex
/codex <custom command>
```

Submissions are written under `submissions/<problem-id>/`.

## Codex Next

`/source codex` makes the Next button call a Codex command first. The default command starts the local app-server daemon and then runs `codex exec` to create one new problem from repo instructions.

Override it with:

```text
/codex codex app-server daemon start; codex exec --cd . --sandbox workspace-write --ask-for-approval never "create the next problem"
```

If the Codex command fails or leaves the current problem unchanged, the app falls back to the local problem bank.

## Repo Tests

```bash
uv run pytest tests -q
```

Problem-specific legacy tests still run when explicitly targeted:

```bash
uv run pytest problems/001-running-sum -q
```
