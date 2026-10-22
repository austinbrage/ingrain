use catppuccin::PALETTE;

use super::guide_card;
use crate::app::QuizState;
use crate::content::{Quiz, QuizKind};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};

// Width of an option's marker: "▶ 1" and two spaces
const MARKER_WIDTH: u16 = 5;

// The options card. Its badge says what is asked, and the result once answered. The selected option
// has the gray block, and the card scrolls to keep it in view
pub fn render_options(frame: &mut Frame, area: Rect, quiz_state: &QuizState, quiz: &Quiz) {
    let palette = PALETTE.mocha.colors;
    let (title, badge) = if !quiz_state.answered {
        let title = match quiz.kind {
            Some(QuizKind::Purpose) => "purpose",
            Some(QuizKind::Output) => "output",
            Some(QuizKind::Compiler) => "cause",
            Some(QuizKind::Error) => "error kind",
            None => "options",
        };

        (title, palette.sky)
    } else if quiz_state.selected == quiz.answer {
        ("right", palette.green)
    } else {
        ("wrong", palette.red)
    };

    let card = guide_card(title, badge.into());
    let inner = card.inner(area);
    frame.render_widget(card, area);

    // Each option wraps in the width left after its marker
    let width = inner.width.saturating_sub(MARKER_WIDTH).max(1);
    let texts: Vec<Paragraph> = quiz
        .options
        .iter()
        .map(|option| Paragraph::new(option.as_str()).wrap(Wrap { trim: false }))
        .collect();
    let heights: Vec<u16> = texts
        .iter()
        .map(|text| text.line_count(width) as u16)
        .collect();

    // The offset puts the end of the selected option at the bottom of the card, when it does not fit
    let selected_end: u16 = heights.iter().take(quiz_state.selected + 1).sum();
    let offset = selected_end.saturating_sub(inner.height);
    let mut top = 0;

    for (i, (text, height)) in texts.into_iter().zip(heights).enumerate() {
        let y = top;
        top += height;

        // An option scrolled past the top is dropped whole
        if y < offset || y - offset >= inner.height {
            continue;
        }

        let row = Rect {
            x: inner.x,
            y: inner.y + y - offset,
            width: inner.width,
            height: height.min(inner.height - (y - offset)),
        };
        let [marker_area, text_area] =
            Layout::horizontal([Constraint::Length(MARKER_WIDTH), Constraint::Fill(1)]).areas(row);

        let marker = match (quiz_state.answered, i) {
            (false, i) if i == quiz_state.selected => Span::raw("▶"),
            (true, i) if i == quiz.answer => {
                Span::styled("✓", Style::new().fg(palette.green.into()))
            }
            (true, i) if i == quiz_state.selected => {
                Span::styled("✗", Style::new().fg(palette.red.into()))
            }
            _ => Span::raw(" "),
        };

        if i == quiz_state.selected {
            frame.render_widget(
                Block::new().style(Style::new().bg(palette.surface1.into())),
                row,
            );
        }
        frame.render_widget(
            Paragraph::new(Line::from(vec![marker, Span::raw(format!(" {}", i + 1))])),
            marker_area,
        );
        frame.render_widget(text, text_area);
    }
}
