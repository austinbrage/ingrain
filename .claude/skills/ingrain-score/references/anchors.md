# Anchors for ingrain-score

Examples of each verdict, with why. All are Go, but the rules they show hold in any language: read the idea, not the syntax.

## How to use these anchors
The rules live in `SKILL.md`: "The scale" sets the default (engaging) and the three verdicts, and "What it measures" says when to move off it. The bar of the module's position lives in `writing-rules.md`, "The bar moves with position". These examples only show those rules in code: find the closest example to the exercise, read its **Why** line, and keep engaging when in doubt.

## Condescending
- **Go, trace the calls (`output` quiz):**
  ```go
  func start() {
      fmt.Println("start")
  }

  func stop() {
      fmt.Println("stop")
  }

  func main() {
      start()
      stop()
  }
  ```
  Options: "start, then stop" (right), "stop, then start", "only start", "It does not compile". **Why condescending:** anyone gets it right by reading top to bottom, and it hinges on none of the five hinges
- **Go, fill-blank on a word every language needs:**
  ```go
  package main

  import "fmt"

  ___ greet(name string) {
      fmt.Println("hello,", name)
  }

  func main() {
      greet("Ana")
  }
  ```
  Answer: `func`. **Why condescending:** the word only declares a function, and the call to `greet` gives it away. It teaches no rule, trap or idiom
- **Go, print a few variables (typing):**
  ```go
  func main() {
      name := "Ana"
      age := 31
      city := "Lima"

      fmt.Println(name)
      fmt.Println(age)
      fmt.Println(city)
  }
  ```
  Output: `Ana`, `31`, `Lima`. **Why condescending:** three `Println` calls of the same shape, with no new form, rule or trap. The reader has already typed `:=` and `Println`, so it adds nothing new

## Engaging
- **Go, shadowing (`output` quiz):**
  ```go
  func main() {
      attempts := 0

      if attempts == 0 {
          attempts := 3
          attempts++
          fmt.Println("inner:", attempts)
      }

      fmt.Println("outer:", attempts)
  }
  ```
  Options: "inner: 4, then outer: 0" (right), "inner: 4, then outer: 4", "inner: 3, then outer: 0", "It does not compile". **Why engaging:** it hinges on a rule of Go: `:=` inside the block declares a new variable instead of changing the outer one. A programmer from another language would likely get it wrong, and the program does something and prints a result
- **Go, a rule shown in a program (typing):**
  ```go
  func main() {
      status := 404

      switch status {
      case 200:
          fmt.Println("ok")
      case 404:
          fmt.Println("not found")
      default:
          fmt.Println("unknown")
      }
  }
  ```
  Output: `not found`. **Why engaging:** it hinges on one rule: each case ends by itself, with no `break` and no fall-through, unlike C and JavaScript. One hinge, with notes on that piece only
- **Go, fill-blank on a practiced idiom:**
  ```go
  package main

  import "fmt"

  func main() {
      stock := map[string]int{"tea": 3}

      if qty, ___ := stock["milk"]; !ok {
          fmt.Println("out of stock", qty)
      }
  }
  ```
  Answer: `ok`. **Why engaging:** it hides the name of the flag in the comma-ok lookup, a Go idiom the reader has already typed. The `!ok` in the condition makes the answer certain, and the gap holds the idea of the exercise

## Engaging at the first module, condescending later (the bar moves)
- **Go, fill the import:**
  ```go
  package main

  import "___"

  func main() {
      fmt.Println("hello")
  }
  ```
  Answer: `fmt`. **Why engaging at the first module:** the reader has never seen a Go package, so the simple form is new knowledge in the first module of the language. The same fill-in in a `medium` module is condescending

## Engaging at `advanced`: a new complex concept, taught plainly
- **Go, a semaphore (typing, in a concurrency module):**
  ```go
  func main() {
      sem := make(chan struct{}, 2)

      sem <- struct{}{}
      sem <- struct{}{}
      fmt.Println(len(sem), cap(sem))

      <-sem
      fmt.Println(len(sem))
  }
  ```
  Output: `2 2` then `1`. **Why engaging at `advanced`:** the concept is new to the reader (a buffered channel used as a counter that limits how many may run at once), and the exercise shows how it works step by step. It needs no trap: a complex concept the reader has not met counts at any tier, taught plainly

