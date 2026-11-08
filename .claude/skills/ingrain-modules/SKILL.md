---
name: ingrain-modules
description: Create or modify the modules of one course of a language in Ingrain, in the course's course.json: its module sections, its modules and their tiers. Use when the user wants the modules of a course drafted, extended, renamed or reordered, or the modules of every new course of a layer drawn in parallel.
---

# ingrain-modules

Writes the modules of one course, in `content/prod/<language>/<course>/course.json`, or of every course of a layer that has none. It is the second of three skills: `ingrain-courses` (the courses of a layer), this one, and `ingrain-exercises` (the exercises of each module). The user calls it on its own; `ingrain-exercises` calls it back only to add modules to a course.

Arguments: `$ARGUMENTS` is `<language> <layer> [<course> [request]]`. The course must already be in `content/prod/<language>/catalog.json`, inside that layer. There are two modes:

**1. One course: `<language> <layer> <course> [request]`.** Without a request, draw all the modules of the course. With a request, change them as it says. The request may carry the **scope** of the course (what it must cover) and the scopes of the courses around it: stay inside the scope, and do not cover a topic that a neighbor course covers. The scope is not stored anywhere else: after this run, the modules themselves are what remembers it.

**2. A whole layer: `<language> <layer>`.** Draw the modules of every course of the layer that has no `course.json`, in parallel, then review the layer.
- Read the catalog and keep the courses in study order. **Skip every course that already has a `course.json`:** launch no subagent for it and do not touch it. List the skipped ones in the report
- Write the **scope table** in the chat: one line per course to create, of what it covers and what it leaves to its neighbors, decided from your own knowledge of the language or the library. **Stop and wait for the author to approve or correct it** before launching anything. The table is the contract of every subagent, and it is not stored anywhere else
- Launch one **fresh subagent** for every course to create, **all in the same turn**. Each gets the one-course call (`<language> <layer> <course>`), so it runs mode 1 and never starts a loop of its own. Tell it to follow this skill, and give it the scope table in two parts:
  - **Before it:** the courses whose concepts it may use but must not draw again
  - **After it:** the courses whose concepts it must not cover. A course never covers what a later course owns
- **A subagent writes only its own `course.json`,** and never edits the catalog or another course. It never calls `ingrain-exercises`
- **Each subagent runs `cargo test --test content` and fixes only the errors of its own course.** The test checks every course, so ignore the errors of the others while the batch runs
- After all subagents return, the parent runs `cargo test --test content` for the whole layer and fixes what it reports. If a course still fails, report which one and why. Then it runs the **layer review** below
- **Layer review.** Launch one fresh subagent that reads every `course.json` of the layer, in order, and checks:
  - **Overlap:** no two courses have a module for the same concept. Using a concept again in another course is fine; drawing it twice is not. A later module may come back to a concept when it adds something (more context, another use, a harder form): that is reinforcement, not overlap. Report it only when the later module would re-introduce the basics
  - **Gaps:** nothing the layer needs falls between two courses
  - **Forward dependency:** no module needs a concept that only a later course teaches
  - **Sections and tiers:** the sections are cohesive, the tiers rise with the difficulty, and no course is thin or oversized next to the others

  The review **only reports**: each problem with its course and module id. It changes nothing. A fix goes through mode 1 with a request that names the course and the change
- Do not commit anything
- At the end, report one section per course with the report its subagent returned, the result of the layer review, then the skipped courses and every warning

## Read first
1. `docs/course-structure.md`: the hierarchy, and "Tiers"
2. `docs/content-structure.md`: "Folders", "Files", "Sessions in the explorer" and "Ids"
3. The course's entry in the catalog, the courses of its group, and the courses next to it
4. The current `course.json` of the course, if it exists, and the files in its folder: a module whose file exists is **written** and has content

The shape of `course.json`:
```json
{
  "id": "syntax",
  "module_sections": [
    {
      "name": "Foundations",
      "tier": "basic",
      "modules": [{ "id": "variables", "name": "Variables", "sessions": 1 }]
    }
  ]
}
```
`id` is the course id and the name of its folder. A module id is a slug, and it is also the name of its file.

