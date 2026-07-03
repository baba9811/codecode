# codecode

Local coding-test practice TUI.

## Run

```bash
uv run codecode
```

Keys:

- `e`: open the problem and current submission in a `vim` split
- `r`: run hidden-style stdin/stdout cases
- `n`: load the next problem
- `p`: load the previous problem from history
- `g`: give up and show the answer
- `l`: cycle language: Python, TS, Java, Rust
- `u`: toggle Korean/English UI text
- `/`: command input, `Esc` exits it

Commands:

```text
/help
/vim
/run
/edit
/next
/prev
/list
/open 2
/giveup
/lang python
/lang ts
/lang java
/lang rust
/ui ko
/ui en
/source bank
/source codex
/next-command <custom next-problem command>
/codex <question about the current problem/code>
/exit
```

Submissions are written under `submissions/<problem-id>/`.

## Problems

The public repo does not track a generated problem bank. If no local bank exists, the app starts with one built-in `Hello World` problem.

Your local/generated practice data is ignored by git:

- `.codecode/problem_bank.json`
- `.codex/problem-state.json`
- `problems/`
- `submissions/`

Customize or generate problems by editing `.codecode/problem_bank.json`, or let `/next` create the next problem through Codex when the local bank is exhausted.

## Codex Next

`/next` or `n` opens the next unseen local bank problem first. If the bank is exhausted, or if `/source codex` is enabled, it calls Codex to create one new problem. The default command starts the local app-server daemon and then runs `codex exec` from repo instructions.

Override it with:

```text
/next-command codex app-server daemon start; codex exec --cd . --sandbox workspace-write "create the next problem"
```

If the Codex command fails or leaves the current problem unchanged, the app falls back to any unseen local bank problem.

`/codex <question>` is separate from next-problem generation. It sends the current problem and current submission to Codex in read-only mode and prints the response in the output pane.

## Repo Tests

```bash
uv run pytest tests -q
```
