use crate::highlight::Highlighted;

use catppuccin::PALETTE;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

// The snippet of a slide in a small read-only window, labeled with the language
pub fn render_snippet(frame: &mut Frame, area: Rect, language: &str, label: &str, code: &str) {
    let window = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(PALETTE.mocha.colors.sky.into()))
        .title(format!(" {label} "));
    let inner = window.inner(area);
    frame.render_widget(window, area);

    let colors = Highlighted::new(language, code);
    let lines: Vec<Line> = code
        .lines()
        .enumerate()
        .map(|(row, text)| {
            let mut spans = vec![Span::raw("  ")];

            spans.extend(
                text.chars()
                    .enumerate()
                    .map(|(column, c)| Span::styled(c.to_string(), colors.style_at(row, column))),
            );
            Line::from(spans)
        })
        .collect();

    frame.render_widget(Paragraph::new(lines), inner);
}
