# Anchors for ingrain-score

One real exercise per verdict, with why. Filled in after the first run on Go `syntax` (step 2 of `17-fix-condescending-content.md`), from content the author has approved. Until then the scorer has no anchors and says so in its report.

## Condescending
## Engaging
## Engaging at the first module, condescending later (the bar moves)
- **Go, fill the import:**
  ```go
  package main

  import "___"

  func main() {
      fmt.Println("hello")
  }
  ```
  Answer: `fmt`. The reader has never seen a Go package, so the simple form is new knowledge in the first module of the language. The same fill-in in a `medium` module is condescending

## Condescending at every position (the bar never covers it)
- **Go, add a comment:**
  ```go
  func main() {
      // A comment explains the code
      fmt.Println("Comments are ignored.")
  }
  ```
  Every programmer knows what a comment is. The `//` form and `/* */` are one line of syntax, enough for a note inside another exercise

## Overwhelming

## Irrelevant (lax: only a clear case like this)
- **Go: bit clear with `&^`.**
  ```go
  a, b := 12, 10
  fmt.Println(a &^ b)   // 4
  ```
  Nobody could guess `&^`, but almost no everyday Go code writes it. It teaches bit manipulation for its own sake, not something the reader needs. Compare with `1 << iota` flags, which are rare but real and get one light exercise
