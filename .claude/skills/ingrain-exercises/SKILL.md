---
name: ingrain-exercises
description: Create or change the exercises of a module of a language in Ingrain, in the module's file: typing exercises, quizzes, fill-in-the-blank exercises and the intros before them. Use it for a whole course (language, course), a whole module (language, course, module) or for a single exercise (language, course, module, exercise, prompt).
---

# ingrain-exercises

Writes the exercises of one module, in `content/prod/<language>/<course>/<module>.json`. It is the third of three skills: `ingrain-courses` (the courses of a layer), `ingrain-modules` (the modules of a course) and this one. The module must already be in the `course.json` of its course.

Arguments: `$ARGUMENTS` is `<language> <course> [module] [exercise] [prompt]`. It has four modes, picked by the arguments.

## The four modes
**1. A whole module: `<language> <course> <module>`.** Write all the exercises the concept needs, plus its intros.
- **Never touch an exercise that is already written,** not its text, not its id, not its place. If the file already has exercises, add only what the concept still needs, with new ids, and say so. If the module looks complete, say so as a warning and change nothing

**2. One exercise: `<language> <course> <module> <exercise> [prompt]`.** The `exercise` is its id, a slug.
- **It does not exist:** run the guard below on the module first, then create it. The prompt, if there is one, says what it must be. Without a prompt, decide from the module. Put it where it makes sense in the session
- **It exists, with a prompt:** change it as the prompt says, and nothing else
- **It exists, without a prompt:** give a **warning** (the exercise is already written and nothing says what to change) and change nothing

**3. A whole course: `<language> <course>`.** Run mode 1 for every module of the course, all in parallel, then review the course once with `ingrain-score`.
- Read the modules of `content/prod/<language>/<course>/course.json`, in the order they are listed
- **Universal-concept guard first.** Before launching anything, run the guard below on every module of the list. List the flagged ones in the chat with the reason and the proposed alternative, launch subagents only for the rest, and write nothing for a flagged module until the author confirms
- **Skip every module whose file already exists,** whether it is complete or not: do not launch a subagent for it and do not touch it. The course call only creates the modules that have no file yet. List the skipped ones in the report
- Launch one **fresh subagent** for every other module, **all in the same turn**. A module is self-contained: its fill-blanks repeat pieces typed in the same module, so no module waits for another
- Each subagent gets the three-argument call (`<language> <course> <module>`), so it runs mode 1 and never starts a loop of its own. Tell it to follow this skill, and give it the ordered module list of the course in two parts:
  - **Before it:** the modules whose concepts it may use but must not teach again
  - **After it:** the modules whose concepts it must not teach or use. A module never teaches what a later module shows
- **A subagent never edits `course.json`,** so parallel agents do not overwrite each other. It returns the session count its module needs, and the parent applies every count in one pass. A subagent never calls `ingrain-modules` either: when its module needs 5 or more sessions, it writes the first 4 and reports "needs a split" with the proposed modules
- **Each subagent runs `cargo test --test content` and fixes only the errors of its own file.** The test checks every module, so ignore the errors of other modules and the session count of its own module, which the parent fixes
- After all subagents return, the parent:
  1. Updates `sessions` in `course.json` for every count reported
  2. Handles every "needs a split": calls `ingrain-modules` to add the modules, then runs a second batch of subagents for the added modules, in parallel, with the same two-part list
  3. Runs `cargo test --test content` for the whole course and fixes what it reports. If a module still fails, report which one and why
  4. Runs `ingrain-score` once for the course, in a fresh subagent (never inline: the agent that wrote the content is biased toward it), shows its report, and stops. There is no flow review and no second pass
- Do not commit anything
- At the end, report the exercises written per module, every warning, every "needs a split" and every session count changed in `course.json`, then the scorer's report and nothing after it. The author decides what changes; a fix goes through mode 2 (a named exercise and a prompt), so the ids and the saved progress stay safe

