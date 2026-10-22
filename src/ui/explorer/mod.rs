mod columns;
mod corners;
mod hint;

use crate::app::{ExplorerState, ModuleMarks};
use crate::content::{Catalog, Course};
use columns::render_columns;
use corners::render_corners;
use hint::render_hint;

use catppuccin::PALETTE;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin},
    style::Style,
    widgets::Paragraph,
};

const TITLE: &str = "Ingrain";

const SIDE_MARGIN: u16 = 2;

pub fn explorer(
    frame: &mut Frame,
    catalog: &Catalog,
    state: ExplorerState,
    course: Option<&Course>,
    marks: &ModuleMarks,
    breadcrumb: String,
    tick: u64,
) {
    render_corners(frame, frame.area());

    let [_gap_top, header, _gap_header, body, _gap_bottom, hint] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(2),
        Constraint::Length(1),
    ])
    .areas(frame.area());

    let header = header.inner(Margin::new(SIDE_MARGIN, 0));
    let body = body.inner(Margin::new(SIDE_MARGIN, 0));

    // 1. Render Header
    let palette = PALETTE.mocha.colors;

    frame.render_widget(
        Paragraph::new(TITLE).style(Style::new().fg(palette.text.into())),
        header,
    );
    frame.render_widget(
        Paragraph::new(breadcrumb)
            .style(Style::new().fg(palette.overlay1.into()))
            .right_aligned(),
        header,
    );

    // 2. Render Body
    render_columns(frame, body, catalog, state, course, marks, tick);

    // 3. Render Hint
    render_hint(frame, hint, state.depth);
}
