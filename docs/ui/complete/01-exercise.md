# Exercise · Complete

Status: Approved

The screen the user sees after the last chunk of a typing exercise. It replaces the old `docs/ui/exercise/05-complete.md`, removed with the move to this folder (git keeps it). A quiz has no complete screen: after a checked quiz Enter goes straight to the next exercise, or to the session or module screen when it is the last. The decisions are in that todo ("The layouts").

The hero is the code the user built, in an editor window, with its output beside it. A title line (the exercise title with `✓`) and a subtitle with the next step sit above the cards. A big word, `DONE`, tops the screen (the `Sextant` size of `tui_big_text`, 3 rows, in green). There is no Enter row in the body: Enter is on the left of the footer, as on the session and module screens (`02-session.md`, `03-module.md`). It keeps the chrome of the exercise screen (`docs/ui/exercise/03-ghost-wide.md`): double header and footer, no frame around the screen, rounded `sky` cards. The drawings are at 100 columns and cannot show colors; they are below.

## Next exercise
```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                          🬨🬕🬪🬏🬵🬆🬪🬏█🬱▐▌🬨🬕🬒🬄                                          
                                          ▐▌🬷🬄🬬🬏🬷🬄█🬁🬬▌▐🬕🬄🬏                                          
                                          🬂🬂🬀  🬂🬀 🬂 🬁🬀🬂🬂🬂🬀                                          
                                                                                                    
                                           ✓ Hello, world                                           
                                       Next: Exercise 2 of 3                                        
                                                                                                    
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
║ Enter next exercise                                                       Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Last exercise of a session
The subtitle and the footer change.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 3/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                          🬨🬕🬪🬏🬵🬆🬪🬏█🬱▐▌🬨🬕🬒🬄                                          
                                          ▐▌🬷🬄🬬🬏🬷🬄█🬁🬬▌▐🬕🬄🬏                                          
                                          🬂🬂🬀  🬂🬀 🬂 🬁🬀🬂🬂🬂🬀                                          
                                                                                                    
                                           ✓ Hello, world                                           
                                        Next: Session recap                                         
                                                                                                    
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
║ Enter session recap                                                       Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Last exercise of a module
```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 4/4 · exercise 3/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                          🬨🬕🬪🬏🬵🬆🬪🬏█🬱▐▌🬨🬕🬒🬄                                          
                                          ▐▌🬷🬄🬬🬏🬷🬄█🬁🬬▌▐🬕🬄🬏                                          
                                          🬂🬂🬀  🬂🬀 🬂 🬁🬀🬂🬂🬂🬀                                          
                                                                                                    
                                           ✓ Hello, world                                           
                                         Next: Module recap                                         
                                                                                                    
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
║ Enter finish module                                                       Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## No output
Only the code card, centered.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                          🬨🬕🬪🬏🬵🬆🬪🬏█🬱▐▌🬨🬕🬒🬄                                          
                                          ▐▌🬷🬄🬬🬏🬷🬄█🬁🬬▌▐🬕🬄🬏                                          
                                          🬂🬂🬀  🬂🬀 🬂 🬁🬀🬂🬂🬂🬀                                          
                                                                                                    
                                           ✓ Hello, world                                           
                                       Next: Exercise 2 of 3                                        
                                                                                                    
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
║ Enter next exercise                                                       Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Code that overflows
The code card scrolls with Up and Down, and the line numbers follow. A long line wraps onto the next row: the number shows on the first row only and the continuation rows have an empty gutter. The scrollbar sits inside the right border, beside the code rows only, and appears only when there is something to scroll. `↑↓ scroll` shows in the footer only then.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                          🬨🬕🬪🬏🬵🬆🬪🬏█🬱▐▌🬨🬕🬒🬄                                          
                                          ▐▌🬷🬄🬬🬏🬷🬄█🬁🬬▌▐🬕🬄🬏                                          
                                          🬂🬂🬀  🬂🬀 🬂 🬁🬀🬂🬂🬂🬀                                          
                                                                                                    
                                           ✓ Hello, world                                           
                                       Next: Exercise 2 of 3                                        
                                                                                                    
