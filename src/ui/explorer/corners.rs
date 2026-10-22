use catppuccin::PALETTE;

use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Borders},
};

// Columns of the longest corner arm
const MAX_ARM_WIDTH: u16 = 8;

pub fn render_corners(frame: &mut Frame, area: Rect) {
    let border = Style::new().fg(PALETTE.mocha.colors.overlay0.into());
    let arm_width = (area.width / 12).clamp(2, MAX_ARM_WIDTH);
    let arm_height = (area.height / 8).max(2);

    let corners = [
        (Borders::TOP | Borders::LEFT, area.x, area.y),
        (
            Borders::TOP | Borders::RIGHT,
            area.right() - arm_width,
            area.y,
        ),
        (
            Borders::BOTTOM | Borders::LEFT,
            area.x,
            area.bottom() - arm_height,
        ),
        (
            Borders::BOTTOM | Borders::RIGHT,
            area.right() - arm_width,
            area.bottom() - arm_height,
        ),
    ];

    for (borders, x, y) in corners {
        frame.render_widget(
            Block::new()
                .borders(borders)
                .border_type(BorderType::Thick)
                .border_style(border),
            Rect::new(x, y, arm_width, arm_height),
        );
    }
}
