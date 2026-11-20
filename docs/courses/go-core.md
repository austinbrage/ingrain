# Go core: scope of each course

The scope file of Go core: what the module names do not show. Read by `ingrain-exercises` before it writes the exercises of a Go core module. The modules themselves are in each `course.json`; this file holds the scope (where a course stops) and the rules per module, so the exercises can be written in a later chat. Terms: `docs/glossary.md`.

**Audience.** The reader already codes in another language and knows nothing about Go. Every module teaches the Go form and what differs from other languages, never a general idea.

Rules:
- An exercise uses what the module teaches and what the modules before it taught (the catalog order, `content/prod/go/catalog.json`). A concept from a later course only when used simply and explained briefly on a slide, never by pointing to the course (`writing-rules.md`, "Concepts taught later")
- A line here wins over what the module name suggests
- When a module is renamed, moved or removed, change its line here
- Typing exercises follow "Order of the code" in `writing-rules.md`: helpers first, `main` last. In Go the `auto` chunk is `"package main\n\nimport \"fmt\"\n"`, each helper starts at line 5 and ends with `\n`, and `main` is the last chunk, one balanced block
- A function declared below its caller cannot be shown in a typing exercise. Say it on an intro slide
- A map or slice literal with 3 or more items is written on several lines, one item per line

## Where each course stops
| Course | Leaves to |
|---|---|
| functions | pointers in depth (pointers), slices and maps in depth (collections), `panic` and `recover` and error values in depth (error-handling), methods (structs-and-methods), generic functions (generics), goroutines (concurrency). Recursion is not taught: nothing in it is specific to Go |

## Rules per module
**functions** (redrawn 2026-11-17: the old course taught what a function is; every module now teaches only what Go does differently)

- `declaring-and-returning` (basic): the form, dense and side by side in one typing exercise. The type goes after the name (`name string`), parameters of one type group (`a, b int`), the return type goes after the parameter list, and no return type means no value. Traps: no default values for parameters. Never explain what a function, a parameter or `return` is. No multiple returns here, no variadics. Print with `fmt.Println` only
  ```go
  func add(a, b int) int { return a + b }
  func greet(name string) {}
  ```
- `multiple-returns` (basic, named `Multiple & named returns`): the `(value, error)` idiom, which is the center of Go error handling.
  - Forms: a function returning two values, `result, err := divide(10, 0)` with `if err != nil`, `_` to discard a value, and `f(g())` when `g` returns exactly what `f` takes
  - Named returns: `(x, y int)`, assigned inside and returned with a naked `return`, starting at their zero values. A naked `return` is discouraged in long functions
  - `error` and `errors.New` are used as a black box, plus `nil` for "no error". Wrapping, `%w` and custom error types belong to error-handling. The effect of `defer` on named returns belongs to `defer`
  ```go
  func divide(a, b float64) (float64, error) {
      if b == 0 { return 0, errors.New("divide by zero") }
      return a / b, nil
  }
  ```
- `variadic-functions` (basic): `nums ...int` is a `[]int` inside the function, called with zero or more arguments, and `xs...` spreads a slice at the call site. It must be the last parameter. Trap: `fmt.Println(strs...)` does not compile for a `[]string`, because it wants `[]any`. A spread slice shares its backing array with the caller: use it simply, and explain what a slice is in a sentence
- There is no `pass-by-value` module. Arguments are copied, and what a copy shares is taught in `pointers` (`pointers-and-functions`, `slices-maps-and-pointers`) and `collections` (`append-and-copy`). Do not add one to `functions`
- `function-values` (medium, named `Function values & types`): function literals (anonymous functions), calling one immediately (an IIFE, `func() {...}()`), passing a function as a callback (own helpers such as `apply`), and the nil function (zero value, guard before calling, the optional callback). Named function types, functions in variables and functions returned from functions are left to `closures` and `generics`. Technical names (function literal, IIFE, callback, higher-order function, function type) appear in guides and notes. No map or slice of functions: collections come later. Method values belong to structs-and-methods and generic callbacks to generics. `sort.Slice` is named as a later idiom only; no exercise depends on it
- `closures` (medium): a closure captures the variable, not its value. Forms: a `counter()` returning a function, two closures sharing one variable, and the Go 1.22 rule that each loop iteration has its own variable (`for i := 0; i < 3; i++ { fns = append(fns, func() int { return i }) }`). Say what changed in 1.22, and show the old behavior only as a note. Goroutine capture belongs to concurrency. Loops are already known from `syntax`
- `defer` (medium): the resource idiom first (`defer f.Close()`, with `os.Open` as an idiom the reader copies and `fmt.Println("done")` for plain ordering), then the traps, each its own exercise:
  - arguments are evaluated at the `defer` statement, not when it runs
  - several defers run last in, first out
  - a deferred closure can change a named return (`func f() (n int) { defer func() { n *= 2 }(); return 5 }` returns 10), which builds on `multiple-returns`
  - a `defer` inside a loop runs when the function ends, not at the end of each iteration
  - `recover` and `panic` belong to error-handling and are never used here
- `what-go-leaves-out` (medium, named `No overloads & init`): there is no overloading (a second `connect` is a "redeclared" compile error), no default arguments and no optional parameters. The substitutes taught are a different name and a zero value that means "not set". A struct of options and functional options are only named on the intro, never typed or quizzed: they need structs and belong to `structs-and-methods`. Then `init()`: runs automatically before `main`, once per package, may appear more than once in a package, and is never called by hand. `main` itself was taught in `syntax` (`program-structure`)

## What `functions` leaves to other courses

`functions` has no `pass-by-value` module (dropped 2026-11-17: the idea is general, and the Go-specific part belongs to the courses below). When `pointers` and `collections` are drawn or redrawn, they must cover it, because no `functions` module does. Each module intro explains what it uses from zero, since the reader has met none of it.

- **pointers / `pointers-and-functions`:** Go copies every argument, so changing a parameter never reaches the caller. The forms `*T`, `&x` and `*p`, and a pointer parameter that changes the caller's variable
- **pointers / `slices-maps-and-pointers`:** a slice or map parameter is a copy of a small header that points at the same data. A write to an element or a key reaches the caller. Reassigning the parameter to a new slice or map only rebinds the copy
- **collections / `append-and-copy`:** `append` inside a function is lost for the caller, so the function returns the slice and the caller stores it (`s = f(s)`)
- **structs-and-methods / a module on options structs** (`constructors` fits): Go has no default parameters (`functions` / `what-go-leaves-out` says so), so a function takes one struct, the caller names only the fields it sets, and the function fills the defaults for fields left at their zero value. `functional-options` builds on it

## Later concepts used simply
| Module | Uses | Taught in |
|---|---|---|
| functions / variadic-functions | `any` | interfaces |
| functions / multiple-returns | `error`, `errors.New` | error-handling |
| functions / defer | `os.Open` as a copied idiom | standard-library |
