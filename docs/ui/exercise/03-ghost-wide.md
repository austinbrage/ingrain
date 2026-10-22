# Exercise · Ghost chunk · Wide

Status: Draft

The definitive exercise look, in the wide layout, for a ghost chunk. The ghost text is dimmed in the editor, the cursor `█` is on the next character, and the note shows under its anchor. No frame around the screen: the header and the footer are boxes, and the cards touch the screen edges. The drawing cannot show colors; they are below.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ Repeat with for ──────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Inside main, loop three       │
│ ▶  1  █fn main() {                                            │  │ times.                        │
│          ╰─ the function every Rust program starts at         │  ╰───────────────────────────────╯
│    2  }                                                       │                                   
│                                                               │  ╭ step ─────────────────────────╮
│                                                               │  │                               │
│                                                               │  │ Every Rust program starts at  │
│                                                               │  │ main. Type the function with  │
│                                                               │  │ its braces.                   │
│                                                               │  │                               │
│                                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │                               │
│ Rust                                              Ln 1, Col 1 │  │                               │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [░░░░░░░░░░░░] 0/3 chunks                                                 Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## The guide cards
- **Upper card:** the badge is the exercise title and the content is the exercise intro. It is as tall as its content
- **Lower card:** the badge is `step` and the content is the chunk guide. It takes the rest of the space. A chunk without a `guide` (an auto chunk) does not draw it
- A blank row between the two cards, and one blank row between the header, the cards and the footer

## Borders and colors
| Part | Border | Color |
| --- | --- | --- |
| Header and footer | double (`╔═╗`) | `overlay0`, muted, so the frame stays quiet |
| Editor card and guide cards | rounded (`╭─╮`) | `sky` |
| Editor rules (`├──┤`) | same as the card | `sky` |
| Card badges | bold `crust` text on a `sky` background, one space on each side | the same as the cards of the complete screen |
| Note hints | gray background (`surface1`), `text` on top, an `overlay1` pointer | |
| Wrong key badge | bold `crust` on `red` | |

The notes inside the editor are in `07-inline-notes.md`, and the older single guide card is in `06-guide-alternative.md`.
