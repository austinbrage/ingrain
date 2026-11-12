# Rust core: boundaries for the exercises

What the module names do not show. Read by `ingrain-exercises` before it writes the exercises of a Rust core module. The modules themselves are in each `course.json`; this file holds only where a course stops and the rules that follow from it.

Rules:
- An exercise uses what the module teaches and what the modules before it taught (the catalog order, `content/prod/rust/catalog.json`). Never a concept from a later course
- A line here wins over what the module name suggests
- When a module is renamed, moved or removed, change its line here

## Where each course stops
| Course | Leaves to |
|---|---|
| syntax | functions, `String`, `Vec`, enums and patterns (only the basic `match` of `match-basics`) |
| functions | ownership of arguments, references, generics, closures |
| ownership | references (borrowing), the `Drop` trait and `Rc` (smart-pointers) |
| borrowing | lifetime annotations, `RefCell`, `&str` slicing (strings-and-text) |
| strings-and-text | `Display` impls (standard-traits), `Cow` (smart-pointers) |
| collections | iterator adapters, `Hash` and `Ord` traits |
| structs-and-methods | trait definitions, lifetimes in structs, enums |
| enums | match patterns beyond the basics (pattern-matching), error idioms (error-handling) |
| pattern-matching | enum definitions |
| traits | generic parameters in depth (generics), std traits, `dyn` |
| generics | `dyn Trait`, lifetimes |
| standard-traits | smart-pointer behavior, error types |
| trait-objects | generics, `Rc<dyn>` |
| closures | iterator adapters, function pointers (functions) |
| iterators | closure mechanics, collection APIs |
| error-handling | the `Result` data shape (enums) |
| testing | cargo flags (tooling) |
| lifetimes | basic borrowing |
| smart-pointers | `Arc` and `Mutex` (concurrency) |
| modules-and-crates | cargo commands, features and workspaces (tooling) |
| macros | procedural macros (out of scope), derive |
| concurrency | `Rc` and `RefCell`, `async` |
| async | OS threads, runtimes such as tokio (ecosystem) |
| standard-library | collections, `Result` idioms |
| tooling | writing tests, module visibility |

## Rules per module
**syntax**
- `printing`: the first module of the course, and it presents the language: the `fn main` program, `println!` with `{}` values, `use` and `std::` paths (`std::cmp::max`, `std::f64::consts::PI`), a few simple methods on a text literal (`to_uppercase`, `repeat`, `len`) as a taste, one brief comments exercise, then the formatting forms (positional, `{:?}`, width, alignment, bases). No variables, `let`, `if` or loops. `format!` and real `String` work belong to strings-and-text
- `match-basics`: integers, literals and `_`; no enums. It is the first contact with `match`
- No exercise needs a function other than `main`, a `String` or a `Vec`

**ownership, borrowing**
- ownership exercises use no references; borrowing starts the `&`
- `slices` and `mutable-slices`: `&[T]` only, not `&str`

**collections**
- `vec-access`, `hashmap`: use `Option` only as a black box (`unwrap`, `unwrap_or`, `if let Some`), because enums comes later
- `sort-dedup-and-search`: only `sort`, `sort_unstable`, `dedup`, `binary_search`, `contains`; no closure-keyed variants

**strings-and-text**
- `format-macro`: `format!` only, no `Display` impl
- `parsing-numbers`: use `Result` only as a black box (`unwrap`, `unwrap_or`, `if let Ok`); no `match` teaching, no `?`

**structs-and-methods**
- `derive-basics`: first contact with `#[derive]`; what each derived trait means is in standard-traits

**enums**
- Use `match` only as `match-basics` showed it; no guards or deep destructuring
- `option`, `result`: the data shape only, no combinators and no `?`

**pattern-matching**
- `enums-and-exhaustiveness`: matching on enums and exhaustiveness, not the first contact with `match`
- `literal-and-range-patterns`: owns plain or-patterns (`1 | 2`) and ranges
- `nested-and-or-patterns`: patterns inside patterns, including or-patterns inside them (`Some(1 | 2)`); it never re-teaches plain `x | y`

**traits**
- `impl-trait-arguments`: only the `impl Trait` argument form; `<T: Trait>` and `where` are taught in generics
- `associated-items`: associated consts and functions only; associated types stay in generics
- `impl-trait`: teach only what `impl-trait-arguments` does not, so the two do not overlap

**generics**
- `generic-function-bounds`: teaches the `<T: Trait>` form. `where-clauses`: teaches `where`
- `turbofish-and-inference`: `collect::<Vec<_>>()` and `parse::<i32>()` are the examples

**standard-traits**
- `clone-and-copy-traits`: derive vs manual impl, and `Copy` requires `Clone`; move and copy semantics stay in ownership
- `deref`: the trait only, no `Rc` or `Box`

**closures**
- `closures-as-arguments`: include `sort_by`, `sort_by_key` and `binary_search_by` as examples
- No iterator adapters (iterators comes next)

**iterators**
- `custom-iterators`, `returning-iterators`: owned data only; no `&'a [T]` struct and no `+ '_` (borrowed iterators belong to lifetimes)
- `enumerate-and-friends`: still covers `rev` and `chain` as ways of shaping which items you walk

**error-handling**
- Closures are allowed in the combinator modules: functional style comes first
- `handling-results`: the decision to recover or propagate, not the `match` on `Ok`/`Err`
- `the-question-mark`: covers `?` on both `Result` and `Option`

**testing**
- `unit-test-modules`: the `#[cfg(test)] mod tests` + `use super::*` idiom as boilerplate; `mod`, `pub` and `use` are taught in modules-and-crates
- `integration-tests`, `doc-tests`: no module concepts beyond that idiom
- `ignored-tests`: only the `#[ignore]` attribute; the flags that run ignored tests belong to tooling `test-flags`

**macros**
- `std-macros-expanded`: how `println!`, `vec!` and `assert!` expand, not how to use them (their use was taught before); plus `todo!`, `unreachable!`, `matches!`, `dbg!`

**modules-and-crates**
- `re-exports-and-public-api`: `pub use` and shaping the crate's public surface
- `dependencies`: using an external crate's paths with `use`; `Cargo.toml` syntax, versions and features belong to tooling

**concurrency, smart-pointers**
- `arc` is here; `Rc`, `Weak` and `RefCell` are in smart-pointers, which never uses `Arc` or `Mutex`

**async**
- No runtime crate: executors are hand-written (`wakers`, `block-on`, `task-queue`, `join`)
