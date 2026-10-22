# Exercise · Complete

Status: Draft

Built in `src/ui/complete/`. The code card is an editor window like the exercise one, and the output card has the `sky` border like the guide cards.

The screen the user sees after the last chunk of an exercise. It has the same chrome as the exercise screen (`03-ghost-wide.md`): the header and the footer are double boxes, there is no frame around the screen, and the cards are rounded and `sky`. The code card looks like the exercise editor (title bar with the dots and the file name, line numbers) and the output card like a guide card. The big `COMPLETE` and the line under it stay. The drawings are at 100 columns and cannot show colors; they are below.

## Next exercise
```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/1 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                              ▄▀▀▀ ▄▀▀▄ █▄ ▄█ █▀▀▄ █    █▀▀▀ ▀█▀ █▀▀▀                               
                              █    █  █ █ ▀ █ █▀▀  █    █▀▀   █  █▀▀                                
                              ▀▄▄▄ ▀▄▄▀ █   █ █    █▄▄▄ █▄▄▄  █  █▄▄▄                               
                                                                                                    
                                      Exercise 1 of 3 complete                                      
                                                                                                    
                                 Press Enter for the next exercise                                  
                                                                                                    
╭────────────────────────────────────────────────────────────────────────╮  ╭ output ──────────────╮
│ ● ● ●   main.rs                                                        │  │                      │
├────────────────────────────────────────────────────────────────────────┤  │  $ cargo run         │
│    1  fn main() {                                                      │  │  0                   │
│    2      for i in 0..3 {                                              │  │  1                   │
│    3          println!("{i}");                                         │  │  2                   │
│    4      }                                                            │  │                      │
│    5  }                                                                │  │                      │
╰────────────────────────────────────────────────────────────────────────╯  ╰──────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓▓▓▓▓▓▓▓▓] 3/3 chunks                                                 Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Next session
Only the summary line and the Enter row change:

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/1 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                              ▄▀▀▀ ▄▀▀▄ █▄ ▄█ █▀▀▄ █    █▀▀▀ ▀█▀ █▀▀▀                               
                              █    █  █ █ ▀ █ █▀▀  █    █▀▀   █  █▀▀                                
                              ▀▄▄▄ ▀▄▄▀ █   █ █    █▄▄▄ █▄▄▄  █  █▄▄▄                               
                                                                                                    
                                      Session 2 of 3 complete                                       
                                                                                                    
                                  Press Enter for the next session                                  
                                                                                                    
╭────────────────────────────────────────────────────────────────────────╮  ╭ output ──────────────╮
│ ● ● ●   main.rs                                                        │  │                      │
├────────────────────────────────────────────────────────────────────────┤  │  $ cargo run         │
│    1  fn main() {                                                      │  │  0                   │
│    2      for i in 0..3 {                                              │  │  1                   │
│    3          println!("{i}");                                         │  │  2                   │
│    4      }                                                            │  │                      │
│    5  }                                                                │  │                      │
╰────────────────────────────────────────────────────────────────────────╯  ╰──────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓▓▓▓▓▓▓▓▓] 3/3 chunks                                                 Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Finish
```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/1 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                              ▄▀▀▀ ▄▀▀▄ █▄ ▄█ █▀▀▄ █    █▀▀▀ ▀█▀ █▀▀▀                               
                              █    █  █ █ ▀ █ █▀▀  █    █▀▀   █  █▀▀                                
                              ▀▄▄▄ ▀▄▄▀ █   █ █    █▄▄▄ █▄▄▄  █  █▄▄▄                               
                                                                                                    
                                          Module complete                                           
                                                                                                    
                                       Press Enter to finish                                        
                                                                                                    
╭────────────────────────────────────────────────────────────────────────╮  ╭ output ──────────────╮
│ ● ● ●   main.rs                                                        │  │                      │
├────────────────────────────────────────────────────────────────────────┤  │  $ cargo run         │
│    1  fn main() {                                                      │  │  0                   │
│    2      for i in 0..3 {                                              │  │  1                   │
│    3          println!("{i}");                                         │  │  2                   │
│    4      }                                                            │  │                      │
│    5  }                                                                │  │                      │
╰────────────────────────────────────────────────────────────────────────╯  ╰──────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓▓▓▓▓▓▓▓▓] 3/3 chunks                                                 Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Code that overflows
The code card scrolls with Up and Down, and the line numbers follow. A long line wraps onto the next row, as in the exercise editor: the number shows on the first row only and the continuation rows have an empty gutter. The scrollbar sits on the right border, beside the code rows only, and appears only when there is something to scroll. `↑↓ scroll` shows in the footer only then.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/1 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                              ▄▀▀▀ ▄▀▀▄ █▄ ▄█ █▀▀▄ █    █▀▀▀ ▀█▀ █▀▀▀                               
                              █    █  █ █ ▀ █ █▀▀  █    █▀▀   █  █▀▀                                
                              ▀▄▄▄ ▀▄▄▀ █   █ █    █▄▄▄ █▄▄▄  █  █▄▄▄                               
                                                                                                    
                                      Exercise 1 of 3 complete                                      
                                                                                                    
                                 Press Enter for the next exercise                                  
                                                                                                    
