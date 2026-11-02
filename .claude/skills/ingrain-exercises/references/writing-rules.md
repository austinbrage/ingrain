# Writing rules for ingrain-exercises

What a test cannot check. The schema, the limits and the ids are in `docs/content-structure.md`, and this file does not copy them. The mechanical rules (ids, session counts, the card limits, quiz options, chunk lines, note anchors) are checked by `cargo test --test content` (`tests/content.rs`). Everything below is judgment, and it is yours to get right.

## The idea behind the content
- **Each course assumes the user knows nothing about its own concept, but knows everything outside it.** The ratatui course assumes nothing about the crate, but assumes the user knows Rust and general programming (loops, variables, `?`, closures, `if let`). Guides and notes never teach what is outside the course concept
- Write every guide and note in your own words, never in the author's phrasing
- The user learns by typing the code, so the code is the lesson. Text only adds what the code does not show

## Modes
**Ghost is the majority at every tier,** around 90% of the chunks of a typing exercise. The rest is `auto` and `copy`, and what decides between them is how many times the user has already written the piece (`docs/content-structure.md`, "The mode of a chunk"):
- **ghost:** new code, and code the user has written only a few times
- **`auto`:** a piece the user has written many times (imports, a repeated function skeleton, boilerplate). Writing it again teaches nothing
- **`copy`:** a piece the user has already written, when you want to challenge them more. Use it sparingly

When a chunk is `auto`, the guide of the next typed chunk says what was added ("the imports are already in the file").

The clearest case is the line every Go program starts with, `package main`: it is `auto` in every module, except in the first module of the first course of the language, where it is ghost so the user types it once. Any piece the user has already written many times follows the same reasoning.

## Go code
- **Indent with 4 spaces, never tabs.** `gofmt` uses tabs, but the editor of the exercise types a Tab key as 4 spaces and fills the indentation of a new line with spaces, so a tab in a chunk can never be typed correctly. That `gofmt` would reformat the code to tabs is fine (the author, 2026-11-02): the goal is to learn by seeing, typing and debugging the code
- A typing exercise is a whole `main.go`: `package main`, the imports and `func main() {` ... `}` come first, as one `auto` chunk (ghost in the first module of the first course), and the other chunks go inside `main`, each at its `line`
- Print with `fmt.Println` while the course has not taught formatting. The `output` command is `go run main.go`

## Guides
- **As short as possible,** copy chunks most of all, because their code takes the card. A guide adds context the code does not show. It never states the obvious and never describes the formatting of the code
  - bad: "Two imports. End with an empty line"
  - good: "Import the main pieces the app needs from ratatui"
- The limits are in rows of the narrow card (`docs/content-structure.md`, "The step card"). A guide that needs more is two chunks, or a smaller chunk
- Copy chunks are short: a few lines, each short, ideally not wider than the card so they do not wrap

## Notes
A note is a hint under a piece of code, shown while the user types it.
- **Only where the user needs one.** Many chunks need none. A piece of the course's own API that does something non-obvious (like `ratatui::run`) is worth one
- **Plain text, one short sentence,** about 60 characters. No markdown, no backticks. It does not repeat the name of its anchor
- **Written for the user's goal:** what the piece does for the app, never where it comes from or how the library is organized. If the user would ask "what is this and why do I need it?", the note answers that in a few words
  - bad: "ratatui's own copy of the crossterm crate, so you never add it to Cargo.toml"
  - good: "Lets the app read key presses"
- The `on` anchor is copied from the chunk's text and occurs in it exactly once. Check it again after any change to the code

## Quizzes
- The shape and the limits are in `docs/content-structure.md` ("Module file: quiz")
- A `purpose` option says what the code is **for**, never what it prints (printed results are the `output` kind)
- The wrong options are plausible mistakes someone could really make reading the code, never jokes or absurdities. None of them is also right
- The right answer is not always the first or the longest. Vary the `answer` index and the `kind` across a module
- The code is short, uses a feature of the module's concept and not general programming, and has no comments that give the answer away
- The `explanation` says why the answer is right, in a sentence or two, and does not repeat the option
- **Never run the code.** The `answer` of an `output` quiz, and the `output` of a typing exercise, are what you can predict by reading the program. Keep every program deterministic, so the output is certain: no randomness, no current time, no map iteration order, no goroutines racing for the output. If you cannot be sure of an output, change the program

## Intros
- **As much as the concept needs, not a fixed number.** An easy concept (conditionals) gets a short intro, a single slide, or none at all. A hard concept (concurrency) gets longer texts, more slides and snippets. Judge it from the course and the module
- One idea per slide, in the markdown subset of the guide. The limits are in `docs/content-structure.md` ("Intro")
- The module intro is an overview of what the module covers. A session intro teaches what its exercises practice
- The text teaches. It never gives away the answer of an exercise that follows
- **Pseudo code is welcome in an intro snippet,** especially for a new concept: it shows the idea before the syntax. Say in the text that it is pseudo code. A snippet that is real code uses real syntax and APIs, and compiles
- A snippet only when the slide is about code. It is illustrative: it may use placeholder names to show a shape (`pattern_a => do_this`), but every piece of syntax and every API in it is real, and a snippet meant as real code compiles. It uses only what the course has taught or what the slide itself introduces
- The snippet does not repeat the code of an exercise, so there is still something to type or to find

## Changing a module that exists
When the user gives a request together with an existing module:
1. Read the whole module file, and its entry in `course.json` (its tier and its session count)
2. Change only what the request asks. The rest stays exactly as it is, the same ids and the same text, even if you would have written it differently
3. **Never rename or reuse an id.** The saved progress points at the ids of sessions and exercises: a renamed one loses what the user did, and a reused one points at the wrong exercise. A new exercise or session gets a new id. Names and texts change freely
4. Keep the session count of `course.json` in step. Adding or removing a session changes it, so say so
5. Say what you changed, and what you left alone that the request touched

## Before you finish
Run `cargo test --test content` and fix everything it reports. A pass means the mechanical rules hold, not that the content is good. Read your own module once more for what a test cannot see:
- the API and the syntax are real and used correctly, in the language of the course
- the explanations are accurate and in a good order
- trace each program and each output by hand once: the code compiles when it is meant to, and the output you wrote is exactly what it prints. You do not run it
- nothing teaches what the course assumes the user already knows
