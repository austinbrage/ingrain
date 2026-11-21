---
name: ingrain-score
description: Score the content of an Ingrain course or module as condescending, engaging or overwhelming, with the evidence and a fix for each weak exercise. Writes only a review draft in docs/drafts/, never the content. Use it to review written content, or let ingrain-exercises and ingrain-modules call it on a draft.
---

# ingrain-score

Judges content already written, in `content/prod/<language>/<course>/`. It is the fourth skill: `ingrain-courses`, `ingrain-modules` and `ingrain-exercises` write, this one judges. **It never changes content:** the only file it writes is the review draft. A fix goes through `ingrain-exercises`, so the ids and the saved progress stay safe.

**It advises; the author decides.** Nothing it reports is applied without the author's answer.

Arguments: `$ARGUMENTS` is `<language> <course> [module]`. With a module, it scores that module. Without one, it scores every written module of the course, then the course as a whole.

## Read first
1. `ingrain-exercises/references/writing-rules.md`: "Audience", "The non-obvious test", "Syntax coverage" and "Engagement". They define what the scorer judges against, and they are the same rules the writer follows
2. `references/anchors.md` next to this file
3. The module file, its entry in `course.json`, and the modules before it. From them, read the **position**: the tier of its section, its place in the course, and whether the course is the first of the language in `catalog.json`

## The scale
One axis, three points. The middle is the goal.

**The default is engaging.** Move off it only when something is clearly wrong, and in doubt keep engaging.
| Verdict | The reader feels | Cause |
|---|---|---|
| **condescending** | "I know this" | the exercise fails the non-obvious test (`ingrain-exercises/references/writing-rules.md`) |
| **engaging** | "I'm learning something, and I can do it" | one new thing the reader could not guess: a form, a rule, a trap, or a complex concept taught plainly |
| **overwhelming** | "too much at once" | too many new things together (density), or a step that uses what was not shown yet and that a slide cannot explain simply (gap) |

A verdict is judged against **a programmer who knows another language and knows only what the modules before it taught, plus any later concept the module names and uses simply** (`writing-rules.md`, "Concepts taught later"), with **the bar of the module's position** (`writing-rules.md`, "The bar moves with position"). The same exercise can be engaging in the first module and condescending in a `medium` one. A general programming idea is condescending at every position.

**A new thing** is a hinge (one of the five) that the module introduces for the first time. A syntax form counts once, when it is first typed, and never again. A quiz, a fill-blank or a note on a piece already typed counts zero. The test: could the reader do this exercise after the ones before it, without reading anything new? If yes, it adds nothing new.

## What it measures
Judgment, backed by counts. Every verdict quotes its evidence.
- **Condescending signals:** the exercise clearly hinges on none of the five hinges (in doubt, it is engaging); a plain trace; a gap on a word any language needs; an intro that defines a general idea. **Scope, not title, decides** whether a module is universal: `Variables` that teaches `var`, `:=` and zero values passes, and a module that would read the same in every language fails
- **Missing notes:** at the low end of the bar (first module, `basic` tier), a chunk that types a new piece of syntax with no note is reported, with the notes it should have
- **Overwhelming comes from density, or from an unseen piece that a slide cannot explain.** The limits are **starting values**, to calibrate on Go `syntax` before they are trusted:
  - **Density** (fix: split the exercise or move a piece to another one):
    - an exercise with 2 or more new things. A typing exercise whose first chunk is new and whose other chunks repeat it is fine
    - a session with more than about 3 new forms
    - a chunk or guide near the card limits (`docs/content-structure.md`)
  - **Unseen piece.** An exercise or an intro uses a form or a rule that no earlier exercise or module showed. Two cases:
    - **A slide can explain it simply** (one or two sentences, in its plain form, with no exercise hinging on it): it is only a **gap**, a flag and not a verdict. Fix: add that slide (`writing-rules.md`, "Concepts taught later"). The slide never says the concept is taught later and never names the course that teaches it
    - **A slide cannot explain it simply,** or the exercise hinges on it: it is **overwhelming**. Fix: a bridge exercise, or move the exercise later
- **Repetitive is a flag, not a verdict.** A module whose exercises repeat the same forms (three exercises of the same declarations), or whose 3 or more sessions are not filled by named traps or forms, is padded. It is not overwhelming. Its fix is to trim or merge, so it gets its own label next to the verdict
- **Irrelevant is a flag, not a verdict, and it is lax.** It marks an exercise whose hinge a working programmer never meets in real code (the test is in `writing-rules.md`, "The hinge must also be used in real code"). Its fix is cut. Flag only the clear cases, like `&^`. A rare-but-real topic is not irrelevant: it gets one light exercise. In doubt, do not flag
- **Engaging signals:** one hinge named and hit, a program that shows a result, variety across the module, the next exercise builds on the last

