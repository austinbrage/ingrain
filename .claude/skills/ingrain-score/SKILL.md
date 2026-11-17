---
name: ingrain-score
description: Score the content of an Ingrain course or module as condescending, engaging or overwhelming, with the evidence and a fix for each weak exercise. Read only. Use it to review written content, or let ingrain-exercises and ingrain-modules call it on a draft.
---

# ingrain-score

Judges content already written, in `content/prod/<language>/<course>/`. It is the fourth skill: `ingrain-courses`, `ingrain-modules` and `ingrain-exercises` write, this one judges. **It changes nothing:** a fix goes through the other skills, so the ids and the saved progress stay safe.

**It advises; the author decides.** Nothing it reports is applied without the author's answer.

Arguments: `$ARGUMENTS` is `<language> <course> [module]`. With a module, it scores that module. Without one, it scores every written module of the course, then the course as a whole.

## Read first
1. `ingrain-exercises/references/writing-rules.md`: "Audience", "The non-obvious test", "Syntax coverage" and "Engagement". They define what the scorer judges against, and they are the same rules the writer follows
2. `references/anchors.md` next to this file
3. The module file, its entry in `course.json`, and the modules before it. From them, read the **position**: the tier of its section, its place in the course, and whether the course is the first of the language in `catalog.json`

## The scale
One axis, three points. The middle is the goal.
| Verdict | The reader feels | Cause |
|---|---|---|
| **condescending** | "I know this" | the exercise fails the non-obvious test (`ingrain-exercises/references/writing-rules.md`) |
| **engaging** | "I'm learning something, and I can do it" | one new thing, a form or a rule or a trap the reader could not guess |
| **overwhelming** | "too much at once" | too many new things together (density), or a step that uses what was not shown yet (gap) |

A verdict is judged against **a programmer who knows another language and knows only what the modules before it taught, plus any later concept the module names and uses simply** (`writing-rules.md`, "Concepts taught later"), with **the bar of the module's position** (`writing-rules.md`, "The bar moves with position"). The same exercise can be engaging in the first module and condescending in a `medium` one. A general programming idea is condescending at every position.

**A new thing** is a hinge (one of the five) that the module introduces for the first time. A syntax form counts once, when it is first typed, and never again. A quiz, a fill-blank or a note on a piece already typed counts zero. The test: could the reader do this exercise after the ones before it, without reading anything new? If yes, it adds nothing new.

## What it measures
Judgment, backed by counts. Every verdict quotes its evidence.
- **Condescending signals:** the exercise hinges on none of the five hinges; a plain trace; a gap on a word any language needs; an intro that defines a general idea. **Scope, not title, decides** whether a module is universal: `Variables` that teaches `var`, `:=` and zero values passes, and a module that would read the same in every language fails
- **Missing notes:** at the low end of the bar (first module, `basic` tier), a chunk that types a new piece of syntax with no note is reported, with the notes it should have
- **Overwhelming has two causes, with different fixes.** The limits are **starting values**, to calibrate on Go `syntax` before they are trusted:
  - **Density** (fix: split the exercise or move a piece to another one):
    - an exercise with 2 or more new things. A typing exercise whose first chunk is new and whose other chunks repeat it is fine
    - a session with more than about 3 new forms
    - a chunk or guide near the card limits (`docs/content-structure.md`)
  - **Gap** (fix: add a bridge exercise, or move the exercise later):
    - an exercise that uses a form or a rule no earlier exercise or module showed
    - an intro that relies on something not taught yet
    - not a gap: a later concept used in its plain form, named on the slide as taught in depth in a later course, with no exercise hinging on it. If the exercise hinges on the later concept, or the slide does not name it, it is a gap
- **Repetitive is a flag, not a verdict.** A module whose exercises repeat the same forms (three exercises of the same declarations), or whose 3 or more sessions are not filled by named traps or forms, is padded. It is not overwhelming. Its fix is to trim or merge, so it gets its own label next to the verdict
- **Irrelevant is a flag, not a verdict, and it is lax.** It marks an exercise whose hinge a working programmer never meets in real code (the test is in `writing-rules.md`, "The hinge must also be used in real code"). Its fix is cut. Flag only the clear cases, like `&^`. A rare-but-real topic is not irrelevant: it gets one light exercise. In doubt, do not flag
- **Engaging signals:** one hinge named and hit, a program that shows a result, variety across the module, the next exercise builds on the last

## What it reports
- **Short:** one line on top with the bar used and the module verdict (the worst common one, not an average), then only the weak exercises. The engaging ones are one line with their count
- **Per weak exercise:** id, the flag (condescending, overwhelming density or gap, repetitive, irrelevant, missing notes), its code as a readable fenced block so the author sees the problem without opening the file, one line with the problem, and the proposed replacement in the same form, or "cut"
  - typing: the program as it ends up in `main.go`, with its output
  - fill-blank: the code with the `___` gaps, and the answers
  - quiz: the code, then the options with the right one marked
- **One line per module** on what to keep, trim, merge, redo or reorder. On a whole-course call, one line on progression
- Never JSON, never a number out of 10

## How it runs
- One **fresh subagent per module**, all in the same turn. A fresh agent has none of the writer's bias. Each reads its module, its entry in `course.json` and the modules before it
- A last fresh subagent scores the course from the module reports
- **Anchors:** they calibrate where the middle is (see "Read first")
- The parent collects the reports and ranks the modules by weak exercises. When it presents them to the author, it shows each weak exercise with its snippet in the chat and never summarizes the code away

## Calling it from another skill
- `ingrain-exercises` and `ingrain-modules` run it **once**, after writing. They show the author **only the scorer's report** and **stop and wait**
- No fixing, no second run, no other review: what to change is the author's answer

## Not its job
The mechanical rules stay in `cargo test --test content`. Forward use that is not simple and named shows up here as a **gap**, and overlap with earlier modules as **repetitive**.
