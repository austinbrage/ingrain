mod code;
mod options;

use catppuccin::PALETTE;

use crate::app::QuizState;
use crate::content::{Quiz, QuizKind};
use code::render_code;
use options::render_options;
use tui_markdown::from_str as markdown;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Padding, Paragraph, Wrap},
};

pub fn render_quiz(
    frame: &mut Frame,
    area: Rect,
    language: &str,
    quiz_state: &QuizState,
    quiz: &Quiz,
) {
    // The code takes 2 parts and the right column 1. A purpose quiz has sentences for options, so 3 and 2
    let [code_part, side_part] = match quiz.kind {
        Some(QuizKind::Purpose) => [3, 2],
        Some(QuizKind::Output | QuizKind::Compiler | QuizKind::Error) | None => [2, 1],
    };
    // One blank row above and below, and none on the sides
    let [code, side] =
        Layout::horizontal([Constraint::Fill(code_part), Constraint::Fill(side_part)])
            .spacing(2)
            .areas(area.inner(Margin::new(0, 1)));

    // Upper card: the guide, and the explanation once answered. As tall as its wrapped rows plus 3
    let (title, text) = if quiz_state.answered {
        ("explanation", &quiz.explanation)
    } else {
        (quiz.title.as_str(), &quiz.guide)
    };
    let intro = Paragraph::new(markdown(text)).wrap(Wrap { trim: false });
    let intro_card = guide_card(title, PALETTE.mocha.colors.sky.into());
    let intro_height = intro.line_count(intro_card.inner(side).width) as u16 + 3;

    // The upper card takes what it needs and the options card takes the rest
    let [upper, lower] = Layout::vertical([Constraint::Length(intro_height), Constraint::Fill(1)])
        .spacing(1)
        .areas(side);

    frame.render_widget(intro.block(intro_card), upper);
    render_options(frame, lower, quiz_state, quiz);
    render_code(frame, code, language, &quiz.code);
}

fn guide_card(title: &str, badge_bg: Color) -> Block<'static> {
    let palette = PALETTE.mocha.colors;
    let badge = Style::new()
        .fg(palette.crust.into())
        .bg(badge_bg)
        .add_modifier(Modifier::BOLD);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(palette.sky.into()))
        .title(Span::styled(format!(" {title} "), badge))
        .padding(Padding::new(1, 1, 1, 0))
}
