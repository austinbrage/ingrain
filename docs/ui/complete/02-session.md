# Exercise · Session complete

Status: Approved

The screen the user sees after the last exercise of a session, when more sessions follow in the module. It is new: today the end of a session is the exercise complete screen with another summary line (the old `docs/ui/exercise/05-complete.md`, in git history). The decisions are at the bottom of this file; the words that were weighed are in `05-alternatives.md`.

A big word, `SESSION DONE`, tops the screen (the `Sextant` size of `tui_big_text`, 3 rows, in `sky`; on the narrow layout it takes two lines, `SESSION` and `DONE`). The hero is one card, a table with a row per exercise of the session, in order. The code and the output do not show here, they belong to the exercise screen. The card badge is the session name. A rule closes the table with a status row: the count of exercises passed on the left and a bar of the same count on the right, so the bar, the count and the marks of the rows say the same thing. Enter is on the left of the footer. It keeps the chrome of the exercise screen (`docs/ui/exercise/03-ghost-wide.md`): double header and footer, no frame around the screen, rounded `sky` card. The drawings are at 100 columns and cannot show colors; they are below.

## Wide
```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 2/3 · exercise 3/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                          🬻🬒🬌 🬨🬕🬒🬄🬻🬒🬌 🬻🬒🬌 🬁█🬀 🬵🬆🬪🬏█🬱▐▌    🬨🬕🬪🬏🬵🬆🬪🬏█🬱▐▌🬨🬕🬒🬄                          
                          🬯🬊🬹 ▐🬕🬄🬏🬯🬊🬹 🬯🬊🬹  █  🬬🬏🬷🬄█🬁🬬▌    ▐▌🬷🬄🬬🬏🬷🬄█🬁🬬▌▐🬕🬄🬏                          
                          🬁🬂🬀 🬂🬂🬂🬀🬁🬂🬀 🬁🬂🬀 🬁🬂🬀  🬂🬀 🬂 🬁🬀    🬂🬂🬀  🬂🬀 🬂 🬁🬀🬂🬂🬂🬀                          
                                                                                                    
                 ╭ Session 2 · Ownership basics ──────────────────────────────────╮                 
                 │                                                                │                 
                 │  1  ✓  Move a value                           typing   passed  │                 
                 │  2  ✓  Who owns it?                             quiz   passed  │                 
                 │  3  ✓  Borrow it                              typing   passed  │                 
                 │                                                                │                 
                 ├────────────────────────────────────────────────────────────────┤                 
                 │  3 of 3 exercises passed                   [▓▓▓▓▓▓▓▓▓▓▓▓] 3/3  │                 
                 ╰────────────────────────────────────────────────────────────────╯                 
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter next session                                                        Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## With a failed exercise
The failed state is built (`docs/progress-flow.md`), and this is how a failed exercise looks. The `✗` and `failed` are red and the count drops. Enter does not change (the retake is not decided here).

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 2/3 · exercise 3/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                          🬻🬒🬌 🬨🬕🬒🬄🬻🬒🬌 🬻🬒🬌 🬁█🬀 🬵🬆🬪🬏█🬱▐▌    🬨🬕🬪🬏🬵🬆🬪🬏█🬱▐▌🬨🬕🬒🬄                          
                          🬯🬊🬹 ▐🬕🬄🬏🬯🬊🬹 🬯🬊🬹  █  🬬🬏🬷🬄█🬁🬬▌    ▐▌🬷🬄🬬🬏🬷🬄█🬁🬬▌▐🬕🬄🬏                          
                          🬁🬂🬀 🬂🬂🬂🬀🬁🬂🬀 🬁🬂🬀 🬁🬂🬀  🬂🬀 🬂 🬁🬀    🬂🬂🬀  🬂🬀 🬂 🬁🬀🬂🬂🬂🬀                          
                                                                                                    
                 ╭ Session 2 · Ownership basics ──────────────────────────────────╮                 
                 │                                                                │                 
                 │  1  ✓  Move a value                           typing   passed  │                 
                 │  2  ✗  Who owns it?                             quiz   failed  │                 
                 │  3  ✓  Borrow it                              typing   passed  │                 
                 │                                                                │                 
                 ├────────────────────────────────────────────────────────────────┤                 
                 │  2 of 3 exercises passed                   [▓▓▓▓▓▓▓▓░░░░] 2/3  │                 
                 ╰────────────────────────────────────────────────────────────────╯                 
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter next session                                                        Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Narrow
Same parts, the card takes the full width. A title longer than the row is cut with `…`, and the kind and the result keep their columns.

```
╔════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops   session 2/3 · exercise 3/3 ║
╚════════════════════════════════════════════════════╝
                                                      
             🬻🬒🬌 🬨🬕🬒🬄🬻🬒🬌 🬻🬒🬌 🬁█🬀 🬵🬆🬪🬏█🬱▐▌             
             🬯🬊🬹 ▐🬕🬄🬏🬯🬊🬹 🬯🬊🬹  █  🬬🬏🬷🬄█🬁🬬▌             
             🬁🬂🬀 🬂🬂🬂🬀🬁🬂🬀 🬁🬂🬀 🬁🬂🬀  🬂🬀 🬂 🬁🬀             
                                                      
                   🬨🬕🬪🬏🬵🬆🬪🬏█🬱▐▌🬨🬕🬒🬄                   
                   ▐▌🬷🬄🬬🬏🬷🬄█🬁🬬▌▐🬕🬄🬏                   
                   🬂🬂🬀  🬂🬀 🬂 🬁🬀🬂🬂🬂🬀                   
                                                      
