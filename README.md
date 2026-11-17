# Ingrain

**Learn a programming language by typing it. Keep it by never stopping.**

A terminal app that teaches Rust and Go the old-fashioned way: you type every character. No autocomplete, no copy-paste, no AI. Just you, the code and your hands.

## 🎬 Demo

<!-- TODO: add the demo video or GIF here -->

> _Demo coming soon._

## 🧠 Why Ingrain

Agents write the code now. Engineers ship things they could not write alone, and the syntax, the idioms and the debugging instincts fade.

Ingrain is the gym for that muscle. It builds it with the oldest method there is: **typing the code by hand until it sticks.**

## ✨ What makes it different

- **⌨️ Ghost typing.** The code appears faded on screen and you type over it, with a short note under each piece saying what it does and why. Your fingers learn the language, not only your eyes.
- **🎯 Made for people who already code.** No lessons on what a variable or a loop is. Every module teaches what the language does *differently*: the rules, the traps and the idioms.
- **🧩 Three ways to practice.** Typing exercises, quizzes and fill-in-the-blank, mixed in short Duolingo-style sessions.
- **🗺️ Real structure, not a random pile of snippets.** Courses, modules and sessions, from the first concept to the hardest, each opening with a short explanation.
- **🚀 Pick up where you left off.** Progress is saved, failed exercises come back for a retake, and Continue drops you in the exact exercise you stopped at.
- **📦 One binary, fully offline.** All the content ships inside it. No account, no server, no AI at runtime, and nothing to wait for.
- **🎨 A terminal you will enjoy.** Syntax highlighting, animations and a full-screen explorer.

## 📚 What you can learn

| | Core | Ecosystem |
|---|---|---|
| **Rust** | syntax, ownership, borrowing, lifetimes, traits, generics, iterators, closures, async, unsafe, macros... | tokio, serde, rayon, reqwest, regex, tracing, anyhow, thiserror... |
| **Go** | syntax, pointers, interfaces, error handling, generics, concurrency, testing, tooling... | cobra, viper, gin, gorm... |

**1,200+ exercises** and counting. The basics are free, the harder tiers are the paid ones.

## 🏃 Run it

You need [Rust](https://rustup.rs).

```bash
git clone <repo-url> ingrain
cd ingrain
cargo run --release
```

Move with the arrow keys, press Enter to start, `Ctrl-F` to finish a chunk when you are tired, and Esc to go back.

## 🛠️ Under the hood

Built with [ratatui](https://ratatui.rs). The content is plain JSON, kept apart from the screens, so the same courses can power a web version.

```
src/
  app/        state and key events
  content/    JSON parsing
  ui/         screen drawing
  progress/   what the user has done
content/      the courses, embedded in the binary
docs/         structure, UI sketches, glossary
```

`ui` reads `app`, and `app` reads `content`.

```bash
cargo test    # end-to-end tests: drive the app with keys and read the screen
```

To see what a test sees, add `println!("{}", screen.text());` to it and run `cargo test <test_name> -- --nocapture`.
