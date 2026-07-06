# Lesson Catalog Assets

Lesson copy is split first by programming language, then by UI language:

```text
assets/lessons/<programming-language>/<ui-language>.json
```

Example:

```text
assets/lessons/python/ko.json
assets/lessons/typescript/en.json
```

Each file follows `schema.json` and contains exactly one programming language plus one UI language. When adding a programming language such as Go, add a new directory with `en.json`, `ko.json`, `ja.json`, `zh.json`, and `es.json`.

Required lesson copy fields:

- `title`
- `concept`
- `worked_example`
- `common_mistakes`
- `self_check`
- `exercise_prompt`

The Rust loader treats these fields as required. Missing study copy should fail tests instead of falling back to generic text at runtime.
