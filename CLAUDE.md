# Ingrain

Rust terminal app (ratatui) that teaches a programming language, and keeps it fresh, by making the user type code by hand. Structured courses with explanations, pre-generated content, no AI at runtime. Built to fight AI dependency in engineers.

## Roles
- The author decides the structure, types and names. Claude implements under `src/` only through the step flow below
- Claude explains ratatui and Rust concepts with small snippets (the API call, not the author's function), proposes options when asked, and reviews what the author wrote
- Claude writes markdown and UI sketches
- When the author says "advise", "agree?", "opinion" or "let's talk", answer only; do not edit files

## Implementation flow
- Split the job in steps (one concept per step, a few files each), following the roadmap of the todo in `docs/todos/`
- For each step: show the diff in the chat as raw text, wait for the author to approve or correct it, and only then apply it. Never send several steps as one batch of edits
- A step that changes behavior has its test first, shown as a diff like any other step. After applying, run `cargo check` and `cargo test`
- At the end of each step, mark its item in the todo and remind the author of the next step or the next open item
- The todo is finished when all its items are checked. Then delete the todo file and update the docs that mention it. That is all

## UI
- One file per screen in `docs/ui/`, with a status: Not designed, Draft or Approved
- A screen is coded only from an Approved file; only the author sets Approved
- How screens map to `src/ui/` files, the explorer view modes and the app state: `docs/ui-structure.md`

## Commands
- Ask before running any command. Exceptions: `cargo check`, `cargo test`, `cargo clippy` (never `--fix`), `cargo fmt --check`, and read-only inspection (`ls`, `git status`, `git log`, `grep`)
- Never run `cargo run` or any long-running process; the author runs the app

## Git
- Conventional commits (`feat:`, `fix:`, `chore:`, `docs:`, `test:`, `refactor:`), one short line, no body
- Never commit without the author saying "commit"; never push
- After a change worth committing, suggest a commit message at the end of the reply

## Formatting
- "fmt & commit" means: run `cargo fmt`, then commit only the files of the change
- If fmt touched code outside the change, commit that separately as `chore: format code`

## Code preferences
- File shape: constants, the main struct, one entry function named after the file, a few helpers in call order
- A helper exists only if it is used twice or is a real algorithm; no one-line wrappers
- No comments: names carry the meaning, test names carry the rules
- Types live in the module that owns the concept; no shared `types.rs`
- Import with `use` instead of repeating full paths
