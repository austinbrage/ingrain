mod code;
mod output;

use catppuccin::PALETTE;

use super::{render_footer, render_header, split};
use crate::app::{ExerciseDone, Next};
use code::render_code;
use output::render_output;

use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use tui_big_text::{BigText, PixelSize};

// The output card never gets narrower than this, so a short output still looks like a card
const MIN_OUTPUT_WIDTH: u16 = 24;
// The code card alone, when there is no output
const CODE_ONLY_WIDTH: u16 = 72;
// The cards never get shorter than this on a short terminal
const MIN_CARDS_HEIGHT: u16 = 5;
const HINT: &str = "Esc back   Ctrl-Q quit ";
const SCROLL_HINT: &str = "↑↓ scroll   Esc back   Ctrl-Q quit ";
// Cells of the chunk bar in the footer
const BAR_WIDTH: usize = 12;

pub fn exercise_done(frame: &mut Frame, breadcrumb: String, state: &ExerciseDone) {
    let palette = PALETTE.mocha.colors;
    let [header, body, footer] = split(frame.area());

    let position = format!(
        "session {}/{} · exercise {}/{} ",
        state.session + 1,
        state.session_count,
        state.exercise + 1,
        state.exercise_count
    );

    // 1. Render Header
    render_header(frame, header, breadcrumb, position);

    // 2. Render Body
    let title = BigText::builder()
        .pixel_size(PixelSize::Quadrant)
        .lines(vec!["COMPLETE".into()])
        .centered()
        .build();

    let output: Vec<Line> = state
        .output
        .iter()
        .flat_map(|output| {
            let command = Line::styled(
                format!("$ {}", output.command),
                Style::new().add_modifier(Modifier::DIM),
            );

            std::iter::once(command).chain(output.text.lines().map(Line::raw))
        })
        .collect();

    // Title, name and Enter row on top, and the cards take the rest down to the footer
    let [title_area, _gap, name_area, _gap2, enter_area, _gap3, cards] = Layout::vertical([
        Constraint::Length(4),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(MIN_CARDS_HEIGHT),
    ])
    .areas(body);

    frame.render_widget(title, title_area);
    // What just finished, and what Enter does, follow the outcome
    let (summary, action) = match state.next() {
        Next::Exercise => (
            format!(
                "Exercise {} of {} complete",
                state.exercise + 1,
                state.exercise_count
            ),
            "for the next exercise",
        ),
        Next::Session => (
            format!(
                "Session {} of {} complete",
                state.session + 1,
                state.session_count
            ),
            "for the next session",
        ),
        Next::Finish => ("Module complete".to_string(), "to finish"),
    };

    frame.render_widget(Paragraph::new(summary).centered(), name_area);

    // The Enter row is bold, with the key in sky, so the next step is seen right away
    let enter = Line::from(vec![
        Span::raw("Press "),
        Span::styled("Enter", Style::new().fg(palette.sky.into())),
        Span::raw(format!(" {action}")),
    ])
    .style(Style::new().add_modifier(Modifier::BOLD));

    frame.render_widget(Paragraph::new(enter).centered(), enter_area);

    let scrollable = if state.output.is_some() {
        // The output card fits its longest line (plus border and padding), so it never wraps
        let output_width = output
            .iter()
            .map(Line::width)
            .max()
            .map_or(0, |width| width as u16 + 6)
            .max(MIN_OUTPUT_WIDTH)
            .min(cards.width / 2);

        let [code_area, output_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(output_width)])
                .spacing(2)
                .areas(cards);

        render_output(frame, output_area, "output", output);
        render_code(frame, code_area, state)
    } else {
        let [code_area] = Layout::horizontal([Constraint::Max(CODE_ONLY_WIDTH)])
            .flex(Flex::Center)
            .areas(cards);

        render_code(frame, code_area, state)
    };

    // 3. Render Footer
    // A typing exercise is finished, so its chunk bar is full; a quiz fills the bar by exercises
    let filled = state.bar_done * BAR_WIDTH / state.bar_total;
    let bar = Line::from(vec![
        Span::raw(" ["),
        Span::styled("▓".repeat(filled), Style::new().fg(palette.sky.into())),
        Span::raw("░".repeat(BAR_WIDTH - filled)),
        Span::raw(format!(
            "] {}/{} {}",
            state.bar_done, state.bar_total, state.bar_unit
        )),
    ]);
    let hint = if scrollable { SCROLL_HINT } else { HINT };

    render_footer(frame, footer, bar, hint);
}
