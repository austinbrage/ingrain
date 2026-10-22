# Exercise · Guide alternative · One card with a dim rule

Status: Draft

The guide as it was before the two cards of `03-ghost-wide.md`: one card with a `guide` badge, the exercise title in bold, the intro, a dim rule, and the chunk guide. It was built and replaced on 2026-10-11. It is kept here, with its code, in case it comes back.

```
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ Rust · Syntax · Loops                                                 session 1/4 · exercise 1/3 ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
                                                                                                    
╭───────────────────────────────────────────────────────────────╮  ╭ guide ────────────────────────╮
│ ● ● ●   main.rs                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │ Repeat with for               │
│ ▶  1  █fn main() {                                            │  │                               │
│          ╰─ the function every Rust program starts at         │  │ Inside main, loop three       │
│    2  }                                                       │  │ times.                        │
│                                                               │  │                               │
│                                                               │  │ ───────────────────────────── │
│                                                               │  │                               │
│                                                               │  │ Every Rust program starts at  │
│                                                               │  │ main. Type the function with  │
│                                                               │  │ its braces.                   │
│                                                               │  │                               │
├───────────────────────────────────────────────────────────────┤  │                               │
│ Rust                                              Ln 1, Col 1 │  │                               │
╰───────────────────────────────────────────────────────────────╯  ╰───────────────────────────────╯
                                                                                                    
╔══════════════════════════════════════════════════════════════════════════════════════════════════╗
║ [░░░░░░░░░░░░] 0/3 chunks                                                 Esc back   Ctrl-Q quit ║
╚══════════════════════════════════════════════════════════════════════════════════════════════════╝
```

## How it looked
- The badge says `guide` (bold `crust` on `sky`), the border is `sky`, and the card has one row of padding under the badge
- The title is bold. The rule is a line of `─` in `overlay0`, as wide as the card's inner width, with a blank row above and below. It is drawn only when the chunk has a `guide`
- It is simpler: one `Paragraph`, no height to measure

## Other ways to divide the two texts that were drawn
- **A rule joined to the card sides** (`├───┤`, like the editor window). It needs the card split in two areas
- **A rule with a label** (`── this step ──`). More explicit, one more word to read
- **An accent bar** (`▎` in `sky` on the left of the chunk guide). No extra row, but the bar repeats on every wrapped line
- **Color only:** the exercise text dim and the chunk guide normal. The most subtle

## The code
`render_guide` in `src/ui/exercise/typing.rs` (it is in `guide/mod.rs` now), before the two cards:

```rust
fn render_guide(frame: &mut Frame, area: Rect, typing: &Typing, chunk: &Chunk) {
    let palette = PALETTE.mocha.colors;
    let badge = Style::new()
        .fg(palette.crust.into())
        .bg(palette.sky.into())
        .add_modifier(Modifier::BOLD);

    let card = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(palette.sky.into()))
        .title(Span::styled(" guide ", badge))
        .padding(Padding::new(1, 1, 1, 0));

    let rule = "─".repeat(card.inner(area).width as usize);

    let title = Line::styled(
        typing.title.as_str(),
        Style::new().add_modifier(Modifier::BOLD),
    );
    let mut text = Text::from(title);

    text.push_line("");
    text.extend(markdown(&typing.guide));

    if let Some(guide) = &chunk.guide {
        text.push_line("");
        text.push_line(Line::styled(rule, Style::new().fg(palette.overlay0.into())));
        text.push_line("");
        text.extend(markdown(guide));
    }

    let guide = Paragraph::new(text).block(card).wrap(Wrap { trim: false });

    frame.render_widget(guide, area);
}
```
