mod code;
mod hints;

use catppuccin::PALETTE;

use crate::app::{Kind, TypingState};
use crate::content::Chunk;
use crate::highlight::Highlighted;
use code::{code_lines, column_of};
use hints::note_positions;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

// Width of an editor row prefix: " ▶ " + 2 digits + 2 spaces
const GUTTER_WIDTH: u16 = 7;
// Rows kept visible under the cursor, enough for its hint
const SCROLL_MARGIN: usize = 6;

pub fn render_editor(
    frame: &mut Frame,
    area: Rect,
    language: &str,
    typing_state: &TypingState,
    chunk: Option<&Chunk>,
) {
    let palette = PALETTE.mocha.colors;
    let border = Style::new().fg(palette.sky.into());
    let card = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(border);
    let inner = card.inner(area);
    frame.render_widget(card, area);

    // The window: title bar, rule, code, rule and status bar
    let [title_bar, top_rule, code_area, bottom_rule, status_bar] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(inner);

    // The code rows, with their notes, and the cursor position in them
    let view = typing_state.editor(chunk);
    let colors = Highlighted::new(language, &view.text);
    // Code columns left once the gutter is taken
    let width = code_area.width.saturating_sub(GUTTER_WIDTH).max(1) as usize;
    let notes = chunk.map_or(Vec::new(), |chunk| note_positions(typing_state, chunk));
    let (lines, cursor) = code_lines(&view, &colors, width, &notes);
    let height = code_area.height as usize;
    let max_scroll = lines.len().saturating_sub(height);
    // First visible line: keeps the cursor and SCROLL_MARGIN rows under it in view, even past the last line
    let offset = cursor.map_or(0, |(_, y)| (y + SCROLL_MARGIN + 1).saturating_sub(height));

    // 1. Render Title bar
    let (file, name) = language_label(language);
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("●", Style::new().fg(palette.red.into())),
        Span::raw(" "),
        Span::styled("●", Style::new().fg(palette.yellow.into())),
        Span::raw(" "),
        Span::styled("●", Style::new().fg(palette.green.into())),
        Span::raw(format!("   {file}")),
    ]);

    frame.render_widget(Paragraph::new(title), title_bar);
    render_rule(frame, area, top_rule, border);

    // 2. Render Code, with the blinking terminal cursor over the next char
    if let Some((x, y)) = cursor {
        frame.set_cursor_position(Position::new(
            code_area.x + GUTTER_WIDTH + x as u16,
            code_area.y + y.saturating_sub(offset) as u16,
        ));
    }
    frame.render_widget(Paragraph::new(lines).scroll((offset as u16, 0)), code_area);

    // Scrollbar on the card border, only over the code rows and only when the code overflows
    if max_scroll > 0 {
        let mut bar = ScrollbarState::new(max_scroll + 1)
            .viewport_content_length(height)
            .position(offset.min(max_scroll));

        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight),
            Rect {
                x: area.x,
                width: area.width,
                ..code_area
            },
            &mut bar,
        );
    }

    // 3. Render Status bar
    render_rule(frame, area, bottom_rule, border);

    let position = view
        .cursor_row
        .map(|i| {
            let column = column_of(&view.rows[i], Kind::Cursor) + 1;

            format!("Ln {}, Col {column}", i + 1)
        })
        .unwrap_or_default();

    frame.render_widget(Paragraph::new(format!(" {name}")), status_bar);
    frame.render_widget(
        Paragraph::new(format!("{position} ")).right_aligned(),
        status_bar,
    );
}

// A horizontal rule that joins the card's side borders: ├────┤
fn render_rule(frame: &mut Frame, card: Rect, row: Rect, style: Style) {
    let rule = format!("├{}┤", "─".repeat(card.width.saturating_sub(2) as usize));

    frame.render_widget(
        Paragraph::new(rule).style(style),
        Rect {
            x: card.x,
            width: card.width,
            ..row
        },
    );
}

// File name for the title bar and language name for the status bar
fn language_label(id: &str) -> (&'static str, &'static str) {
    match id {
        "go" => ("main.go", "Go"),
        _ => ("main.rs", "Rust"),
    }
}
