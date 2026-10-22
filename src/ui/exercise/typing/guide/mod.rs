mod copy;

use catppuccin::PALETTE;

use crate::app::TypingState;
use crate::content::{Chunk, Mode, Typing};
use copy::copy_lines;
use tui_markdown::from_str as markdown;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        Wrap,
    },
};

pub fn render_guide(
    frame: &mut Frame,
    area: Rect,
    language: &str,
    typing_state: &TypingState,
    typing: &Typing,
    chunk: &Chunk,
) {
    // Upper card: the exercise intro, as tall as its wrapped rows plus 3 (borders and top padding)
    let intro = Paragraph::new(markdown(&typing.guide)).wrap(Wrap { trim: false });
    let intro_card = guide_card(&typing.title);
    let intro_height = intro.line_count(intro_card.inner(area).width) as u16 + 3;

    // The upper card takes what it needs and the step card takes the rest
    let [upper, lower] = Layout::vertical([Constraint::Length(intro_height), Constraint::Fill(1)])
        .spacing(1)
        .areas(area);

    frame.render_widget(intro.block(intro_card), upper);

    let palette = PALETTE.mocha.colors;
    let copy = matches!(chunk.mode, Mode::Copy);

    // Lower card: the chunk guide and, for a copy chunk, the code to copy. An auto chunk has neither
    if chunk.guide.is_some() || copy {
        let card = guide_card("step");
        let inner = card.inner(lower);
        let width = inner.width as usize;
        let mut text = Text::default();
        let mut badge_row = None;

        if let Some(guide) = &chunk.guide {
            text.extend(markdown(guide));
        }

        if copy {
            // A dim rule separates the guide text from the code
            if chunk.guide.is_some() {
                let rule = "─".repeat(width);

                text.push_line("");
                text.push_line(Line::styled(rule, Style::new().fg(palette.overlay0.into())));
                text.push_line("");
            }
            let next = typing_state.next_at(chunk);
            let wrong = typing_state.wrong_at(chunk).is_some();

            // Rows of text above the code, to find the badge row in the whole card
            let above = Paragraph::new(text.clone())
                .wrap(Wrap { trim: false })
                .line_count(inner.width);
            let (lines, row) = copy_lines(chunk, language, next, wrong, width);

            badge_row = row.map(|row| above + row);
            text.extend(lines);
        }

        // Scroll only when the content is taller than the card, to keep the badge and the next code row in view
        let total = Paragraph::new(text.clone())
            .wrap(Wrap { trim: false })
            .line_count(inner.width);
        let height = inner.height as usize;
        let max_scroll = total.saturating_sub(height);
        let offset = badge_row.map_or(0, |row| (row + 2).saturating_sub(height).min(max_scroll));

        let step = Paragraph::new(text)
            .block(card)
            .wrap(Wrap { trim: false })
            .scroll((offset as u16, 0));

        frame.render_widget(step, lower);

        // Scrollbar on the right border, shown only when there is something to scroll
        if max_scroll > 0 {
            let mut bar = ScrollbarState::new(max_scroll + 1)
                .viewport_content_length(height)
                .position(offset);

            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                lower.inner(Margin::new(0, 1)),
                &mut bar,
            );
        }
    }
}

fn guide_card(title: &str) -> Block<'static> {
    let palette = PALETTE.mocha.colors;
    let badge = Style::new()
        .fg(palette.crust.into())
        .bg(palette.sky.into())
        .add_modifier(Modifier::BOLD);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(palette.sky.into()))
        .title(Span::styled(format!(" {title} "), badge))
        .padding(Padding::new(1, 1, 1, 0))
}