## Engaging at `advanced`: a concept already taught, used another way
- **Go, a nil pointer inside an interface (typing, in an interfaces module):**
  ```go
  type MyErr struct{}

  func (e *MyErr) Error() string {
      return "my error"
  }

  func main() {
      var e *MyErr
      var err error = e

      fmt.Println(e == nil)
      fmt.Println(err == nil)
  }
  ```
  Output: `true` then `false`. **Why engaging at `advanced`:** it hinges on an interaction between two features, interfaces and nil pointers. `e` is nil, but once it is in an `error` interface, the interface still holds the type `*MyErr`, so `err == nil` is false. Both lines print, so the reader sees the two results side by side. It is a trap people really hit, and a programmer from another language would not guess it. A note goes on `var err error = e`, for example "The interface keeps the type, so it is not nil."
- **Contrast, the same topic at `advanced`:** a plain `if err != nil { return err }` exercise is condescending here. The form is from the first modules and hinges on nothing new

## Engaging when typed, condescending as a quiz (a form counts once)
- **Go, type the loop (typing):**
  ```go
  func main() {
      total := 0

      for i := 1; i <= 3; i++ {
          total += i
      }

      fmt.Println(total)
  }
  ```
  Output: `6`. **Why engaging:** the reader types a Go form for the first time: no parentheses around the header, braces required, and `for` is the only loop keyword. Every programmer knows loops, but not how Go writes one, so a syntax exercise like this is never condescending
- **Go, trace the same loop afterwards (`output` quiz):**
  ```go
  total := 0

  for i := 1; i <= 3; i++ {
      total += i
  }

  fmt.Println(total)
  ```
  Options: `3`, `5`, `6` (right), `10`. **Why condescending:** the reader has already typed this exact form, and a syntax form counts once, when it is first typed. Tracing it again adds nothing new. The loop is not the problem here, the quiz after it is

## Condescending at every position (the bar never covers it)
- **Go, add a comment:**
  ```go
  func main() {
      // A comment explains the code
      fmt.Println("Comments are ignored.")
  }
  ```
  **Why condescending:** every programmer knows what a comment is. The `//` form and `/* */` are one line of syntax, enough for a note inside another exercise

## Overwhelming
- **Go, density: five new things in one structs exercise (typing):**
  ```go
  type Server struct {
      Host string
      Port int
  }

  func NewServer(host string, port int) *Server {
      return &Server{Host: host, Port: port}
  }

  func (s *Server) Addr() string {
      return fmt.Sprintf("%s:%d", s.Host, s.Port)
  }

  func main() {
      servers := map[string]*Server{
          "api": NewServer("localhost", 8080),
      }
      fmt.Println(servers["api"].Addr())
  }
  ```
  Output: `localhost:8080`. **Why overwhelming (density):** five new things arrive together: the struct type, a constructor returning a pointer, `&Server{Host: ...}` with named fields, a pointer-receiver method and a map of pointers. The reader cannot tell which piece the exercise is about. Fix: one exercise per hinge, the struct and its literal first, then the method, then the constructor and the map
- **Go, density: three error forms in one error-handling exercise (typing):**
  ```go
  var ErrEmpty = errors.New("empty name")

  func greet(name string) (string, error) {
      if name == "" {
          return "", fmt.Errorf("greet: %w", ErrEmpty)
      }
      return "hello, " + name, nil
  }

  func main() {
      _, err := greet("")
      if errors.Is(err, ErrEmpty) {
          fmt.Println("caught:", err)
      }
  }
  ```
  Output: `caught: greet: empty name`. **Why overwhelming (density):** a sentinel error variable, wrapping with `%w` and `errors.Is` are each a hinge, and `errors.Is` only makes sense once wrapping is known. Fix: one exercise on the sentinel error and its comparison, then one on wrapping and `errors.Is`
