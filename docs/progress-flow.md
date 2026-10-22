# Progress Flow

How Ingrain remembers what the user has done, and how the Continue button on the Home uses it to bring them back to the same point. This describes what is built today. When a new piece of progress lands (the explorer counts, the `next` marker, the sessions column), add it here. The progress todo was finished and removed; its open questions are at the end of this file. The next structural work is in `todos/05-deep-refactor.md`.

## The idea in one minute
- The user types exercises. Every time they **open** one or **finish** one, the app writes a small file, `progress.json`
- The file says which course they are practicing, which sessions they finished, and which exercise they were about to do in a session left half done
- When the app starts, it reads the file. If there is something to continue, the Home shows `Continue   Rust · Syntax`. Pressing Enter on it opens the exact exercise where the user stopped

Two halves, so it is easy to follow: **writing** (while the user practices) and **reading** (when they press Continue).

## What is stored
One JSON file in the OS data directory (on macOS `~/Library/Application Support/ingrain/progress.json`). For development, `INGRAIN_DATA_DIR=/tmp/ingrain-dev` points it somewhere else, so real progress is never touched. `make dev:fresh` does it for you: it deletes `/tmp/ingrain-new` and runs the app with that folder, so every run starts as a new user.

```json
{
  "last": { "language": "rust", "course": "ratatui" },
  "modules": [
    { "id": "rust/syntax/loops", "done": ["session-1", "session-2"], "in_progress": {}, "practiced": 1760400000 },
    { "id": "rust/ratatui/first-frame", "done": ["session-1"], "in_progress": { "session-2": "frame-area" }, "practiced": 1760403600 }
  ]
}
```

| Field | Means |
|---|---|
| `last` | the language and the course the user practiced last. Nothing more: not the module, not the session |
| `modules[].id` | a module, as `language/course/module`. Module ids alone are not unique, so the path is used |
| `modules[].done` | the sessions of that module the user finished |
| `modules[].in_progress` | a session that was started but not finished, and the id of the **next exercise** to do in it |
| `modules[].practiced` | the time (Unix seconds) the user last finished an exercise in that module, 0 if unknown. `Progress` stamps it itself in `complete_exercise` |

- Everything points at content **ids**, never positions, so reordering the content does not break saved progress
- A module has an entry only once it has something saved. An untouched module is not in the file
- `modules` is ordered: when a session is finished, its module moves to the end. So the list always ends with the module finished last. Continue relies on that
- That order is not the same as `practiced`: re-practicing a module only updates its time and never reorders the list, so "the module finished last" stays true

The code lives in `src/progress/`:

| File | Holds |
|---|---|
| `progress/mod.rs` | the data (`Progress`, `Last`, `ModuleProgress`), load and save, the small questions about the file, and the two Continue entry points (`continue_label`, `continue_at`) |
| `progress/resume.rs` | the decisions: which module, which session and which exercise come next |
| `progress/storage.rs` | the disk: the data folder, reading the file and writing it safely |

Around it, two app files help:

| File | Holds |
|---|---|
| `app/content.rs` | `Content`: the loaded courses of every language, and the questions about a position in them (`catalog`, `course`, `module`, `course_id`, `module_path`) |
| `app/mod.rs` | `App`: the screens and keys, and nothing else |
| `app/home.rs` | the Continue button: thin methods that forward to `Progress` (`continue_label`, `continue_target`) |

The rule of thumb: **`Progress` decides, `App` only translates.** `Progress` never imports `app/` types (`HomeLanguage`, `ExplorerState`), so it answers with plain numbers and `App` turns them into a screen. The content has its own small lookups (`Catalog::course_index`, `Module::session_index`, `Session::exercise_index`) that find an id's position, so nobody searches by hand.

## Writing: what happens while the user practices

```
open an exercise ─────────────► remember_course ──► set_last ──► save
finish an exercise ───────────► record_completion ─► complete_exercise ─► remember_course ─► save
```

All of it is in `src/app/exercise/mod.rs`.

### Opening an exercise
`ExerciseState::open` loads the module file and then calls `remember_course`, which writes the **language and course** into `last` and saves. This runs for every opened exercise, including the next one reached from the Complete screen.

### Finishing an exercise
An exercise ends when the last character of a typing exercise is typed (`handle_typing_key`), or when Enter is pressed on a quiz that was already checked (`handle_quiz_action`). Either one builds the Complete screen and, just before switching to it, calls `record_completion`. It has to run while the screen is still the exercise, because it reads the exercise from there. It only needs the id of the next exercise (`Exercise::id()`), so progress works the same for every exercise type.

A quiz is saved as finished whatever the answer: a wrong answer is not tracked yet. That, the cross in the explorer and the retake of only the failed exercises are in `todos/09-failed-exercises.md`.

`record_completion` does three things:
1. `Content::module_path` builds the module id (`rust/ratatui/first-frame`) from the explorer position
2. `Progress::complete_exercise` records the result:
   - the finished exercise was **not** the last of its session: `in_progress` saves the id of the next exercise
   - it **was** the last: the session leaves `in_progress`, goes to `done` (once, never twice) and the module moves to the end of `modules`