## What the modules must be
- **A module is one concept.** Its name says which: `Variables`, `Slices`, `Channels`
- **A module name is at most 24 characters,** spaces included, so it fits its cell in the explorer (`Slices in functions` has 19). Write `&` instead of `and` to stay under it (`Stacks, queues & copies`), which saves 2 characters, and spell it `and` in the id. If the concept still needs a longer name, shorten the name and let the module's content carry the detail
- **Mastery is the measure:** completing all the modules of the course must let the user master the course. No fewer, and none that the course does not need. There is no fixed number
- **Keep the list lean:** cover every topic needed for mastery, but give no module to a rare edge case. Merge small related topics into one module instead of one module each, and prefer a few well-sized modules over many thin ones
- **Module sections** group related modules, stacked from the easiest concepts to the hardest. A section has a name that says what its modules share (`Foundations`, `Data types`), at most 24 characters, with `&` for `and`. The tier takes cells on the same row, so a short name reads best
- **The tier of a section is the difficulty of its concepts,** not the shape of the exercises (`docs/course-structure.md`, "Tiers"): `basic`, `medium` or `advanced`. Decide it **from your own knowledge of the language or the library**: how many other concepts a concept needs first, how abstract it is, how easy it is to get wrong. Do not match it to a list of examples
- **A section is cohesive:** every module in it fits the section's name and tier. When you add or move a module, check it against its section. If it does not fit, rename the section so the name covers all its modules, move the module to the section where it belongs, or make a new section for it (a module that is a rare topic goes to its own section with the medium tier, not into a basic one). Check the sections after a split too: a section must never become a pile of unrelated modules
- **The order is the order of study** inside a section and across sections, and it follows what depends on what
- **A concept may come back in a later module** when that module adds something: more context, another use, or a harder form (plain or-patterns first, then or-patterns inside other patterns). Its name and scope say the new angle, and it never re-introduces the basics
- **One session per module to start:** `"sessions": 1`. The exercises skill raises it up to 4 when a module needs more, and asks this skill for more modules when a concept needs 5 or more sessions
- **A module made by splitting a concept** is named for its theme (`Slices in functions`, `Maps of maps`), never `Slices 2`. Each one is still one concept
- A core course covers the language itself, nothing tied to an external library. An ecosystem course covers its own library

## Writing
- **A general call is a call with no request.** On a course with no `course.json`, write the whole file and create the course folder. On a course that already has one, **warn** that it has, add only the modules that are missing, and touch nothing that is already there
- **To change a module that exists** (rename, move, remove, change a tier), the user names it in a request. Change only what the request asks and leave everything else exactly as it is
- Keep every existing id, and the `id` of the course

## Called by the exercises skill
When `ingrain-exercises` needs more modules because a concept does not fit in 4 sessions, it calls this skill with a request: the modules to add, the module they go right after, and the tier. Add them right after that module, in its section when they fit it, or in a section of their own, with the right tier, when they do not (rare topics get a medium section of their own). Each starts with `"sessions": 1` and has no file. Do not touch the module they follow. This skill never calls `ingrain-exercises` back

## Modifying: be careful
- **Never rename or reuse the id of a module that has a file.** The saved progress points at `<language>/<course>/<module>`
- **Never remove a module that has a file.** A module with content stays in the list. If the request asks for it, say you did not, and why
- A module with **no file** has no content and no progress, so it can be renamed, moved or removed freely
- Names, tiers and the names of the sections can change freely. Reordering is allowed, but say so: it changes the order in which users study
- Keep `sessions` as it is for a module that has a file: the number must match its file
- Adding a module next to a module that has a file is allowed, and leaves the existing one exactly as it is
- If the request would remove, rename or reorder modules that exist, say exactly what will change and ask before writing

## Finish
Run `cargo test --test content` and fix everything it reports. In the layer mode, this is the parent's run after the subagents return. Among other checks, it fails when an id is not a slug or repeats, when a module file is not listed in `course.json`, and when a written module has a different id or number of sessions than `course.json` says, and when a module name or a section name is longer than 24 characters.

Then report:
- what you added, changed or left as it was
- what the request asked and you did not do, and why
- the modules that have no file yet, so `ingrain-exercises` is the next step for each
