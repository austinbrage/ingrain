# Exercise · Copy chunk · Narrow

Status: Draft

The narrow layout of `01-ghost-narrow.md` for a copy chunk: the same code box in the lower card, with more room, so the code is centered in a wider card. The code box is definitive, as in `04-copy-wide.md`.

```
╔════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops   session 1/4 · exercise 1/3 ║
╚════════════════════════════════════════════════════╝
                                                      
╭────────────────────────────────────────────────────╮
│ ● ● ●   main.rs                                    │
├────────────────────────────────────────────────────┤
│    1  fn main() {                                  │
│ ▶  2      for i i█                                 │
│    3  }                                            │
├────────────────────────────────────────────────────┤
│ Rust                                  Ln 2, Col 12 │
╰────────────────────────────────────────────────────╯
                                                      
╭ Repeat with for ───────────────────────────────────╮
│                                                    │
│ Inside main, loop three times.                     │
╰────────────────────────────────────────────────────╯
                                                      
╭ step ──────────────────────────────────────────────╮
│                                                    │
│ i counts from 0 up to 3, not including 3.          │
│                                                    │
│ ────────────────────────────────────────────────── │
│                                                    │
│                  for i in 0..3 {                   │
│                         ^  next                    │
│                  }                                 │
╰────────────────────────────────────────────────────╯
                                                      
╔════════════════════════════════════════════════════╗
║ [▓▓▓▓░░░░░░░░] 1/3 chunks   Esc back   Ctrl-Q quit ║
╚════════════════════════════════════════════════════╝
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

This one needs about 33 rows. The look and the colors of the rest are the ones in `03-ghost-wide.md`.
