# Writing rules for ingrain-exercises

What a test cannot check. The schema, the limits and the ids are in `docs/content-structure.md`, and this file does not copy them. The mechanical rules (ids, session counts, the card limits, quiz options, fill-blank gaps, chunk lines, note anchors) are checked by `cargo test --test content` (`tests/content.rs`). Everything below is judgment, and it is yours to get right.

## Audience
The reader is a programmer who already codes. They come to learn a new language from scratch, to reinforce one they know, or to learn a library of its ecosystem. They never come to learn what programming is.
- **Assume general programming.** The reader knows what a variable, a function, a loop, a conditional, a type, recursion, an error and a collection are. They need how this language writes them, and what is not obvious beyond the syntax
- **Core course:** assume programming, know nothing about the language. **Ecosystem course:** assume the language too, know nothing about the library
- **Never define a general concept.** Guides, notes and intros name the form and the difference, never the idea
  - bad: "A variable gives a name to a value"
  - good: "`let` is immutable here. Add `mut` to allow a change"
- Write every guide and note in your own words, never in the author's phrasing
- The user learns by typing the code, so the code is the lesson. Text only adds what the code does not show

## The non-obvious test
Every exercise hinges on one of these. Name which one when you report:
1. **A syntax form the reader could not guess** (`..=`, `'label: loop`, `let else`, `impl` blocks)
2. **A rule of this language** that differs from the ones they know (immutability by default, shadowing, expression-vs-statement, exhaustive `match`, overflow behavior, move semantics)
3. **A trap or a bug** people really hit
4. **An idiom or a standard-library helper** people reach for
5. **For a library:** how it is used, wired and configured

If a programmer from another language would get it right without reading anything, cut the exercise or merge it into a richer one. "Print three variables" and "call the right function" fail the test.

**The hinge must also be used in real code.** A form nobody could guess is still not worth an exercise if a working programmer never writes it. Be lax: the bar is clearly-never, not rarely.
- **Used every week:** keep
- **Rare, but standard in real code** (the standard library and well-known projects use it): at most one light exercise, in the module where it is used (`1 << iota` flags in `constants`)
- **Never in everyday work:** cut. The model case is Go's `&^`: it is unguessable and almost nobody writes it, so it hinges on nothing the reader needs. Boolean algebra for its own sake is the same
- When in doubt, keep it

**The bar moves with position.** The reader always knows programming, but their knowledge of the language grows as the course goes on. The test is always "is something new here for this reader?", and the position only changes what counts as new:
- **First module of the first course of a language:** a simple form counts (`import "fmt"`, `Println`), because the reader has seen none of the language
- **Any other module:** a simple form counts only if it is new to the language and the modules before did not show it
- **A concept the reader has not met** counts at any tier, taught plainly. It needs no rule or trap: the concept is the new thing. The tier of its section says how hard it is: the concepts of `medium` and `advanced` sections (semaphores, closures, generics) are the complex ones
- **A concept the reader was already taught** counts only if the exercise adds a rule, a trap or a new combination of features
- **A general programming idea never counts, at any position**

- A general idea is anything a programmer knows from any language: what a comment, a variable, a call order or a loop is. Typing it never earns the exercise, even in the first module
- The tier changes only what counts as non-obvious. It never changes the shape of the exercises (the mix of modes, the quiz kinds)
- Be lax at the top too: when in doubt about the bar, keep the exercise

## Syntax coverage
Syntax that must be known still gets taught: every kind of loop, `if`, `match`, `fn`, `struct`. Teach it as "how this language writes it", dense and compact:
- one typing exercise shows the forms side by side (`loop`, `while`, `for`, a range) and the one non-obvious part (`if` is an expression, `loop` returns a value, `for` takes a range or an iterator)
- never one exercise per trivial form, and never a concept explained from zero
- the traps of the construct then get their own exercises

