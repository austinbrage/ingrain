use catppuccin::PALETTE;

use super::GUTTER_WIDTH;
use crate::app::TypingState;
use crate::content::Chunk;

use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

// Least columns a hint keeps for its text
const HINT_MIN: usize = 24;

// A revealed note in the file: its anchor's row and column, and the text
pub struct NoteAt<'a> {
    pub row: usize,
    pub column: usize,
    pub text: &'a str,
}

// Where each revealed note's anchor sits: file row (0-based, like Row.line) and column
pub fn note_positions<'a>(typing_state: &'a TypingState, chunk: &'a Chunk) -> Vec<NoteAt<'a>> {
    typing_state
        .notes(chunk)
        .filter_map(|note| {
            // The content rules say an anchor occurs in its chunk. One that does not is skipped
            let byte = chunk.text.find(note.on.as_str())?;
            let before = &chunk.text[..byte];
            let line_start = before.rfind('\n').map_or(0, |i| i + 1);

            Some(NoteAt {
                row: chunk.line - 1 + before.matches('\n').count(),
                column: before[line_start..].chars().count(),
                text: note.text.as_str(),
            })
        })
        .collect()
}

// "╰─ text" on a gray block under the anchor, then a blank row
pub fn note_hint(column: usize, text: &str, width: usize) -> Vec<Line<'static>> {
    let palette = PALETTE.mocha.colors;
    let block = Style::new()
        .fg(palette.text.into())
        .bg(palette.surface1.into());
    let pointer = Style::new().fg(palette.overlay1.into());

    // Near the right edge the pointer moves left, to keep HINT_MIN columns for the text
    let indent = column.min(width.saturating_sub(HINT_MIN));
    // Room for the words: the width minus the indent, the pointer (3) and the block padding (2)
    let room = width.saturating_sub(indent + 3 + 2).max(1);
    let wrapped = wrap(text, room);
    // Every line is padded to the longest, so the gray block is a rectangle
    let longest = wrapped.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    let mut lines: Vec<Line> = wrapped
        .iter()
        .enumerate()
        .map(|(n, words)| {
            let lead = if n == 0 { "╰─ " } else { "   " };

            Line::from(vec![
                Span::raw(" ".repeat(GUTTER_WIDTH as usize + indent)),
                Span::styled(lead, pointer),
                Span::styled(format!(" {words:<longest$} "), block),
            ])
        })
        .collect();

    // The blank row after the hint, so the text does not touch the next code line
    lines.push(Line::default());
    lines
}

// Greedy word wrap at `width` chars
fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = vec![String::new()];

    for word in text.split_whitespace() {
        let last = lines.last_mut().unwrap();

        if !last.is_empty() && last.chars().count() + 1 + word.chars().count() > width {
            lines.push(word.to_string());
        } else {
            if !last.is_empty() {
                last.push(' ');
            }
            last.push_str(word);
        }
    }

    lines
}

// "^ expected f" badge, with the ^ under the wrong key
pub fn wrong_hint(column: usize, expected: char) -> Line<'static> {
    let key = match expected {
        '\n' => "enter".to_string(),
        ' ' => "space".to_string(),
        c => c.to_string(),
    };
    let palette = PALETTE.mocha.colors;
    let badge = Style::new()
        .fg(palette.crust.into())
        .bg(palette.red.into())
        .add_modifier(Modifier::BOLD);

    Line::from(vec![
        Span::raw(" ".repeat(GUTTER_WIDTH as usize + column)),
        Span::styled("^ ", Style::new().fg(palette.red.into())),
        Span::styled(format!(" expected {key} "), badge),
    ])
}
