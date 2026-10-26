# Exercise · Module failed

Status: Approved

The screen the user sees after the last exercise of a module when at least one session of the module is failed (`docs/progress-flow.md`: a module is failed when a session is failed). It takes the place of the module celebration (`03-module.md`), which shows only when every session passed. The decisions are in `docs/todos/11-complete-screens.md`.

A big word, `ALMOST THERE`, tops the screen (the `Sextant` size of `tui_big_text`, 3 rows, in `peach`, the color of a pending state, not red). It says the module is not complete yet without calling the user's work a failure. The rest is the session recap (`02-session.md`) one level up: one card, a table with a row per session of the module, and a status row. Enter retakes only the failed exercises, so the user goes on without the explorer, and when the retake passes the module is complete and the celebration shows. Same chrome as the other end screens: double header and footer, no frame around the screen, rounded `sky` card. The drawings are at 100 columns and cannot show colors; they are below.

## Wide
```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 3/3 · exercise 2/2 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                          🬵🬎🬱 🬨🬕  █🬱🬻▌🬵🬆🬪🬏🬻🬒🬌 🬆█🬊     🬆█🬊 █ █ 🬨🬕🬒🬄🬨🬕🬨🬓🬨🬕🬒🬄                          
                          █🬋█ ▐▌🬞🬓█🬊🬨▌🬬🬏🬷🬄🬯🬊🬹  █       █  █🬂█ ▐🬕🬄🬏▐🬕🬬🬏▐🬕🬄🬏                          
                          🬂 🬂 🬂🬂🬂🬀🬂 🬁🬀 🬂🬀 🬁🬂🬀 🬁🬂🬀     🬁🬂🬀 🬂 🬂 🬂🬂🬂🬀🬂🬀🬁🬀🬂🬂🬂🬀                          
                                                                                                    
                 ╭ Loops ─────────────────────────────────────────────────────────╮                 
                 │                                                                │                 
                 │  1  ✓  Session 1                              3 of 3   passed  │                 
                 │  2  ✗  Session 2                              2 of 3   failed  │                 
                 │  3  ✓  Session 3                              3 of 3   passed  │                 
                 │                                                                │                 
                 ├────────────────────────────────────────────────────────────────┤                 
                 │  2 of 3 sessions passed                    [▓▓▓▓▓▓▓▓░░░░] 2/3  │                 
                 ╰────────────────────────────────────────────────────────────────╯                 
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter retake failed                                                       Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Narrow
```
╔════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops   session 3/3 · exercise 2/2 ║
╚════════════════════════════════════════════════════╝
                                                      
   🬵🬎🬱 🬨🬕  █🬱🬻▌🬵🬆🬪🬏🬻🬒🬌 🬆█🬊     🬆█🬊 █ █ 🬨🬕🬒🬄🬨🬕🬨🬓🬨🬕🬒🬄   
   █🬋█ ▐▌🬞🬓█🬊🬨▌🬬🬏🬷🬄🬯🬊🬹  █       █  █🬂█ ▐🬕🬄🬏▐🬕🬬🬏▐🬕🬄🬏   
   🬂 🬂 🬂🬂🬂🬀🬂 🬁🬀 🬂🬀 🬁🬂🬀 🬁🬂🬀     🬁🬂🬀 🬂 🬂 🬂🬂🬂🬀🬂🬀🬁🬀🬂🬂🬂🬀   
                                                      
╭ Loops ─────────────────────────────────────────────╮
│                                                    │
│  1  ✓  Session 1                  3 of 3   passed  │
│  2  ✗  Session 2                  2 of 3   failed  │
│  3  ✓  Session 3                  3 of 3   passed  │
│                                                    │
├────────────────────────────────────────────────────┤
│  2 of 3 sessions passed        [▓▓▓▓▓▓▓▓░░░░] 2/3  │
╰────────────────────────────────────────────────────╯
                                                      
╔════════════════════════════════════════════════════╗
║ Enter retake failed         Esc back   Ctrl-Q quit ║
╚════════════════════════════════════════════════════╝
```

## Rows
- **Big word:** `ALMOST THERE`, in the `Sextant` size, centered, in `peach`. It is 48 columns, so it fits the narrow layout on one line
- **Number:** the position of the session in the module
- **Mark:** `✓` or `✗`
- **Name:** `Session 1`, `Session 2`
- **Count:** the exercises passed of the session (`2 of 3`)
- **Result:** `passed` or `failed`
- **Status row:** `N of M sessions passed` on the left and a bar of the same count on the right
- **Footer:** `Enter retake failed` on the left, `Esc back` and `Ctrl-Q quit` on the right

## Borders and colors
| Part | Border | Color |
| --- | --- | --- |
| Header and footer | double (`╔═╗`) | `overlay0` |
| Big word `ALMOST THERE` | none | `peach` |
| Card | rounded (`╭─╮`), rule `├─┤` above the status row | `sky`, with the module name as the badge in the top border |
| Number | none | `overlay0` |
| `✓` and `passed` | none | green |
| `✗` and `failed` | none | red |
| Name and count | none | default text color |
| Status row | none | default text color; the filled part of the bar in `sky` |
| `Enter retake failed` | none | bold, with `Enter` in `sky` |

## Decisions
Taken with the author on 2026-10-23 and 2026-10-25: a failed module has its own screen with the big word `ALMOST THERE`, it reuses the session card with sessions as rows, and Enter goes on to retake the failed exercises.
1. **Session names:** `Session 2` alone, until a session has a `title` in the content (open in `02-session.md`)
2. **Retake order:** the failed exercises of the first failed session, then the next, as Continue does
3. **Last module of the catalog:** the same screen. The course line of the celebration waits until the module passes
