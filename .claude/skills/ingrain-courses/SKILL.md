---
name: ingrain-courses
description: Create or modify the courses of one layer (Core or Ecosystem) of a language in Ingrain's catalog.json. Use when the user wants the list of courses of a layer written, extended, renamed or reordered.
---

# ingrain-courses

Writes the courses of one layer of a language, in `content/prod/<language>/catalog.json`, and calls `ingrain-modules` to draw the modules of every course it creates. It is the first of three skills: this one, `ingrain-modules` (the modules of one course) and `ingrain-exercises` (the exercises of each module).

Arguments: `$ARGUMENTS` is `<language> <layer> [request]`. The layer is `core` or `ecosystem`. Without a request, write the whole list of courses of that layer. With a request, change the list as it says.

## Read first
1. `docs/course-structure.md`: the hierarchy (language, layer, course, module section, module) and what each level means
2. `docs/content-structure.md`: "Folders", "Files" and "Ids", for the shape of `catalog.json` and the rules of ids
3. The current `content/prod/<language>/catalog.json`, and the folders in `content/prod/<language>/`: a course whose folder exists is **written** and has content

The shape of a layer in the catalog:
```json
{ "id": "core", "name": "Core", "courses": [{ "id": "syntax", "name": "Syntax" }] }
```
A course id is a slug (lowercase letters, digits and hyphens), and it is also the name of its folder. The name is what the user reads.

## What the courses of a layer must be
- **A course is one topic** of the language, and its modules are the concepts of that topic
- **Core** is the language itself. Its courses must together evaluate every everyday aspect of the language, and nothing tied to an external library or crate
- **Ecosystem** is the libraries and frameworks of the language, one course for each (for example `cobra`, `gin`, `ratatui`). The same rule of mastery holds
- **Mastery is the measure:** completing all the courses of a layer must let the user master that layer. Add what is missing, leave out what is not needed to master it
- **There is no fixed number.** A layer has as many courses as it needs. If that is 100, write 100: the explorer column scrolls
- **The order is the order of study.** The first course is the first one to learn, and the app proposes the next course in this order. Decide what a layer needs and in what order **from your own knowledge of the language or the library** (what everyday code uses, what depends on what, how hard each topic is), not from a list given to you
- A course name is short and plain (`Syntax`, `Pointers`, `Error handling`)

## Writing
- **A general call is a call with no request.** On a layer with no courses, write the full list, in the order of study. On a layer that already has courses, **warn** that it has, add only the courses that are missing, and touch nothing that is already in the catalog
- **To change a course that exists** (rename, reorder, remove), the user names it in a request. Change only what the request asks and leave everything else exactly as it is
- Keep every existing id. Keep the layer's `id` and `name`

## Modifying: be careful
- **Never rename or reuse the id of a course that has a folder.** The saved progress points at `<language>/<course>/<module>`, so a renamed id loses what the user did
- **Never remove a course that has a folder.** A course with content stays in the catalog. If the request asks for it, say you did not, and why
- A course with **no folder** has no content and no progress, so it can be renamed, reordered or removed freely
- Names can change freely
- Reordering is allowed, but say so: it changes the order in which users are taken through the courses
- If the request would remove, rename or reorder courses that exist, say exactly what will change and ask before writing

## The modules of each new course
The catalog only stores an id and a name, so what a course covers would be lost after this run. Keep it by drawing the modules now:
1. For each course you **created** (one with no folder yet), decide its **scope**: one line of what it covers, and what it leaves to its neighbors
2. Then call the `ingrain-modules` skill for it, as `<language> <layer> <course>`, and pass as its request the scope of the course and the scopes of the courses around it, so that no topic is covered twice and none falls between two courses
3. Do it one course at a time, in the order of study. Do not call it for a course that already has a folder, and do not touch the modules of an existing course

## Finish
Run `cargo test --test content` and fix everything it reports. Among other checks, it fails when a course id is not a slug, when an id repeats, and when a course folder is not listed in the catalog.

Then report:
- what you added, changed or left as it was
- what the request asked and you did not do, and why
- for each new course, its scope and the modules `ingrain-modules` drew for it
