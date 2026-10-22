use catppuccin::PALETTE;

use crate::content::Chunk;
use crate::highlight::Highlighted;

use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

const NEXT: &str = " next ";
const WRONG: &str = " wrong key ";

// The chunk's code without the indentation all its lines share, highlighted, centered and
// wrapped at `width`, with a "^ next" (or "^ wrong key") badge under the next char.
// Also returns the row of the badge in the lines
pub fn copy_lines(
    chunk: &Chunk,
    language: &str,
    next: usize,
    wrong: bool,
    width: usize,
) -> (Vec<Line<'static>>, Option<usize>) {
    let width = width.max(1);
    let (label, caret, badge) = hint_style(wrong);

    let original: Vec<&str> = chunk.text.split('\n').collect();
    let (indent, code) = dedent(&original);
    let colors = Highlighted::new(language, &code.join("\n"));

    // The next char as a row and column of the dedented code
    let (next_row, next_column) = next_position(&original, next);
    let next_column = next_column.saturating_sub(indent);

    // Center the block: pad by half of the free width, and the lines stay left-aligned inside it
    let widest = code
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0)
        .min(width);
    let pad = (width - widest) / 2;
    let mut lines = Vec::new();
    let mut badge_row = None;

    for (row, line) in code.iter().enumerate() {
        // Hard wrap at the card width, so the badge can sit under the right piece
        let chars: Vec<char> = line.chars().collect();
        let pieces: Vec<&[char]> = if chars.is_empty() {
            vec![&[]]
        } else {
            chars.chunks(width).collect()
        };

        for (n, piece) in pieces.iter().enumerate() {
            // `from..end` is the range of this line's chars in the piece. The last piece also takes the newline
            let from = n * width;
            let is_last = n + 1 == pieces.len();
            let end = from + piece.len() + usize::from(is_last);
            let mut spans = vec![Span::raw(" ".repeat(pad))];

            spans.extend(
                piece
                    .iter()
                    .enumerate()
                    .map(|(i, c)| Span::styled(c.to_string(), colors.style_at(row, from + i))),
            );
            lines.push(Line::from(spans));

            // The badge goes under the next char, in the piece that holds it
            if row == next_row && (from..end).contains(&next_column) {
                let column = (next_column - from).min(width - 1);
                let mut hint = vec![
                    Span::raw(" ".repeat(pad + column)),
                    Span::styled("^", caret),
                ];

                // Without room for the label only the caret shows
                if pad + column + 2 + label.len() <= width {
                    hint.push(Span::raw(" "));
                    hint.push(Span::styled(label, badge));
                }

                badge_row = Some(lines.len());
                lines.push(Line::from(hint));
            }
        }
    }

    (lines, badge_row)
}

// The indentation all the non-blank lines share, and the lines without it
fn dedent<'a>(lines: &[&'a str]) -> (usize, Vec<&'a str>) {
    let indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    // Blank lines shorter than the indent become empty
    let code = lines
        .iter()
        .map(|line| line.get(indent..).unwrap_or(""))
        .collect();

    (indent, code)
}

// The label, the caret style and the badge style of the hint under the next char
fn hint_style(wrong: bool) -> (&'static str, Style, Style) {
    let palette = PALETTE.mocha.colors;

    if wrong {
        (
            WRONG,
            Style::new().fg(palette.red.into()),
            Style::new()
                .fg(palette.crust.into())
                .bg(palette.red.into())
                .add_modifier(Modifier::BOLD),
        )
    } else {
        (
            NEXT,
            Style::new().fg(palette.overlay1.into()),
            Style::new()
                .fg(palette.text.into())
                .bg(palette.surface1.into()),
        )
    }
}

// Row and column in `lines` of the char at index `next` of the text they were split from
fn next_position(lines: &[&str], next: usize) -> (usize, usize) {
    let mut start = 0;

    for (row, line) in lines.iter().enumerate() {
        let len = line.chars().count();

        // `<=` so the newline at the end of a line still belongs to that row
        if next <= start + len {
            return (row, next - start);
        }
        start += len + 1;
    }

    (lines.len(), 0)
}
