use catppuccin::PALETTE;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph, Wrap},
};

// Draws a card with the output. Text that does not fit is cut at the bottom
pub fn render_output(frame: &mut Frame, area: Rect, title: &str, lines: Vec<Line>) {
    let palette = PALETTE.mocha.colors;
    let badge = Style::new()
        .fg(palette.crust.into())
        .bg(palette.sky.into())
        .add_modifier(Modifier::BOLD);

    let card = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(palette.sky.into()))
        .padding(Padding::symmetric(2, 1))
        .title(Span::styled(format!(" {title} "), badge));

    let paragraph = Paragraph::new(lines).block(card).wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}
