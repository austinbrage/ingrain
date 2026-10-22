#hNLACGGxG4Ym4EZVQogCS7zngmc0oYyWPEQeEEaAMCcQL3TV#svtpp1loJJeG42BjHndvyIN7ixqrpDoRbq4q7gftCEw Exercise · Inline notes · The notes as hints in the editor

Status: Draft

The notes of a chunk (explanations tied to a piece of its code) show in the editor, right below the line of the code they explain. This is the built behavior. The alternative, notes in the guide, is at the end.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · ratatui · First frame                                          session 1/3 · exercise 1/1 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ Hello, ratatui ───────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Build the smallest ratatui    │
│    1  use ratatui::crossterm::event;                          │  │ app: it draws one line of     │
│                    ╰─ ratatui's copy of crossterm: it         │  │ text and quits when you       │
│                       reads keys and mouse events.            │  │ press any key.                │
│ ▶  2  use ratatui::{DefaultTerm█inal, Frame};                 │  ╰───────────────────────────────╯
│                     ╰─ the terminal type ratatui builds.      │                                   
│    3                                                          │  ╭ step ─────────────────────────╮
│                                                               │  │                               │
│                                                               │  │ Two imports. End with an      │
│                                                               │  │ empty line: it is part of     │
├───────────────────────────────────────────────────────────────┤  │ the style.                    │
│ Rust                                             Ln 2, Col 26 │  │                               │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓░░░░░░░░░░░] 1/6 chunks                                                 Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Behavior
- A note shows when the typing reaches the start of its anchor (`on`), and goes away with the chunk
- It shows below the anchor's line as a `╰─ text` block with a pointer, under the anchor's column. A blank row follows it, so the text does not touch the next line
- The text wraps under the pointer, inside the editor width. In a narrow editor the pointer moves left so the text keeps at least 24 columns
- The hint rows are not code: no line number, never typed, the cursor never enters them
- The wrong key `^ expected f` badge works the same way, a row under the wrong key. When both are on one line, the badge goes first
- Style: a gray background (`surface1`), `text` on top, an `overlay1` pointer. The code stays the brightest thing in the card

## The content
- `text` is plain text, no markdown, no backticks: one short sentence of about 60 characters, two lines at most, that does not repeat the anchor's name
- `on` is a piece of the chunk's own text, copied from it, that occurs exactly once. If it is not found the note shows from the start
- The rules are in `docs/content-structure.md` and in the content skill todo (`docs/todos/03-content-skill.md`)

## In the code
- `ui/exercise/editor/hints.rs`: `note_positions` (the row and column of each anchor), `note_hint` (the block), `wrap`, and `wrong_hint`
- `ui/exercise/editor/code.rs`: pushes the hint rows after each code row
- A candidate to move into `app/`: `note_positions` has no ratatui in it

## Trade-offs
- Good: the explanation sits next to what it explains, and the guide is shorter
- Cost: the hints push the code below them down while typing, and a narrow editor wraps them into many rows
- Cost: the editor has no scroll, so many notes can push code off the bottom (todo 4)

## The alternative: the notes in the guide
The first design. The notes are paragraphs in the `step` card, under the chunk guide, in the order of their anchors. They appear and go away the same way. The text was markdown then, and is plain text now.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · ratatui · First frame                                          session 1/3 · exercise 1/1 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ Hello, ratatui ───────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Build the smallest            │
│    1  use ratatui::crossterm::event;                          │  │ ratatui app.                  │
│ ▶  2  use ratatui::{DefaultTerm█inal, Frame};                 │  ╰───────────────────────────────╯
│    3                                                          │                                   
│                                                               │  ╭ step ─────────────────────────╮
│                                                               │  │                               │
│                                                               │  │ Two imports. End with an      │
│                                                               │  │ empty line.                   │
│                                                               │  │                               │
│                                                               │  │ ratatui's copy of crossterm:  │
│                                                               │  │ it reads keys and mouse       │
│                                                               │  │ events.                       │
│                                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ The terminal type ratatui     │
│ Rust                                             Ln 2, Col 26 │  │ builds.                       │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓░░░░░░░░░░░] 1/6 chunks                                                 Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

- Good: the editor never shifts, and the code is all the user sees there
- Cost: the eyes travel from the code to the guide and have to find which part a note is about
- The old code, in `render_guide`: `for note in state.typing.notes(chunk) { text.push_line(""); text.extend(markdown(&note.text)); }`
