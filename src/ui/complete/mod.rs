mod exercise;
mod module;
mod module_failed;
mod session;

use crate::app::{CompleteState, Content};

use catppuccin::PALETTE;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

pub fn complete(frame: &mut Frame, content: &Content, state: &CompleteState, tick: u64) {
    match state {
        CompleteState::Exercise(done) => {
            exercise::exercise_done(frame, content.breadcrumb(done.explorer), done)
        }
        CompleteState::Session(done) => {
            session::session_done(frame, content.breadcrumb(done.explorer), done, tick)
        }
        CompleteState::Module(done) => {
            module::module_done(frame, content.breadcrumb(done.explorer), done, tick)
        }
        CompleteState::ModuleFailed(done) => {
            module_failed::module_failed(frame, content.breadcrumb(done.explorer), done, tick)
        }
    }
}

// The header and the footer share one double border, muted so the frame stays quiet
fn chrome() -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Double)
        .border_style(Style::new().fg(PALETTE.mocha.colors.overlay0.into()))
}

// Header and footer are boxes of 3 rows (one of text), with a blank row between each part
fn split(area: Rect) -> [Rect; 3] {
    Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .spacing(1)
    .areas(area)
}

fn render_header(frame: &mut Frame, area: Rect, breadcrumb: String, position: String) {
    let boxed = chrome();
    let inner = boxed.inner(area);

    frame.render_widget(boxed, area);
    frame.render_widget(Paragraph::new(breadcrumb), inner);
    frame.render_widget(Paragraph::new(position).right_aligned(), inner);
}

fn render_footer(frame: &mut Frame, area: Rect, left: Line, right: &str) {
    let boxed = chrome();
    let inner = boxed.inner(area);

    frame.render_widget(boxed, area);
    frame.render_widget(Paragraph::new(left), inner);
    frame.render_widget(Paragraph::new(right.to_string()).right_aligned(), inner);
}

// The bar of the end screens. It fills from empty to `value` (a cell every 2 ticks) and holds,
// counting the text after it along. With a `total` the text is `value/total`, without it `value`
// is a percentage. With `hold`, it starts over after that many ticks full
fn fill_bar(tick: u64, value: usize, total: Option<usize>, hold: Option<u64>) -> Line<'static> {
    const WIDTH: usize = 12;
    const FILL_TICKS: u64 = 2;

    let target = value * WIDTH / total.unwrap_or(100);
    let tick = match hold {
        Some(hold) => tick % (target as u64 * FILL_TICKS + hold),
        None => tick,
    };
    let filled = ((tick / FILL_TICKS) as usize).min(target);
    let counted = if target == 0 {
        0
    } else {
        value * filled / target
    };
    let after = match total {
        Some(total) => format!("] {counted}/{total}  "),
        None => format!("] {counted}%"),
    };

    Line::from(vec![
        Span::raw("["),
        Span::styled(
            "▓".repeat(filled),
            Style::new().fg(PALETTE.mocha.colors.sky.into()),
        ),
        Span::raw("░".repeat(WIDTH - filled)),
        Span::raw(after),
    ])
}
