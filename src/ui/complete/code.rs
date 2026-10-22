use catppuccin::PALETTE;

use crate::app::CompleteState;
use crate::highlight::Highlighted;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

// Width of a code row prefix: 3 spaces + 2 digits + 2 spaces, as in the exercise editor
const GUTTER_WIDTH: u16 = 7;

// Draws the finished file as an editor window. Returns true when it overflows, so it can be scrolled
pub fn render_code(frame: &mut Frame, area: Rect, state: &CompleteState) -> bool {
    let palette = PALETTE.mocha.colors;
    let border = Style::new().fg(palette.sky.into());
    let card = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(border);
    let inner = card.inner(area);
    frame.render_widget(card, area);

    let [title_bar, rule, code_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .areas(inner);

    // Code columns left once the gutter is taken
    let width = code_area.width.saturating_sub(GUTTER_WIDTH).max(1) as usize;
    let colors = Highlighted::new(state.explorer.language.id(), &state.code.join("\n"));
    let mut lines = Vec::new();

    for (i, text) in state.code.iter().enumerate() {
        let spans: Vec<Span> = text
            .chars()
            .enumerate()
            .map(|(column, c)| Span::styled(c.to_string(), colors.style_at(i, column)))
            .collect();
        // Soft wrap: one terminal line for every `width` chars of the row
        let parts: Vec<&[Span]> = if spans.is_empty() {
            vec![&[]]
        } else {
            spans.chunks(width).collect()
        };

        for (n, part) in parts.into_iter().enumerate() {
            // The line number only on the first piece of a wrapped row
            let prefix = if n == 0 {
                format!("   {:>2}  ", i + 1)
            } else {
                " ".repeat(GUTTER_WIDTH as usize)
            };
            let mut line = vec![Span::raw(prefix)];

            line.extend_from_slice(part);
            lines.push(Line::from(line));
        }
    }

    // How far the code can scroll. The offset is clamped so it never passes the end
    let max_scroll = lines.len().saturating_sub(code_area.height as usize);
    let offset = state.scroll.min(max_scroll);

    // 1. Render Title bar
    let file = match state.explorer.language.id() {
        "go" => "main.go",
        _ => "main.rs",
    };
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("●", Style::new().fg(palette.red.into())),
        Span::raw(" "),
        Span::styled("●", Style::new().fg(palette.yellow.into())),
        Span::raw(" "),
        Span::styled("●", Style::new().fg(palette.green.into())),
        Span::raw(format!("   {file}")),
    ]);

    frame.render_widget(Paragraph::new(title), title_bar);
    // A rule that joins the card's side borders: ├────┤
    let rule_line = format!("├{}┤", "─".repeat(area.width.saturating_sub(2) as usize));
    frame.render_widget(
        Paragraph::new(rule_line).style(border),
        Rect {
            x: area.x,
            width: area.width,
            ..rule
        },
    );

    // 2. Render Code
    frame.render_widget(Paragraph::new(lines).scroll((offset as u16, 0)), code_area);

    // Scrollbar on the card border, only over the code rows and only when the code overflows
    if max_scroll > 0 {
        let mut bar = ScrollbarState::new(max_scroll + 1)
            .viewport_content_length(code_area.height as usize)
            .position(offset);

        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight),
            Rect {
                x: area.x,
                width: area.width,
                ..code_area
            },
            &mut bar,
        );
    }

    max_scroll > 0
}
