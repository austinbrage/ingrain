# Content Structure

How the course content is stored. The hierarchy itself is in `course-structure.md`.

## Principles
- Content is pre-generated and the same for everyone. There are no users
- Content is read-only at runtime and embedded in the binary
- Progress is separate from content (see below)
- The JSON is shared with the future web app, so it knows nothing about ratatui
- The real content and the content of the tests are separate folders, and the code picks one with `Source::Prod` or `Source::Test` (`src/content/mod.rs`)

## Folders
```
content/
  prod/                what ships in the binary
    rust/
      catalog.json
      syntax/
        course.json
        loops.json
      structs-and-enums/
        course.json
        structs.json
        enums.json
    go/
      catalog.json
  test/                a small frozen content for the end-to-end tests
    rust/
      catalog.json
      basics/
        course.json
        one.json
        many.json
      ...
    go/
      catalog.json
```
- Both folders are embedded at compile time, and `build.rs` rebuilds when anything in `content/` changes
- The paths in the rest of this file are inside `content/prod` (or `content/test`)
- `content/test` has the same shape and the same rules as `content/prod`, with only what the tests need. It grows with the tests in `tests/`, so do not edit it for a content change. Tests that start with a saved progress read it from `tests/fixtures/progress/`

## Files
| File | Holds | Loaded |
|---|---|---|
| `<language>/catalog.json` | language name, course sections, course list | at startup |
| `<language>/<course>/course.json` | module sections and modules (name, tier, session count) | when the course is selected |
| `<language>/<course>/<module>.json` | sessions and exercises | when a session of the module starts |

- The folder name is the course id and the file name is the module id
- The module ids in `course.json` must match the file names
- A course folder always has a `course.json`

## Sessions in the explorer
- Enter on a module opens its first session that is not done, at the exercise where the user stopped (the first one if there is none). If the module has a finished session and more than one session, it opens a sessions column instead (see `docs/ui/02-explorer.md`), so the user can pick the session
- Sessions are named `Session 1`, `Session 2`... from their position, so a session has no name in the JSON for now
- `course.json` keeps the `sessions` count per module: the `done/total` numbers on the module rows, and the rule for a complete module, need it
- A module that is listed in `course.json` but has no `<module>.json` yet is skipped when the app looks for the next module to study

## Module file: typing
`<module>.json` holds `sessions`, each with `exercises`. An exercise has a `type`, which is `"typing"` or `"quiz"` today (`"fill-blank"` and `"find-error"` are planned, see `docs/todos/08-new-exercise-type.md`). The type names the **input style**: typing, picking an option, typing into gaps, picking a line. The shape below is for `"typing"`: the user types code that is shown. The quiz is in the next section.

| Field | Where | Meaning |
|---|---|---|
| `id` | exercise | slug, unique inside its session. The full path is `<language>/<course>/<module>/<session>/<exercise>` |
| `title`, `guide` | exercise | the title and the task text shown in the guide |
| `chunks` | exercise | pieces of the file, in writing order |
| `output` | exercise | optional: `command` and the `text` it prints, shown when the exercise ends |
| `mode` | chunk | `ghost` (explanation in the guide, dimmed code in the editor), `copy` (code in the guide, only typed text in the editor) or `auto` (the app inserts the chunk by itself and the user types nothing; for boilerplate the user already knows, like repeated imports; needs no `guide`) |
| `line` | chunk | line where the chunk's first line goes, counted in the file as it is when the chunk starts |
| `text` | chunk | balanced code: a block opened is closed inside the same chunk |
| `guide` | chunk | optional: replaces the exercise guide for this chunk. Ghost chunks need it, because they show an explanation |
| `notes` | chunk | optional list of `{ "on", "text" }`: explanations shown while typing. `on` is a piece of the chunk's own text, copied from it, that must occur exactly once. The note appears when the user reaches the start of it, and disappears with the chunk. `text` is plain text, no markdown: it shows as a hint row under the anchor in the editor, so write one short sentence (about 60 characters, two lines at most) that does not repeat the anchor's name |

- The `step` card shows the chunk `guide` and, for a copy chunk, the code to copy, and it should never need to scroll: keep the chunk guide short, and copy chunks short with short lines (the limits are open, see `docs/todos/00-exercise-view.md` todo 7)
- Both `guide` fields are markdown, limited to a small subset so the terminal and the web render them the same:
  - paragraphs, separated by a blank line (`\n\n` in the JSON string)
  - **bold** and `inline code`
  - bullet lists (`- item`)
  - not allowed: headings (the screen has its own labels), tables, images, links and fenced code blocks (code goes in chunks)
- **Mode by tier** (a guideline for authors and for the content skill; the tier of a module section sets how much help the user gets):
  - **basic:** about 90% ghost. New code is always ghost. Boilerplate the user already typed (imports, a repeated function skeleton) is `auto`, and copy is used sparingly for identical logic the user already typed, to introduce the mode gently
  - when a chunk is `auto`, the guide of the next typed chunk should say what was added ("the imports are already in the file")
  - **medium:** about half ghost and half copy. Copy for code seen before, ghost for new code
  - **advanced:** mostly copy, ghost only for what is new
- **Code width.** The code is shown in cards and in the editor, whose width depends on the terminal, so write it as if formatted for a narrow screen:
  - aim for 40 columns or less per line, and never go over 50
  - break long lines the way `rustfmt` does with a small `max_width`: one argument per line, a closure body on its own lines, a long signature split over lines
  - for example `terminal.draw(|frame| render(frame, presses))?;` becomes `terminal.draw(|frame| {` then `render(frame, presses);` then `})?;`
  - the app wraps what still does not fit, but a wrapped line is harder to read and to type, so the author should avoid relying on it
