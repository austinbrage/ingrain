# Exercise · Fill in the blank · Wide

Status: Approved

The look of a `fill-blank` exercise (`docs/todos/08-new-exercise-type.md`, T3), in the wide layout. The user sees the whole code with gaps in it and types only the missing word of each gap. Enter moves to the next gap, and Enter on the last one checks them all. It can be passed or failed, and it stays on its own screen, like the quiz: no complete screen, so its output is not shown. It keeps the chrome of the quiz (`08-quiz.md`) and the ghost exercise (`03-ghost-wide.md`): double header and footer, no frame around the screen, rounded `sky` cards touching the edges. The code is in an editor window on the left, with the cursor while typing, and the guide card and the gaps card are on the right. The drawings are at 100 columns and cannot show colors: a gap is drawn as underscores, and the colors are in the tables below.

The sample code, with two gaps whose answers are `in` and `i`:

```rust
fn main() {
    for i ___ 0..3 {
        println!("{}", ___);
    }
}
```

## Opening
The first gap is active: the cursor is on it. The other gaps wait as three dim underscores. The upper card is the title and the guide of the exercise, and the lower card lists the gaps, with `▶` on the active one.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 2/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ Loop with for ────────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Fill the gaps of the loop.    │
│    1  fn main() {                                             │  ╰───────────────────────────────╯
│ ▶  2      for i █__ 0..3 {                                    │                                   
│    3          println!("{}", ___);                            │  ╭ gaps ─────────────────────────╮
│    4      }                                                   │  │                               │
│    5  }                                                       │  │ ▶ 1                           │
│                                                               │  │   2                           │
│                                                               │  │                               │
│                                                               │  │                               │
│                                                               │  │                               │
│                                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │                               │
│ Rust                                             Ln 2, Col 11 │  │                               │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [░░░░░░░░░░░░] 0/2 gaps                                  Enter next gap   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Typing a gap
What the user types replaces the underscores, in bold, and shows in its row of the gaps card. The gap grows when the word is longer than its three cells. There is no verdict yet: a gap is not marked right or wrong until the check.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 2/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ Loop with for ────────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Fill the gaps of the loop.    │
│    1  fn main() {                                             │  ╰───────────────────────────────╯
│ ▶  2      for i in█ 0..3 {                                    │                                   
│    3          println!("{}", ___);                            │  ╭ gaps ─────────────────────────╮
│    4      }                                                   │  │                               │
│    5  }                                                       │  │ ▶ 1  in                       │
│                                                               │  │   2                           │
│                                                               │  │                               │
│                                                               │  │                               │
│                                                               │  │                               │
│                                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │                               │
│ Rust                                             Ln 2, Col 13 │  │                               │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [░░░░░░░░░░░░] 0/2 gaps                                  Enter next gap   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Last gap
Enter moved on: the first gap keeps its word, with an underscore for each cell it does not use, and the next one is active. The cursor, the `▶` marker and the line of the status bar move to it. In the drawing the user typed `of` in the first gap, which is wrong. On the last gap Enter checks, and the footer says so.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 2/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ Loop with for ────────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Fill the gaps of the loop.    │
│    1  fn main() {                                             │  ╰───────────────────────────────╯
│    2      for i of_ 0..3 {                                    │                                   
│ ▶  3          println!("{}", i█_);                            │  ╭ gaps ─────────────────────────╮
│    4      }                                                   │  │                               │
│    5  }                                                       │  │   1  of                       │
│                                                               │  │ ▶ 2  i                        │
│                                                               │  │                               │
│                                                               │  │                               │
│                                                               │  │                               │
│                                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │                               │
│ Rust                                             Ln 3, Col 25 │  │                               │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓▓▓░░░░░░] 1/2 gaps                                     Enter check   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Checked, failed
Every gap shows its verdict. The text of a right gap is `green` and the text of a wrong one `red`, with what was typed still in it, so the user sees which one failed in the code. The gaps card says `wrong`, and a row has `✓` or `✗`, and for a wrong one the typed word, an arrow and the right word. The cursor is gone and the status bar says `read only`. Enter goes on to what follows, as in the quiz. A failed exercise is saved in the `failed` list of its session and can be retaken.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 2/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ Loop with for ────────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Fill the gaps of the loop.    │
│    1  fn main() {                                             │  ╰───────────────────────────────╯
│    2      for i of_ 0..3 {                                    │                                   
│    3          println!("{}", i__);                            │  ╭ wrong ────────────────────────╮
│    4      }                                                   │  │                               │
│    5  }                                                       │  │ ✗ 1  of  →  in                │
│                                                               │  │ ✓ 2  i                        │
│                                                               │  │                               │
│                                                               │  │                               │
│                                                               │  │                               │
│                                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │                               │
│ Rust                                                read only │  │                               │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓▓▓▓▓▓▓▓▓] 2/2 gaps                                  Enter continue   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Checked, passed
The same screen with the text of every gap `green`, and the card says `right`.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 2/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ Loop with for ────────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Fill the gaps of the loop.    │
│    1  fn main() {                                             │  ╰───────────────────────────────╯
│    2      for i in_ 0..3 {                                    │                                   
│    3          println!("{}", i__);                            │  ╭ right ────────────────────────╮
│    4      }                                                   │  │                               │
│    5  }                                                       │  │ ✓ 1  in                       │
│                                                               │  │ ✓ 2  i                        │
│                                                               │  │                               │
│                                                               │  │                               │
│                                                               │  │                               │
│                                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │                               │
│ Rust                                                read only │  │                               │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓▓▓▓▓▓▓▓▓] 2/2 gaps                                  Enter continue   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Keys
| Key | Typing | Checked |
| --- | --- | --- |
| Letters, symbols | type in the active gap | nothing |
| Backspace | erase the last char of the active gap | nothing |
| Enter | next gap, or check on the last one | go on, to what follows |
| Esc | back to the explorer | back to the explorer |

