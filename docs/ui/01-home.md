# Home

Status: Approved

The home with a typing demo as the centerpiece. The frame is four thick corners that grow with the window, the demo card sits between the tagline and the buttons, and the two buttons are equal: plain text that gets a border and a highlight only when selected. Enter on Choose a language swaps the buttons for one card per language, so the language is picked on the home itself. Everything is centered, and the title and the demo move.

The home has two views: the buttons view (the start) and the languages view.

```
┏━━━━                                                  ━━━━┓
┃          ▀█▀  █▄ █  ▄▀▀▀  █▀▀▄  ▄▀▀▄  ▀█▀  █▄ █          ┃
┃           █   █ ▀█  █ ▀█  █▀█▀  █▀▀█   █   █ ▀█          ┃
           ▄█▄  █  █  ▀▄▄▀  █ ▀▄  █  █  ▄█▄  █  █           
                                                            
                No AI, Just you and the keys                
                                                            
        ╭──────────────────────────────────────────╮        
        │ ● ● ●   main.rs                          │        
        ├──────────────────────────────────────────┤        
        │ fn main() {                              │        
        │     println!("hello, ingrain");█         │        
        │ }                                        │        
        ╰──────────────────────────────────────────╯        
                                                            
        ┌──────────────────────────────────────────┐        
        │       Continue   Rust · Ownership        │        
        └──────────────────────────────────────────┘        
                                                            
                                                            
                     Choose a language                      
┃                                                          ┃
┃                                                          ┃
┗━━━━       ↑↓ move   Enter select   Ctrl-Q quit       ━━━━┛
```

The languages view, with the Rust card selected:

```
┏━━━━                                                  ━━━━┓
┃          ▀█▀  █▄ █  ▄▀▀▀  █▀▀▄  ▄▀▀▄  ▀█▀  █▄ █          ┃
┃           █   █ ▀█  █ ▀█  █▀█▀  █▀▀█   █   █ ▀█          ┃
           ▄█▄  █  █  ▀▄▄▀  █ ▀▄  █  █  ▄█▄  █  █           
                                                            
                No AI, Just you and the keys                
                                                            
        ╭──────────────────────────────────────────╮        
        │ ● ● ●   main.rs                          │        
        ├──────────────────────────────────────────┤        
        │ fn main() {                              │        
        │     println!("hello, ingrain");█         │        
        │ }                                        │        
        ╰──────────────────────────────────────────╯        
                                                            
                                                            
                                                            
                                                            
                                                            
        ┌───────────────────┐                               
        │       Rust        │           Go                  
┃       └───────────────────┘                              ┃
┃                                                          ┃
┗━━━━ ←→ move   Enter select   Esc back   Ctrl-Q quit  ━━━━┛
```

This layout needs about 24 rows and 46 columns. The first sketch shows Continue selected, at 60 columns. The demo card is the same window as the exercise editor (traffic lights, file name, rules), so the home previews the real thing.

## Frame
- Four thick corners (`┏ ┓ ┗ ┛`) with arms along the edges, in `overlay0`. The bottom corners share the hint row
- The arms grow with the window: the width is 1/12 of it (at least 2 and at most 8 columns) and the height is 1/8 of it (at least 2 rows)
- The content is centered inside the corners

## Title and tagline
- The title is 38 columns wide, 3 rows tall, with two spaces between the letters
- The tagline is one line under it, with `No AI` bold `red` and the rest `overlay1`

## Buttons
- Both are 44 columns wide and 3 rows tall, with the text in the middle row. The area never changes, so nothing moves when the selection changes
- **Continue** shows the last language and the last course studied after the name (`Rust · Ownership` here). Without a last study session the button does not exist and only `Choose a language` is shown, selected from the start (see `docs/progress-flow.md`)
- **Choose a language** is only the name
- **Not selected**: plain text, no border, `overlay1`
- **Selected**: a full border and a `sky` fill with bold `crust` text:
```
        ┌──────────────────────────────────────────┐        
        │       Continue   Rust · Ownership        │        
        └──────────────────────────────────────────┘        
```

## Languages view
Opened with Enter on Choose a language.
- The buttons are not drawn, Continue included. One card per language takes the place of the Choose a language button, so nothing else moves. The Continue slot stays empty
- Each card is 21 columns wide and 3 rows tall, with the name in the middle row. Two cards and a gap of 2 columns make the 44 columns of the block
- Same look as the buttons: not selected is plain `overlay1` text with no border, selected is a full border and a `sky` fill with bold `crust` text. Every language uses `sky`: brand colors on the selected card were tried and dropped
- Rust is selected when the view opens, and the demo follows the selected card: its snippets and its colors are the language's
- `←→` move between cards, Enter opens the explorer for that language
- Esc goes back to the buttons view with Choose a language selected. Esc in the explorer comes back here with that language's card selected
- In the buttons view Esc does nothing: Ctrl-Q is the only way to quit

## Color
All from the catppuccin mocha palette, like the other screens.
- **Title**: horizontal gradient across the columns, `blue` → `sky` → `teal`
- **Demo card**: border and rule `overlay0`, the three dots `red`, `yellow`, `green` as in the editor, the file name `overlay1`. The code uses the colors of the `highlight` module, the typed part at full color and the cursor `█` in `text`
- **Corners**: `overlay0`
- **Hint**: `overlay1`, with the keys in `text`. One list per view: `↑↓ move   Enter select   Ctrl-Q quit` in the buttons view and `←→ move   Enter select   Esc back   Ctrl-Q quit` in the languages view

## Motion
One tick every 50 ms (20 frames a second) while the home is open. Nothing else redraws, and other screens still wait for a key.
- **Title wave**: the gradient slides along the title and loops with no seam, one cycle every 2.5 seconds
- **Typing demo**: one snippet at a time. It types 1 char every 2 ticks, holds the full snippet for 2 seconds, erases 1 char per tick, then goes to the next snippet and loops. The first snippet is always `hello, ingrain`, and the others are short and funny or motivational. Each snippet is at most 3 lines and 40 columns, so it fits the card
- The animation pauses while a key is held, because the tick only advances when no key arrives

## Short terminals
- Below about 24 rows the demo card is not drawn and the rest keeps its places
- Nothing needs more than 46 columns

## Open
- The demo language in the buttons view should be the last language studied. It is Rust until the progress file exists (`docs/progress-flow.md`)
- A logo or mascot on the cards. Embedded images (`ratatui-image`) were considered and left for the web mirror: terminal support is uneven and the half-block fallback is unreadable at 3 rows
- More than 3 or 4 languages need a wrap or a scroll in the cards row
- The wide layout (about 80 columns, the demo on the right) and the width where it switches
- A bigger title on wide terminals (`tui-big-text`)
- More effects (a fade-in on startup, transitions between screens) with `tachyonfx`, a ratatui effects library. Check that its version supports `ratatui` 0.30, and keep the hand-made wave unless many effects are wanted