## Concepts taught later
A module may use a concept that a later course teaches in depth, when leaving it out would make the topic false or thin. Example: a module whose code passes a slice or a pointer needs the reader to know what it is. Its intro says what the concept is and how it behaves, in a few sentences, so the reader can follow the module without having met it. Never skip a concept for the sake of order.
- **Keep it simple:** the plain behavior only, one form, nothing the reader must have learned to follow the exercise. The exercise hinges on the module's own concept, never on the later one
- **Explain it briefly, never point to a course:** the intro says what the concept is and how it behaves, only as far as this module needs. No slide, guide or note says it is "taught later" or names the course that teaches it
- **It raises no questions:** no exercise, quiz or fill-blank is about the later concept. If the reader would need to ask what it is, show less of it
- Using a later concept this way is not a gap for the scorer

## Modes
**Ghost is the majority at every tier,** around 90% of the chunks of a typing exercise. The rest is `auto` and `copy`, and what decides between them is how many times the user has already written the piece (`docs/content-structure.md`, "The mode of a chunk"):
- **ghost:** new code, and code the user has written only a few times
- **`auto`:** a piece the user has written many times (imports, a repeated function skeleton, boilerplate). Writing it again teaches nothing
- **`copy`:** a piece the user has already written, when you want to challenge them more. Use it sparingly

When a chunk is `auto`, the guide of the next typed chunk says what was added ("the imports are already in the file").

The clearest case is the line every Go program starts with, `package main`: it is `auto` in every module, except in the first module of the first course of the language, where it is ghost so the user types it once. Any piece the user has already written many times follows the same reasoning.

## Order of the code
- **In a typing exercise, helper functions come first and the entry function (`main`) comes last,** in every language. It is coherent and good teaching: nobody uses a function before defining it, so the user types the function and then the code that uses it
- Never start a chunk with `\n`, and never put a chunk after the closing brace of `main`: the editor has no line to insert it before, so the learner has to press Enter first
- A blank line between two pieces is the trailing `\n` of the first chunk
- **A literal with several items (any language: a map, an object, an array, a struct) goes on several lines, one item per line.** Do not align the values: the reader would type extra spaces for nothing

## Go code
- **Indent with 4 spaces, never tabs.** `gofmt` uses tabs, but the editor of the exercise types a Tab key as 4 spaces and fills the indentation of a new line with spaces, so a tab in a chunk can never be typed correctly. That `gofmt` would reformat the code to tabs is fine (the author, 2026-11-02): the goal is to learn by seeing, typing and debugging the code
- A typing exercise is a whole `main.go`: `package main`, the imports and `func main() {` ... `}` come first, as one `auto` chunk (ghost in the first module of the first course), and the other chunks go inside `main`, each at its `line`
- **Name variables for what they hold,** like a real program: `host`, `port`, `scores`, `stock`, `queue`, `price`. Use `a`, `b`, `x`, `n` or `nums` only when the point is the shape or the math (copy and alias demos, `gcd(a, b)`, `fib(n)`, a bare list of numbers in the first exercise of a concept). `i` and `j` are fine for loop indexes
- **Check the chunks make a program.** The editor puts each chunk into the file at its `line`, before the line that is there. After writing a typing exercise, assemble the chunks that way and check the result: each chunk's `line` is the number of lines already in the file plus one, up to the closing `}` of `main` (so a last chunk goes at the line where `}` is, not after it). `cargo test --test content` checks this for Go
- Print with `fmt.Println` while the course has not taught formatting. The `output` command is `go run main.go`

## Guides
- **As short as possible,** copy chunks most of all, because their code takes the card. A guide adds context the code does not show. It never states the obvious and never describes the formatting of the code
  - bad: "Two imports. End with an empty line"
  - good: "Import the main pieces the app needs from ratatui"
- The limits are in rows of the narrow card (`docs/content-structure.md`, "The step card"). A guide that needs more is two chunks, or a smaller chunk
- Copy chunks are short: a few lines, each short, ideally not wider than the card so they do not wrap

