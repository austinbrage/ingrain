# Intro · Alternatives

Status: Draft

Other looks for the intro screen (`06-intro.md`), which mirrors the exercise layout: two cards side by side. These five are different in structure, not only in color. All use the same sample (slide 1 of 3 of the session intro of pattern matching) and the same content model, now with **slides** (see "Slides and the indicator" below). The chrome is the one of the end screens. The drawings are at 100 columns and cannot show colors or motion. Pick one, mix two, or say what is missing; the chosen one becomes `06-intro.md`.

## Slides and the indicator
An intro has one or more slides, passed with Enter. Every drawing below shows slide 1 of 3, so each carries an indicator, and the footer says `Enter next` (the last slide says `Enter start`, or `Enter next` when a session intro follows a module intro). The indicator has three possible looks; the drawings use the first:

```
dots      ●  ○  ○
counter   1/3
bar       [▓▓▓▓░░░░░░░░] 1/3
```
- **Dots** are the lightest and read at a glance, but past about 8 slides they stop being countable
- **Counter** is exact and short, and it fits any corner
- **Bar** is the bar of the exercise footer and of the end screens, so it is already part of the app, but it is wide

The keys: Enter goes to the next slide (and on the last one, on), Left goes back a slide (a hint `← back` shows from slide 2), Esc leaves the intro to the explorer from any slide, and Up and Down scroll a slide whose text overflows (the guideline is that a slide fits, so this is the fallback). The last slide of a session intro, in the layout of E, looks like this:

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Pattern matching                                                     session 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                                                                                    
                                        The match expression                                        
                                                                                                    
                                              ○  ○  ●                                               
                                                                                                    
                                Try it in the exercises that follow.                                
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter start                                                      ← back   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## A · The page
No cards at all: a column of text like a README, with the title underlined, the snippet as a block with a bar on its left, and the dots at the right end of the title row. Needs about 25 rows.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Pattern matching                                                     session 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
               The match expression                                           ●  ○  ○               
               ──────────────────────────────────────────────────────────────────────               
                                                                                                    
               A match compares a value against patterns and runs the first one that                
               fits.                                                                                
                                                                                                    
               Every possible value must be covered. If one is missing, the code does               
               not compile.                                                                         
                                                                                                    
               • _ matches anything left                                                            
               • a pattern can bind the value to a name                                             
                                                                                                    
               ▎ match age {                                                                        
               ▎     0..=17 => "minor",                                                             
               ▎     18..=64 => "adult",                                                            
               ▎     _ => "senior",                                                                 
               ▎ }                                                                                  
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter next                                                                Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```
- **For:** the least boxed screen of the app, quiet and easy to read; each slide can be any length, because it is one scrolling column
- **Against:** nothing frames the screen, so it can feel empty on a wide terminal; the snippet has no window, so it looks less like the code of the exercises

## B · The family of the end screens
The look of the module screen (`complete/03-module.md`): a caption with the sliding gradient, the title as a bold colored line, the dots under it, a centered column of text and the snippet in a small window. The caption plays the part of the big word, because an intro title has any length. Needs about 28 rows.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Pattern matching                                                     session 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                           ──  intro  ──                                            
                                                                                                    
                                        The match expression                                        
                                              ●  ○  ○                                               
                                                                                                    
               A match compares a value against patterns and runs the first one that                
               fits.                                                                                
                                                                                                    
               Every possible value must be covered. If one is missing, the code does               
               not compile.                                                                         
                                                                                                    
               • _ matches anything left                                                            
               • a pattern can bind the value to a name                                             
                                                                                                    
                          ╭──────────────────────────────────────────────╮                          
                          │  match age {                                 │                          
                          │      0..=17 => "minor",                      │                          
                          │      18..=64 => "adult",                     │                          
                          │      _ => "senior",                          │                          
                          │  }                                           │                          
                          ╰──────────────────────────────────────────────╯                          
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter next                                                                Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```
- **For:** it matches the end screens, so the app has one visual language for "a screen between exercises"; it can animate with the same helpers (`wave_line`)
- **Against:** the tallest of the five; it looks like a celebration, and an intro is not one

## C · One frame, the code on top
A single card with the title as its badge and the dots on the right of the top border. The snippet is the figure on top, a rule, and the text under it in two columns. It uses the full width and needs about 25 rows.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Pattern matching                                                     session 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭ The match expression ────────────────────────────────────────────────────────────────── ●  ○  ○ ─╮
│                                                                                                  │
│   match age {                                                                                    │
│       0..=17 => "minor",                                                                         │
│       18..=64 => "adult",                                                                        │
│       _ => "senior",                                                                             │
│   }                                                                                              │
│                                                                                                  │
├──────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                  │
│   A match compares a value against patterns     │  • _ matches anything left                     │
│   and runs the first one that fits.             │  • a pattern can bind the value to a name      │
│                                                 │                                                │
│   Every possible value must be covered. If one  │                                                │
│   is missing, the code does not compile.        │                                                │
│                                                                                                  │
╰──────────────────────────────────────────────────────────────────────────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter next                                                                Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```
- **For:** the snippet comes first, so the user sees the code before reading about it; the wide screen is used
- **Against:** two columns of text are hard to wrap, and a slide with no snippet leaves the top empty, so it would be a different layout

## D · The terminal story
Left-aligned, like a command that prints: a prompt line with the intro title and the counter at its right end, the text, and the snippet with a bar. The text **types itself** when a slide opens, as the home demo does, and Enter finishes the typing before it moves on; the cursor stays at the end. The drawing is the final frame. Needs about 22 rows.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Pattern matching                                                     session 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
    ❯ intro "The match expression"                                                           1/3    
                                                                                                    
    A match compares a value against patterns and runs the first one that fits.                     
                                                                                                    
    Every possible value must be covered. If one is missing, the code does not compile.             
                                                                                                    
    • _ matches anything left                                                                       
    • a pattern can bind the value to a name                                                        
                                                                                                    
    │ match age {                                                                                   
    │     0..=17 => "minor",                                                                        
    │     18..=64 => "adult",                                                                       
    │     _ => "senior",                                                                            
    │ }█                                                                                            
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter next                                                                Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```
- **For:** it is the most Ingrain: a terminal that prints, using the typing motion the app already has
- **Against:** the animation hides text at the start (tests would `wait`), and a user who wants to read fast must skip it on every slide

## E · One idea per slide, large
The slide is only a short text, centered, with the title above and the dots under it. It is the layout that fits slides best, where each slide is one idea; the snippet shows in a code window under the text on the slides that have one.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Pattern matching                                                     session 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                                                                                    
                                        The match expression                                        
                                                                                                    
                                              ●  ○  ○                                               
                                                                                                    
                      Every possible value must be covered. If one is missing,                      
                                     the code does not compile.                                     
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter next                                                                Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```
- **For:** nothing to scroll, one idea at a time, the calmest of the five
- **Against:** it wants very short slides, so an author who writes long ones gets a scroll; with a snippet it becomes B

## Which one
**A and D** need the least new drawing code (one column, no cards), and D adds the typing motion. **C** is the only one that cuts the screen in zones. **B** is the biggest screen, and **E** works best when every slide is one short idea. I lean to **A with the typing of D**, with the dots on the title row, but that is the author's call.
