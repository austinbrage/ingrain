---
name: ingrain-exercises
description: Create or change the exercises of a module of a language in Ingrain, in the module's file: typing exercises, quizzes and the intros before them. Use it for a whole module (language, course, module) or for a single exercise (language, course, module, exercise, prompt).
---

# ingrain-exercises

Writes the exercises of one module, in `content/prod/<language>/<course>/<module>.json`. It is the third of three skills: `ingrain-courses` (the courses of a section), `ingrain-modules` (the modules of a course) and this one. The module must already be in the `course.json` of its course.

Arguments: `$ARGUMENTS` is `<language> <course> <module> [exercise] [prompt]`. It has two modes.

## The two modes
**1. A whole module: `<language> <course> <module>`.** Write all the exercises the concept needs, plus its intros.
- **Never touch an exercise that is already written,** not its text, not its id, not its place. If the file already has exercises, add only what the concept still needs, with new ids, and say so. If the module looks complete, say so as a warning and change nothing

**2. One exercise: `<language> <course> <module> <exercise> [prompt]`.** The `exercise` is its id, a slug.
- **It does not exist:** create it. The prompt, if there is one, says what it must be. Without a prompt, decide from the module. Put it where it makes sense in the session
- **It exists, with a prompt:** change it as the prompt says, and nothing else
- **It exists, without a prompt:** give a **warning** (the exercise is already written and nothing says what to change) and change nothing

**The same idea holds for the other two skills:** a general call, with no request, never touches what already exists. Only a specific, named target with a request can change something that is written.

## Read first
1. `docs/content-structure.md`: the schema of the module file, the typing exercise, the quiz and the intro, with their limits
2. `docs/course-structure.md`: "Tiers"
3. `references/writing-rules.md` next to this file: how to write guides, notes, quizzes and intros
4. The module's entry in the `course.json` of its course (its tier and its session count), and the modules before it in the course: the user has done them
5. The module file, if it exists
6. For the format, an existing module: `content/prod/rust/ratatui/first-frame.json` (typing) and `content/prod/rust/syntax/pattern-matching.json` (quizzes and intros)

## What the exercises must be
- **The user must master the concept.** A module has as many exercises as that takes, from a few to many. There is no fixed number: decide from the course and the module
- **Combine every type of exercise when possible,** and do not force it: typing exercises, and quizzes of every kind (`purpose`, `output`, `compiler`, `error`). Alternate them. For Go, favor `compiler` and `error` quizzes, because its compiler errors teach a lot
- **Typing exercises are short and focused:** about 3 to 5 chunks and 10 to 15 lines of code. If a concept is long, write **several typing exercises, each on one specific thing**, and not one big one
- **Ghost is the majority,** around 90% of the chunks. Use `auto` for a piece the user has already written many times, and `copy` to challenge them more (`references/writing-rules.md`, "Modes")
- **One file,** the same in every exercise: the editor shows `main.go` for Go and `main.rs` for Rust. Each typing exercise builds a complete, runnable program in it. In Go, `package main` is an `auto` chunk, except in the first module of the first course of the language, where it is ghost so the user types it once
- **The output is predicted, never run.** Do not run any code. Fill `output` (the `command` and the `text` it prints) from what you can work out by reading the program, so keep every program deterministic: no randomness, no current time, no map iteration order, no goroutines racing for the output. If you cannot be sure of an output, change the program
- **Do not repeat what other exercises already teach.** Before you create an exercise, read the titles, the guides and the code of the other exercises of the module, and of the modules before it. Each exercise is about one specific thing that none of them covers. When you write a whole module, check its exercises against each other the same way, so no two practice the same point
- **Use only what the user has learned:** the concept of the module, and what the modules before it taught. An exercise never needs something that comes later in the course
- **Intros are as long as the concept is hard.** An easy concept (conditionals) gets a short intro, one slide, or none. A hard concept (concurrency) gets longer texts, more slides and snippets, and pseudo code in a snippet is welcome for a new concept. A module intro says what the user is about to see, and the session intro explains how the concept works in the abstract and what the short guides cannot (`references/writing-rules.md`, "Intros")
- **Sessions:** group the exercises in sessions of about 4 to 6. A module that needs more gets more sessions (`session-2`, ...), and you update its `sessions` number in the course's `course.json` to match, and say so

## Writing
- Ids are slugs, unique inside their session. The tier of the module only says how hard the concept is: it does not change the shape of the exercises
- Follow `references/writing-rules.md` for the guides, the notes, the quizzes and the intros, and `docs/content-structure.md` for the limits (the card limits, the quiz limits, the intro limits)
- Keep the file in the shape of the existing modules: the same fields, the same order

## Changing what exists: be careful
- **Never rename or reuse an id** of a session or an exercise: the saved progress points at them
- **Never remove an exercise** unless the prompt names it and says to. Say what you removed
- Change only what the prompt asks, and leave everything else exactly as it is
- If the prompt would change the session count, say so and update `course.json`

## Finish
Run `cargo test --test content` and fix everything it reports. It checks the ids, the session count, the card limits, the quiz rules, the intro limits and the chunks (the `line` places, the ghost guides, the balanced code and the note anchors).

A pass does not mean the content is good. Read each program as a compiler would, and trace each output by hand once.

Then report:
- what you wrote or changed, exercise by exercise, and what you left as it was
- every warning: an exercise that exists and had no prompt, a module that looked complete
- any session count you changed in `course.json`