╭ Session 2 · Ownership basics ──────────────────────╮
│                                                    │
│  1  ✓  Move a value               typing   passed  │
│  2  ✓  Who owns it?                 quiz   passed  │
│  3  ✓  Borrow it                  typing   passed  │
│                                                    │
├────────────────────────────────────────────────────┤
│  3 of 3 exercises passed       [▓▓▓▓▓▓▓▓▓▓▓▓] 3/3  │
╰────────────────────────────────────────────────────╯
                                                      
╔════════════════════════════════════════════════════╗
║ Enter next session          Esc back   Ctrl-Q quit ║
╚════════════════════════════════════════════════════╝
```

## Narrow, with a failed exercise
```
╔════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops   session 2/3 · exercise 3/3 ║
╚════════════════════════════════════════════════════╝
                                                      
             🬻🬒🬌 🬨🬕🬒🬄🬻🬒🬌 🬻🬒🬌 🬁█🬀 🬵🬆🬪🬏█🬱▐▌             
             🬯🬊🬹 ▐🬕🬄🬏🬯🬊🬹 🬯🬊🬹  █  🬬🬏🬷🬄█🬁🬬▌             
             🬁🬂🬀 🬂🬂🬂🬀🬁🬂🬀 🬁🬂🬀 🬁🬂🬀  🬂🬀 🬂 🬁🬀             
                                                      
                   🬨🬕🬪🬏🬵🬆🬪🬏█🬱▐▌🬨🬕🬒🬄                   
                   ▐▌🬷🬄🬬🬏🬷🬄█🬁🬬▌▐🬕🬄🬏                   
                   🬂🬂🬀  🬂🬀 🬂 🬁🬀🬂🬂🬂🬀                   
                                                      
╭ Session 2 · Ownership basics ──────────────────────╮
│                                                    │
│  1  ✓  Move a value               typing   passed  │
│  2  ✗  Who owns it?                 quiz   failed  │
│  3  ✓  Borrow it                  typing   passed  │
│                                                    │
├────────────────────────────────────────────────────┤
│  2 of 3 exercises passed       [▓▓▓▓▓▓▓▓░░░░] 2/3  │
╰────────────────────────────────────────────────────╯
                                                      
╔════════════════════════════════════════════════════╗
║ Enter next session          Esc back   Ctrl-Q quit ║
╚════════════════════════════════════════════════════╝
```

## Rows
- **Big word:** `SESSION DONE`, in the `Sextant` size, centered. Chosen from `05-alternatives.md`
- **Number:** the position of the exercise in the session
- **Mark:** `✓` or `✗`
- **Title:** the exercise title from the content
- **Kind:** `typing` or `quiz`, in the dim color
- **Result:** `passed` or `failed`. A wrong quiz answer makes it `failed` (`docs/progress-flow.md`)
- **Status row:** `N of M exercises passed` on the left, and on the right the bar of the exercises passed (`[▓▓▓▓▓▓▓▓░░░░] 2/3`; full when all passed). The drawings show the last frame: on opening, the bar fills one cell every 2 ticks from empty, with the count beside it going `0/3`, `1/3`, `2/3` up to the real value, and then holds. The module progress is not drawn here: the header already says `session 2/3`
- **Many exercises:** the card is as tall as its rows. If they do not fit the body, the card scrolls with Up and Down like the code card of `01-exercise.md`

## Borders and colors
| Part | Border | Color |
| --- | --- | --- |
| Header and footer | double (`╔═╗`) | `overlay0` |
| Big word `SESSION DONE` | none | `sky` |
| Card | rounded (`╭─╮`), rule `├─┤` above the status row | `sky`, with the session name as the badge in the top border |
| Number | none | `overlay0` |
| `✓` and `passed` | none | green |
| `✗` and `failed` | none | red |
| Title | none | default text color |
| Kind | none | `overlay0` |
| Status row | none | default text color; the filled part of the bar in `sky` |
| `Enter next session` | none | bold, with `Enter` in `sky` |

## Decisions
Taken with the author on 2026-10-22 and 2026-10-25: the session has the big word `SESSION DONE`, the session is a table inside one card, there is no code on the screen, Enter is in the footer, the bar in the status row is the exercises passed (the same count as the marks), the module progress is only in the header, and the recap only shows (no retake: the retake is Continue and the explorer, see `docs/progress-flow.md`).
1. **The session name:** the badge reads `Session 2 · Ownership basics`, but a session has only an `id` in the content today. Either the session gets a `title` in the content model (shared with the web mirror), or the badge is `Session 2` alone
2. **Card width:** 66 columns centered (drawn above), or the width of the longest row
