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
    { "id": "rust/syntax/loops", "done": ["session-1", "session-2"], "in_progress": {}, "failed": {}, "practiced": 1760400000 },
    { "id": "rust/ratatui/first-frame", "done": ["session-1"], "in_progress": { "session-2": "frame-area" }, "failed": { "session-1": ["draw-block"] }, "practiced": 1760403600 }
  ]
}
```

| Field | Means |
|---|---|
| `last` | the language and the course the user practiced last. Nothing more: not the module, not the session |
| `modules[].id` | a module, as `language/course/module`. Module ids alone are not unique, so the path is used |
| `modules[].done` | the sessions of that module the user finished |
| `modules[].in_progress` | a session that was started but not finished, and the id of the **next exercise** to do in it |
| `modules[].failed` | for each session, the ids of the exercises answered wrong and not passed since. Only quizzes can fail. A session with an entry is **failed**. `complete_exercise` adds an id on a wrong answer and removes it on a right one, and an entry whose list empties is removed. A file with no `failed` reads as nothing failed |
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
| `app/home.rs` | the Continue button: thin methods that forward to `Progress` (`continue_label`), and `ExplorerState::continue_target` in `app/explorer.rs` turns its answer into a position |

The rule of thumb: **`Progress` decides, `App` only translates.** `Progress` never imports `app/` types (`HomeLanguage`, `ExplorerState`), so it answers with plain numbers and `App` turns them into a screen. The content has its own small lookups (`Catalog::course_index`, `Module::session_index`, `Session::exercise_index`) that find an id's position, so nobody searches by hand.

## Writing: what happens while the user practices

```
open an exercise ─────────────► remember_course ──► set_last ──► save
finish an exercise ───────────► record_completion ─► complete_exercise ─► remember_course ─► save
```

All of it is in `src/app/exercise/mod.rs`.

### Opening an exercise
`ExerciseState::open` loads the module file and then calls `remember_course`, which writes the **language and course** into `last` and saves. This runs for every opened exercise, including the next one reached from an end screen or straight from a quiz.

### Finishing an exercise
An exercise ends when the last character of a typing exercise is typed (`handle_typing_key`), or when Enter is pressed on a quiz that was already checked (`handle_quiz_action`). A typing exercise builds its complete screen and, just before switching to it, calls `record_completion`. A quiz has no complete screen: it calls `record_completion` and goes straight to what follows (the next exercise, the session recap, the module screen or the explorer). `record_completion` has to run while the screen is still the exercise, because it reads the exercise from there. It only needs the id of the next exercise (`Exercise::id()`), so progress works the same for every exercise type.

A quiz is finished whatever the answer, but the answer is remembered. `handle_quiz_action` passes `selected == answer` as `passed`, and typing passes `true`, because a typing exercise cannot fail. An exercise left with Esc writes nothing: it is not failed, only not visited.

`record_completion` does three things:
1. `Content::module_path` builds the module id (`rust/ratatui/first-frame`) from the explorer position
2. `Progress::complete_exercise(module, session, exercise, passed, next_exercise)` records the result:
   - a **wrong** result adds the exercise id to the session's `failed` list (once), and a **right** one removes it, and the session's entry when its list is empty
   - the finished exercise was **not** the last of its session: `in_progress` saves the id of the next exercise
   - it **was** the last: the session leaves `in_progress`, goes to `done` (once, never twice) and the module moves to the end of `modules`. A wrong answer on the last exercise still saves the session as done, because `done` means played through
   - in a **retake** (see below) `record_completion` passes no next exercise, so nothing is saved to `in_progress` for a session that is already done
3. `remember_course` updates `last`, and everything is saved

So, to answer "what is recorded": the language and the course go in `last`. The module, the sessions and the exercise live inside `modules`.

## Reading: what happens when the user presses Continue

```
app starts ──► continue_label ──► Home shows "Continue   Rust · Syntax"
Enter ──► ExplorerState::continue_target ──► Progress::continue_at ──► ExerciseState::open
```

### Does the Continue button exist?
`App::new()` asks `App::continue_label()`. It finds the language of `last`, picks that language's loaded library, and asks `Progress::continue_label(library)`, which answers with a `ContinueLabel`. The label travels `app/home.rs` → `ui/mod.rs` → `ui/home/mod.rs` → `ui/home/buttons.rs`. There are three answers:
- **`Course`**: `Rust · Syntax`, the course Continue will open. The button reads `Continue   Rust · Syntax`. It can be a different course than `last`: once a course has nothing left to study, the label names the next course that has
- **`Review`**: nothing is left to study in the whole language. The button reads `Rust completed · Let's review`, with dots that come and go slowly (a few ticks each, in a fixed width so the text does not shift). Enter opens the review (see below)
- **none** (a new user, or `last` names a course that no longer exists): no Continue button, the home starts on `Choose a language`, and Up and Down do nothing

### Pressing Enter on Continue
`handle_buttons_key` (`app/home.rs`) calls `ExplorerState::continue_target` (`app/explorer.rs`, which the module screen also uses to open the next module). It does only two things:

