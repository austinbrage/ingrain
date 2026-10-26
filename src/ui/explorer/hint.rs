use crate::app::{Depth, ExplorerState, ModuleMarks};

use catppuccin::PALETTE;

use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};

const OPEN_HINT: &[(&str, &str)] = &[
    ("↑↓", "move"),
    ("→", "open"),
    ("←", "back"),
    ("Ctrl-Q", "quit"),
];
const START_HINT: &[(&str, &str)] = &[
    ("↑↓", "move"),
    ("Enter", "start"),
    ("←", "back"),
    ("Ctrl-Q", "quit"),
];
const RETAKE_HINT: &[(&str, &str)] = &[
    ("↑↓", "move"),
    ("Enter", "retake"),
    ("←", "back"),
    ("Ctrl-Q", "quit"),
];

pub fn render_hint(frame: &mut Frame, area: Rect, state: ExplorerState, marks: &ModuleMarks) {
    let failed = marks
        .sessions_failed
        .get(state.session)
        .copied()
        .unwrap_or(false);
    let keys = match state.depth {
        Depth::Sections | Depth::Courses => OPEN_HINT,
        Depth::Modules => START_HINT,
        Depth::Sessions if failed => RETAKE_HINT,
        Depth::Sessions => START_HINT,
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
