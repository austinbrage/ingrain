use super::GUTTER_WIDTH;
use super::hints::{NoteAt, note_hint, wrong_hint};
use crate::app::{EditorView, Kind, Row};
use crate::highlight::Highlighted;

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

// Editor rows as terminal lines, wrapped at `width` chars, plus the cursor (x, y) in them
pub fn code_lines(
    view: &EditorView,
    colors: &Highlighted,
    width: usize,
    notes: &[NoteAt],
) -> (Vec<Line<'static>>, Option<(usize, usize)>) {
    let mut lines = Vec::new();
    let mut cursor = None;

    for (i, row) in view.rows.iter().enumerate() {
        let on_cursor = view.cursor_row == Some(i);
        let marker = if on_cursor { "▶" } else { " " };
        let spans = row_spans(row, colors);
        // Soft wrap: one terminal line for every `width` chars of the row
        let parts: Vec<&[Span]> = if spans.is_empty() {
            vec![&[]]
        } else {
            spans.chunks(width).collect()
        };

        if on_cursor {
            let column = column_of(row, Kind::Cursor);

            cursor = Some((column % width, lines.len() + column / width));
        }

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

            // Hint row under the wrong key
            if on_cursor && let Some(expected) = view.wrong_expected {
                let wrong = column_of(row, Kind::Wrong);

                if wrong / width == n {
                    lines.push(wrong_hint(wrong % width, expected));
                }
            }
        }

        // The notes anchored on this row go after all its wrapped pieces
        for note in notes.iter().filter(|note| note.row == row.line) {
            lines.extend(note_hint(note.column % width, note.text, width));
        }
    }

    (lines, cursor)
}

// How many chars come before the first piece of this kind
pub fn column_of(row: &Row, kind: Kind) -> usize {
    row.pieces
        .iter()
        .take_while(|piece| piece.kind != kind)
        .map(|piece| piece.text.chars().count())
        .sum()
}

// One span per char: token color plus the state style
fn row_spans(row: &Row, colors: &Highlighted) -> Vec<Span<'static>> {
    let chars = row
        .pieces
        .iter()
        .flat_map(|piece| piece.text.chars().map(move |c| (c, piece.kind)));

    chars
        .enumerate()
        .map(|(column, (c, kind))| {
            let style = colors.style_at(row.line, column).patch(kind_style(kind));

            Span::styled(c.to_string(), style)
        })
        .collect()
}

fn kind_style(kind: Kind) -> Style {
    match kind {
        Kind::Typed => Style::new(),
        Kind::Wrong => Style::new().fg(Color::Red),
        // The terminal cursor draws the block, so the cursor piece has no style
        Kind::Cursor => Style::new(),
        Kind::Ghost => Style::new().add_modifier(Modifier::DIM),
    }
}
