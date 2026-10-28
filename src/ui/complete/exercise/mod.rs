mod code;
mod output;

use catppuccin::PALETTE;

use crate::app::{ExerciseDone, Next};
use crate::ui::chrome::{render_footer, render_header, split};
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

    // 2. Render Body: the word, the title and the next step, then the cards
    let word = BigText::builder()
        .pixel_size(PixelSize::Sextant)
        .style(Style::new().fg(palette.green.into()))
        .lines(vec![Line::from("DONE")])
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

    // The word, the title and the subtitle on top, and the cards take the rest down to the footer
    let [word_area, _gap, title_area, subtitle_area, _gap2, cards] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Min(MIN_CARDS_HEIGHT),
    ])
    .areas(body);

    frame.render_widget(word, word_area);

    let title = Line::from(vec![
        Span::styled("✓ ", Style::new().fg(palette.green.into())),
        Span::styled(
            state.title.as_str(),
            Style::new().add_modifier(Modifier::BOLD),
        ),
    ]);
    frame.render_widget(Paragraph::new(title).centered(), title_area);

    // What comes next, and what Enter does, follow the outcome
    let (subtitle, label) = match state.next() {
        Next::Exercise => {
            let next = state.next_failed.unwrap_or(state.exercise + 1) + 1;
            (
                format!("Next: Exercise {next} of {}", state.exercise_count),
                " next exercise",
            )
        }
        Next::Session => ("Next: Session recap".to_string(), " session recap"),
        Next::Finish => ("Next: Module recap".to_string(), " finish module"),
    };
    frame.render_widget(
        Paragraph::new(subtitle)
            .style(Style::new().fg(palette.overlay0.into()))
            .centered(),
        subtitle_area,
    );

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
    let enter = Line::from(vec![
        Span::styled(
            " Enter",
            Style::new()
                .fg(palette.sky.into())
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(label, Style::new().add_modifier(Modifier::BOLD)),
    ]);
    let hint = if scrollable { SCROLL_HINT } else { HINT };

    render_footer(frame, footer, enter, hint);
}
