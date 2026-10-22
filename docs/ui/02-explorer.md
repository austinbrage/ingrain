# Explorer

Status: Draft

The course tree as Miller columns, in the home's frame: the four thick corners around the whole screen, and three columns inside, separated by a thin line. One screen drawn at four depths (course sections, courses, modules, sessions). The drilldown mode (one level per screen) has its own drafts, `03` to `05`.

The courses depth, with the courses column focused and the modules of Syntax as the preview:

```
┏━━━━━━━                                                                                    ━━━━━━━┓
┃ Ingrain                                                                              Rust › Core ┃
┃                                                                                                  ┃
   RUST                          │ CORE                          │ SYNTAX                           
                                 │                               │                                  
   ▸ Core                      › │ ▌ Syntax                    › │   FOUNDATIONS            basic   
     Ecosystem                 › │   Structs and enums         › │     Variables              4/4   
                                 │   Borrow checker            › │     Conditionals           3/3   
                                 │   Error handling            › │     Loops                  1/4   
                                 │   Concurrency               › │                                  
                                 │   Async                     › │   DATA TYPES             basic   
                                 │                               │     Structs                0/4   
                                 │                               │     Enums                  0/3   
                                 │                               │     Pattern matching       0/5   
                                 │                               │                                  
                                 │                               │   ABSTRACTIONS          medium   
                                 │                               │     Traits                 0/5   
                                 │                               │     Generics               0/4   
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
┃                                                                                                  ┃
┃                                                                                                  ┃
┗━━━━━━━                      ↑↓ move   → open   ← back   Ctrl-Q quit                       ━━━━━━━┛
```

The modules depth, with the modules column focused and Loops selected:

```
┏━━━━━━━                                                                                    ━━━━━━━┓
┃ Ingrain                                                                     Rust › Core › Syntax ┃
┃                                                                                                  ┃
   RUST                          │ CORE                          │ SYNTAX                           
                                 │                               │                                  
   ▸ Core                      › │ ▸ Syntax                    › │   FOUNDATIONS            basic   
     Ecosystem                 › │   Structs and enums         › │     Variables              4/4   
                                 │   Borrow checker            › │     Conditionals           3/3   
                                 │   Error handling            › │ ▌   Loops                  1/4   
                                 │   Concurrency               › │                                  
                                 │   Async                     › │   DATA TYPES             basic   
                                 │                               │     Structs                0/4   
                                 │                               │     Enums                  0/3   
                                 │                               │     Pattern matching       0/5   
                                 │                               │                                  
                                 │                               │   ABSTRACTIONS          medium   
                                 │                               │     Traits                 0/5   
                                 │                               │     Generics               0/4   
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
┃                                                                                                  ┃
┃                                                                                                  ┃
┗━━━━━━━                    ↑↓ move   Enter start   ← back   Ctrl-Q quit                    ━━━━━━━┛
```

Drawn at 100x24. The sketch shows the selected row of the focused column as `▌` (a `surface1` fill with bold `sky` text) and the selected row of an unfocused column as `▸` (`sky` text, no fill).

## Frame
- Four thick corners (`┏ ┓ ┗ ┛`) in `overlay0`, the same as the home, with the same arm rule: 1/12 of the width (2 to 8 columns), 1/8 of the height (at least 2 rows). The corners do not touch each other: the `┃` shows only on the 3 rows of each arm, and the sides between the arms are empty
- No box around the body: the corners are the whole frame
- Row 1: `Ingrain` on the left, the breadcrumb on the right. Row 2 is empty. The body starts on row 3
- The hint sits on the last row, between the bottom corners, centered
- Content is 2 columns in from the screen edges

## Columns
- Three columns of equal width. At 100 wide each one takes 32 columns, and the first two give their last column to the divider
- A thin `│` in `overlay0` between columns. It runs from the title row to the last list row and touches neither the header, the hint nor the corners
- Each column: a title row (the parent of the list), one blank row, then the list
- A row has the text on the left and, on the right edge, a chevron `›`, a count (`4/4`) or a tier (`basic`)
- The third column shows the preview of the selected item's children. At the sections depth the selected section's courses, at the courses depth the selected course's modules. At the sessions depth it is the sessions of the selected module, in place of the modules
- The modules column groups the modules under section labels (`FOUNDATIONS`, with its tier on the right), with a blank row between groups

