# Ingrain
Ingrain: learn and keep a programming language, by hand

## Modules

On src folder:

- `app` handles the state and key events
- `content` handles the JSON parsing
- `ui` handles the screens drawing

> `ui` reads `app`, & `app` reads `content`

## Tests

`cargo test` runs the screen tests in `tests/`, on the fixtures in `content/test` and `tests/fixtures`.

To see what a test sees, add `println!("{}", screen.text());` to it and run:

```
cargo test <test_name> -- --nocapture
``` 
