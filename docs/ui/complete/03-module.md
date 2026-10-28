# Complete · Module complete

Status: Approved

The screen the user sees after the last exercise of a module when every session passed (when one failed, `04-module-failed.md` shows instead). It is new: today the end of a module is the exercise complete screen with `Module complete` as its summary line (the old `docs/ui/exercise/05-complete.md`, in git history). The decisions are in `docs/todos/11-complete-screens.md` ("The layouts").

The hero is a big word, `LEVEL UP`, in the same `Sextant` size of `tui_big_text` as the other end screens (`01-exercise.md`, `02-session.md`), so the module does not stand out by the size of its letters. It stands out by having no card, by the gradient sliding along the word and the caption, by the bar that fills under them and by the color of the module name. Under the word: a small caption, a bar, the module name, and the numbers of the module. Enter is in the footer. It keeps the chrome of the exercise screen (`docs/ui/exercise/03-ghost-wide.md`): double header and footer. The drawings are at 100 columns and cannot show colors or motion; they show the last frame, with the bar full. The words that were weighed are in `05-alternatives.md`.

## Motion
The screen is on the tick (`app.tick`, restarted each time the screen opens):
- The gradient of the home title slides along the word and the caption
- The bar fills one cell every 2 ticks, with the text beside it counting `0%` up to `100%` along with the cells. It holds full for 30 ticks and starts over, forever
- Nothing moves the layout: the sketches are the last frame

## Wide
```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 3/3 · exercise 2/2 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                  🬨🬕  🬨🬕🬒🬄█ █ 🬨🬕🬒🬄🬨🬕      █ █ 🬨🬕🬨🬓                                  
                                  ▐▌🬞🬓▐🬕🬄🬏🬬🬭🬝 ▐🬕🬄🬏▐▌🬞🬓    █ █ ▐🬕🬂                                   
                                  🬂🬂🬂🬀🬂🬂🬂🬀 🬂  🬂🬂🬂🬀🬂🬂🬂🬀    🬂🬂🬂 🬂🬂                                    
                                                                                                    
                                      ──  module complete  ──                                       
                                                                                                    
                                        [▓▓▓▓▓▓▓▓▓▓▓▓] 100%                                         
                                                                                                    
                                                                                                    
                                               Loops                                                
                                      3 sessions · 9 exercises                                      
                                                                                                    
                                                                                                    
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter next module                                                         Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## The end of a course
When the module just finished was the last one left in its course, a bold line names the course (`You completed the Syntax course`), the footer says `Enter explorer` and Enter goes to the explorer (the author, 2026-10-27). It is the same whether another course follows or nothing is left in the language. When the module is in the middle of its course, there is no course line, the footer says `Enter next module` and Enter opens the next one.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 3/3 · exercise 2/2 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                  🬨🬕  🬨🬕🬒🬄█ █ 🬨🬕🬒🬄🬨🬕      █ █ 🬨🬕🬨🬓                                  
                                  ▐▌🬞🬓▐🬕🬄🬏🬬🬭🬝 ▐🬕🬄🬏▐▌🬞🬓    █ █ ▐🬕🬂                                   
                                  🬂🬂🬂🬀🬂🬂🬂🬀 🬂  🬂🬂🬂🬀🬂🬂🬂🬀    🬂🬂🬂 🬂🬂                                    
                                                                                                    
                                      ──  module complete  ──                                       
                                                                                                    
                                        [▓▓▓▓▓▓▓▓▓▓▓▓] 100%                                         
                                                                                                    
                                                                                                    
                                               Loops                                                
                                      3 sessions · 9 exercises                                      
                                  You completed the Syntax course                                   
                                                                                                    
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter explorer                                                            Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Narrow
The same parts.

```
╔════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops   session 3/3 · exercise 2/2 ║
╚════════════════════════════════════════════════════╝
                                                      
           🬨🬕  🬨🬕🬒🬄█ █ 🬨🬕🬒🬄🬨🬕      █ █ 🬨🬕🬨🬓           
           ▐▌🬞🬓▐🬕🬄🬏🬬🬭🬝 ▐🬕🬄🬏▐▌🬞🬓    █ █ ▐🬕🬂            
           🬂🬂🬂🬀🬂🬂🬂🬀 🬂  🬂🬂🬂🬀🬂🬂🬂🬀    🬂🬂🬂 🬂🬂             
                                                      
               ──  module complete  ──                
                                                      
                 [▓▓▓▓▓▓▓▓▓▓▓▓] 100%                  
                                                      
                                                      
                        Loops                         
               3 sessions · 9 exercises               
                                                      
                                                      
                                                      