3. `remember_course` updates `last`, and everything is saved

So, to answer "what is recorded": the language and the course go in `last`. The module, the sessions and the exercise live inside `modules`.

## Reading: what happens when the user presses Continue

```
app starts ──► continue_label ──► Home shows "Continue   Rust · Syntax"
Enter ──► HomeState::continue_target ──► Progress::continue_at ──► ExerciseState::open
```

### Does the Continue button exist?
`App::new()` asks `App::continue_label()`. It finds the language of `last`, picks that language's loaded library, and asks `Progress::continue_label(library)`, which answers with a `ContinueLabel`. The label travels `app/home.rs` → `ui/mod.rs` → `ui/home/mod.rs` → `ui/home/buttons.rs`. There are three answers:
- **`Course`**: `Rust · Syntax`, the course Continue will open. The button reads `Continue   Rust · Syntax`. It can be a different course than `last`: once a course has nothing left to study, the label names the next course that has
- **`Review`**: nothing is left to study in the whole language. The button reads `Rust completed · Let's review`, with dots that come and go slowly (a few ticks each, in a fixed width so the text does not shift). Enter opens the review (see below)
- **none** (a new user, or `last` names a course that no longer exists): no Continue button, the home starts on `Choose a language`, and Up and Down do nothing

### Pressing Enter on Continue
`handle_buttons_key` (`app/home.rs`) calls `continue_target` (also in `app/home.rs`). It does only two things:

1. **Ask `Progress`.** `Progress::continue_at(library)` works out everything and answers with a `ContinueAt`: five plain numbers (section, course, module, session, exercise). In the review state it answers with the module to review
2. **Translate.** `HomeState` copies the numbers into an `ExplorerState` and calls `ExerciseState::open`, which gives the exercise screen

The explorer is set to the module list of the course, so Esc from the exercise lands there.

### Enter on a module row in the explorer
The explorer works the same way, for the module the user picked instead of the one Continue picks. `handle_modules_key` (`app/explorer.rs`) gets the module path and the module file from `Content`, asks `Progress::resume_point` for the session and exercise, and calls `ExerciseState::open`. So a module left in the middle opens at the next session, at the exercise where the user stopped, and not at the first exercise.

This opening directly is only for modules with no finished session, or with a single session. Otherwise the user picks the session (next section).

### The sessions column
When the module has a finished session **and more than one session**, Enter does not open an exercise: it opens a sessions column, so the user can retake a finished session or skip ahead. The column replaces the modules column, so the sections and courses do not move.
- `Depth::Sessions` and a `session` index are part of `ExplorerState`. The cursor starts on the first session not done
- The column shows `Session 1`, `Session 2`..., from the position, with a green `✓` on the finished ones and a `○` on the rest. The first session not done is the next one, with its name in the accent color and its right side typed like the next module
- The finished flags come from `ModuleMarks.sessions`, filled by `App::module_marks` through `Progress::sessions_done`. The module file is read on every frame the column is open and nothing is cached: it is embedded and small, so the flags can never be stale
- Enter on a session opens it at the exercise saved for it, with `Progress::resume_exercise` (the same rule `resume_point` uses). Esc from the exercise lands back on the column
- Left and Esc go back to the modules
### The language of the Home demo
The typing demo on the Home types snippets of one language. `HomeState::language(last)` (`app/home.rs`) decides which:
- with the language cards open, the card under the cursor, so moving to Go shows Go
- with the buttons showing, the language of `last`, so a user who practiced Go sees Go snippets (colored with the Go grammar, and `main.go` in the title bar). Without `last`, Rust

`ui/mod.rs` asks it with `state.language(app.last_language())` and passes the answer to `home`, which hands it to the demo. It is read on every frame, so the demo follows the progress without any extra state.

### The marks on the module rows
The explorer shows how far the user is in each module and which one comes next. On every frame, `ui/mod.rs` asks `App::module_marks` for the course on screen. It answers with a `ModuleMarks` (defined in `app/explorer.rs`, together with `module_marks`):
- `done`: one number per module, in course order, from `Progress::done_counts`: how many of its sessions are in `done`
- `next`: the index of the module to study next, from `Progress::next_module`: the same function Continue uses, so the two always agree

- `sessions`: only while the sessions column is open, which sessions of the selected module are done, from `Progress::sessions_done`

The explorer then draws each row:
- `done/total` (the total comes from `course.json`), in green with a `✓` when the two are equal
- the next module has its name in the accent color, and its right side is typed like the Home demo: the word `next` appears letter by letter with a block cursor, stays for a moment and is erased, then the count is typed and erased the same way, and the loop repeats. The cursor never disappears between the two words, and each word sits where its static text would. The explorer redraws on a tick, so the animation needs no extra state. The speeds are three constants at the top of `ui/explorer/columns/modules.rs` (`TYPE_TICKS`, `ERASE_TICKS`, `HOLD_TICKS`)

