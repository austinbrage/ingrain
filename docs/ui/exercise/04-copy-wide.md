# Exercise · Copy chunk · Wide

Status: Draft

The wide layout for a copy chunk. The editor shows what is typed, and the lower card shows the code to copy. This is the built copy box, and its look is definitive.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ Repeat with for ──────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Inside main, loop three       │
│    1  fn main() {                                             │  │ times.                        │
│ ▶  2      for i i█                                            │  ╰───────────────────────────────╯
│    3  }                                                       │                                   
│                                                               │  ╭ step ─────────────────────────╮
│                                                               │  │                               │
│                                                               │  │ i counts from 0 up to 3, not  │
│                                                               │  │ including 3.                  │
│                                                               │  │                               │
│                                                               │  │ ───────────────────────────── │
│                                                               │  │                               │
│                                                               │  │        for i in 0..3 {        │
├───────────────────────────────────────────────────────────────┤  │               ^  next         │
│ Rust                                             Ln 2, Col 12 │  │        }                      │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓░░░░░░░░] 1/3 chunks               Ctrl-P previous   Ctrl-F finish   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## The code box
- It is in the lower `step` card, under the chunk guide, with a dim rule (`overlay0`) between them and a blank row above and below. A copy chunk with no guide shows only the code
- The code is the chunk's text in full color (syntax highlighted), without the indentation all its lines share (here the 4 spaces of the chunk are gone), with the inner indentation kept
- The block is centered across the card width: the widest line sets a left pad, and the lines stay left-aligned inside it
- A badge under the character the user types next. If a line is wider than the card it wraps, and the badge goes under the piece that holds the character. If the badge does not fit, only the `^` shows
- The editor shows only what is typed, with its real indentation. The cursor `█` is on the next expected character

## The badges
The drawing cannot show the colors. The two states are the same shape:

| State | Badge | Colors |
| --- | --- | --- |
| Next character | `^  next ` | quiet: a `surface1` background with `text` on top, and an `overlay1` caret, not bold, so the code stays the main thing |
| After a wrong key | `^  wrong key ` | loud: bold `crust` on a `red` background, and a `red` caret, like the editor's `^ expected n` badge. It stays under the expected character, and goes back to `next` after Backspace |

## After a wrong key
The editor marks the wrong key and shows `^ expected n`, and the badge in the code box changes to `wrong key`, still under the expected character.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ Repeat with for ──────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Inside main, loop three       │
│    1  fn main() {                                             │  │ times.                        │
│ ▶  2      for i ix█                                           │  ╰───────────────────────────────╯
│                  ^  expected n                                │                                   
│    3  }                                                       │  ╭ step ─────────────────────────╮
│                                                               │  │                               │
│                                                               │  │ i counts from 0 up to 3, not  │
│                                                               │  │ including 3.                  │
│                                                               │  │                               │
│                                                               │  │ ───────────────────────────── │
│                                                               │  │                               │
│                                                               │  │        for i in 0..3 {        │
├───────────────────────────────────────────────────────────────┤  │               ^  wrong key    │
│ Rust                                             Ln 2, Col 13 │  │        }                      │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓░░░░░░░░] 1/3 chunks               Ctrl-P previous   Ctrl-F finish   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

The look and the colors of the rest are the ones in `03-ghost-wide.md`.
