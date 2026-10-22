# UI Structure

How the screens map to code. Sketches live in `docs/ui/`; the course hierarchy is in `course-structure.md`.

## Rule
A `.rs` file in `src/ui/` is a screen. A draft in `docs/ui/` is one state of a screen, so several drafts can belong to one file.

## Flow
Home → Explorer → Exercise

The home has two views: the buttons and the language cards, so there is no separate languages screen.

## Screens
| Draft | File | Notes |
|---|---|---|
| `01-home.md` | `ui/home/` | splash: gradient title with a wave, tagline, typing demo. Two views: the buttons, or one card per language. The Continue button has three states: hidden for a new user, a course (`Continue   Rust · Syntax`), and a finished language (`Rust completed · Let's review`, with slow dots). The demo types the language of the last course practiced |
| `02-explorer.md` | `ui/explorer/` | one screen drawn at three depths (course sections, courses, modules), in the home's corner frame. The module rows show progress: `done/total`, a green `✓` when complete, and the next module in the accent color with a typed `next`. At the sessions depth, a fourth level, the sessions column replaces the modules column (`✓` and `○`, and the next session typed like the next module) |
| `exercise/01` to `04` | `ui/exercise/` (`mod.rs` header and footer, `typing/mod.rs` the 2:1 layout, `typing/guide/` the two guide cards and the copy box, `typing/editor/` the window, the code rows and the hints) | the definitive look in two layouts and two chunk modes: `03` ghost wide (built), `04` copy wide (built), `01` and `02` narrow (decided, not built). Boxed header and footer, the editor window and the guide cards flush to the screen edges. See `docs/todos/00-exercise-view.md` |
| `exercise/08-quiz` | `ui/exercise/quiz/` (`mod.rs` the layout by `kind` and the guide card, `code.rs` the code window, `options.rs` the options card with its wrapping, markers and scroll) | the quiz exercise: the code in a read only window on the left, the guide card (the explanation once answered) and the options card on the right. The right column is 2:1 of the code, 3:2 wider for the `purpose` kind. It repeats a few pieces of `typing/guide/` and `typing/editor/` on purpose (the guide card, the rule, the language label, the title bar), to merge in `todos/05-deep-refactor.md` (6.7 and 6.8) |
| `exercise/06-guide-alternative` | none | the guide as one card with a dim rule, the version before the two cards. Kept with its code |
| `exercise/07-inline-notes` | `ui/exercise/typing/editor/hints.rs` | the notes as hints in the editor, and the alternative of the notes in the guide |
| `exercise/05-complete` | `ui/complete/` (`mod.rs` header, title block, Enter row, layout and footer, `code.rs` the finished file as an editor window with numbered, wrapped rows and the scroll, `output.rs` the output card) | the screen after the last chunk, with the same chrome as the exercise screen: boxed header and footer, cards flush to the edges, the cards fill the body down to the footer. Three outcomes (next exercise, next session, finish) change the summary line and the Enter row. The output card is as wide as its longest line (24 to half the area); with no output, only the code card, centered. See `docs/ui/exercise/05-complete.md` |

## Explorer
- Named after the content (the course tree), not the mechanism (`navigator` was rejected)
- Two view modes share one state, the path through the tree (one selected index per level), and the same keys
  - **Columns (default):** Miller columns, three levels visible, preview of the selected item's children. One screen drawn at different depths. Built first
  - **Drilldown:** a mode, not a screen. One level per screen, so it contains three screens with different looks: course sections (buttons), courses (list with progress), modules (stacked sections). Taken from the drafts in `docs/ui/` (`03` to `05`). Built later
- The language is not a column: it shows in the breadcrumb and as the accent color
- The mode is a setting: `auto` (columns when wide enough, drilldown when narrow), `columns` or `drilldown`

### Files
`ui/explorer/` is a folder. Today it holds the columns mode:

| File | Draws |
|---|---|
| `explorer/mod.rs` | shell: corners, header, body, hint, and the call to `render_columns` |
| `explorer/corners.rs` | the four thick corners, a copy of the home's |
| `explorer/hint.rs` | the hint line, one key list per depth |
| `explorer/columns/mod.rs` | `render_columns` (builds, lays out and renders the three columns, with the sessions column in place of the modules column at its depth), `render_single_column`, `Column` and `Item`, the title wave and the cascade, and the typed `next` (`typed_text`) that the modules and sessions columns share |
| `explorer/columns/sections.rs` | builds the sections column |
| `explorer/columns/courses.rs` | builds the courses column |
| `explorer/columns/modules.rs` | builds the modules column |
| `explorer/columns/sessions.rs` | builds the sessions column of the selected module |

A new level adds a file under `columns/`. When drilldown arrives, `mod.rs` picks the mode with a `match` between `render_columns` and the drilldown files. Drilldown adds one file per level:

