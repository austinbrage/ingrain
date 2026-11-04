---
name: ingrain-modules
description: Create or modify the modules of one course of a language in Ingrain, in the course's course.json: its module sections, its modules and their tiers. Use when the user wants the modules of a course drafted, extended, renamed or reordered. ingrain-courses calls it for every new course.
---

# ingrain-modules

Writes the modules of one course, in `content/prod/<language>/<course>/course.json`. It is the second of three skills: `ingrain-courses` (the courses of a layer), this one, and `ingrain-exercises` (the exercises of each module). `ingrain-courses` calls this skill once for every course it creates, and the user can call it on its own.

Arguments: `$ARGUMENTS` is `<language> <layer> <course> [request]`. The course must already be in `content/prod/<language>/catalog.json`, inside that layer. Without a request, draw all the modules of the course. With a request, change them as it says.

When `ingrain-courses` calls this skill, the request carries the **scope** of the course (what it must cover) and the scopes of the courses around it. Stay inside the scope, and do not cover a topic that a neighbor course covers. The scope is not stored anywhere else: after this run, the modules themselves are what remembers it.

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
- **Mastery is the measure:** completing all the modules of the course must let the user master the course. No fewer, and none that the course does not need. There is no fixed number
- **Module sections** group related modules, stacked from the easiest concepts to the hardest. A section has a name that says what its modules share (`Foundations`, `Data types`)
- **The tier of a section is the difficulty of its concepts,** not the shape of the exercises (`docs/course-structure.md`, "Tiers"): `basic`, `medium` or `advanced`. Decide it **from your own knowledge of the language or the library**: how many other concepts a concept needs first, how abstract it is, how easy it is to get wrong. Do not match it to a list of examples
- **A section is cohesive:** every module in it fits the section's name and tier. When you add or move a module, check it against its section. If it does not fit, rename the section so the name covers all its modules, move the module to the section where it belongs, or make a new section for it (a module that is a rare topic goes to its own section with the medium tier, not into a basic one). Check the sections after a split too: a section must never become a pile of unrelated modules
- **The order is the order of study** inside a section and across sections, and it follows what depends on what
- **One session per module to start:** `"sessions": 1`. The exercises skill raises it up to 3 when a module needs more, and asks this skill for more modules when a concept needs more than 3 sessions
- **A module made by splitting a concept** is named for its theme (`Slices in functions`, `Maps of maps`), never `Slices 2`. Each one is still one concept
- A core course covers the language itself, nothing tied to an external library. An ecosystem course covers its own library

## Writing
- **A general call is a call with no request.** On a course with no `course.json`, write the whole file and create the course folder. On a course that already has one, **warn** that it has, add only the modules that are missing, and touch nothing that is already there
- **To change a module that exists** (rename, move, remove, change a tier), the user names it in a request. Change only what the request asks and leave everything else exactly as it is
- Keep every existing id, and the `id` of the course

## Called by the exercises skill
When `ingrain-exercises` needs more modules because a concept does not fit in 3 sessions, it calls this skill with a request: the modules to add, the module they go right after, and the tier. Add them right after that module, in its section when they fit it, or in a section of their own, with the right tier, when they do not (rare topics get a medium section of their own). Each starts with `"sessions": 1` and has no file. Do not touch the module they follow. This skill never calls `ingrain-exercises` back

## Modifying: be careful
- **Never rename or reuse the id of a module that has a file.** The saved progress points at `<language>/<course>/<module>`
- **Never remove a module that has a file.** A module with content stays in the list. If the request asks for it, say you did not, and why
- A module with **no file** has no content and no progress, so it can be renamed, moved or removed freely
- Names, tiers and the names of the sections can change freely. Reordering is allowed, but say so: it changes the order in which users study
- Keep `sessions` as it is for a module that has a file: the number must match its file
- Adding a module next to a module that has a file is allowed, and leaves the existing one exactly as it is
- If the request would remove, rename or reorder modules that exist, say exactly what will change and ask before writing

## Finish
Run `cargo test --test content` and fix everything it reports. Among other checks, it fails when an id is not a slug or repeats, when a module file is not listed in `course.json`, and when a written module has a different id or number of sessions than `course.json` says.

Then report:
- what you added, changed or left as it was
- what the request asked and you did not do, and why
- the modules that have no file yet, so `ingrain-exercises` is the next step for each
