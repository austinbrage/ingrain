use catppuccin::PALETTE;

use crate::highlight::Highlighted;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

// Width of a code row prefix: 3 spaces + 2 digits + 2 spaces, as in the exercise editor
const GUTTER_WIDTH: u16 = 7;

// The code in an editor window with no cursor: title bar, rule, numbered rows, rule and status bar
pub fn render_code(frame: &mut Frame, area: Rect, language: &str, code: &str) {
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
    let colors = Highlighted::new(language, code);
    let mut lines = Vec::new();

    for (i, text) in code.lines().enumerate() {
        let spans: Vec<Span> = text
            .chars()
            .enumerate()
            .map(|(column, c)| Span::styled(c.to_string(), colors.style_at(i, column)))
            .collect();
        // Soft wrap: one terminal line for every `width` chars of the row
        let parts: Vec<&[Span]> = if spans.is_empty() {
            vec![&[]]
        } else {
            spans.chunks(width).collect()
        };

        for (n, part) in parts.into_iter().enumerate() {
            // The line number only on the first piece of a wrapped row
            let prefix = if n == 0 {
                format!("   {:>2}  ", i + 1)
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

    frame.render_widget(Paragraph::new(title), title_bar);
    render_rule(frame, area, top_rule, border);
    frame.render_widget(Paragraph::new(lines), code_area);
    render_rule(frame, area, bottom_rule, border);
    frame.render_widget(Paragraph::new(format!(" {name}")), status_bar);
    frame.render_widget(Paragraph::new("read only ").right_aligned(), status_bar);
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
