# Intro

Status: Approved

The screen the user sees before the first exercise of a module or of a session that has an `intro` in its content (built, 2026-10-29). It teaches a concept in one or more slides, each with a text and an optional snippet, and Enter goes to the next slide. The rules for when it shows are under "Decisions": on a fresh start, never when a half-finished session is resumed, and never on a retake.

The look is the one of the module screen (`docs/ui/complete/03-module.md`), chosen from `07-intro-alternatives.md` (option B): a caption with the sliding gradient, the intro title as a colored line, the dots of the slides under it, a centered column of text and, when the slide has one, a small window with the snippet. It keeps the boxed header and footer of the end screens. **Every part of this draft is a proposal** until the author sets Approved. The drawings are at 100 columns and cannot show colors, bold, inline code or the motion of the caption.

## A session intro, slide 1 of 3, with a snippet
The header shows the position of the session on the right. The footer says `Enter next`, and there is no `← back` on the first slide. A slide with a snippet needs about 28 rows; the rest of the slides need less.

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
                                                                                                    
                           ╭ Rust ──────────────────────────────────────╮                           
                           │  match age {                               │                           
                           │      0..=17 => "minor",                    │                           
                           │      18..=64 => "adult",                   │                           
                           │      _ => "senior",                        │                           
                           │  }                                         │                           
                           ╰────────────────────────────────────────────╯                           
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter next                                                                Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Slide 2 of 3, only text
A slide without a snippet is the same screen with no window. `← back` is in the footer from the second slide on.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Pattern matching                                                     session 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                           ──  intro  ──                                            
                                                                                                    
                                        The match expression                                        
                                              ○  ●  ○                                               
                                                                                                    
               Every possible value must be covered. If one is missing, the code does               
               not compile.                                                                         
                                                                                                    
               A pattern can also bind the value to a name, and the names live inside               
               the arm.                                                                             
                                                                                                    
                                                                                                    
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter next                                                       ← back   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## The last slide
The footer says `Enter start` when the first exercise follows, and `Enter next` when a session intro follows a module intro.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Pattern matching                                                     session 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                           ──  intro  ──                                            
                                                                                                    
                                        The match expression                                        
                                              ○  ○  ●                                               
                                                                                                    
               Try it in the exercises that follow: the first one asks what a `match`               
               prints.                                                                              
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter start                                                      ← back   Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## A module intro
The same screen. The header has nothing on the right, because a module has no position. This module intro has two slides and its own indicator; when the user passes it, the session intro starts again at its first slide.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Pattern matching                                                                 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
                                           ──  intro  ──                                            
                                                                                                    
                                          Pattern matching                                          
                                                ●  ○                                                
                                                                                                    
               This module is about match and the patterns it uses. By its end you                  
               will read and write patterns without looking them up.                                
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Enter next                                                                Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## Narrow
The same parts in the same order, with the text column at the width of the terminal. It is drawn now and built later.

```
╔════════════════════════════════════════════════════╗
║ Rust · Syntax · Pattern matching       session 1/3 ║
╚════════════════════════════════════════════════════╝
                                                      
                    ──  intro  ──                     
                                                      
                 The match expression                 
                       ●  ○  ○                        
                                                      
  A match compares a value against patterns and runs  
  the first one that fits.                            
                                                      
  Every possible value must be covered. If one is     
  missing, the code does not compile.                 
                                                      
  • _ matches anything left                           
  • a pattern can bind the value to a name            
                                                      
    ╭ Rust ──────────────────────────────────────╮    
    │  match age {                               │    
    │      0..=17 => "minor",                    │    
    │      18..=64 => "adult",                   │    
    │      _ => "senior",                        │    
    │  }                                         │    
    ╰────────────────────────────────────────────╯    
                                                      
╔════════════════════════════════════════════════════╗
║ Enter next                  Esc back   Ctrl-Q quit ║
╚════════════════════════════════════════════════════╝
```