╔════════════════════════════════════════════════════╗
║ Enter next module           Esc back   Ctrl-Q quit ║
╚════════════════════════════════════════════════════╝
```

## Narrow, the end of a course
```
╔════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops   session 3/3 · exercise 2/2 ║
╚════════════════════════════════════════════════════╝
                                                      
           🬨🬕  🬨🬕🬒🬄█ █ 🬨🬕🬒🬄🬨🬕      █ █ 🬨🬕🬨🬓           
           ▐▌🬞🬓▐🬕🬄🬏🬬🬭🬝 ▐🬕🬄🬏▐▌🬞🬓    █ █ ▐🬕🬂            
           🬂🬂🬂🬀🬂🬂🬂🬀 🬂  🬂🬂🬂🬀🬂🬂🬂🬀    🬂🬂🬂 🬂🬂             
                                                      
               ──  module complete  ──                
                                                      
                 [▓▓▓▓▓▓▓▓▓▓▓▓] 100%                  
                                                      
                                                      
                        Loops                         
               3 sessions · 9 exercises               
           You completed the Syntax course            
                                                      
                                                      
╔════════════════════════════════════════════════════╗
║ Enter explorer              Esc back   Ctrl-Q quit ║
╚════════════════════════════════════════════════════╝
```

## Parts
- **Big word:** `LEVEL UP`, in the `Sextant` size (3 rows), centered. Chosen from `05-alternatives.md`
- **Caption:** `──  module complete  ──`, centered, with the same gradient as the word
- **Bar:** `[▓▓▓▓▓▓▓▓▓▓▓▓] 100%`, centered, 12 cells, one blank row above it and two below. The text beside it counts with the cells
- **Module name:** `Loops`, centered, in bold `yellow`. Not in big letters
- **Numbers line:** `3 sessions · 9 exercises`
- **Course line:** only when the module ends its course: `You completed the Syntax course`, in bold
- **Footer:** `Enter next module` on the left (`Enter explorer` when the module ends its course), `Esc back` and `Ctrl-Q quit` on the right

## Borders and colors
| Part | Border | Color |
| --- | --- | --- |
| Header and footer | double (`╔═╗`) | `overlay0` |
| Big word `LEVEL UP` | none | the gradient of the home title (`blue`, `sky`, `teal`), sliding along the letters |
| Caption | none | the same gradient as the word |
| Bar | none | the filled cells in `sky`, the empty ones and the text in the default color |
| Module name | none | bold `yellow` |
| Numbers line | none | default text color |
| Course line | none | bold, default text color |
| Enter in the footer | none | bold, with `Enter` in `sky` |

## Decisions
Taken with the author on 2026-10-22 and 2026-10-25: the module has the big word `LEVEL UP` in the same size as the other screens, no card, a caption, a bar, the module name as a colored line, the numbers of the module, and Enter to the next module. The sparkles were tried and dropped (2026-10-28): they looked unrelated to the rest, and the bar took their place. Changed on 2026-10-27: when the module ends its course, whether or not another course follows, the course line shows and Enter goes to the explorer.
1. **The glyphs:** the sketch uses the real glyphs of `tui_big_text` in `Sextant`, which needs a terminal font with the Unicode legacy computing blocks. If a terminal cannot show them, the fallback is the `Quadrant` size (4 rows), which the app already uses
2. **The gradient:** the word takes the home title's gradient, which is built (`wave_title` in `ui/explorer/columns/mod.rs` is the same idea). It slides along the word and the caption (`wave_line` in `ui/wave.rs`, shared with the home and explorer titles)