1. **Ask `Progress`.** `Progress::continue_at(library)` works out everything and answers with a `ContinueAt`: five plain numbers (section, course, module, session, exercise). In the review state it answers with the module to review
2. **Translate.** `ExplorerState::continue_target` copies the numbers into an `ExplorerState`, and the caller calls `ExerciseState::open`, which gives the exercise screen

The explorer is set to the module list of the course, so Esc from the exercise lands there.

### Enter on a module row in the explorer
The explorer works the same way, for the module the user picked instead of the one Continue picks. `handle_modules_key` (`app/explorer.rs`) gets the module path and the module file from `Content`, asks `Progress::resume_point` for the session and exercise, and calls `ExerciseState::open`. So a module left in the middle opens at the next session, at the exercise where the user stopped, and not at the first exercise.

This opening directly is only for modules with no finished session, or with a single session. Otherwise the user picks the session (next section). A module whose only session is failed opens its failed exercise: see "Retaking failed exercises".

### The sessions column
When the module has a finished session **and more than one session**, Enter does not open an exercise: it opens a sessions column, so the user can retake a finished session or skip ahead. The column replaces the modules column, so the sections and courses do not move.
- `Depth::Sessions` and a `session` index are part of `ExplorerState`. The cursor starts on the first session not done
- The column shows `Session 1`, `Session 2`..., from the position, with a green `✓` on the finished ones, a red `✗` on the failed ones and a `○` on the rest. The first session not done is the next one, with its name in the accent color and its right side typed like the next module. A failed session that is also the next one (a wrong answer in the middle of its first pass) alternates `next` and `✗`
- The flags come from `ModuleMarks.sessions` and `ModuleMarks.sessions_failed`, filled by `App::module_marks` through `Progress::sessions_done` and `Progress::sessions_failed`. The module file is read on every frame the column is open and nothing is cached: it is embedded and small, so the flags can never be stale
- Enter on a session opens it at the exercise saved for it, with `Progress::resume_exercise` (the same rule `resume_point` uses). On a failed session that is a retake. Esc from the exercise lands back on the column
- The hint says `Enter retake` instead of `Enter start` on a failed session row, and only there (`ui/explorer/hint.rs`). On the modules column Enter may open the sessions column first, so it stays `Enter start`
- Left and Esc go back to the modules
### The language of the Home demo
The typing demo on the Home types snippets of one language. `HomeState::language(last)` (`app/home.rs`) decides which:
- with the language cards open, the card under the cursor, so moving to Go shows Go
- with the buttons showing, the language of `last`, so a user who practiced Go sees Go snippets (colored with the Go grammar, and `main.go` in the title bar). Without `last`, Rust

`ui/mod.rs` asks it with `state.language(app.last_language())` and passes the answer to `home`, which hands it to the demo. It is read on every frame, so the demo follows the progress without any extra state.

### The marks on the module rows
The explorer shows how far the user is in each module and which one comes next. On every frame, `ui/mod.rs` asks `App::module_marks` for the course on screen. It answers with a `ModuleMarks` (defined in `app/explorer.rs`, together with `module_marks`):
- `passed`: one number per module, in course order, from `Progress::passed_counts`: how many of its sessions are in `done` and not failed
- `failed`: one flag per module, from `Progress::modules_failed`: whether it has a failed session
- `next`: the index of the module to study next, from `Progress::next_module`: the same function Continue uses, so the two always agree
- `sessions` and `sessions_failed`: only while the sessions column is open, which sessions of the selected module are done and which are failed, from `Progress::sessions_done` and `Progress::sessions_failed`

The explorer then draws each row:
- `passed/total` (the total comes from `course.json`), in green with a `✓` when the two are equal. A module with a failed session shows a red `✗` before the count (`✗ 1/4`). Any `✗` in the right-hand text of a column is drawn red (`ui/explorer/columns/mod.rs`)
- the next module has its name in the accent color, and its right side is typed like the Home demo: the word `next` appears letter by letter with a block cursor, stays for a moment and is erased, then the count is typed and erased the same way, and the loop repeats. The cursor never disappears between the two words, and each word sits where its static text would. The explorer redraws on a tick, so the animation needs no extra state. The speeds are three constants at the top of `ui/explorer/columns/modules.rs` (`TYPE_TICKS`, `ERASE_TICKS`, `HOLD_TICKS`)

`next_module` skips modules whose file is not written yet. Once every module of a course is complete, or every unfinished one has no file, there is no next module and nothing is marked.

### How `continue_at` decides
It first picks the course with `next_course`, then the module in it with `next_module`, and then asks `resume_point` for the session and exercise. `resume_point` calls two small functions. All of them live in `progress/resume.rs`, one rule each:

