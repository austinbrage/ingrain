use crate::ui::wave::wave_line;

use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

const TITLE: &str = "\
▀█▀  █▄ █  ▄▀▀▀  █▀▀▄  ▄▀▀▄  ▀█▀  █▄ █
 █   █ ▀█  █ ▀█  █▀█▀  █▀▀█   █   █ ▀█
▄█▄  █  █  ▀▄▄▀  █ ▀▄  █  █  ▄█▄  █  █";

pub fn render_title(frame: &mut Frame, area: Rect, tick: u64) {
    frame.render_widget(Paragraph::new(title_lines(tick)).centered(), area);
}

// The title rows with a color per char, a gradient that slides with the tick
fn title_lines(tick: u64) -> Vec<Line<'static>> {
    TITLE.lines().map(|line| wave_line(line, tick)).collect()
}
