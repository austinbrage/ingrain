# Explorer

Status: Draft

The course tree as Miller columns, in the home's frame: the four thick corners around the whole screen, and three columns inside, separated by a thin line. One screen drawn at four depths (layers, courses, modules, sessions). The drilldown mode (one level per screen) has its own drafts, `03` to `05`.

The courses depth, with the courses column focused and the modules of Syntax as the preview. The courses of Core are in three course groups:

```
┏━━━━━━━                                                                                    ━━━━━━━┓
┃ Ingrain                                                                              Rust › Core ┃
┃                                                                                                  ┃
   RUST                          │ CORE                          │ SYNTAX                           
                                 │                               │                                  
   ▸ Core                      › │   FUNDAMENTALS                │   FOUNDATIONS            basic   
     Ecosystem                 › │ ▌   Syntax                  › │     Variables              4/4   
                                 │     Structs and enums       › │     Conditionals           3/3   
                                 │                               │     Loops                  1/4   
                                 │   OWNERSHIP                   │                                  
                                 │     Borrow checker          › │   DATA TYPES             basic   
                                 │     Error handling          › │     Structs                0/4   
                                 │                               │     Enums                  0/3   
                                 │   CONCURRENCY                 │     Pattern matching       0/5   
                                 │     Concurrency             › │                                  
                                 │     Async                   › │   ABSTRACTIONS          medium   
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
   ▸ Core                      › │   FUNDAMENTALS                │   FOUNDATIONS            basic   
     Ecosystem                 › │ ▸   Syntax                  › │     Variables              4/4   
                                 │     Structs and enums       › │     Conditionals           3/3   
                                 │                               │ ▌   Loops                  1/4   
                                 │   OWNERSHIP                   │                                  
                                 │     Borrow checker          › │   DATA TYPES             basic   
                                 │     Error handling          › │     Structs                0/4   
                                 │                               │     Enums                  0/3   
                                 │   CONCURRENCY                 │     Pattern matching       0/5   
                                 │     Concurrency             › │                                  
                                 │     Async                   › │   ABSTRACTIONS          medium   
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
- The third column shows the preview of the selected item's children. At the layers depth the selected layer's courses, at the courses depth the selected course's modules. At the sessions depth it is the sessions of the selected module, in place of the modules
- The modules column groups the modules under module section labels (`FOUNDATIONS`, with its tier on the right), with a blank row between groups
- The courses column groups the courses under course group labels (`FUNDAMENTALS`) in the same way: a blank row between groups, and the courses indented under their label. A course group label has only its name: no tier, no count, no right-hand text, and the courses keep their `›`. The layers column has no groups
- The labels of both columns are rows that cannot be selected: ↑↓ skips them and the blank rows. The breadcrumb does not name the course group (`Rust › Core › Syntax`), because it only labels and holds no state of its own

## Breadcrumb and hint
- **Breadcrumb:** the path down to the focused column: `Rust` at the layers depth, `Rust › Core` at the courses depth and `Rust › Core › Syntax` at the modules depth and `Rust › Core › Syntax › Variables` at the sessions depth
- **Hint:** the same style as the home, keys in `text` and labels in `overlay1`. `↑↓ move   → open   ← back   Ctrl-Q quit` at the layers and courses depths, and `↑↓ move   Enter start   ← back   Ctrl-Q quit` at the modules and sessions depths
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
Sessions are a fourth level. They appear only when the module has a session completed and more than one session (see `docs/progress-flow.md`): any other module opens the exercise directly. The sessions column **replaces the modules column**, so the layers and courses columns stay where they are, with their selected row marked `▸`, and the focus is on the sessions column. The sketch shows the module Variables, with its first session done and the second one as the next (its right side is typed, so it shows `next█`). A first version slid the window one column left instead, and it was dropped because the eye had to follow every column:

```
┏━━━━━━━                                                                                    ━━━━━━━┓
┃ Ingrain                                                         Rust › Core › Syntax › Variables ┃
┃                                                                                                  ┃
   RUST                          │ CORE                          │ VARIABLES                        
                                 │                               │                                  
   ▸ Core                      › │   FUNDAMENTALS                │   Session 1                  ✓   
     Ecosystem                 › │ ▸   Syntax                  › │ ▌ Session 2              next█   
                                 │     Structs and enums       › │   Session 3                  ○   
                                 │                               │   Session 4                  ○   
                                 │   OWNERSHIP                   │                                  
                                 │     Borrow checker          › │                                  
                                 │     Error handling          › │                                  
                                 │                               │                                  
                                 │   CONCURRENCY                 │                                  
                                 │     Concurrency             › │                                  
                                 │     Async                   › │                                  
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
| Layers | layers | courses (preview) | modules (preview) |
| Courses | layers | courses | modules (preview) |
| Modules | layers | courses | modules |
| Sessions | layers | courses | sessions (replaces modules) |

