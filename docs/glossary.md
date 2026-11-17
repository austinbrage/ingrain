# Glossary

The terms Ingrain uses for its content. One word for one idea; the files named here hold the full rules.

## Structure
Defined in `docs/course-structure.md`.
- **Language:** Rust or Go
- **Layer:** a group of related courses. Core holds the courses about the language itself, Ecosystem holds the courses about the libraries of the language
- **Course group:** a label that gathers related courses inside a layer. It has only a name
- **Course:** one topic inside a course group (`functions`, `pointers`). Lives in `content/prod/<language>/<course>/`
- **Module section:** a group of related modules inside a course, stacked from basic to advanced
- **Module:** one thing a programmer must learn about the language or library. Never a concept every programmer already knows
- **Session:** a set of exercises inside a module
- **Exercise:** one typing, quiz or fill-blank task
- **Tier:** the difficulty of the concepts of a module section: `basic`, `medium` or `advanced`. It is about the concepts, not the shape of the exercises

## Scope
- **Scope:** what a course covers and what it leaves to the courses around it. A course never covers what a later course owns
- **Scope file:** `docs/courses/<language>-<layer>.md`. It stores the scope of each course of a layer, and `ingrain-exercises` reads it before writing. `ingrain-modules` writes it, after the author says yes
- **Scope table:** the table in the chat that sets the scope of every course of a layer before `ingrain-modules` draws them in parallel
- **Later concept:** a concept that a later course teaches in depth. A module may use one in its plain form, named on the slide as taught later. Rule: `writing-rules.md`, "Concepts taught later"

## Judging content
Defined in `.claude/skills/ingrain-exercises/references/writing-rules.md`.
- **Audience:** a programmer who already codes. Content never explains or tests an idea every programmer knows
- **Universal concept:** an idea every programmer already knows (`Variables`, `Conditionals`). A module is never named for one
- **Hinge:** what an exercise is about. One of five: a syntax form the reader could not guess, a rule of the language, a trap, an idiom or helper, or (for a library) how it is used
- **Non-obvious test:** an exercise passes only if a programmer from another language could get it wrong. Otherwise it is cut or merged
- **Bar:** how much an exercise must teach to pass. It moves with the tier and the position in the course
- **Condescending, engaging, overwhelming:** the three verdicts of `ingrain-score`. Engaging is the goal
- **Gap (scorer):** a step that uses something not shown yet. Not the same as a fill-blank gap
- **Gap (fill-blank):** the `___` where the user types a missing word

## Exercise parts
Defined in `docs/content-structure.md`.
- **Chunk:** a piece of a typing exercise, typed in one go. Its mode is `ghost`, `auto` or `copy`
- **Guide:** the short line above a chunk
- **Note:** the hint under a piece of code, shown while the user types it
- **Intro:** the slides before a module or a session