- **Go, gap: an exercise that uses forms no earlier one showed (typing, in a `conditionals` module):**
  ```go
  func main() {
      n, err := strconv.Atoi("42")
      if err != nil {
          fmt.Println("bad number")
          return
      }

      if n > 40 {
          fmt.Println("big")
      }
  }
  ```
  Output: `big`. **Why overwhelming (gap):** the exercise hinges on `if`, but it also uses `strconv`, two results, `err` and `nil`, and no earlier exercise or module showed any of them. The reader meets four unknowns before the one the exercise is about, and one slide cannot explain four things simply, so it is overwhelming and not only a gap. Fix: replace the call with a plain value (`n := 42`), or move the exercise after the modules that teach those pieces

## Before and after (the fix keeps the topic and adds one thing the reader could not guess)
A condescending exercise next to its rewrite on the same topic. The proposed fix is a rewrite, not only a cut.
- **Typing, declaring variables.** Before: print three variables (see "Condescending"). After:
  ```go
  func main() {
      host := "localhost"
      host, port := "example.com", 8080

      fmt.Println(host, port)
  }
  ```
  Output: `example.com 8080`. **What changed:** the topic stays, but now it hinges on a rule of Go. `:=` may reuse a name as long as at least one name on the left is new, so `host` is reassigned and `port` is declared
- **Quiz, the order things run.** Before: the `start()` then `stop()` call trace (see "Condescending"). After (an `output` quiz):
  ```go
  func main() {
      defer fmt.Println("first")
      defer fmt.Println("second")

      fmt.Println("working")
  }
  ```
  Options: "working, second, first" (right), "working, first, second", "first, second, working", "second, first, working". **What changed:** the quiz is still about order, but now it hinges on the Go rule that deferred calls run last in, first out, after the function ends
- **Fill-blank, declaring a function.** Before: the `___ greet(name string) {` gap, answer `func` (see "Condescending"). After:
  ```go
  package main

  import "fmt"

  func divmod(a, b int) (int, int) {
      return a / b, a % b
  }

  func main() {
      quotient, ___ := divmod(17, 5)
      fmt.Println(quotient)
  }
  ```
  Answer: `_`. Guide: "Ignore the remainder." **What changed:** the gap moves from a word any language needs to a Go idiom. An unused variable is an error, so only `_` fits

## Repetitive (a flag, not a verdict)
- **Go, two exercises of a `conditionals` module:**
  ```go
  func main() {
      temp := 30

      if temp > 25 {
          fmt.Println("hot")
      } else {
          fmt.Println("mild")
      }
  }
  ```
  ```go
  func main() {
      score := 80

      if score > 60 {
          fmt.Println("pass")
      } else {
          fmt.Println("fail")
      }
  }
  ```
  **Why repetitive:** the same `if`/`else` shape with another number and other words, so the second exercise adds nothing new. Fix: merge them into one, or make the second show something new, such as an `if` with an init statement

## Missing notes (a flag, only at the low end of the bar)
- **Go, a `variables` module, `basic` tier (typing chunk):**
  ```go
  var count int
  name := "Ana"
  const limit = 10

  fmt.Println(count, name, limit)
  ```
  **Why missing notes:** the chunk types three new forms and none has a note. At the low end of the bar (first module, `basic` tier) every new piece of syntax gets one. Fix: a note on each, in plain text of about 60 characters: on `var count int` "Declares count and starts it at 0.", on `:=` "Declares name and infers its type.", on `const` "Fixed at compile time, never changes."

## Not a gap (a later concept used simply)
- **Go, a `functions` module, `medium` tier.** A slide says: "A slice is a list of values, like `[]string{"go", "cli"}`." Then the typing exercise:
  ```go
  func count(words ...string) int {
      return len(words)
  }

  func main() {
      names := []string{"ana", "li"}
      fmt.Println(count(names...))
  }
  ```
  Output: `2`. **Why it is not a gap:** the slice appears in its plain form, a slide explains it in one sentence without naming the course that teaches it, and the exercise hinges on `...`, not on slices. If no slide explained the slice, it would be a gap, fixed by adding that slide. If the exercise hinged on slices, it would be overwhelming

## Irrelevant (lax: only a clear case like this)
- **Go: bit clear with `&^`.**
  ```go
  a, b := 12, 10
  fmt.Println(a &^ b)   // 4
  ```
  **Why irrelevant:** nobody could guess `&^`, but almost no everyday Go code writes it. It teaches bit manipulation for its own sake, not something the reader needs. Compare with `1 << iota` flags, which are rare but real and get one light exercise
