# Exercise · Quiz · Wide

Status: Approved

The look of a `quiz` exercise (`docs/todos/08-new-exercise-type.md`), in the wide layout. The user reads code and picks one option. It keeps the chrome of the ghost exercise (`03-ghost-wide.md`): double header and footer, no frame around the screen, rounded `sky` cards touching the edges. The code sits in an editor window on the left with no cursor, and the guide card and the options card are on the right. The drawings are at 100 columns and cannot show colors; they are below.

## Choosing
The three states below are drawn for the `output` kind, where the options are short outputs. The `purpose` kind, the first to build, has a wider options card: see "Purpose kind".

The first option is selected when the exercise opens. The selected row has the gray background (`surface1`).

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 2/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ What does it print? ──────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Read the code, then pick the  │
│    1  fn main() {                                             │  │ output.                       │
│    2      for i in 0..3 {                                     │  ╰───────────────────────────────╯
│    3          println!("{i}");                                │                                   
│    4      }                                                   │  ╭ options ──────────────────────╮
│    5  }                                                       │  │                               │
│                                                               │  │ ▶ 1  0 1 2                    │
│                                                               │  │   2  1 2 3                    │
│                                                               │  │   3  0 1 2 3                  │
│                                                               │  │                               │
│                                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │                               │
│ Rust                                                read only │  │                               │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓░░░░░░░░] 1/3 exercises                    ↑↓ choose   Enter check   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Right answer
Enter checks. The guide card becomes the `explanation` card, the options card badge becomes `right`, and the picked option gets a `✓`. Up and Down do nothing now. Enter goes on, straight to what follows: the next exercise, the session recap (`docs/ui/complete/02-session.md`), the module screen (`03-module.md`, `04-module-failed.md`) or the explorer. A quiz has no complete screen of its own.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 2/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ explanation ──────────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ 0..3 stops before 3.          │
│    1  fn main() {                                             │  │                               │
│    2      for i in 0..3 {                                     │  ╰───────────────────────────────╯
│    3          println!("{i}");                                │                                   
│    4      }                                                   │  ╭ right ────────────────────────╮
│    5  }                                                       │  │                               │
│                                                               │  │ ✓ 1  0 1 2                    │
│                                                               │  │   2  1 2 3                    │
│                                                               │  │   3  0 1 2 3                  │
│                                                               │  │                               │
│                                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │                               │
│ Rust                                                read only │  │                               │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓░░░░░░░░] 1/3 exercises                             Enter continue   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Wrong answer
Same screen. The picked option gets a red `✗` and the right one a green `✓`, and the badge says `wrong`. There is no retry and no score: Enter continues, like a typed exercise ("always passes once done").

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 2/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ explanation ──────────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ 0..3 stops before 3.          │
│    1  fn main() {                                             │  │                               │
│    2      for i in 0..3 {                                     │  ╰───────────────────────────────╯
│    3          println!("{i}");                                │                                   
│    4      }                                                   │  ╭ wrong ────────────────────────╮
│    5  }                                                       │  │                               │
│                                                               │  │ ✓ 1  0 1 2                    │
│                                                               │  │ ✗ 2  1 2 3                    │
│                                                               │  │   3  0 1 2 3                  │
│                                                               │  │                               │
│                                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │                               │
│ Rust                                                read only │  │                               │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓░░░░░░░░] 1/3 exercises                             Enter continue   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Purpose kind
"What does this code do?": the options are sentences about the purpose of the code ("Adds up all the prices in a list"), not outputs. They are longer, so the right column is wider: the split is 3 parts for the code window and 2 for the right column, against 2 and 1 in the other kinds (about 59 and 39 columns at 100). The states, the keys and the colors are the same as above; only the choosing state is drawn.

