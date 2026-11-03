mod fill_blank;
mod quiz;
mod typing;

use catppuccin::PALETTE;

use crate::app::{Body, ExerciseState};
use crate::content::Exercise;
use fill_blank::render_fill_blank;
use quiz::render_quiz;
use typing::{render_finish_modal, render_typing};

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

const HINT: &str = "Ctrl-F finish   Esc back   Ctrl-Q quit ";
const PREVIOUS_HINT: &str = "Ctrl-P previous   Ctrl-F finish   Esc back   Ctrl-Q quit ";
const CHOOSE_HINT: &str = "↑↓ choose   Enter check   Esc back   Ctrl-Q quit ";
const ANSWERED_HINT: &str = "Enter continue   Esc back   Ctrl-Q quit ";
const NEXT_GAP_HINT: &str = "Enter next gap   Esc back   Ctrl-Q quit ";
const CHECK_HINT: &str = "Enter check   Esc back   Ctrl-Q quit ";
const BAR_WIDTH: usize = 12; // Cells of the chunk bar in the footer

pub fn exercise(frame: &mut Frame, breadcrumb: String, state: &ExerciseState) {
    let palette = PALETTE.mocha.colors;

    // The header and the footer share one double border, muted so the frame stays quiet
    let chrome = Block::bordered()
        .border_type(BorderType::Double)
        .border_style(Style::new().fg(palette.overlay0.into()));

    // Header and footer are boxes of 3 rows (one of text), and the body takes the rest
    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .areas(frame.area());

    let session = format!(
        "session {}/{} · exercise {}/{} ",
        state.session + 1,
        state.session_count(),
        state.exercise + 1,
        state.exercise_count()
    );

    // 1. Render Header
    let header_box = chrome.clone();
    let header_inner = header_box.inner(header);

    frame.render_widget(header_box, header);
    frame.render_widget(Paragraph::new(breadcrumb), header_inner);
    frame.render_widget(Paragraph::new(session).right_aligned(), header_inner);

    // 2. Render Body, which also sets the bar and the hint of the footer
    let language = state.explorer.language.id();
    let exercise = &state.module.sessions[state.session].exercises[state.exercise];

    let (bar, hint) = match (exercise, &state.body) {
        (Exercise::Typing(typing), Body::Typing(typing_state)) => {
            render_typing(frame, body, language, typing_state, typing);

            let bar = bar(typing_state.chunk, typing.chunks.len(), "chunks");
            let hint = if typing_state.can_go_back(typing) {
                PREVIOUS_HINT
            } else {
                HINT
            };

            (bar, hint)
        }
        (Exercise::Quiz(quiz), Body::Quiz(quiz_state)) => {
            render_quiz(frame, body, language, quiz_state, quiz);

            let bar = bar(state.exercise, state.exercise_count(), "exercises");
            let hint = if quiz_state.answered {
                ANSWERED_HINT
            } else {
                CHOOSE_HINT
            };

            (bar, hint)
        }
        (Exercise::FillBlank(fill_blank), Body::FillBlank(fill_blank_state)) => {
            render_fill_blank(frame, body, language, fill_blank_state, fill_blank);

            // The bar counts the gaps left behind with Enter, and is full once checked
            let gaps = fill_blank.gaps.len();
            let done = if fill_blank_state.checked {
                gaps
            } else {
                fill_blank_state.gap
            };
            let hint = if fill_blank_state.checked {
                ANSWERED_HINT
            } else if fill_blank_state.gap + 1 >= gaps {
                CHECK_HINT
            } else {
                NEXT_GAP_HINT
            };

            (bar(done, gaps, "gaps"), hint)
        }
        (Exercise::Typing(_), Body::Quiz(_) | Body::FillBlank(_))
        | (Exercise::Quiz(_), Body::Typing(_) | Body::FillBlank(_))
        | (Exercise::FillBlank(_), Body::Typing(_) | Body::Quiz(_)) => {
            unreachable!("the body is built from its exercise")
        }
    };

    // 3. Render Footer
    let footer_box = chrome;
    let footer_inner = footer_box.inner(footer);

    frame.render_widget(footer_box, footer);
    frame.render_widget(Paragraph::new(bar), footer_inner);
    frame.render_widget(Paragraph::new(hint).right_aligned(), footer_inner);

    if matches!(&state.body, Body::Typing(typing_state) if typing_state.modal_open) {
        render_finish_modal(frame);
    }
}

// The bar fills with what is done out of the total
fn bar(done: usize, total: usize, unit: &str) -> Line<'static> {
    let filled = done * BAR_WIDTH / total;

    Line::from(vec![
        Span::raw(" ["),
        Span::styled(
            "▓".repeat(filled),
            Style::new().fg(PALETTE.mocha.colors.sky.into()),
        ),
        Span::raw("░".repeat(BAR_WIDTH - filled)),
        Span::raw(format!("] {done}/{total} {unit}")),
    ])
}