## The pieces
- **Code window:** the editor window of the typing exercise (title bar with the dots and the file name, rule, line numbers in a gutter of 7 columns), highlighted. Long lines wrap like in the editor. Its status row says `Rust` on the left, and on the right the line and column of the cursor while typing, and `read only` once checked
- **Gap:** underscores with no block, 3 cells at least. The typed word replaces them from the left, and the rest of the cells stay as underscores. The gap grows to fit what is typed, and the active one has a cell more, for the cursor
- **Upper card:** the badge is the exercise title and the content is the guide. It does not change after the check
- **Gaps card:** a row per gap: marker, number and the typed word. The badge is `gaps` while typing, and `right` or `wrong` once checked
- **Footer:** the bar of the gaps (`1/2 gaps`), as the typing exercise counts its chunks: the gaps left behind with Enter, and full once checked. The keys are on the right

## Borders and colors
| Part | Border | Color |
| --- | --- | --- |
| Header and footer | double (`╔═╗`) | `overlay0` |
| Code window, guide and gaps cards | rounded (`╭─╮`) | `sky`; the rule under the title bar the same |
| Card badges | none | bold `crust` text on a `sky` background, one space on each side. After the check, `right` on `green` and `wrong` on `red` |
| Gap, waiting | none | dim underscores (`overlay1`) |
| Gap, filled, not active | none | its word in `text`, the rest of the cells as dim underscores |
| Gap, active | none | bold `text` for the typed word and `text` for the underscores, so it is brighter than the others |
| Gap, right after the check | none | its word in bold `green`, the rest of the cells as dim underscores |
| Gap, wrong after the check | none | its word in bold `red`, the rest of the cells as dim underscores |
| `✓` and `✗` | none | `green` and `red` |
| Active row of the gaps card | none | `▶` and `surface1` background, as the selected option of the quiz |

## Not drawn yet
These wait for their own decision, and this file grows when they are settled:
- **Moving back:** whether Backspace on an empty gap goes back to the one before, or a key does
- **How a gap is graded:** the accepted answers of a gap and whether case or spaces count
- **Narrow terminals:** the cards stack, as in the other types. It waits for the narrow layout of the ghost exercise (`01-ghost-narrow.md`)
