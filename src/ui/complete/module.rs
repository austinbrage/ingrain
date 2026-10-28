use super::{fill_bar, render_footer, render_header, split};
use crate::app::ModuleDone;
use crate::ui::wave::wave_line;

use catppuccin::PALETTE;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

const HINT: &str = "Esc back   Ctrl-Q quit ";
// The ticks the bar stays full before it starts over
const HOLD_TICKS: u64 = 30;
// The word LEVEL UP, drawn like the home title: three rows, padded to the same width
const WORD: &str = "\
🬨🬕  🬨🬕🬒🬄█ █ 🬨🬕🬒🬄🬨🬕      █ █ 🬨🬕🬨🬓
▐▌🬞🬓▐🬕🬄🬏🬬🬭🬝 ▐🬕🬄🬏▐▌🬞🬓    █ █ ▐🬕🬂 
🬂🬂🬂🬀🬂🬂🬂🬀 🬂  🬂🬂🬂🬀🬂🬂🬂🬀    🬂🬂🬂 🬂🬂  ";

pub fn module_done(frame: &mut Frame, breadcrumb: String, state: &ModuleDone, tick: u64) {
    let palette = PALETTE.mocha.colors;
    let [header, body, footer] = split(frame.area());

    // 1. Render Header
    let position = format!(
        "session {}/{} · exercise {}/{} ",
        state.session_count, state.session_count, state.last_exercises, state.last_exercises
    );
    render_header(frame, header, breadcrumb, position);

    // 2. Render Body: the word, the caption, the bar, the name and the numbers
    let [
        word_area,
        _gap2,
        caption,
        _gap3,
        bar_area,
        _gap4,
        name,
        numbers,
        course_area,
    ] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(2),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .flex(Flex::Center)
    .areas(body);

    let word: Vec<Line> = WORD.lines().map(|row| wave_line(row, tick)).collect();
    frame.render_widget(Paragraph::new(word).centered(), word_area);

    frame.render_widget(
        Paragraph::new(wave_line("──  module complete  ──", tick)).centered(),
        caption,
    );
    frame.render_widget(
        Paragraph::new(state.name.as_str())
            .style(
                Style::new()
                    .fg(palette.yellow.into())
                    .add_modifier(Modifier::BOLD),
            )
            .centered(),
        name,
    );
    frame.render_widget(
        Paragraph::new(format!(
            "{} · {}",
            pluralize(state.session_count, "session"),
            pluralize(state.exercise_count, "exercise")
        ))
        .centered(),
        numbers,
    );

    // The bar fills from empty to full, holds, and starts over
    let bar = fill_bar(tick, 100, None, Some(HOLD_TICKS));
    frame.render_widget(Paragraph::new(bar).centered(), bar_area);

    // The line with the course, only when this module ends it
    if let Some(course) = &state.finished_course {
        frame.render_widget(
            Paragraph::new(format!("You completed the {course} course"))
                .style(Style::new().add_modifier(Modifier::BOLD))
                .centered(),
            course_area,
        );
    }

    // 3. Render Footer
    let label = if state.finished_course.is_some() {
        " explorer"
    } else {
        " next module"
    };
    let enter = Line::from(vec![
        Span::styled(
            " Enter",
            Style::new()
                .fg(palette.sky.into())
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(label, Style::new().add_modifier(Modifier::BOLD)),
    ]);
    render_footer(frame, footer, enter, HINT);
}

// "1 session", "2 sessions"
fn pluralize(number: usize, word: &str) -> String {
    let plural = if number == 1 { "" } else { "s" };

    format!("{number} {word}{plural}")
}
