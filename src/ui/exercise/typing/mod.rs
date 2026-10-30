mod editor;
mod finish_modal;
mod guide;

use crate::app::TypingState;
use crate::content::Typing;
use editor::render_editor;
use guide::render_guide;

pub use finish_modal::render_finish_modal;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
};

pub fn render_typing(
    frame: &mut Frame,
    area: Rect,
    language: &str,
    typing_state: &TypingState,
    typing: &Typing,
) {
    let chunk = typing_state.current(typing);

    // The editor takes 2 parts and the guide 1. One blank row above and below, and none on the sides
    let [editor, guide] = Layout::horizontal([Constraint::Fill(2), Constraint::Fill(1)])
        .spacing(2)
        .areas(area.inner(Margin::new(0, 1)));

    if let Some(chunk) = chunk {
        render_guide(frame, guide, language, typing_state, typing, chunk);
    }
    render_editor(frame, editor, language, typing_state, chunk);
}