- The sessions column shows `Session N`, with `✓` in `green` on the completed sessions and `○` on the rest. A failed session shows a red `✗` instead (see Failed) The first session not done is the next one, in the accent color with the typed `next`, like the next module
- The title of the column is the module, and the breadcrumb adds it too (`Rust › Core › Syntax › Loops`). The hint is the modules one

## Failed
Status: Approved for coding by the author on 2026-10-24. The rest of the file stays Draft.

A session with an exercise answered wrong is **failed** (`docs/progress-flow.md`). The explorer shows it with a `✗` in red where a passed one has the green `✓`, on both columns. Only quizzes can fail, and an exercise left with Esc is not failed, only not visited.

The modules depth. Conditionals and Loops each have a failed session. The `✗` goes in front of the count, as the `✓` does on a complete module, and the count is the sessions passed, so a failed session is not in it (Loops is `1/4`):

```
┏━━━━━━━                                                                                    ━━━━━━━┓
┃ Ingrain                                                                     Rust › Core › Syntax ┃
┃                                                                                                  ┃
   RUST                          │ CORE                          │ SYNTAX                           
                                 │                               │                                  
   ▸ Core                      › │   FUNDAMENTALS                │   FOUNDATIONS            basic   
     Ecosystem                 › │ ▸   Syntax                  › │     Variables            ✓ 4/4   
                                 │     Structs and enums       › │     Conditionals         ✗ 2/3   
                                 │                               │ ▌   Loops                ✗ 1/4   
                                 │   OWNERSHIP                   │                                  
                                 │     Borrow checker          › │   DATA TYPES             basic   
                                 │     Error handling          › │     Structs                0/4   
                                 │                               │     Enums                  0/3   
                                 │   CONCURRENCY                 │     Pattern matching       0/5   
                                 │     Concurrency             › │                                  
                                 │     Async                   › │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
┃                                                                                                  ┃
┃                                                                                                  ┃
┗━━━━━━━                    ↑↓ move   Enter start   ← back   Ctrl-Q quit                    ━━━━━━━┛
```

The sessions depth, with the failed session selected. The `✗` takes the place of the `✓`. The hint on a failed row says `Enter retake`, because Enter opens only the failed exercises of that session:

```
┏━━━━━━━                                                                                    ━━━━━━━┓
┃ Ingrain                                                         Rust › Core › Syntax › Variables ┃
┃                                                                                                  ┃
   RUST                          │ CORE                          │ VARIABLES                        
                                 │                               │                                  
   ▸ Core                      › │   FUNDAMENTALS                │   Session 1                  ✓   
     Ecosystem                 › │ ▸   Syntax                  › │ ▌ Session 2                  ✗   
                                 │     Structs and enums       › │   Session 3              next█   
                                 │                               │   Session 4                  ○   
                                 │   OWNERSHIP                   │                                  
                                 │     Borrow checker          › │                                  
                                 │     Error handling          › │                                  
                                 │                               │                                  
                                 │   CONCURRENCY                 │                                  
                                 │     Concurrency             › │                                  
                                 │     Async                   › │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
                                 │                               │                                  
┃                                                                                                  ┃
┃                                                                                                  ┃
┗━━━━━━━                   ↑↓ move   Enter retake   ← back   Ctrl-Q quit                    ━━━━━━━┛
```

- **The next marker:** a module with failed sessions that Continue goes to alternates its right side between the typed `next█` and the count, which keeps its `✗` (`✗ 3/4`)
- **A failed session that is also the next one:** a wrong answer in the middle of the first pass of a session makes it failed (see `docs/progress-flow.md`) while it is still the first session not done, so its right side alternates `next█` and `✗`
- **The hint:** `Enter retake` only on a failed row of the sessions column, where Enter is always a retake. On the modules column it stays `Enter start`, because Enter there may open the sessions column first
- **Color:** the `✗` in the mocha `red`, the same as in the recap drafts (`docs/ui/complete/`). The count after it keeps the color of its row

## Open
- Fill or a bold `sky` bar for the focused row. The fill is built, the bar was not tried
- The right-hand text and the module section and course group labels are not dimmer than the rows. If the list looks flat they can be dimmed with a row built by hand
- The breadcrumb and the hint labels are `overlay1`, dimmer than the rows. Decide with the home whether they should be `subtext0`
- The header as the gradient title on wide terminals
- Below about 90 columns the drilldown mode takes over (`docs/ui-structure.md`). Until it exists the layout breaks on narrow terminals