## Parts
- **Caption:** `──  intro  ──`, centered, with the sliding gradient of the module screen (`wave_line`)
- **Title:** the intro `title`, centered, in bold `yellow` like the module name of the module screen
- **Indicator:** the dots of the slides under the title, `●` for the current one and `○` for the others. One dot per slide of this intro, so the module intro and the session intro each have their own
- **Text:** the `text` of the slide in a centered column of at most 70 columns, left-aligned inside it. The markdown subset of the exercise guide: paragraphs, **bold**, `inline code` and bullet lists (`•`). It is the only part that scrolls, with Up and Down, when it overflows the space between the pinned top and the snippet
- **Snippet window:** a small read-only window under the text, 46 columns wide, with the course language as its label (`Rust`) and the code highlighted. Up to 50 columns of code and about 14 lines. Only on the slides that have a `code`. It sits under the last line of the text, after a margin of 2 rows. When the text and the window do not fit, the window is pinned at the bottom and the text scrolls above it, so the code is always visible
- **Pinned top:** the caption, the title and the dots never scroll, so the user always sees where they are in the intro
- **Footer:** `Enter next` or `Enter start` on the left, and on the right `← back` (from the second slide), `Esc back` and `Ctrl-Q quit`, with `↑↓ scroll` before them when the slide overflows
- **Header:** the breadcrumb on the left, `session 1/3` on the right for a session intro and nothing for a module intro

## Keys
| Key | Does |
| --- | --- |
| Enter | the next slide; on the last one, on: the session intro after a module intro, or the first exercise |
| Left | the previous slide |
| Up, Down | scroll the text of the slide (the caption, title, dots and snippet stay), when it overflows |
| Esc | back to the explorer, from any slide |

## Borders and colors
| Part | Border | Color |
| --- | --- | --- |
| Header and footer | double (`╔═╗`) | `overlay0` |
| Caption | none | the gradient of the home title (`blue`, `sky`, `teal`), sliding |
| Title | none | bold `yellow` |
| Current dot `●` | none | `sky` |
| Other dots `○` | none | `overlay0` |
| Text | none | default text color; **bold** in bold; `inline code` in the code color of the guide |
| Bullets `•` | none | `sky` |
| Snippet window | rounded (`╭─╮`) | `sky`, with the language as its label |
| Enter in the footer | none | bold, with `Enter` in `sky` |

## Motion
The caption slides its gradient on the tick, as on the module screen (`app.tick`). The tick is not restarted: the gradient has no first frame to start from. Nothing else moves. Changing slides is instant.

## Decisions
Taken with the author on 2026-10-28:
1. **The model:** an optional `intro` on a module and on a session, with a `title` and a list of `slides`. Each slide has a `text` (markdown) and an optional `code`. It is a field and not an exercise, so it never counts in the header position, `done`, `in_progress` or `failed`. It is called `intro` because a quiz already has an `explanation`. The shape and the limits are in `docs/content-structure.md`, "Intro"
2. **When it shows:** on a fresh start, derived from progress, with no `seen` fact. The exercise is the first of its session and it is not a retake. The module intro shows only at the first exercise of the first session, and then the session intro follows. Resuming a half-finished session and retaking never show it. A finished session or module started over shows it again. Quitting on it saves nothing
3. **No way to see it again** in the first version. A `Ctrl` key on the exercise screen, drawn as an overlay so the typed chunk is kept, was weighed and left for later
4. **The text:** the same markdown subset and renderer as the exercise `guide` (paragraphs, **bold**, `inline code`, bullet lists). Headings and tables are left for a version 2. A slide is about 8 lines, and the text scrolls when it overflows
5. **The snippet:** one optional `code` per slide, in the language of the course, with the width rule of the quiz code (40 columns or less, never over 50, about 14 lines). A small read-only window, no `notes`
6. **The layout** is alternative B of `07-intro-alternatives.md`. The snippet sits under the last line of the text after a margin of 2 rows. Centering the text and the window together was tried and dropped, because the text moved between slides
7. **The keys:** Enter and Right go to the next slide, Left goes back, Up and Down scroll, Esc leaves to the explorer from any slide. Right never leaves the intro, only Enter does
8. **The code:** `app/intro.rs` (`IntroState`, `applies`), `ui/intro/` and `ExerciseState::start`. The dev screen is `make intro`

## Open
- The caption word: `──  intro  ──` is built, but it could be more specific (`new concept`, `before you start`)
- The dots are shown with a single slide too (one dot). Hide them there, or leave them
- The header on the right for a session intro: whether it should also name its module
- Left on the first slide of a session intro does nothing; it does not go back to the last slide of the module intro
- With a snippet and a very short terminal, the text area can get very small; a minimum height for it, or the snippet scrolling away too