## Notes
A note is a hint under a piece of code, shown while the user types it.
- **New syntax gets a note.** The learner reads the code they are typing, not the guide above it, so the note is where a piece is explained. Put one on every new piece of syntax the first time it appears, and on each new form of it (`[1:3]`, then `[:2]` and `[3:]` each get their own). Do not note a piece the learner has already typed, and do not note every line: a chunk with one note on its new piece is the norm
- **Only where the user needs one.** Many chunks need none. A piece of the course's own API that does something non-obvious (like `ratatui::run`) is worth one
- **Notes scale with position** (see "The bar moves with position"): in the first module of a language and in the `basic` tier, every new piece of syntax gets a note, even a simple one, and a chunk may carry two or three. In `medium` and `advanced`, only the non-obvious piece gets one. A chunk holding a piece the reader has not typed yet is never left without a note at the low end of the bar
- **Plain text, one short sentence,** about 60 characters. No markdown, no backticks. It does not repeat the name of its anchor
- **Written for the user's goal:** what the piece does for the app, never where it comes from or how the library is organized. If the user would ask "what is this and why do I need it?", the note answers that in a few words
  - bad: "ratatui's own copy of the crossterm crate, so you never add it to Cargo.toml"
  - good: "Lets the app read key presses"
- The `on` anchor is copied from the chunk's text and occurs in it exactly once. Check it again after any change to the code

## Quizzes
- The shape and the limits are in `docs/content-structure.md` ("Module file: quiz")
- **The author's least liked quizzes are the `compiler` and the `error` kinds.** The `compiler` one shows an error message and asks for its cause, and its options often restate the message. The `error` one asks whether code compiles, panics or is fine, and its answer is almost always "A compile error". Use them rarely. A `purpose` or an `output` quiz, or a fill-blank, teaches more
- A `purpose` option says what the code is **for**, never what it prints (printed results are the `output` kind)
- The wrong options are plausible mistakes someone could really make reading the code, never jokes or absurdities. None of them is also right
- The right answer is not always the first or the longest. Vary the `answer` index and the `kind` across a module
- The code is short, hinges on a rule the reader could get wrong (shadowing scope, integer wrap, a trailing `;` changing a value), and has no comments that give the answer away
- **An `output` quiz never traces a plain sequence** (two calls in order, three assignments). If anyone gets it right by reading top to bottom, it is cut
- The `explanation` says why the answer is right, in a sentence or two, and does not repeat the option
- **Never run the code.** The `answer` of an `output` quiz, and the `output` of a typing exercise, are what you can predict by reading the program. Keep every program deterministic, so the output is certain: no randomness, no current time, no map iteration order, no goroutines racing for the output. If you cannot be sure of an output, change the program
- **`output` only for plain printed text.** The output card shows plain printed lines only. When the program prints plain text, even in an ecosystem course (a cobra command that prints a line), write `output`. When its result is a rendered TUI screen, a running web server or an interactive session, omit `output` right away. This does not apply to `output` quizzes

## Fill in the blank
- The shape and the limits are in `docs/content-structure.md` ("Module file: fill-blank")
- **Only for a pattern the user has already practiced.** A fill-blank repeats something that was typed before, in an earlier exercise of the module or in a module before it, and never teaches a new thing. Before you write one, find where the user typed that piece. A rule that only a quiz explained (for instance, that an unused variable is an error) is not established yet: a fill-blank on it would be a first contact. Its order matters too: it goes after the typing exercise that practices the piece, never before
- **The author prefers fill-blanks to quizzes overall,** so choose one when a slot could be either and the pattern is established. A fill-blank is its own type, not a replacement for a `compiler` or `error` quiz, and it does not take a one-word fix just because a quiz did. Never quote a compiler message in a fill-blank guide: it is not the real output and it gives hints away. State the goal in plain words
- **A gap hides the word that holds the idea of the exercise:** a keyword, a name, a type or an operator that the module's concept is about, and it must be specific to this language (`mut`, `&`, `?`, `as`, `u8`), never a word every language needs (`fn`, `if`, `return`). Never trivia (a brace, `main`) and never a whole expression, because an answer is one word
- **The rest of the code must make the answer certain.** Someone who understands the concept can work the word out from the lines around it, and nothing else fits. If two words would work, list both in `answers`, only when each is really right in that place, and put the best one first because it is the one shown after a mistake
- Two or three gaps are usual, one is fine, six is the limit. At most two gaps on a line, never next to each other, and each one asks a different thing
- **The guide says what to fill, never the answers.** Nothing else in the exercise gives one away: no comment, no name, no other line that repeats the word
- The code is a complete, valid program once the gaps hold their first answers. You do not run it: read it as a compiler would. Never write `___` in the code for any other reason
- A fill-blank has no `output` and no `explanation`, so the title and the guide must be enough to know what to do
- The answers are spelled as in the code, letter for letter: the check is exact, case included
- Vary the place of the gaps across a module (a keyword here, a type there), and alternate the exercise with typing exercises and quizzes