╭────────────────────────────────────────────────────────────────────────╮  ╭ output ──────────────╮
│ ● ● ●   main.rs                                                        │  │                      │
├────────────────────────────────────────────────────────────────────────┤  │  $ cargo run         │
│    1  fn main() {                                                      ↑  │  0                   │
│    2      let mut sum = 0;                                             █  │  1                   │
│    3      for i in 0..3 {                                              ║  │  2                   │
│    4          sum += i; // adds the current number to the running total║  │                      │
│       , so after the loop sum holds 0 + 1 + 2                          ↓  │                      │
╰────────────────────────────────────────────────────────────────────────╯  ╰──────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓▓▓▓▓▓▓▓▓] 3/3 chunks                                     ↑↓ scroll   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## No output
Only the code card, centered. Today the output card is drawn empty.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/1 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                              ▄▀▀▀ ▄▀▀▄ █▄ ▄█ █▀▀▄ █    █▀▀▀ ▀█▀ █▀▀▀                               
                              █    █  █ █ ▀ █ █▀▀  █    █▀▀   █  █▀▀                                
                              ▀▄▄▄ ▀▄▄▀ █   █ █    █▄▄▄ █▄▄▄  █  █▄▄▄                               
                                                                                                    
                                      Exercise 1 of 3 complete                                      
                                                                                                    
                                 Press Enter for the next exercise                                  
                                                                                                    
              ╭──────────────────────────────────────────────────────────────────────╮              
              │ ● ● ●   main.rs                                                      │              
              ├──────────────────────────────────────────────────────────────────────┤              
              │    1  fn main() {                                                    │              
              │    2      for i in 0..3 {                                            │              
              │    3          println!("{i}");                                       │              
              │    4      }                                                          │              
              │    5  }                                                              │              
              ╰──────────────────────────────────────────────────────────────────────╯              
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [▓▓▓▓▓▓▓▓▓▓▓▓] 3/3 chunks                                                 Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## What changes from the old draft
- **Code card:** an editor window: a title bar (`● ● ●   main.rs`), a rule under it and line numbers in a gutter of 7 columns. No footer rule, no `code` badge. Long lines wrap onto the next row, so all the code is visible; the number is on the first row only
- **Output card:** the same rounded `sky` border as the guide cards on the exercise screen (before, only its badge was `sky`)
- **Header:** a double box with the breadcrumb and `session 1/4 · exercise 1/1` (before: only the session)
- **Enter row:** a line under the summary, so the user sees right away what to press. It follows the outcome
- **Footer:** a double box. Left: the bar of the chunks, full. Right: `Esc back` and `Ctrl-Q quit` (Enter moved to the row above)
- **No frame around the screen,** no separator lines. One blank row between header, body and footer

## Borders and colors
| Part | Border | Color |
| --- | --- | --- |
| Header and footer | double (`╔═╗`) | `overlay0` |
| Code card (editor window) | rounded (`╭─╮`), rule `├─┤` under the title bar | `sky`; dots, file name and gutter as in the exercise editor |
| Output card | rounded (`╭─╮`) | `sky` |
| Output badge | none | bold `crust` text on a `sky` background, one space on each side |
| Scrollbar | on the right border of the code rows | default style |
| `COMPLETE` and the summary line | none | default text color |
| Enter row | none | bold, with `Enter` in `sky` |

## Decisions
Taken with the author on 2026-10-19: the proposals marked below are accepted, decision 4 changed after part 4 (the cards fill the body), and decision 6 waits. The Enter row under the summary line was agreed too. Decision 7 was taken with the author after the redraw.
1. **Left of the footer:** the full chunk bar (proposed), the position (`exercise 1/1`), or nothing
2. **Cards and the screen edges:** the cards touch the edges like on the exercise screen (proposed, drawn above), or keep a margin so the title block reads as a centered unit
3. **Output card width:** content-based (proposed, drawn above): the longest line plus 6 columns, minimum about 24, cap half of the area, and the code card takes the rest. The alternative is 50/50
4. **Card height:** the rest of the body, from under the Enter row to the blank row above the footer, with a minimum of 5 rows. The sketches use 9 rows as an example
5. **No output:** only the code card, centered (proposed, drawn above at 72 columns)
6. **Narrow terminals:** stack the cards under some width. Waits for the narrow layout of the exercise (`00-exercise-view.md`, todo 3)
7. **Long code lines:** wrapped onto the next row at the width of the code area (the card width minus the 7-column gutter and the border), with the number on the first row only, like the exercise editor. All the code stays visible
