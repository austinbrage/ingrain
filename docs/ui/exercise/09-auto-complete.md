# Exercise · Auto-complete a chunk

Status: Approved

A shortcut for the moment the user is tired of typing: `Ctrl-F` types the rest of the current chunk by itself, character by character, as if the user had typed it. It is on the typing exercise only (a quiz has no chunks), and it is listed in the footer. After a few uses in a row, a small modal reminds the user that typing is the point and asks whether to finish it anyway. It is built (2026-10-30): `app/exercise/typing.rs` (`Autotype`, `ShortcutUses`), `app/exercise/mod.rs` (the keys, `tick`) and `ui/exercise/typing/finish_modal.rs`. The decisions are at the bottom of this file. The drawings are at 100 columns and cannot show colors or motion; they are below, on top of the look of `03-ghost-wide.md`.

## The footer
`Ctrl-F finish` joins the hints on the right, before `Esc back`. It is plain text like the other hints. A quiz footer does not have it. From the second chunk on, `Ctrl-P previous` goes before it (see "Going back" in `03-ghost-wide.md`), and the sketch below is a first chunk, so it does not show it.

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

## While it types
After `Ctrl-F` the rest of the chunk appears one character at a time, with the cursor following it. The characters look like typed ones (nothing marks them as coming from the shortcut), the indentation fills in as it does for the user, and a wrong tail the user had typed is erased first. The animation takes about 2 seconds at most, however long the chunk is. Any key interrupts it and leaves it where it reached. The frames below are the editor card only, with the user having typed `for i` before pressing the shortcut.

```
╭───────────────────────────────────────────────────────────────╮
│ ● ● ●   main.rs                                               │
├───────────────────────────────────────────────────────────────┤
│    1  fn main() {                                             │
│ ▶  2      for i█                                              │
│    3  }                                                       │
╰───────────────────────────────────────────────────────────────╯
```

A few ticks later, the chunk is complete and the cursor is still at its end:

```
╭───────────────────────────────────────────────────────────────╮
│ ● ● ●   main.rs                                               │
├───────────────────────────────────────────────────────────────┤
│    1  fn main() {                                             │
│ ▶  2      for i in 0..3 {█                                    │
│    3  }                                                       │
╰───────────────────────────────────────────────────────────────╯
```

When the chunk is complete the app moves on as for a typed chunk: the next chunk starts, with the cursor on its first character.

```
╭───────────────────────────────────────────────────────────────╮
│ ● ● ●   main.rs                                               │
├───────────────────────────────────────────────────────────────┤
│    1  fn main() {                                             │
│    2      for i in 0..3 {                                     │
│ ▶  3          █println!("{i}");                               │
│    4      }                                                   │
│    5  }                                                       │
╰───────────────────────────────────────────────────────────────╯
```

## The modal
On the use that crosses the limit, the chunk is not completed yet: a modal opens in the middle of the screen. The rest of the screen stays as it is, dimmed, and the typing keys do nothing while it is open. `Enter` completes the chunk with the animation above, and `Esc` closes the modal and leaves the user at the same place in the chunk. The wording is a proposal, in neutral words.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ Repeat with for ──────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Inside main, loop three       │
│ ▶  1  █fn main() {    ╭ Finish it? ──────────────────────────────────────╮                       │
│          ╰─ the functi│                                                  │───────────────────────╯
│    2  }               │  Typing is what makes it stick.                  │                        
│                       │  Finish this one for you anyway?                 │───────────────────────╮
│                       │                                                  │                       │
│                       │  Enter finish it                Esc I'll type it │ust program starts at  │
│                       ╰──────────────────────────────────────────────────╯ype the function with  │
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

The limit grows: the first modal comes on the 4th use in a row, the next on the 9th, then the 16th, and so on (the perfect squares from 4). A modal that is cancelled still counts as a use. A chunk typed by hand resets the count and the limit.

## Keys
| Key | While typing | While it types | In the modal |
| --- | --- | --- | --- |
| Ctrl-F | types the rest of the chunk (or opens the modal on the limit) | interrupts it | nothing |
| Enter | as in the exercise | interrupts it | finish the chunk |
| Esc | back to the explorer | back to the explorer | close the modal |
| Ctrl-Q | quit | quit | quit |
| any other key | typed, as always | interrupts it, and is not typed | nothing |

## Borders and colors
| Part | Border | Color |
| --- | --- | --- |
| Footer hint `Ctrl-F finish` | none | as the other hints |
| Typed characters while it types | none | the same as typed ones |
| Modal | rounded (`╭─╮`) | `sky`, with the title `Finish it?` as the badge (bold `crust` on `sky`) |
| Modal text | none | default text color |
| `Enter` in the modal | none | bold, with `Enter` in `sky` |
| The rest of the screen behind the modal | none | dimmed |

## Decisions
Taken with the author on 2026-10-29:
1. **The key** is `Ctrl-F`, for "finish". In a typing exercise every printable character is typing, so it has to be a Ctrl chord. Avoided: `Ctrl-I`, `H`, `M`, `J`, `[` (a terminal sends them as Tab, Backspace, Enter, a newline and Esc), `C`, `Z`, `D`, `S`, `A`, `B` and `Q`
2. **What it completes** is the rest of the chunk from where the user is, dropping a wrong tail first. Not the whole chunk, and not only the rest of the line
3. **What it counts as:** an exercise finished with it counts as passed, like a typed one, and nothing about it is recorded. Nothing can fail a typing exercise today, and the progress file does not change
4. **"In a row"** is a count of uses that starts over when the user finishes a chunk by typing it. It goes across chunks, exercises and sessions. Quizzes and `auto` chunks neither add nor reset. It resets when the app closes. The limit grows: the 4th use, then the 9th, the 16th (the perfect squares from 4), so the state is one number
5. **The modal** asks, before the chunk is completed, once at each limit. A cancelled one still counts as a use
6. **The animation** types one character per tick, and never takes more than about 2 seconds. A key interrupts it and leaves it where it reached, and the key is not typed (the author, 2026-10-29: first a key finished it at once; the author chose to interrupt it)
7. **Not built:** turning the shortcut off in settings, and disabling it per course or module. Neither is needed

## Open
- The wording of the modal
- Whether the footer hint `Ctrl-F finish` hides while the animation plays (it stays today)
- Whether the modal should show the count (`4th in a row`); it shows none today
- How much the screen behind the modal is dimmed (the terminal's dim attribute today)