## Engagement
The module must make the user want to continue. It fails when the user types code that does nothing, or answers questions whose answer is a keyword. Check every exercise against this list:
- **A typing exercise builds a program that runs and shows a result.** It has an `output` (unless the result is a TUI or a server). An empty skeleton (`fn main() {}`, braces with nothing in them) or a program made of comments only is not an exercise; it is at most a step of a larger one
- **No more than 3 lines of the same kind in a typing exercise.** Four or more `println!` calls that only repeat the same shape (printing more text, one more placeholder, one more call of the same function) tire the user of typing the same thing: cut to at most 3, and keep a line only when it adds something new (a new placeholder form, a new function, a new escape). Lines that repeat a pattern already typed can be `auto` chunks. Concise beats complete: drop a piece that renders badly or adds nothing (the author, 2026-11-11)
- **One exercise, one thing the user can see working,** with enough code to feel like building something: about 4 lines or more typed. When a concept is thin, write fewer and richer exercises (or one), never a string of small ones that each add a trivial piece
- **The first module of a course runs a program in its first minute.** Do not hold back the thing the user came for (printing, a result) to keep a topic pure; a plain use of a later piece, named in the guide as "covered later", is better than a module with nothing to see
- **Quizzes test understanding, not trivia.** "What does a comment change?", "which part is a comment?" and a fill-blank on `fn` or `main` teach nothing. A quiz asks what a program prints, why it is written one way, or which version is right, about code that does something
- **A fill-blank hides a word the lesson is about,** and the user must think to find it
- **Realistic names and situations** (a price, a score, a host) over `foo`, `x`, `a` and `b`, wherever the point is not the math
- **Variety inside the module:** the programs differ from each other in what they do, not only in the keyword they show

- **No condescension.** The exercise never treats the reader as a beginner. Banned: printing a few variables, calling the function that matches a name, "which part is a comment", tracing calls in order, a fill-blank on a word any language needs, an intro that defines a general concept. Each of these fails the non-obvious test

A module is **condescending** when most of its exercises fail the non-obvious test, **empty** when most fail the rest of this list, **thin** when a few do, and **engaging** otherwise. A condescending or empty module is rewritten, not patched.

## Balance
Cutting has a floor. The rules above remove weak exercises; this says what must stay.
- **A module is mostly typing:** about two thirds of its exercises are typing exercises that build a program and show a result
- **Quizzes and fill-blanks are for real traps and practiced patterns,** about a third at most
- **Every syntax form the module teaches is typed at least once, with its notes** (see "Syntax coverage")
- **Simple is correct at the low end of the bar.** A module is never trimmed below what it needs to teach its forms, even when every exercise left is simple
- **The scorer's cuts are advice.** When a cut would break this floor, keep the exercise

## Intros
- **As much as the difference needs, not a fixed number.** A concept that works as in other languages (conditionals) gets one slide on what is different, or none. A hard concept (ownership, concurrency) gets longer texts, more slides and snippets. An intro never defines a general concept: it says what is different here and what the user is about to see. Judge it from the course and the module
- One idea per slide, in the markdown subset of the guide. The limits are in `docs/content-structure.md` ("Intro")
- The module intro is an overview of what the module covers. A session intro teaches what its exercises practice
- **The module intro shows only once.** A later session that introduces a new construct gets a short intro with its definition and form, never what its exercises hinge on
- The text teaches. It never gives away the answer of an exercise that follows
- **Pseudo code is welcome in an intro snippet,** especially for a new concept: it shows the idea before the syntax. Say in the text that it is pseudo code. A snippet that is real code uses real syntax and APIs, and compiles
- A snippet only when the slide is about code. It is illustrative: it may use placeholder names to show a shape (`pattern_a => do_this`), but every piece of syntax and every API in it is real, and a snippet meant as real code compiles. It uses only what the course has taught, what the slide itself introduces, or a later concept used simply and explained on a slide (see "Concepts taught later")
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
