use catppuccin::{PALETTE, Rgb};

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

// How far the wave moves each tick, 1.0 is a full cycle
const WAVE_SPEED: f32 = 0.02;

const TITLE: &str = "\
▀█▀  █▄ █  ▄▀▀▀  █▀▀▄  ▄▀▀▄  ▀█▀  █▄ █
 █   █ ▀█  █ ▀█  █▀█▀  █▀▀█   █   █ ▀█
▄█▄  █  █  ▀▄▄▀  █ ▀▄  █  █  ▄█▄  █  █";

pub fn render_title(frame: &mut Frame, area: Rect, tick: u64) {
    let phase = tick as f32 * WAVE_SPEED;

    frame.render_widget(Paragraph::new(title_lines(phase)).centered(), area);
}

// The title rows with a color per char, a gradient that slides with the phase
fn title_lines(phase: f32) -> Vec<Line<'static>> {
    let palette = PALETTE.mocha.colors;
    let stops = [palette.blue.rgb, palette.sky.rgb, palette.teal.rgb];

    TITLE
        .lines()
        .map(|line| {
            let last = line.chars().count().saturating_sub(1).max(1);
            let spans: Vec<Span> = line
                .chars()
                .enumerate()
                .map(|(x, ch)| {
                    let wave = (x as f32 / last as f32 * 0.5 + phase) % 1.0;
                    let position = (1.0 - (2.0 * wave - 1.0).abs()) * (stops.len() - 1) as f32;
                    let stop = (position as usize).min(stops.len() - 2);
                    let color = blend(stops[stop], stops[stop + 1], position - stop as f32);

                    Span::styled(ch.to_string(), Style::new().fg(color))
                })
                .collect();

            Line::from(spans)
        })
        .collect()
}

// The color a fraction of the way from one color to another, 0.0 to 1.0
fn blend(from: Rgb, to: Rgb, amount: f32) -> Color {
    let channel = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * amount).round() as u8;

    Color::Rgb(
        channel(from.r, to.r),
        channel(from.g, to.g),
        channel(from.b, to.b),
    )
}
