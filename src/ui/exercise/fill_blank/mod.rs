mod code;
mod gaps;

use catppuccin::PALETTE;

use crate::app::FillBlankState;
use crate::content::FillBlank;
use code::render_code;
use gaps::render_gaps;
use tui_markdown::from_str as markdown;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Padding, Paragraph, Wrap},
};

pub fn render_fill_blank(
    frame: &mut Frame,
    area: Rect,
    language: &str,
    fill_blank_state: &FillBlankState,
    fill_blank: &FillBlank,
) {
    // The code takes 2 parts and the right column 1. One blank row above and below, and none on the sides
    let [code, side] = Layout::horizontal([Constraint::Fill(2), Constraint::Fill(1)])
        .spacing(2)
        .areas(area.inner(Margin::new(0, 1)));

    // Upper card: the guide. As tall as its wrapped rows plus 3
    let intro = Paragraph::new(markdown(&fill_blank.guide)).wrap(Wrap { trim: false });
    let intro_card = guide_card(&fill_blank.title, PALETTE.mocha.colors.sky.into());
    let intro_height = intro.line_count(intro_card.inner(side).width) as u16 + 3;

    // The upper card takes what it needs and the gaps card takes the rest
    let [upper, lower] = Layout::vertical([Constraint::Length(intro_height), Constraint::Fill(1)])
        .spacing(1)
        .areas(side);

    frame.render_widget(intro.block(intro_card), upper);
    render_gaps(frame, lower, fill_blank_state, fill_blank);
    render_code(frame, code, language, fill_blank_state, fill_blank);
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