`next_module` skips modules whose file is not written yet. Once every module of a course is complete, or every unfinished one has no file, there is no next module and nothing is marked.

### How `continue_at` decides
It first picks the course with `next_course`, then the module in it with `next_module`, and then asks `resume_point` for the session and exercise. `resume_point` calls two small functions. All of them live in `progress/resume.rs`, one rule each:

| Function | Rule |
|---|---|
| `next_course` | start at the course of `last`: if it has a module left to study, that is the one. If not, take the first course after it (in catalog order, across the sections) that has one, and wrap around to the start. Courses without a `course.json` are skipped. If no course has anything left, there is none, and the language is complete |
| `next_module` | take the module finished most recently in this course, and go to the first incomplete module after it. If there is none after it, go back to the first incomplete one. If none was finished yet, the first module. If all are complete, there is none |
| `next_session` | the first session of the module that is not in `done` |
| `next_exercise` | the exercise saved in `in_progress` for that session, or the first exercise if there is none (or if its id no longer exists) |

A module is complete when the number of its sessions in `done` equals the `sessions` number in `course.json`.

### The review
When no course has anything left to study, `next_course` finds nothing and the button turns into `Rust completed · Let's review`. Pressing it does not need a separate path: `continue_at` falls back to `next_review`, and the rest of the steps are the same.

`next_review` goes through every written module of the language and picks the one with the oldest `practiced` time. Modules saved before the field existed have 0, so they come first, and ties go to catalog order. The module opens at its first session and its first exercise, as a completed module has nothing in progress.

Reviewing is practice like any other: finishing an exercise stamps that module's `practiced` with the current time, so the next review picks the next-oldest module and the user cycles through everything. It never duplicates a finished session and never reorders `modules`.

## Example
The user finishes session 1 of `first-frame`, opens session 2, finishes its first exercise, and quits.

- the file now has `first-frame` with `done: ["session-1"]` and `in_progress: { "session-2": "frame-area" }`, and `last` is `ratatui`
- next day the Home shows `Continue   Rust · ratatui`
- Enter: `next_module` gives `first-frame` (not complete), `next_session` gives `session-2`, `next_exercise` gives `frame-area`. The user lands on that exercise

## Where each piece is

| Piece | File | Function |
|---|---|---|
| save the course on open | `app/exercise/mod.rs` | `ExerciseState::open`, `remember_course` |
| save a finished exercise | `app/exercise/mod.rs` | `record_completion` |
| ids of a position | `app/content.rs` | `course_id`, `module_path` |
| the label and the button | `app/home.rs`, `progress/mod.rs`, `ui/home/buttons.rs` | `continue_label`, `Progress::continue_label`, `ContinueLabel`, `render_menu`, `review_text` |
| marks on module rows | `app/explorer.rs`, `progress/mod.rs`, `ui/explorer/columns/modules.rs` | `module_marks`, `Progress::done_counts`, `Progress::next_module`, `modules_column` |
| Enter on Continue | `app/home.rs` | `handle_buttons_key`, `continue_target`, `last_language` |
| Enter on a module row | `app/explorer.rs`, `progress/mod.rs` | `handle_modules_key`, `resume_point` |
| the sessions column | `app/explorer.rs`, `progress/mod.rs`, `ui/explorer/columns/sessions.rs` | `handle_sessions_key`, `module_marks`, `Progress::sessions_done`, `Progress::resume_exercise`, `sessions_column` |
| the file | `progress/mod.rs`, `progress/storage.rs` | `load`, `save`, `read`, `write`, `set_last`, `complete_exercise` |
| the decisions | `progress/mod.rs`, `progress/resume.rs` | `continue_at`, `resume_point`, `next_course`, `next_module`, `next_session`, `next_exercise`, `next_review` |
| finding an id's position | `content/` | `Catalog::course_index`, `Module::session_index`, `Session::exercise_index` |

## Not built yet
- Failed exercises (a wrong quiz answer, an exercise left unfinished), and retaking only those: `todos/09-failed-exercises.md`
- Session names (`Session N (name)`) were dropped: the sessions column is too narrow for them

## Open
- `last` holds one course for the whole app. Should it be one per language?
- A way to reset progress (a command or a setting)
- What happens when `course.json` says a module has N sessions but the module file has a different number (the content check should catch it)
- Module ids are path strings (`rust/syntax/loops`), built in `Progress::continue_at`, in `Progress::done_counts`, in `next_review` and in `Content::module_path`. Storing `language`, `course` and `module` as three fields would remove the building and parsing
- Name cleanups proposed, not applied: `continue_at` to `continue_point` (and `ContinueAt` to `ContinuePoint`), `set_last` to `set_last_course`, the "Continuing" section to "Where to continue"
- The breadcrumbs still live in `App` and `Content`; they are strings for the UI and could move to `ui/`
