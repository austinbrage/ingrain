use catppuccin::PALETTE;

use crate::app::FillBlankState;
use crate::content::FillBlank;
use crate::highlight::Highlighted;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

// Width of a code row prefix: " ▶ " + 2 digits + 2 spaces, as in the exercise editor
const GUTTER_WIDTH: u16 = 7;
// What marks a gap in the code, and the least cells its block takes
const GAP_MARKER: &str = "___";
const GAP_CELLS: usize = 3;

// A row of the code as one span per char, and the column of the cursor when it is on the row
struct CodeRow {
    spans: Vec<Span<'static>>,
    cursor: Option<usize>,
}

// The code in an editor window: title bar, rule, numbered rows with the gaps as blocks, rule and
// status bar. The terminal cursor is on the active gap until the gaps are checked
pub fn render_code(
    frame: &mut Frame,
    area: Rect,
    language: &str,
    fill_blank_state: &FillBlankState,
    fill_blank: &FillBlank,
) {
    let palette = PALETTE.mocha.colors;
    let border = Style::new().fg(palette.sky.into());
    let card = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(border);
    let inner = card.inner(area);
    frame.render_widget(card, area);

    let [title_bar, top_rule, code_area, bottom_rule, status_bar] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(inner);

    // Code columns left once the gutter is taken
    let width = code_area.width.saturating_sub(GUTTER_WIDTH).max(1) as usize;
    let colors = Highlighted::new(language, &fill_blank.code);
    let mut lines = Vec::new();
    let mut cursor = None;
    let mut position = String::new();
    let mut first_gap = 0;

    for (i, text) in fill_blank.code.lines().enumerate() {
        let row = code_row(i, text, first_gap, &colors, fill_blank_state, fill_blank);
        let marker = if row.cursor.is_some() { "▶" } else { " " };
        first_gap += text.matches(GAP_MARKER).count();

        if let Some(column) = row.cursor {
            cursor = Some((column % width, lines.len() + column / width));
            position = format!("Ln {}, Col {}", i + 1, column + 1);
        }

        // Soft wrap: one terminal line for every `width` chars of the row
        let parts: Vec<&[Span]> = if row.spans.is_empty() {
            vec![&[]]
        } else {
            row.spans.chunks(width).collect()
        };

        for (n, part) in parts.into_iter().enumerate() {
            // The line number and the marker only on the first piece of a wrapped row
            let prefix = if n == 0 {
                format!(" {marker} {:>2}  ", i + 1)
            } else {
                " ".repeat(GUTTER_WIDTH as usize)
            };
            let mut line = vec![Span::raw(prefix)];

            line.extend_from_slice(part);
            lines.push(Line::from(line));
        }
    }

    let (file, name) = language_label(language);
    // The title bar of the editor window: the three dots and the file name
    let title = Line::from(vec![
        Span::raw(" "),
        Span::styled("●", Style::new().fg(palette.red.into())),
        Span::raw(" "),
        Span::styled("●", Style::new().fg(palette.yellow.into())),
        Span::raw(" "),
        Span::styled("●", Style::new().fg(palette.green.into())),
        Span::raw(format!("   {file}")),
    ]);
    let status = if fill_blank_state.checked {
        "read only ".to_string()
    } else {
        format!("{position} ")
    };

    frame.render_widget(Paragraph::new(title), title_bar);
    render_rule(frame, area, top_rule, border);
    frame.render_widget(Paragraph::new(lines), code_area);
    render_rule(frame, area, bottom_rule, border);
    frame.render_widget(Paragraph::new(format!(" {name}")), status_bar);
    frame.render_widget(Paragraph::new(status).right_aligned(), status_bar);

    if let Some((x, y)) = cursor {
        frame.set_cursor_position(Position::new(
            code_area.x + GUTTER_WIDTH + x as u16,
            code_area.y + y as u16,
        ));
    }
}

// One span per char: the syntax color of the code, and the block of each gap. `first_gap` is the
// number of the first gap of the row, and the cursor is the column of the active gap's next char
fn code_row(
    line: usize,
    text: &str,
    first_gap: usize,
    colors: &Highlighted,
    fill_blank_state: &FillBlankState,
    fill_blank: &FillBlank,
) -> CodeRow {
    let mut spans = Vec::new();
    let mut cursor = None;
    // The column in the code with its markers, which is where the syntax colors are
    let mut column = 0;
    let mut gap = first_gap;

    for (n, part) in text.split(GAP_MARKER).enumerate() {
        if n > 0 {
            let word = fill_blank_state.words.get(gap).map_or("", String::as_str);
            let active = !fill_blank_state.checked && gap == fill_blank_state.gap;
            let typed = word.chars().count();
            // The active gap has one cell more for the cursor, so it stays on the block
            let cells = (typed + usize::from(active)).max(GAP_CELLS);
            let (word_style, blank_style) = gap_styles(fill_blank_state, fill_blank, gap);

            if active {
                cursor = Some(spans.len() + typed);
            }
            // The typed chars, then underscores up to the cells of the gap
            spans.extend((0..cells).map(|n| match word.chars().nth(n) {
                Some(c) => Span::styled(c.to_string(), word_style),
                None => Span::styled("_", blank_style),
            }));
            gap += 1;
            column += GAP_MARKER.len();
        }

        for c in part.chars() {
            spans.push(Span::styled(c.to_string(), colors.style_at(line, column)));
            column += 1;
        }
    }

    CodeRow { spans, cursor }
}

// A gap is underscores with no block: the style of its typed word and the style of the underscores.
// The active gap is bold with bright underscores, and once checked the word is `green` or `red`
fn gap_styles(
    fill_blank_state: &FillBlankState,
    fill_blank: &FillBlank,
    gap: usize,
) -> (Style, Style) {
    let palette = PALETTE.mocha.colors;
    let dim = Style::new().fg(palette.overlay1.into());

    if fill_blank_state.checked && gap < fill_blank_state.words.len() {
        let verdict = if fill_blank_state.is_right(fill_blank, gap) {
            palette.green
        } else {
            palette.red
        };

        (
            Style::new().fg(verdict.into()).add_modifier(Modifier::BOLD),
            dim,
        )
    } else if !fill_blank_state.checked && gap == fill_blank_state.gap {
        (
            Style::new()
                .fg(palette.text.into())
                .add_modifier(Modifier::BOLD),
            Style::new().fg(palette.text.into()),
        )
    } else {
        (Style::new().fg(palette.text.into()), dim)
    }
}

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

fn language_label(id: &str) -> (&'static str, &'static str) {
    match id {
        "go" => ("main.go", "Go"),
        _ => ("main.rs", "Rust"),
    }
}