**4. Review: `<language> <course> <module> review`, or `<language> <course> review`.** It runs `ingrain-score` in a fresh subagent (never inline, to avoid the writer's bias) and shows its report. It changes nothing.

**The same idea holds for the other two skills:** a general call, with no request, never touches what already exists. Only a specific, named target with a request can change something that is written.

## Read first
1. `docs/content-structure.md`: the schema of the module file, the typing exercise, the quiz, the fill-blank and the intro, with their limits
2. `docs/course-structure.md`: "Tiers"
3. `references/writing-rules.md` next to this file: how to write guides, notes, quizzes, fill-blanks and intros. Start with "Audience", "The non-obvious test" and "The bar moves with position": they decide what an exercise may teach
4. The module's entry in the `course.json` of its course (its tier and its session count), and the modules before it in the course: the user has done them
5. The module file, if it exists
6. If the scope file `docs/courses/<language>-<layer>.md` exists (for example `docs/courses/go-core.md`, `docs/courses/rust-core.md`): the scope of the course (where it stops, and the later concepts its modules may use simply) and the rules of the module. A line there wins over what the module name suggests. `ingrain-modules` writes it, after the author says yes
7. For the format, an existing module: `content/prod/go/syntax/conditionals.json` (typing) and `content/prod/go/pointers/nil-pointers.json` (quizzes and intros)

## What the exercises must be
- **Guard: a universal concept is never written.** Before writing (modes 1, 2 for a new exercise, and 3), check the module. A module is **universal** when it is named for an idea every programmer already knows (`Variables`, `Functions`, `Booleans`, `Conditionals`, `Comments`, loops taught from zero) and not for what the language does differently with it. If it is, **stop and warn, and write nothing:** say which module, why it is universal, and propose the alternative (fold it into a dense neighbor, or retitle it for the Rust-specific part, for example `Immutability & shadowing`). Go on only when the author confirms or changes the module in `course.json`. A module that already has a file is not guarded: mode 2 on it works as before
- **The reader is a programmer, and must master what is new to them.** There is no fixed number of exercises: decide from the course and the module, and never pad
- **Mastery is checked against a list, not felt.** Before writing a module, list what a programmer who knows another language still needs: the form this language uses, how it differs from other languages, its traps and bugs, its idioms and the standard-library helpers people reach for. Leave out what the reader already knows: an item that works as everywhere gets no exercise, unless its syntax must be shown (then it goes in one dense exercise, `references/writing-rules.md`, "Syntax coverage"). Each item gets an exercise, or is left out on purpose because a later module covers it (name that module). After drafting, check the exercises against the list and add what is missing. Mastery means the reader can use the concept in real code and avoid its traps
- **A module has 1 or 2 sessions by default, 3 at most, or 4 when one more session is all it needs.** A syntax module is usually 1 or 2. A concept with real traps and forms (ownership, slices, interfaces, concurrency) gets up to 3, each on a theme. More than 2 sessions needs a reason in the report (the traps or forms that fill them), never "a rich concept". Change the module's `sessions` number in `course.json`. A 4th session is allowed when the mastery list needs just one more session, because 4 sessions are better than a module of 3 and another of 1. When the list needs 5 or more, split the concept into more modules (see "When a module needs 5 or more sessions")
- **Combine every type of exercise when possible,** and do not force it: typing exercises, quizzes of every kind (`purpose`, `output`, `compiler`, `error`) and fill-blanks. Alternate them
- **The author likes the `compiler` and `error` quizzes the least.** Write them rarely, and only when the lesson is the message itself or a layout rule that one word cannot show (the place of a brace, the scope of a variable). The `error` quiz is the weakest: its answer is almost always "A compile error".
- **A fill-blank repeats an established pattern.** It is never the first contact with something new. Use it only for a piece the user has already met and practiced: typed in an earlier exercise of the same module (a module never counts on the exercises of another). Check the earlier exercises before writing one, and if the piece has not been typed yet, write a typing exercise (or leave the slot to a quiz) instead
- **The author likes fill-blanks more than quizzes overall.** Look for a spot in every session: after you draft the typing exercises, check which piece the user has just typed that fits one word (a keyword, a name, a type, an operator), and add a fill-blank after that exercise, about one per session. When a slot could be a quiz or a fill-blank and the pattern is established, choose the fill-blank. It is its own type, not a stand-in for a `compiler` or `error` quiz, and it never forces a pattern that is not there: with no established piece, leave it out
- **Typing exercises are short and focused:** about 3 to 5 chunks and 10 to 15 lines of code. If a concept is long, write **several typing exercises, each on one specific thing**, and not one big one
- **Ghost is the majority,** around 90% of the chunks. Use `auto` for a piece the user has already written many times, and `copy` to challenge them more (`references/writing-rules.md`, "Modes")
- **One file,** the same in every exercise: the editor shows `main.go` for Go and `main.rs` for Rust. Each typing exercise builds a complete, runnable program in it. In Go, `package main` is an `auto` chunk, except in the first module of the first course of the language, where it is ghost so the user types it once
- **The output is predicted, never run.** Do not run any code. Fill `output` of a typing exercise (the `command` and the `text` it prints) from what you can work out by reading the program, so keep every program deterministic: no randomness, no current time, no map iteration order, no goroutines racing for the output. If you cannot be sure of an output, change the program
- **Use `output` only when the program prints plain text.** The output card on the complete screen is for plain printed lines (the command and the text it prints). Decide by what the program does, not by the course: a program whose result is plain lines (a cobra command that prints a greeting, the help text) keeps `output`. A program whose result a card of lines cannot show (a rendered TUI screen, a running web server, an interactive session) gets no `output`: leave it out right away and do not think about what it would print. The complete screen then shows only the code card. An `output` quiz is not affected
- **Do not repeat what other exercises already teach.** Before you create an exercise, read the titles, the guides and the code of the other exercises of the module, and the entries in `course.json` of the modules before it (their files may not exist yet when the course is written in parallel; read them when they do). Each exercise is about one specific thing that none of them covers. When you write a whole module, check its exercises against each other the same way, so no two practice the same point. Using a piece a previous module taught, for another purpose, is fine, and so is a deeper or richer form of a concept an earlier module introduced (the exercise starts from what was taught and never re-introduces it)
- **Use only what the user has learned:** the concept of the module, and what the modules before it taught. An exercise never needs something that comes later in the course, and a module never teaches what a later module is there to show
- **Intros are as long as the difference is hard.** A concept that works as in other languages gets one slide on what differs, or none; it never defines a general concept. A hard concept (ownership, concurrency) gets longer texts, more slides and snippets, and pseudo code in a snippet is welcome for a new concept. A module intro says what the user is about to see, and the session intro explains how the concept works in the abstract and what the short guides cannot (`references/writing-rules.md`, "Intros")
- **Sessions:** group the exercises in sessions of about 4 to 6. A module that needs more gets more sessions (`session-2`, `session-3`), up to 4, and you update its `sessions` number in the course's `course.json` to match, and say so

## When a module needs 5 or more sessions
- **A module that has no exercises yet (modes 1 and 3):** split it while you write. Write the first 4 sessions in this module. For the rest of the mastery list, call the `ingrain-modules` skill (`<language> <layer> <course> <request>`: read the layer from the course's entry in `content/prod/<language>/catalog.json`, and put in the request the modules to add, the module they follow and the tier) to add new modules right after this one in `course.json`, in the same section only if they fit it, or in a new cohesive section with its own tier (rare topics get a medium section of their own, never into a basic one), then write each new module with this skill, in order. Say which modules you added
- **Whole-course mode:** the subagents never split on their own. They report "needs a split", and the parent adds the modules and writes them in a second parallel batch (mode 3)
- **A module that already has exercises:** never split it on your own. Splitting moves exercises to another file, and the saved progress points at them. Report "needs a split" with the proposed modules and which exercises move, and change nothing. Only a prompt that names the module and asks for the split can do it
- `ingrain-modules` never calls this skill back: this skill writes the exercises of the modules it added

## Draft first
Modes 1 and 2 never write the file before the author has seen it. Mode 3 is exempt: its subagents write in parallel and the scorer reviews the result.
1. Draft the module in the chat, with nothing written to disk. Per session, each exercise gets its type and title, and its code as readable Go (never JSON):
   - typing: the program as it ends up in `main.go`, with its output and the notes it will carry
   - fill-blank: the code with the `___` gaps, and the answers
   - quiz: the code, the options with the right one marked, and the explanation
   - the intro slides: their text and snippets
2. **Say what was left out and why:** every item of the mastery list that got no exercise, with the reason (a later module owns it, every language does it the same way, or it is too rare to matter), and the module that owns it. Add a one-line mastery check: which exercise covers which item
3. Stop and wait. The author approves or corrects the draft
4. Write exactly what was approved. If `cargo test --test content` forces a change (a card limit, a line count), say what changed and why

## Writing
- Ids are slugs, unique inside their session. The tier of the module only says how hard the concept is: it does not change the shape of the exercises
- Follow `references/writing-rules.md` for the guides, the notes, the quizzes and the intros, and `docs/content-structure.md` for the limits (the card limits, the quiz limits, the intro limits)
- Keep the file in the shape of the existing modules: the same fields, the same order

## Changing what exists: be careful
- **Never rename or reuse an id** of a session or an exercise: the saved progress points at them
- **Never remove an exercise** unless the prompt names it and says to. Say what you removed
- Change only what the prompt asks, and leave everything else exactly as it is
- If the prompt would change the session count, say so and update `course.json`

## Engagement
Before you finish, check the module against "Balance" in `references/writing-rules.md`: mostly typing exercises that build something that runs, every syntax form typed at least once with its notes, and quizzes only for real traps.

## Finish
Run `cargo test --test content` and fix everything it reports. It checks the ids, the session count, the card limits, the quiz rules, the fill-blank rules (the gaps and the answers), the intro limits and the chunks (the `line` places, the ghost guides, the balanced code and the note anchors).

A pass does not mean the content is good. Read each program as a compiler would, and trace each output by hand once.

Then report:
- the exercises written or changed: id, type and title, per session
- warnings: an exercise that exists and had no prompt, a module that looked complete, a universal-concept module
- any session count changed in `course.json`, every module added with `ingrain-modules`, and every existing module that "needs a split"
- then the `ingrain-score` report, run once in a fresh subagent (never inline, to avoid the writer's bias), and nothing after it. Stop and wait for the author
