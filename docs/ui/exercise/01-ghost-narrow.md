# Exercise · Ghost chunk · Narrow

Status: Draft

The narrow layout, decided and not built yet: the editor on top and the two guide cards stacked under it, all the full width. The width where the layout switches from wide to narrow is still open (todo 3 in `docs/todos/00-exercise-view.md`).

```
╔════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops   session 1/4 · exercise 1/3 ║
╚════════════════════════════════════════════════════╝
                                                      
╭────────────────────────────────────────────────────╮
│ ● ● ●   main.rs                                    │
├────────────────────────────────────────────────────┤
│ ▶  1  █fn main() {                                 │
│    2  }                                            │
│                                                    │
├────────────────────────────────────────────────────┤
│ Rust                                   Ln 1, Col 1 │
╰────────────────────────────────────────────────────╯
                                                      
╭ Repeat with for ───────────────────────────────────╮
│                                                    │
│ Inside main, loop three times.                     │
╰────────────────────────────────────────────────────╯
                                                      
╭ step ──────────────────────────────────────────────╮
│                                                    │
│ Every Rust program starts at main. Type the        │
│ function with its braces.                          │
╰────────────────────────────────────────────────────╯
                                                      
╔════════════════════════════════════════════════════╗
║ [░░░░░░░░░░░░] 0/3 chunks   Esc back   Ctrl-Q quit ║
╚════════════════════════════════════════════════════╝
```

This layout needs about 28 rows. On a shorter terminal the guide cards would be cut, which is the long guide question (todo 4). The look and the colors are the ones in `03-ghost-wide.md`.