An option is at most about 50 characters, so it takes two rows at most. Under the marker, a wrapped option continues aligned with its text.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Functions                                             session 1/4 · exercise 2/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭─────────────────────────────────────────────────────────╮  ╭ What does this code do? ────────────╮
│ ● ● ●   main.rs                                         │  │                                     │
├─────────────────────────────────────────────────────────┤  │ Read the code, then pick what it    │
│    1  fn total(prices: &[u32]) -> u32 {                 │  │ does.                               │
│    2      let mut sum = 0;                              │  ╰─────────────────────────────────────╯
│    3      for price in prices {                         │                                         
│    4          sum += price;                             │  ╭ purpose ────────────────────────────╮
│    5      }                                             │  │                                     │
│    6      sum                                           │  │ ▶ 1  Adds up all the prices in a    │
│    7  }                                                 │  │      list                           │
│                                                         │  │   2  Finds the highest price in a   │
│                                                         │  │      list                           │
│                                                         │  │   3  Counts how many prices are in  │
├─────────────────────────────────────────────────────────┤  │      a list                         │
│ Rust                                          read only │  │                                     │
╰─────────────────────────────────────────────────────────╯  ╰─────────────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓░░░░░░░░] 1/3 exercises                    ↑↓ choose   Enter check   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Keys
| Key | Choosing | Answered |
| --- | --- | --- |
| Up, Down | move the selected option | nothing |
| Enter | check the selected option | go on, to what follows |
| Esc | back to the explorer | back to the explorer |

## The pieces
- **Code window:** the editor window of the exercise (title bar with the dots and the file name, rule, line numbers in a gutter of 7 columns), highlighted, with no cursor. Long lines wrap like in the editor. Its status row says `Rust` on the left and `read only` on the right. The window scrolls with Up and Down only when there is nothing to choose, so while choosing the code must fit; the content width rule keeps it short
- **Upper card:** the badge is the exercise title and the content is the guide. Once answered it is replaced by the `explanation` card
- **Options card:** takes the rest of the right column. Each option is `marker`, its number, and the text; a long option wraps under its own text. Between 2 and 4 options. When the options are taller than the card, the card scrolls and keeps the selected option in view
- **Marker:** `▶` on the selected option while choosing. After answering: `✓` (green) on the right option and `✗` (red) on a wrong pick

## The `kind` label
The options card badge says what is being asked, from the optional `kind` of the exercise. After answering, it shows the result.

| `kind` | Badge while choosing | Code window |
| --- | --- | --- |
| none | `options` | the code |
| `purpose` | `purpose` | the code, with the wider options card |
| `output` | `output` | the code |
| `compiler` | `cause` | the compiler message, in red, with the window title `cargo build` and no line numbers |
| `error` | `error kind` | the code |

## The footer
- **Left:** the bar of the exercises of the session (`1/3 exercises`), not of chunks
- **Right:** `↑↓ choose   Enter check   Esc back   Ctrl-Q quit` while choosing, and `Enter continue   Esc back   Ctrl-Q quit` after answering

## Narrow terminals
Not drawn yet. The cards stack: the code window on top, then the guide as a one-line card, then the options card. It waits for the narrow layout of the ghost exercise (`01-ghost-narrow.md`) to follow its stacking rules.

## Borders and colors
| Part | Border | Color |
| --- | --- | --- |
| Header and footer | double (`╔═╗`) | `overlay0` |
| Code window, guide, explanation and options cards | rounded (`╭─╮`) | `sky`; the rule under the title bar the same |
| Card badges | none | bold `crust` text on a `sky` background, one space on each side |
| Result badge | none | bold `crust` on `green` for `right`, on `red` for `wrong` |
| Selected option | none | `surface1` background, `text` on top |
| `✓` and `✗` | none | `green` and `red` |

## Decisions
1. **Explanation placement:** it replaces the guide card once answered (proposed, drawn above). The alternative is a third card under the options, which needs about 6 more rows
2. **First selected option:** the first one (proposed). The right answer can be any index, so this gives nothing away
3. **Footer bar:** exercises of the session (proposed). The alternative is no bar on the left
4. **Width of the right column:** 2 parts against the code's 3 for `purpose` (proposed, drawn in "Purpose kind"), and 1 against 2 for the other kinds. The alternative is a width based on the longest option
5. **Overflow:** the options card scrolls, following the selected option (proposed). Four options of two rows plus the blank row need 9 rows, and the card has 8 in the drawing
6. **`compiler` kind:** the message goes in the code window (proposed) so the window stays the one place where the "code" is read
