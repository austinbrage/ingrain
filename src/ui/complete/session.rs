use super::{render_footer, render_header, split};
use crate::app::SessionDone;

use catppuccin::PALETTE;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};
use tui_big_text::{BigText, PixelSize};

// The card never gets wider than this
const CARD_WIDTH: u16 = 66;
// Below this width the word takes two lines
const ONE_LINE_WIDTH: u16 = 56;
// Cells of the bar in the status row
const BAR_WIDTH: usize = 12;
const HINT: &str = "Esc back   Ctrl-Q quit ";

pub fn session_done(frame: &mut Frame, breadcrumb: String, state: &SessionDone) {
    let palette = PALETTE.mocha.colors;
    let [header, body, footer] = split(frame.area());
    let total = state.rows.len();
    let passed = state.passed();

    // 1. Render Header
    let position = format!(
        "session {}/{} · exercise {total}/{total} ",
        state.session + 1,
        state.session_count
    );
    render_header(frame, header, breadcrumb, position);

    // 2. Render Body: the word, then the card with the table
    // The word is 3 rows, and 6 when it takes two lines
    let words = if body.width < ONE_LINE_WIDTH {
        vec!["SESSION", "DONE"]
    } else {
        vec!["SESSION DONE"]
    };
    let [word_area, _gap, card_area] = Layout::vertical([
        Constraint::Length(3 * words.len() as u16),
        Constraint::Length(1),
        Constraint::Length(total as u16 + 6),
    ])
    .flex(Flex::Center)
    .areas(body);

    let word = BigText::builder()
        .pixel_size(PixelSize::Sextant)
        .style(Style::new().fg(palette.sky.into()))
        .lines(words.into_iter().map(Line::from).collect::<Vec<_>>())
        .centered()
        .build();
    frame.render_widget(word, word_area);

    let [card] = Layout::horizontal([Constraint::Max(CARD_WIDTH)])
        .flex(Flex::Center)
        .areas(card_area);
    let badge = Style::new()
        .fg(palette.crust.into())
        .bg(palette.sky.into())
        .add_modifier(Modifier::BOLD);
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(palette.sky.into()))
        .title(Span::styled(
            format!(" Session {} ", state.session + 1),
            badge,
        ));
    let inner = block.inner(card);
    frame.render_widget(block, card);

    let [_top, rows_area, _gap, rule_area, status_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(total as u16),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(inner);

    // One row per exercise: number, mark, title, then the kind and the result at the right edge
    let rows: Vec<Line> = state
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let (mark, result, color) = if row.passed {
                ("✓", "passed", palette.green)
            } else {
                ("✗", "failed", palette.red)
            };
            let number = format!("  {}  ", index + 1);
            let title = format!("  {}", row.title);
            let kind = format!("{}   ", row.kind);
            let used = number.chars().count()
                + 1
                + title.chars().count()
                + kind.chars().count()
                + result.len()
                + 2;
            let gap = (inner.width as usize).saturating_sub(used);

            Line::from(vec![
                Span::styled(number, Style::new().fg(palette.overlay0.into())),
                Span::styled(mark, Style::new().fg(color.into())),
                Span::raw(title),
                Span::raw(" ".repeat(gap)),
                Span::styled(kind, Style::new().fg(palette.overlay0.into())),
                Span::styled(result, Style::new().fg(color.into())),
                Span::raw("  "),
            ])
        })
        .collect();
    frame.render_widget(Paragraph::new(rows), rows_area);

    // The rule joins the card borders, and the status row has the count and a bar of it
    let rule = format!("├{}┤", "─".repeat(card.width as usize - 2));
    frame.render_widget(
        Paragraph::new(rule).style(Style::new().fg(palette.sky.into())),
        Rect::new(card.x, rule_area.y, card.width, 1),
    );

    let filled = passed * BAR_WIDTH / total;
    let bar = Line::from(vec![
        Span::raw("["),
        Span::styled("▓".repeat(filled), Style::new().fg(palette.sky.into())),
        Span::raw("░".repeat(BAR_WIDTH - filled)),
        Span::raw(format!("] {passed}/{total}  ")),
    ]);
    frame.render_widget(
        Paragraph::new(format!("  {passed} of {total} exercises passed")),
        status_area,
    );
    frame.render_widget(Paragraph::new(bar).right_aligned(), status_area);

    // 3. Render Footer
    let enter = Line::from(vec![
        Span::styled(
            " Enter",
            Style::new()
                .fg(palette.sky.into())
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" next session", Style::new().add_modifier(Modifier::BOLD)),
    ]);
    render_footer(frame, footer, enter, HINT);
}