| Function | Rule |
|---|---|
| `next_course` | start at the course of `last`: if it has a module left to study, that is the one. If not, take the first course after it (in catalog order, across the sections) that has one, and wrap around to the start. Courses without a `course.json` are skipped. If no course has anything left, there is none, and the language is complete |
| `next_module` | take the module finished most recently in this course, and go to the first incomplete module after it. If there is none after it, go back to the first incomplete one. If none was finished yet, the first module. If all are complete, there is none |
| `next_session` | the first session of the module that is not in `done`. When all are done, the first one with failed exercises (the retake). When none is failed, the first session |
| `next_exercise` | for a session played through (in `done`) with failed exercises, its first failed exercise in content order (`failed_from`). Otherwise the exercise saved in `in_progress` for that session, or the first exercise if there is none (or if its id no longer exists) |

A module is complete when the number of its sessions in `done` and not failed (`Progress::passed_count`) equals the `sessions` number in `course.json`. So a module with a failed session is not complete: Continue goes back to it, and the `next` marker sits on it.

### Retaking failed exercises
A **retake** replays only the failed exercises of a session that was played through. There is no door of its own: Continue, Enter on a module row and Enter on a session row all end in `resume_point` or `resume_exercise`, so the rule lives once, in `progress/resume.rs`:
- **Forward first, retake last.** Continue goes to the first session never played, or the one in progress. Only when every session of the module was played through and some are failed does it open the first failed session at its first failed exercise, then the next failed session. After that it goes on to the next module (`next_session`, `next_exercise` and `next_module` above)
- **Inside a session in progress** a wrong answer does not pull the user back: `next_exercise` keeps the saved exercise until the session is played through
- **Enter on a failed session row** in the explorer opens its first failed exercise. On a session played through with nothing failed it replays the whole session, as before
- **Content order, content numbers:** the failed exercises come in the order of the content, and the header shows their position in the session (`exercise 2/5`, then `exercise 4/5`)
- **The step after an exercise.** `ExerciseState::open` asks `Progress::is_retake` (a session in `done` with failed exercises) and, in a retake, `Progress::next_failed_exercise` (the first failed exercise after this one, from `failed_from`). It stores `retake` and `next_failed`, and `CompleteState` copies them. In a retake, Enter on the complete screen opens `next_failed`, and after the last failed exercise the retake ends: if the module is played through (`Progress::sessions_done` all true), it shows the module celebration when nothing is failed any more, or the failed module screen when a session still is; if the module is not played through, it goes to the explorer, where the cross is gone. Outside a retake it opens `exercise + 1` as before
- **A retake saves no resume marker:** `record_completion` passes no next exercise, so `in_progress` stays empty
- **A retried quiz** is the same quiz: the same options in the same order, with the selection on the first option

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
| marks on module rows | `app/explorer.rs`, `progress/mod.rs`, `ui/explorer/columns/modules.rs` | `module_marks`, `Progress::passed_counts`, `Progress::modules_failed`, `Progress::next_module`, `modules_column` |
| Enter on Continue | `app/home.rs`, `app/explorer.rs` | `handle_buttons_key`, `ExplorerState::continue_target`, `last_language` |
| Enter on a module row | `app/explorer.rs`, `progress/mod.rs` | `handle_modules_key`, `resume_point` |
| the sessions column | `app/explorer.rs`, `progress/mod.rs`, `ui/explorer/columns/sessions.rs`, `ui/explorer/hint.rs` | `handle_sessions_key`, `module_marks`, `Progress::sessions_done`, `Progress::sessions_failed`, `Progress::resume_exercise`, `sessions_column`, `render_hint` |
| a failed exercise and the retake | `progress/mod.rs`, `progress/resume.rs`, `app/exercise/mod.rs`, `app/complete/` | `complete_exercise`, `is_retake`, `next_failed_exercise`, `failed_from`, `ExerciseState::open`, `record_completion`, `CompleteState::next` |
| the file | `progress/mod.rs`, `progress/storage.rs` | `load`, `save`, `read`, `write`, `set_last`, `complete_exercise` |
| the decisions | `progress/mod.rs`, `progress/resume.rs` | `continue_at`, `resume_point`, `next_course`, `next_module`, `next_session`, `next_exercise`, `failed_from`, `next_review` |
| finding an id's position | `content/` | `Catalog::course_index`, `Module::session_index`, `Session::exercise_index` |

## Not built yet
- Session names (`Session N (name)`) were dropped: the sessions column is too narrow for them

## Open
- `last` holds one course for the whole app. Should it be one per language?
- A way to reset progress (a command or a setting)
- What happens when `course.json` says a module has N sessions but the module file has a different number (the content check should catch it)
- Module ids are path strings (`rust/syntax/loops`), built in `Progress::continue_at`, in `Progress::passed_counts`, in `Progress::modules_failed`, in `next_review` and in `Content::module_path`. Storing `language`, `course` and `module` as three fields would remove the building and parsing
- Name cleanups proposed, not applied: `continue_at` to `continue_point` (and `ContinueAt` to `ContinuePoint`), `set_last` to `set_last_course`, the "Continuing" section to "Where to continue"
- The breadcrumbs still live in `App` and `Content`; they are strings for the UI and could move to `ui/`
