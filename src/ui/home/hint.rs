use crate::app::HomeState;

use catppuccin::PALETTE;

use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};

const BUTTONS_HINT: &[(&str, &str)] = &[("↑↓", "move"), ("Enter", "select"), ("Ctrl-Q", "quit")];
const SINGLE_BUTTON_HINT: &[(&str, &str)] = &[("Enter", "select"), ("Ctrl-Q", "quit")];
const CARDS_HINT: &[(&str, &str)] = &[
    ("←→", "move"),
    ("Enter", "select"),
    ("Esc", "back"),
    ("Ctrl-Q", "quit"),
];

pub fn render_hint(frame: &mut Frame, area: Rect, state: HomeState, has_continue: bool) {
    let keys = match state {
        HomeState::Buttons(_) if has_continue => BUTTONS_HINT,
        HomeState::Buttons(_) => SINGLE_BUTTON_HINT,
        HomeState::Languages(_) => CARDS_HINT,
    };

    frame.render_widget(Paragraph::new(hint_line(keys)).centered(), area);
}

// The keys and their labels in one line, the keys brighter
fn hint_line(keys: &[(&'static str, &'static str)]) -> Line<'static> {
    let palette = PALETTE.mocha.colors;
    let mut spans = Vec::new();

    for (i, (key, label)) in keys.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("   "));
        }
        spans.push(Span::styled(*key, Style::new().fg(palette.text.into())));
        spans.push(Span::styled(
            format!(" {label}"),
            Style::new().fg(palette.overlay1.into()),
        ));
    }

    Line::from(spans)
}
