use std::sync::OnceLock;

use catppuccin::{Color as Swatch, PALETTE};
use ratatui::style::Style;

// Highlight names with their colors, in the order tree-sitter indexes them
pub fn theme() -> &'static [(&'static str, Style)] {
    static THEME: OnceLock<Vec<(&'static str, Style)>> = OnceLock::new();

    THEME.get_or_init(|| {
        let c = PALETTE.mocha.colors;
        let paint = |swatch: Swatch| Style::new().fg(swatch.into());

        vec![
            ("attribute", paint(c.yellow)),
            ("comment", paint(c.overlay0).italic()),
            ("comment.documentation", paint(c.overlay0).italic()),
            ("constant", paint(c.peach)),
            ("constant.builtin", paint(c.peach)),
            ("constructor", paint(c.sapphire)),
            ("escape", paint(c.pink)),
            ("function", paint(c.blue)),
            ("function.macro", paint(c.mauve)),
            ("function.method", paint(c.blue)),
            ("keyword", paint(c.mauve)),
            ("label", paint(c.sapphire)),
            ("operator", paint(c.sky)),
            ("property", paint(c.lavender)),
            ("punctuation.bracket", paint(c.overlay2)),
            ("punctuation.delimiter", paint(c.overlay2)),
            ("string", paint(c.green)),
            ("type", paint(c.yellow)),
            ("type.builtin", paint(c.yellow)),
            ("variable.builtin", paint(c.red)),
            ("variable.parameter", paint(c.maroon)),
        ]
    })
}
