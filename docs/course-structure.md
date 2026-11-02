# Course Structure

Reference: `legacy/docs/course-structure.md` (frozen). Read it for tiers, sessions, pass criteria and its open questions. This file only records what Ingrain does differently.

## Hierarchy
```
Language (Rust, Go)
 └─ Course section (Core | Ecosystem)
     └─ Course (Rust Core: syntax, borrow checker, concurrency. Rust Ecosystem: ratatui, clap, actix)
         └─ Module section (related modules, stacked from basic to advanced)
             └─ Module (one concept: conditionals, loops, enums)
                 └─ Session (a set of exercises, as in legacy)
                     └─ Exercise
```

## Definitions
- **Course section:** the two main groups of a language. Core is the language itself (syntax, structs, enums). Ecosystem is frameworks and libraries
- **Course:** one topic inside a course section
- **Module section:** the legacy "Section": a group of related modules inside a course, ordered from basic to advanced
- **Module, Session, Exercise:** unchanged from legacy

## Navigation
Home → Languages → Course navigator → Exercise

- **Languages** is its own full-width screen: the brand moment where the user picks Rust or Go (big name, language color)
- **Course navigator** covers course sections, courses, modules and sessions. The language is not a column; it shows in the breadcrumb and as the accent color
- **State:** the path through the tree, one selected index per level. Both view modes share it, along with the keys (↑↓ move, → in, ← out, Esc back to Languages)

### View modes
- **Columns (default):** Miller columns, up to three levels visible at once, with a preview of the selected item's children. The view slides left when the path gets deeper. Draft in `docs/ui/02-explorer.md`
- **Single:** one level per screen, full-screen drill-down. It is the fallback when the terminal is too narrow for columns. Drafts in `docs/ui/` (`03` to `05`)
- The explorer stops at modules: Enter starts the next unfinished session, and there is no sessions column for now (sketch kept in `docs/ui/02-explorer.md`, reasons in `content-structure.md`)
- A setting picks the mode: `auto` (default: columns when wide enough, single when narrow), `columns` or `single`
- Build order: columns first, single when the narrow fallback is needed

## Tiers
A module section belongs to a tier, and **a tier is the difficulty of the concepts it shows, not the shape of the exercises** (the author, 2026-11-02). Every tier uses the same kinds of exercises and the same mix of modes (`docs/content-structure.md`, "The mode of a chunk"). What changes from a tier to the next is what the user learns.

| Tier | Concepts | Access |
|---|---|---|
| basic | the first concepts, the ones everything else builds on | Free |
| medium | concepts that build on the basics | Paid |
| advanced | the hardest concepts of the language or library | Paid |

- **The author gave examples, not a list:** conditionals and loops are basic, and things like traits, async and concurrency are harder. The tier of a concept is decided by whoever writes the modules, from what they know of the language or the library: how many other concepts it needs first, how abstract it is, how easy it is to get wrong. The skill that writes the modules decides it that way, not by matching a list of examples
- **Access** is unchanged from the legacy definition (`legacy/docs/course-structure.md`): basic is free, the others are paid
- A mode where the user types from memory, with no ghost text, is not a tier now, because a tier is about concepts. It is not decided if or how it comes back
- **Rule:** a new tier is added only when the content needs another level of difficulty of concepts
- `tier` is a free string in `course.json` for now, and only displayed. It becomes an enum when it starts driving behavior

## Open
- "Section" names two levels (course section and module section); rename one
- Course sections (Core, Ecosystem) are a different axis from the legacy tiers (basic, medium, advanced); define how they relate
- Is Ecosystem in the first release, or shown as coming soon?
- Minimum terminal width for columns mode, and where the setting lives (config file, settings screen)
- Look of locked (paid) modules and scrolling when a column has more items than fit