| File | Draws |
|---|---|
| `explorer/drilldown_sections.rs` | drilldown: course sections screen |
| `explorer/drilldown_courses.rs` | drilldown: courses screen |
| `explorer/drilldown_modules.rs` | drilldown: modules screen |

A new level or mode adds a file instead of growing one.

## App state
One file per screen in `src/app/`, with the screen's state type and its key handler together:

| File | Holds |
|---|---|
| `app/mod.rs` | `App` (current screen, tick, quit flag, `Progress` and `Content`), `Screen`, and `handle_key`, which only routes |
| `app/content.rs` | `Content`: the loaded courses of every language and the questions about a position (`catalog`, `course`, `module`, `course_id`, `module_path`, `breadcrumb`) |
| `app/input.rs` | `Action` and `to_action` (arrows and `hjkl` move, Enter and Tab select, Esc goes back), and `TypingKey` and `to_typing_key` for the exercise screen. The only place that knows keys |
| `app/home.rs` | `HomeState` (buttons or language cards), `HomeButton`, `HomeLanguage`, `handle_home_key`, and the Continue button: `continue_label`, `continue_target`, `last_language`, which forward to `Progress` |
| `app/explorer.rs` | `ExplorerState`, `Depth`, `ModuleMarks` (what the explorer marks on the module rows), one handler per depth, `module_marks`, `explorer_breadcrumb`, `step` |
| `app/exercise/` | `mod.rs`: `ExerciseState`, `Body` (one variant per exercise type), `ExerciseState::open`, the two handlers (`handle_typing_key` for raw keys, `handle_quiz_action` for actions) and `record_completion`, which saves progress when an exercise ends. `typing.rs`: `TypingState`. `quiz.rs`: `QuizState` (the selected option and whether it was checked) |
| `app/complete.rs` | `CompleteState` (built by `new` for a typing exercise and by `from_quiz` for a quiz, with the footer bar as `bar_done`, `bar_total` and `bar_unit`), `Next` (what Enter does on the complete screen) and its key handler. A quiz will leave this screen, see `todos/11-complete-screens.md` |
| `src/dev.rs` (outside `app/`) | opens a screen from command line arguments, for development. Called by `main.rs` |

- `Screen::Explorer(ExplorerState)` carries the language and the path (depth, section, course)
- `mod.rs` re-exports the types, so `ui/` imports them from `crate::app`
- `Screen` is not `Copy`: the exercise and complete screens own their data (the loaded module, the typed code)
- `App` owns one `Progress` and one `Content`. Both are plain data with methods: `Progress` never imports `app/` types, so it answers with plain numbers and `App` translates them into screens. `ui/` reads progress only through the `App` methods above
- The router (`App::handle_key`) picks the key mapping by the body of the exercise: a typing exercise skips `to_action` (the user types `h j k l` as code) and uses `to_typing_key`, while a quiz uses `to_action`

## Exercise types
A new exercise type follows the same steps, and the compiler lists what is missing because the `match`es have no `_` arm:
1. `src/content/module/<type>.rs` with the content struct, a variant of `Exercise` (the `type` key in the JSON) and its arm in `Exercise::id()`
2. A state struct in `src/app/exercise/<type>.rs`, a variant of `Body`, its arm in `ExerciseState::open` and its handler. The router adds its arm to choose the keys
3. A view folder `src/ui/exercise/<type>/` (its `mod.rs` has the render function, and the big parts get their own files, as `typing/` and `quiz/` do), called from the one `match` on the exercise and the body in `ui/exercise/mod.rs`, which also sets the footer bar and hint
4. Progress needs nothing: it works by exercise ids (`Exercise::id()`)
5. A draft in `docs/ui/exercise/`, approved first, and the end-to-end tests in `tests/exercise.rs` (a `mod` per type), with sample content in `content/test`

## Progress
The progress code lives outside `app/` and `ui/`, in `src/progress/`:

| File | Holds |
|---|---|
| `progress/mod.rs` | `Progress` (`last` and `modules`), load and save, the small questions about the file, `continue_label`, `continue_at`, `resume_point`, `done_counts`, `next_module`, and the writes (`complete_exercise`, `set_last`) |
| `progress/storage.rs` | the disk: the data folder, reading the file (a broken one is kept as `.bak`) and writing it safely |
| `progress/resume.rs` | the decisions as free functions over `&Progress`: `next_course`, `next_module`, `next_session`, `next_exercise`, `next_review` |

The data shape is in `content-structure.md` and the full walkthrough of how it is written and read is in `progress-flow.md`.

## Code conventions
- Functions that draw start with `render_` (`render_button`, `render_card`)
- A visual state is a small enum picked with a `match` (`ButtonState`)
- Draft dimensions: 56x24 for the single-column screens, 100x24 for the columns

## Pending
- Merge the drafts into one md per screen, so docs and Rust map one to one
