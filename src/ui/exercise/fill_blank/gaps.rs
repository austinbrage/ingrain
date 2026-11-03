use catppuccin::PALETTE;

use super::guide_card;
use crate::app::FillBlankState;
use crate::content::FillBlank;

use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

// The gaps card: a row per gap with the word typed. Its badge says `gaps`, and the result once
// checked, when a row also says the right word of a wrong gap. The active row has the gray block
pub fn render_gaps(
    frame: &mut Frame,
    area: Rect,
    fill_blank_state: &FillBlankState,
    fill_blank: &FillBlank,
) {
    let palette = PALETTE.mocha.colors;
    let checked = fill_blank_state.checked;
    let (title, badge) = if !checked {
        ("gaps", palette.sky)
    } else if fill_blank_state.passed(fill_blank) {
        ("right", palette.green)
    } else {
        ("wrong", palette.red)
    };

    let card = guide_card(title, badge.into());
    let inner = card.inner(area);
    frame.render_widget(card, area);

    for (i, word) in fill_blank_state
        .words
        .iter()
        .enumerate()
        .take(inner.height as usize)
    {
        let row = Rect {
            x: inner.x,
            y: inner.y + i as u16,
            height: 1,
            ..inner
        };
        let right = fill_blank_state.is_right(fill_blank, i);

        let marker = match (checked, right) {
            (false, _) if i == fill_blank_state.gap => Span::raw("▶"),
            (true, true) => Span::styled("✓", Style::new().fg(palette.green.into())),
            (true, false) => Span::styled("✗", Style::new().fg(palette.red.into())),
            _ => Span::raw(" "),
        };
        let mut spans = vec![marker, Span::raw(format!(" {}  {word}", i + 1))];

        if checked && !right {
            let answer = fill_blank.gaps[i]
                .answers
                .first()
                .map_or("", String::as_str);

            spans.push(Span::raw(format!("  →  {answer}")));
        }

        if !checked && i == fill_blank_state.gap {
            frame.render_widget(
                Block::new().style(Style::new().bg(palette.surface1.into())),
                row,
            );
        }
        frame.render_widget(Paragraph::new(Line::from(spans)), row);
    }
}