## Breadcrumb and hint
- **Breadcrumb:** the path down to the focused column: `Rust` at the sections depth, `Rust › Core` at the courses depth and `Rust › Core › Syntax` at the modules depth and `Rust › Core › Syntax › Variables` at the sessions depth
- **Hint:** the same style as the home, keys in `text` and labels in `overlay1`. `↑↓ move   → open   ← back   Ctrl-Q quit` at the sections and courses depths, and `↑↓ move   Enter start   ← back   Ctrl-Q quit` at the modules and sessions depths
- Esc and ← at the first column go back to the home, with that language's card selected

## Color
Catppuccin mocha, like the home. Every language uses `sky`.
- **Rows**: `subtext0`. The right-hand text (chevron, count, tier) takes the color of its row
- **Selected row, focused column**: bold `sky` text on a `surface1` fill. The `crust` text on a `sky` fill of the home cards was tried and dropped as too bright next to the rows
- **Selected row, unfocused column**: `sky` text with `▸`, no fill. The path taken
- **Column titles**: bold `text`. The focused column's title has the wave (see Motion)
- **Dividers and corners**: `overlay0`
- **Header**: `Ingrain` in `text`, the breadcrumb in `overlay1`
- **Hint**: `overlay1`, with the keys in `text`

## Motion
The explorer ticks every 50 ms, like the home, and the tick only advances when no key arrives.
- **Title wave**: the focused column's title takes the home's gradient (`blue` → `sky` → `teal`), sliding along the letters, one cycle every 2.5 seconds
- **Cascade**: when Enter on a language card opens the explorer, the three columns show their rows from the top, 2 rows per tick, so the longest column takes about 0.35 seconds. The corners, the header, the titles and the hint are there from the first frame. It does not replay when coming back from an exercise

## Sessions
Sessions are a fourth level. They appear only when the module has a session completed and more than one session (see `docs/progress-flow.md`): any other module opens the exercise directly. The sessions column **replaces the modules column**, so the sections and courses columns stay where they are, with their selected row marked `▸`, and the focus is on the sessions column. The sketch shows the module Variables, with its first session done and the second one as the next (its right side is typed, so it shows `next█`). A first version slid the window one column left instead, and it was dropped because the eye had to follow every column:

```
┏━━━━━━━                                                                                    ━━━━━━━┓
┃ Ingrain                                                         Rust › Core › Syntax › Variables ┃
┃                                                                                                  ┃
   RUST                          │ CORE                          │ VARIABLES                        
                                 │                               │                                  
   ▸ Core                      › │ ▸ Syntax                    › │   Session 1                  ✓   
     Ecosystem                 › │   Structs and enums         › │ ▌ Session 2              next█   
                                 │   Borrow checker            › │   Session 3                  ○   
                                 │   Error handling            › │   Session 4                  ○   
                                 │   Concurrency               › │                                  
                                 │   Async                     › │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
┃                                                                                                  ┃
┃                                                                                                  ┃
┗━━━━━━━                    ↑↓ move   Enter start   ← back   Ctrl-Q quit                    ━━━━━━━┛
```

| Depth | Column 1 | Column 2 | Column 3 |
|---|---|---|---|
| Sections | sections | courses (preview) | modules (preview) |
| Courses | sections | courses | modules (preview) |
| Modules | sections | courses | modules |
| Sessions | sections | courses | sessions (replaces modules) |

- The sessions column shows `Session N`, with `✓` in `green` on the completed sessions and `○` on the rest. The first session not done is the next one, in the accent color with the typed `next`, like the next module
- The title of the column is the module, and the breadcrumb adds it too (`Rust › Core › Syntax › Loops`). The hint is the modules one

## Open
- Fill or a bold `sky` bar for the focused row. The fill is built, the bar was not tried
- The right-hand text and the section labels are not dimmer than the rows. If the list looks flat they can be dimmed with a row built by hand
- The breadcrumb and the hint labels are `overlay1`, dimmer than the rows. Decide with the home whether they should be `subtext0`
- The header as the gradient title on wide terminals
- Below about 90 columns the drilldown mode takes over (`docs/ui-structure.md`). Until it exists the layout breaks on narrow terminals
