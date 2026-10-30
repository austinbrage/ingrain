use catppuccin::PALETTE;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph},
};

const WIDTH: u16 = 52;

// The modal over the whole screen, dimmed: it asks before the shortcut types a chunk
pub fn render_finish_modal(frame: &mut Frame) {
    let palette = PALETTE.mocha.colors;
    let area = frame.area();
    frame
        .buffer_mut()
        .set_style(area, Style::new().add_modifier(Modifier::DIM));

    let [row] = Layout::vertical([Constraint::Length(7)])
        .flex(Flex::Center)
        .areas(area);
    let [modal] = Layout::horizontal([Constraint::Length(WIDTH)])
        .flex(Flex::Center)
        .areas(row);
    let badge = Style::new()
        .fg(palette.crust.into())
        .bg(palette.sky.into())
        .add_modifier(Modifier::BOLD);
    let card = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(palette.sky.into()))
        .title(Span::styled(" Finish it? ", badge));
    let inner = card.inner(modal);

    frame.render_widget(Clear, modal);
    frame.render_widget(card, modal);

    let [_, first, second, _, keys] = Layout::vertical([Constraint::Length(1); 5]).areas(inner);
    let bold = Style::new().add_modifier(Modifier::BOLD);
    let enter = Line::from(vec![
        Span::styled("  Enter", bold.fg(palette.sky.into())),
        Span::styled(" finish it", bold),
    ]);

    frame.render_widget(Paragraph::new("  Typing is what makes it stick."), first);
    frame.render_widget(Paragraph::new("  Finish this one for you anyway?"), second);
    frame.render_widget(Paragraph::new(enter), keys);
    frame.render_widget(Paragraph::new("Esc I'll type it ").right_aligned(), keys);
}
