use super::{fill_bar, render_footer, render_header, split};
use crate::app::ModuleFailed;

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
const HINT: &str = "Esc back   Ctrl-Q quit ";

pub fn module_failed(frame: &mut Frame, breadcrumb: String, state: &ModuleFailed, tick: u64) {
    let palette = PALETTE.mocha.colors;
    let [header, body, footer] = split(frame.area());
    let total = state.rows.len();
    let passed = state.passed_sessions();

    // 1. Render Header
    let position = format!(
        "session {total}/{total} · exercise {}/{} ",
        state.last_exercises, state.last_exercises
    );
    render_header(frame, header, breadcrumb, position);

    // 2. Render Body: the word, then the card with the table
    let [word_area, _gap, card_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(total as u16 + 6),
    ])
    .flex(Flex::Center)
    .areas(body);

    let word = BigText::builder()
        .pixel_size(PixelSize::Sextant)
        .style(Style::new().fg(palette.peach.into()))
        .lines(vec![Line::from("ALMOST THERE")])
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
        .title(Span::styled(format!(" {} ", state.name), badge));
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

    // One row per session: number, mark, name, then the count and the result at the right edge
    let rows: Vec<Line> = state
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let (mark, result, color) = if row.passed == row.total {
                ("✓", "passed", palette.green)
            } else {
                ("✗", "failed", palette.red)
            };
            let number = format!("  {}  ", index + 1);
            let name = format!("  Session {}", index + 1);
            let count = format!("{} of {}   ", row.passed, row.total);
            let used = number.chars().count()
                + 1
                + name.chars().count()
                + count.chars().count()
                + result.len()
                + 2;
            let gap = (inner.width as usize).saturating_sub(used);

            Line::from(vec![
                Span::styled(number, Style::new().fg(palette.overlay0.into())),
                Span::styled(mark, Style::new().fg(color.into())),
                Span::raw(name),
                Span::raw(" ".repeat(gap)),
                Span::raw(count),
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

    // The bar fills once, up to the sessions passed, and holds
    let bar = fill_bar(tick, passed, Some(total), None);
    frame.render_widget(
        Paragraph::new(format!("  {passed} of {total} sessions passed")),
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
        Span::styled(" retake failed", Style::new().add_modifier(Modifier::BOLD)),
    ]);
    render_footer(frame, footer, enter, HINT);
}
