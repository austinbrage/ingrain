use crate::app::HomeLanguage;
use crate::highlight::Highlighted;

use catppuccin::PALETTE;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

const RUST_FILE: &str = "main.rs";
const RUST_SNIPPETS: &[&str] = &[
    " fn main() {\n     println!(\"hello, ingrain\");\n }",
    " unsafe fn text_my_ex() {}",
    " fn never_give_up() -> ! {\n     loop { try_again() }\n }",
    " fn deploy_on_friday() {\n     panic!(\"yolo\")\n }",
];

const GO_FILE: &str = "main.go";
const GO_SNIPPETS: &[&str] = &[
    " func main() {\n     fmt.Println(\"hello, ingrain\")\n }",
    " if err != nil {\n     learn(err)\n }",
    " func ship() {\n     defer fixLater()\n }",
    " for !tired() {\n     practice()\n }",
];

// Ticks to type one char
const TYPE_TICKS: u64 = 2;
// Ticks the full snippet stays on screen
const HOLD_TICKS: u64 = 40;

pub fn render_demo(frame: &mut Frame, area: Rect, tick: u64, language: HomeLanguage) {
    let palette = PALETTE.mocha.colors;
    let border = Style::new().fg(palette.overlay0.into());
    let card = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(border);
    let inner = card.inner(area);
    frame.render_widget(card, area);

    let [title_bar, rule, code] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .areas(inner);

    let (file, snippets) = match language {
        HomeLanguage::Rust => (RUST_FILE, RUST_SNIPPETS),
        HomeLanguage::Go => (GO_FILE, GO_SNIPPETS),
    };

    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("●", Style::new().fg(palette.red.into())),
        Span::raw(" "),
        Span::styled("●", Style::new().fg(palette.yellow.into())),
        Span::raw(" "),
        Span::styled("●", Style::new().fg(palette.green.into())),
        Span::styled(
            format!("   {file}"),
            Style::new().fg(palette.overlay1.into()),
        ),
    ]);

    frame.render_widget(Paragraph::new(title), title_bar);
    render_rule(frame, area, rule, border);

    let (snippet, typed) = demo_frame(snippets, tick);
    frame.render_widget(Paragraph::new(typed_lines(language, snippet, typed)), code);
}

// The typed part of the snippet, a span per char with its color, and the cursor after the last one
fn typed_lines(language: HomeLanguage, snippet: &str, typed: usize) -> Vec<Line<'static>> {
    let palette = PALETTE.mocha.colors;
    let colors = Highlighted::new(language.id(), snippet);
    let mut lines = vec![Vec::new()];
    let mut row = 0;
    let mut column = 0;

    for ch in snippet.chars().take(typed) {
        if ch == '\n' {
            lines.push(Vec::new());
            row += 1;
            column = 0;
            continue;
        }

        lines[row].push(Span::styled(ch.to_string(), colors.style_at(row, column)));
        column += 1;
    }

    lines[row].push(Span::styled("█", Style::new().fg(palette.text.into())));

    lines.into_iter().map(Line::from).collect()
}

// The snippet on screen at this tick and how many of its chars are typed
pub fn demo_frame(snippets: &[&'static str], tick: u64) -> (&'static str, usize) {
    let total: u64 = snippets.iter().map(|snippet| cycle_ticks(snippet)).sum();
    let mut tick = tick % total;

    for &snippet in snippets {
        let cycle = cycle_ticks(snippet);

        if tick >= cycle {
            tick -= cycle;
            continue;
        }

        let length = snippet.chars().count() as u64;
        let typing = length * TYPE_TICKS;
        let held = typing + HOLD_TICKS;

        let typed = if tick < typing {
            tick / TYPE_TICKS
        } else if tick < held {
            length
        } else {
            length - (tick - held)
        };

        return (snippet, typed as usize);
    }

    unreachable!("the tick is below the total of the cycles")
}

// Ticks one snippet takes: typing, holding and erasing
fn cycle_ticks(snippet: &str) -> u64 {
    let length = snippet.chars().count() as u64;

    length * TYPE_TICKS + HOLD_TICKS + length
}

// A horizontal rule that joins the card's side borders: ├────┤
fn render_rule(frame: &mut Frame, card: Rect, row: Rect, style: Style) {
    let rule = format!("├{}┤", "─".repeat(card.width.saturating_sub(2) as usize));

    frame.render_widget(
        Paragraph::new(rule).style(style),
        Rect {
            x: card.x,
            width: card.width,
            ..row
        },
    );
}
