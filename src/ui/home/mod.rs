mod buttons;
mod corners;
mod demo;
mod hint;
mod title;

use crate::app::{ContinueLabel, HomeLanguage, HomeState};
use buttons::render_buttons;
use corners::render_corners;
use demo::render_demo;
use hint::render_hint;
use title::render_title;

use catppuccin::PALETTE;

use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};

const TAGLINE_NO_AI: &str = "No AI";
const TAGLINE_REST: &str = ", Just you and the keys";

const BLOCK_WIDTH: u16 = 44;

pub fn home(
    frame: &mut Frame,
    state: HomeState,
    tick: u64,
    continue_label: Option<&ContinueLabel>,
    language: HomeLanguage,
) {
    // Without a Continue button its rows and the gap under it are not reserved
    let (continue_rows, continue_gap) = if continue_label.is_some() {
        (3, 1)
    } else {
        (0, 0)
    };

    let [
        _gap_top,
        title,
        _gap_title,
        tagline,
        _gap_tagline,
        demo,
        _gap_demo,
        continue_btn,
        _gap_buttons,
        choose_btn,
        _gap_bottom,
        hint,
    ] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(7),
        Constraint::Length(1),
        Constraint::Length(continue_rows),
        Constraint::Length(continue_gap),
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    let palette = PALETTE.mocha.colors;

    render_corners(frame, frame.area());

    render_title(frame, title, tick);

    let tagline_text = Line::from(vec![
        Span::styled(TAGLINE_NO_AI, Style::new().fg(palette.red.into()).bold()),
        Span::styled(TAGLINE_REST, Style::new().fg(palette.overlay1.into())),
    ]);
    frame.render_widget(Paragraph::new(tagline_text).centered(), tagline);

    render_demo(frame, center(demo, BLOCK_WIDTH), tick, language);

    render_buttons(
        frame,
        state,
        continue_label,
        tick,
        center(continue_btn, BLOCK_WIDTH),
        center(choose_btn, BLOCK_WIDTH),
    );

    render_hint(frame, hint, state, continue_label.is_some());
}

fn center(area: Rect, width: u16) -> Rect {
    let [centered] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);

    centered
}