## What it reports
- **The report is a draft file the author edits:** `docs/drafts/score-<language>-<course>-<module>.md` (the folder is in `.gitignore`). The chat gets only the verdict line, the path and the count of weak exercises
- **First line:** module, bar used, verdict (the worst common one, not an average), engaging and weak counts. Below it: "Mark `[x]` the changes worth doing, edit the replacements directly"
- **Per weak exercise:** a header (id, type, session, flag: condescending, overwhelming density or gap, repetitive, irrelevant, missing notes), a checkbox with the action (cut, trim, redo), the code as readable Go, one line with the problem, and the proposed replacement in the same form. Never paragraphs
  - typing: the program as it ends up in `main.go`, with its output
  - fill-blank: the code with the `___` gaps, and the answers
  - quiz: the code, then the options with the right one marked
- **Every change to an existing exercise shows two snippets: "Now" and "Proposed".** "Now" is the code as it is in the file, copied from it, never retyped from memory. "Proposed" is the code after the change, in the same form. A change to one chunk of a typing exercise shows that chunk before and after. A note-only change shows the current code with a `Notes:` list below it, never a comment inside the code. A new exercise shows only the new code. The author must be able to compare without opening the file
- **Optional block:** reorders and soft flags, each with a checkbox and no verdict. **Every item shows its code as a fenced Go block, like a weak exercise** (the exercise it is about, with the answers or the output), then one plain line on why it is flagged and what the change does. Never a description in prose alone: the author reads the code, not the explanation. An intro slide shows its text and snippet
- **Last:** the engaging ids on one line, and the balance after the changes (typing share against "Balance" in `writing-rules.md`)
- **One line per module** on what to keep, trim, merge, redo or reorder. On a whole-course call, one line on progression
- Never JSON, never a number out of 10

## How it runs
- **The scorer always runs in a fresh subagent, never in the agent that wrote the content, and never inline, even for one module.** The writer knows why each exercise is the way it is and judges it by its own intent, so its verdict is biased. A fresh agent has none of that context and reads the content as the learner would. A scorer that skips this is not a review
- One **fresh subagent per module**, all in the same turn. Each reads its module, its entry in `course.json` and the modules before it
- A last fresh subagent scores the course from the module reports
- **Anchors:** they calibrate where the middle is (see "Read first")
- Each fresh subagent writes its own draft file. The parent collects the verdict lines, ranks the modules by weak exercises, and gives the author the paths. It does not paste the snippets into the chat

## Review loop
Same as the draft loop of `ingrain-exercises` ("Draft first"). The parent runs it, since the subagents end after writing.
1. After writing, save a copy as `<draft>.last` in the same folder, give the path and one line on the main choices. Stop and wait
2. **On every message of the author, read the draft file first** and compare it with `<draft>.last`:
   - **Different:** the differences are the author's decisions: a ticked box, an unticked one, an edited replacement. The edit wins, except when it breaks a rule that can be checked (a card or line limit, a program that does not compile or an output that cannot be traced, a fill-blank or quiz rule, the universal-concept guard, the typing floor). Then say so in the chat and leave that spot as it was. Redraft: apply the accepted edits to the file, replace a short "Round N" block at the top (what was applied, what was not and why), save a new `.last`, and stop and wait
   - **Not different:** ask for approval once, in the chat
3. Any other message means reread the draft, never approval. Only an explicit "apply" or "approved" applies it
4. **Apply:** with no box ticked, "apply" means leave the module as it is: change nothing, delete the draft and its `.last`, and do not ask. Otherwise hand the ticked changes to `ingrain-exercises` (mode 2: a replaced exercise gets a new id, nothing is removed without the author's word), update `sessions` in `course.json`, run `cargo test --test content`, then delete the draft and its `.last`. No re-scoring: the scorer runs once per module
5. **Dismiss:** when the author drops the module, delete the draft and its `.last`. Nothing else changes

## Calling it from another skill
- `ingrain-exercises` mode 3 runs it once after a whole course, **in a fresh subagent** (see "How it runs"), because the agent that wrote the content is biased toward it. They give the author **only the path of the draft** and start the review loop above
- `ingrain-exercises` modes 1 and 2 never run it after applying: the author approved the draft, and a module is scored on request
- No second run and no other review: what to change is the author's answer, given by editing the draft

## Not its job
The mechanical rules stay in `cargo test --test content`. Forward use that is not simple and named shows up here as a **gap**, and overlap with earlier modules as **repetitive**.
