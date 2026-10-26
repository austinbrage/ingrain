use ratatui::{
    buffer::{Buffer, Cell},
    style::Color,
};

// The drawn screen as a grid of cells, read by text or by position
pub struct Screen {
    buffer: Buffer,
}

impl Screen {
    pub(super) fn new(buffer: Buffer) -> Self {
        Self { buffer }
    }

    // One row as text, without trailing spaces
    fn row(&self, y: u16) -> String {
        (0..self.buffer.area.width)
            .map(|x| self.buffer[(x, y)].symbol())
            .collect::<String>()
            .trim_end()
            .to_string()
    }

    // All rows joined by new lines
    pub fn text(&self) -> String {
        let rows: Vec<String> = (0..self.buffer.area.height).map(|y| self.row(y)).collect();

        rows.join("\n")
    }

    // Whether the text is anywhere on the screen
    pub fn contains(&self, text: &str) -> bool {
        self.text().contains(text)
    }

    // The column and row where the text starts, None if it is not on the screen
    fn find(&self, text: &str) -> Option<(u16, u16)> {
        (0..self.buffer.area.height).find_map(|y| {
            let row = self.row(y);
            let byte = row.find(text)?;

            Some((row[..byte].chars().count() as u16, y))
        })
    }

    // Whether one row holds all the parts, in any order
    pub fn has_row(&self, parts: &[&str]) -> bool {
        (0..self.buffer.area.height).any(|y| {
            let row = self.row(y);
            parts.iter().all(|part| row.contains(part))
        })
    }

    // Whether the parts show in rows from top to bottom, in this order
    pub fn has_rows_in_order(&self, parts: &[&str]) -> bool {
        let mut parts = parts.iter();
        let mut next = parts.next();

        for y in 0..self.buffer.area.height {
            if let Some(part) = next
                && self.row(y).contains(part)
            {
                next = parts.next();
            }
        }

        next.is_none()
    }

    // The column where the text starts, None if it is not on the screen
    pub fn column_of(&self, text: &str) -> Option<u16> {
        self.find(text).map(|(x, _)| x)
    }

    // The background color of the first character of the text, None if it is not on the screen
    pub fn background_of(&self, text: &str) -> Option<Color> {
        let (x, y) = self.find(text)?;

        Some(self.cell(x, y).bg)
    }

    // The foreground color of the first character of the text, None if it is not on the screen
    pub fn foreground_of(&self, text: &str) -> Option<Color> {
        let (x, y) = self.find(text)?;

        Some(self.cell(x, y).fg)
    }

    // The cell at a position, to check its colors and style
    fn cell(&self, x: u16, y: u16) -> &Cell {
        &self.buffer[(x, y)]
    }
}
