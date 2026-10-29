mod code;

use crate::app::IntroState;
use crate::ui::chrome::{render_footer, render_header, split};
use crate::ui::wave::wave_line;
use code::render_snippet;

use catppuccin::PALETTE;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
};
use tui_markdown::from_str as markdown;

const HINT: &str = "Esc back   Ctrl-Q quit ";
const BACK: &str = "← back   ";
const SCROLL: &str = "↑↓ scroll   ";
const TEXT_WIDTH: u16 = 70;
const SNIPPET_WIDTH: u16 = 46;
const SNIPPET_MARGIN: u16 = 2;
const CAPTION: &str = "──  intro  ──";

pub fn intro(frame: &mut Frame, breadcrumb: String, state: &IntroState, language: &str, tick: u64) {
    let palette = PALETTE.mocha.colors;
    let [header, body, footer] = split(frame.area());

    // 1. Render Header: a module has no position, a session shows its own
    let position = if state.of_module {
        String::new()
    } else {
        format!(
            "session {}/{} ",
            state.session + 1,
            state.module.sessions.len()
        )
    };
    render_header(frame, header, breadcrumb, position);

    // 2. Render Body: the title, the dots of the slides, the text in a centered column and,
    // when the slide has code, its window under the text. When they do not fit, the text
    // takes the room and the window stays at the bottom
    let intro = state.intro();
    let slide = &intro.slides[state.slide];
    let text_width = TEXT_WIDTH.min(body.width);
    let text = Paragraph::new(markdown(&slide.text)).wrap(Wrap { trim: false });
    let text_rows = text.line_count(text_width) as u16;
    let snippet_rows = slide
        .code
        .as_ref()
        .map_or(0, |code| code.lines().count() as u16 + 2);
    let snippet_margin = if snippet_rows > 0 { SNIPPET_MARGIN } else { 0 };
    let [caption_area, _gap0, title_area, dots_area, _gap, rest] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .areas(body);
    let fits = text_rows + snippet_margin + snippet_rows <= rest.height;
    let [text_area, _gap2, snippet_area, _empty] = if fits {
        Layout::vertical([
            Constraint::Length(text_rows),
            Constraint::Length(snippet_margin),
            Constraint::Length(snippet_rows),
            Constraint::Fill(1),
        ])
        .areas(rest)
    } else {
        Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(snippet_rows),
            Constraint::Length(0),
        ])
        .areas(rest)
    };
    let [column] = Layout::horizontal([Constraint::Length(text_width)])
        .flex(Flex::Center)
        .areas(text_area);
    let max_scroll = text_rows.saturating_sub(text_area.height);
    let offset = (state.scroll as u16).min(max_scroll);

    frame.render_widget(
        Paragraph::new(wave_line(CAPTION, tick)).centered(),
        caption_area,
    );
    frame.render_widget(
        Paragraph::new(intro.title.as_str())
            .style(
                Style::new()
                    .fg(palette.yellow.into())
                    .add_modifier(Modifier::BOLD),
            )
            .centered(),
        title_area,
    );
    let mut dots = Vec::new();
    for slide in 0..intro.slides.len() {
        if slide > 0 {
            dots.push(Span::raw("  "));
        }
        dots.push(if slide == state.slide {
            Span::styled("●", Style::new().fg(palette.sky.into()))
        } else {
            Span::styled("○", Style::new().fg(palette.overlay0.into()))
        });
    }
    frame.render_widget(Paragraph::new(Line::from(dots)).centered(), dots_area);

    frame.render_widget(text.scroll((offset, 0)), column);

    if let Some(code) = &slide.code {
        let [window] = Layout::horizontal([Constraint::Length(SNIPPET_WIDTH)])
            .flex(Flex::Center)
            .areas(snippet_area);

        render_snippet(frame, window, state.explorer.language.id(), language, code);
    }

    // 3. Render Footer
    let label = if state.starts_exercise() {
        " start"
    } else {
        " next"
    };
    let enter = Line::from(vec![
        Span::styled(
            " Enter",
            Style::new()
                .fg(palette.sky.into())
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(label, Style::new().add_modifier(Modifier::BOLD)),
    ]);
    let scroll_hint = if max_scroll > 0 { SCROLL } else { "" };
    let back_hint = if state.slide > 0 { BACK } else { "" };
    render_footer(
        frame,
        footer,
        enter,
        &format!("{scroll_hint}{back_hint}{HINT}"),
    );
}