╭────────────────────────────────────────────────────────────────────────╮  ╭ output ──────────────╮
│ ● ● ●   main.rs                                                        │  │                      │
├────────────────────────────────────────────────────────────────────────┤  │  $ cargo run         │
│    1  fn main() {                                                   ↑  │  │  0                   │
│    2      let mut sum = 0;                                          █  │  │  1                   │
│    3      for i in 0..3 {                                           ║  │  │  2                   │
│    4          sum += i; // adds the current number to the           ║  │  │                      │
│       , running total, so sum holds 0 + 1 + 2                       ↓  │  │                      │
╰────────────────────────────────────────────────────────────────────────╯  ╰──────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter next exercise                                           ↑↓ scroll   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Narrow
The cards stack, both at the full width. The title line, the subtitle and the footer are the same.

```
╔════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops   session 1/4 · exercise 1/3 ║
╚════════════════════════════════════════════════════╝
                                                      
                   🬨🬕🬪🬏🬵🬆🬪🬏█🬱▐▌🬨🬕🬒🬄                   
                   ▐▌🬷🬄🬬🬏🬷🬄█🬁🬬▌▐🬕🬄🬏                   
                   🬂🬂🬀  🬂🬀 🬂 🬁🬀🬂🬂🬂🬀                   
                                                      
                    ✓ Hello, world                    
                Next: Exercise 2 of 3                 
                                                      
╭────────────────────────────────────────────────────╮
│ ● ● ●   main.rs                                    │
├────────────────────────────────────────────────────┤
│    1  fn main() {                                  │
│    2      for i in 0..3 {                          │
│    3          println!("{i}");                     │
│    4      }                                        │
│    5  }                                            │
╰────────────────────────────────────────────────────╯
                                                      
╭ output ────────────────────────────────────────────╮
│                                                    │
│  $ cargo run                                       │
│  0                                                 │
│  1                                                 │
│  2                                                 │
│                                                    │
╰────────────────────────────────────────────────────╯
                                                      
╔════════════════════════════════════════════════════╗
║ Enter next exercise         Esc back   Ctrl-Q quit ║
╚════════════════════════════════════════════════════╝
```

## Parts
- **Big word:** `DONE`, in the `Sextant` size of `tui_big_text` (3 rows), centered. Chosen from `05-alternatives.md`
- **Title line:** `✓` and the exercise title, centered. A title longer than the width is cut with `…`
- **Subtitle:** `Next: Exercise 2 of 3`, `Next: Session recap` or `Next: Module recap`, centered
- **Footer:** `Enter next exercise`, `Enter session recap` or `Enter finish module` on the left; `Esc back` and `Ctrl-Q quit` on the right
- **Code card:** an editor window with a title bar (`● ● ●   main.rs`), a rule under it and line numbers in a gutter of 7 columns
- **Output card:** the badge `output`, the command and its lines. It is as wide as its longest line plus 6 columns, at least about 24 and at most half of the area; the code card takes the rest

## Borders and colors
| Part | Border | Color |
| --- | --- | --- |
| Header and footer | double (`╔═╗`) | `overlay0` |
| Code card (editor window) | rounded (`╭─╮`), rule `├─┤` under the title bar | `sky`; dots, file name and gutter as in the exercise editor |
| Output card | rounded (`╭─╮`) | `sky` |
| Output badge | none | bold `crust` text on a `sky` background, one space on each side |
| Big word `DONE` | none | green |
| `✓` | none | green |
| Exercise title | none | bold, default text color |
| Subtitle | none | `overlay0` |
| Scrollbar | inside the right border of the code rows | default style |
| `Enter` in the footer | none | bold, in `sky` |

## Decisions
Taken with the author on 2026-10-23 and 2026-10-25: the exercise screen has the big word `DONE`, a title line with `✓` and a subtitle with the next step, and Enter in the footer.
1. **Title block:** centered above the cards (drawn above)
2. **Subtitle on the last exercise:** `Next: Session recap` and `Next: Module recap` (drawn above)
3. **Card height:** the rest of the body, with a minimum of 5 code rows. The sketches use 5 rows as an example