- Inserting every chunk at its `line`, in order, must give a file that compiles. `content/prod/*/syntax/loops.json` can be checked this way
- Chunks have no names; the wide `CHUNKS` list shows the mode only

## Module file: quiz
A `"quiz"` exercise shows code and the user picks one option. It sits in the same `exercises` list as the typing ones, and `deny_unknown_fields` applies. Its screen is `docs/ui/exercise/08-quiz.md`.

| Field | Meaning |
|---|---|
| `id`, `title`, `guide` | as in a typing exercise. The `guide` is markdown (the subset below) and says how to answer |
| `kind` | optional: `purpose`, `output`, `compiler` or `error`. It names what is asked and sets the badge of the options card; any other value fails at load. `purpose` also gets a wider options card |
| `code` | the code shown, highlighted and read only, one string with `\n` |
| `options` | the choices, between 2 and 4, all different |
| `answer` | the index of the right option in `options`, counted from 0 |
| `explanation` | markdown (the same subset): why the answer is right. It shows after the user checks |
| `output` | optional: `command` and the `text` it prints, the same type as in a typing exercise |

- **`kind` and its options:**
  - `purpose`: "what does this code do?". Each option says what the code is **for** ("Adds up all the prices in a list"), never what it prints
  - `output`: "what does it print?". Each option is an output
  - `compiler`: the code is a compiler message and each option is its cause. The look is not built yet (the message shows as plain code)
  - `error`: "does this code have an error?". The options are the kinds of error
  - without `kind`, the exercise is a plain quiz and the title says what it asks
- **Options:** at most about 50 characters each, so an option takes two rows at most in the options card. Wrong options must be plausible: a mistake someone could really make reading the code, not a joke or an obvious absurdity. Vary the index of the right one (`answer`) across a module, not always the first
- **Code width:** the same rule as typing (40 columns or less, never over 50), and short: the code window does not scroll while the user chooses, so it should fit about 14 lines
- **`output` is checked by running:** when it is there, the author runs the code once while writing the exercise. The same for the `answer` of an `output` quiz
- No score, and no retry inside the exercise. A wrong answer is saved in the `failed` list of its session, and the failed exercises can be retaken (`docs/progress-flow.md`)
- The rules about options and `answer` are not checked when the module loads, because a panic there would take the app down with its own data. They belong to the content validation (see Open)

## Ids
- Slugs (`syntax`, `borrow-checker`, `loops`), never array positions
- An id is never renamed or reused, because progress points at it. This holds for courses, modules, sessions and exercises
- Names can change freely

## Progress
Stored outside the repo, one small file per machine, in the OS data directory (found with the `directories` crate). On macOS:
```
~/Library/Application Support/ingrain/progress.json
```
`INGRAIN_DATA_DIR` overrides the folder during development (`make dev:fresh` uses it). The file is created the first time something is saved.
```json
{
  "last": { "language": "rust", "course": "ratatui" },
  "modules": [
    { "id": "rust/syntax/loops", "done": ["session-1"], "in_progress": {}, "failed": {}, "practiced": 1760400000 },
    { "id": "rust/ratatui/first-frame", "done": ["session-1"], "in_progress": { "session-2": "frame-area" }, "failed": { "session-1": ["draw-block"] }, "practiced": 1760403600 }
  ]
}
```
| Field | Means |
|---|---|
| `last` | the language and course the user practiced last. It feeds the Continue button |
| `modules[].id` | a module as `<language>/<course>/<module>`, built from the content ids |
| `modules[].done` | the session ids of that module the user finished |
| `modules[].in_progress` | a session started but not finished, and the id of the next exercise to do in it |
| `modules[].failed` | for each session, the exercise ids answered wrong and not passed since (only quizzes fail). A file without it reads as nothing failed |
| `modules[].practiced` | the time (Unix seconds) of the last exercise finished in the module. It feeds the review |

- A module has an entry only once it has something saved
- `modules` is ordered: a module moves to the end when one of its sessions is newly finished, so the list ends with the module finished last
- Counts like `1/4` are the sessions of a module in `done` and not in `failed`, over the `sessions` number in `course.json`
- A saved id that no longer exists in the content is ignored. A corrupt file is renamed to `progress.json.bak` and a fresh one starts
- How it is written and read, step by step: `docs/progress-flow.md`

## Growing into a database
One file is one partition, so each screen reads one item group:
- `catalog.json` is `LANG#rust`
- `course.json` is `COURSE#rust#syntax`
- `<module>.json` is `MODULE#rust#syntax#loops`

A later import script walks the folders and writes the items. The path of the folder is already the key.

## Open
- `tier` in `course.json` is a free string, not validated. Check the allowed values (and the possible `expert` tier, see `course-structure.md`) once tiers drive behavior
- Exercise JSON shape for the types still to come (`fill-blank`, `find-error`); the typing and quiz shapes are above
- A content validation that runs before shipping (the tiers, the anchors of the notes, the quiz rules: 2 to 4 different options, `answer` inside them, options of about 50 characters). It is a test or script over `content/prod/`, not a check at load. See `docs/todos/03-content-skill.md`, "The validator"
- Content `version` field, so changed exercises do not invalidate saved progress
