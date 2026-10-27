use super::{render_footer, render_header, split};
use crate::app::ModuleDone;

use catppuccin::PALETTE;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use tui_big_text::{BigText, PixelSize};

const HINT: &str = "Esc back   Ctrl-Q quit ";
// The three rows of sparkles: one above the word and two below the numbers
const SPARKLES: [&str; 3] = [
    "  ✦          ·           ✧           ·           ✦           ·  ",
    "·         ✧          ·           ✦          ·          ✧        ",
    "     ✧          ✦          ·          ✧          ✦          ·   ",
];

pub fn module_done(frame: &mut Frame, breadcrumb: String, state: &ModuleDone) {
    let palette = PALETTE.mocha.colors;
    let [header, body, footer] = split(frame.area());

    // 1. Render Header
    let position = format!(
        "session {}/{} · exercise {}/{} ",
        state.session_count, state.session_count, state.last_exercises, state.last_exercises
    );
    render_header(frame, header, breadcrumb, position);

    // 2. Render Body: sparkles, the word, the caption, the name and the numbers
    let [
        sparkles_top,
        _gap,
        word_area,
        _gap2,
        caption,
        _gap3,
        name,
        numbers,
        course_area,
        _gap4,
        sparkles_mid,
        _gap5,
        sparkles_low,
    ] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .flex(Flex::Center)
    .areas(body);

    let sparkle = Style::new().fg(palette.yellow.into());
    for (area, text) in [sparkles_top, sparkles_mid, sparkles_low]
        .into_iter()
        .zip(SPARKLES)
    {
        frame.render_widget(Paragraph::new(text).style(sparkle).centered(), area);
    }

    let word = BigText::builder()
        .pixel_size(PixelSize::Sextant)
        .style(Style::new().fg(palette.sky.into()))
        .lines(vec![Line::from("LEVEL UP")])
        .centered()
        .build();
    frame.render_widget(word, word_area);

    frame.render_widget(
        Paragraph::new("──  module complete  ──")
            .style(Style::new().fg(palette.overlay1.into()))
            .centered(),
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
            "{} sessions · {} exercises",
            state.session_count, state.exercise_count
        ))
        .centered(),
        numbers,
    );

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
