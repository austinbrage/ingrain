use catppuccin::{PALETTE, Rgb};
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

// How far the wave moves each tick, 1.0 is a full cycle
const WAVE_SPEED: f32 = 0.02;

// A line of text with a color per char: a gradient that slides with the tick
pub fn wave_line(text: &str, tick: u64) -> Line<'static> {
    let width = text.chars().count();
    let spans: Vec<Span> = text
        .chars()
        .enumerate()
        .map(|(column, ch)| {
            Span::styled(
                ch.to_string(),
                Style::new().fg(wave_color(column, width, tick)),
            )
        })
        .collect();

    Line::from(spans)
}

// The color at a column of a text `width` columns wide
fn wave_color(column: usize, width: usize, tick: u64) -> Color {
    let phase = tick as f32 * WAVE_SPEED;
    let palette = PALETTE.mocha.colors;
    let stops = [palette.blue.rgb, palette.sky.rgb, palette.teal.rgb];
    let last = width.saturating_sub(1).max(1);

    let wave = (column as f32 / last as f32 * 0.5 + phase) % 1.0;
    let position = (1.0 - (2.0 * wave - 1.0).abs()) * (stops.len() - 1) as f32;
    let stop = (position as usize).min(stops.len() - 2);

    blend(stops[stop], stops[stop + 1], position - stop as f32)
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
