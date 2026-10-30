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
║ [░░░░░░░░░░░░] 0/3 chunks                                 Ctrl-F finish   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## The guide cards
- **Upper card:** the badge is the exercise title and the content is the exercise intro. It is as tall as its content
- **Lower card:** the badge is `step` and the content is the chunk guide. It takes the rest of the space. A chunk without a `guide` (an auto chunk) does not draw it
- A blank row between the two cards, and one blank row between the header, the cards and the footer

## The footer hints
The footer has the chunk bar on the left and the key hints on the right, in this order:
- `Ctrl-P previous`: when there is a chunk to go back to, that is, a chunk before the current one that is not `auto`. See "Going back" below. This sketch is the first chunk, so it has no `Ctrl-P`; `04-copy-wide.md` and `07-inline-notes.md` are second chunks and show it
- `Ctrl-F finish`: always, on a typing exercise. It types the rest of the chunk (`09-auto-complete.md`)
- `Esc back` and `Ctrl-Q quit`: always

The narrow footers (`01-ghost-narrow.md`, `02-copy-narrow.md`) are 54 columns and do not fit the new hints; how they shorten is open.

## Going back
`Ctrl-P` goes back to the previous chunk, so the user can read again its guide and its inline notes, which go away when a chunk is finished (`go_back` in `app/exercise/typing.rs`). It is the opposite of `Ctrl-F`, which goes forward (`09-auto-complete.md`). Taken with the author on 2026-10-30:
1. **The key is `Ctrl-P`,** for "previous". The natural pair of `Ctrl-F` would be `Ctrl-B`, but it is the default `tmux` prefix. In a typing exercise every printable char is typing, so it has to be a Ctrl chord
2. **It drops only the last char of the previous chunk,** not the whole chunk: what is typed of the current chunk goes, the lines of the previous chunk come out of the file, and that chunk comes back with its text minus its last char. The user types that char again and is back at the end of the chunk, with its guide and notes on screen
3. **The hint shows only when there is a chunk to go back to,** so not on the first chunk, and not when the only chunk before the current one is `auto`
4. **An `auto` chunk is skipped and taken out:** the app writes `auto` chunks by itself and the user never types them. Going back goes to the last chunk before the current one that is not `auto`, and takes out every chunk from there on, `auto` ones included, the latest first. They come back when the chunk is finished again
5. **On the first chunk the key does nothing,** and it does not go back to the previous exercise
6. **A chunk finished by typing after going back counts as typed by hand,** so it starts the count of `Ctrl-F` over. A small loophole, left as it is
7. **It does not work while the animation of `Ctrl-F` plays or while its modal is open:** the animation stops at any key and the key is not typed, and the modal takes the keys
8. **Nothing is saved:** going back changes only the state of the exercise, never the progress file

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
