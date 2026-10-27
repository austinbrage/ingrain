mod courses;
mod modules;
mod sections;
mod sessions;

use crate::app::{Depth, ExplorerState, ModuleMarks};
use crate::content::{Catalog, Course};
use crate::ui::wave::wave_line;
use courses::courses_column;
use modules::modules_column;
use sections::sections_column;
use sessions::sessions_column;

use catppuccin::PALETTE;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph},
};

// Rows each column shows more every tick after the explorer opens
const ROWS_PER_TICK: usize = 2;

const NEXT_WORD: &str = "next";

// Ticks to type one char
const TYPE_TICKS: u64 = 4;
// Ticks to erase one char
const ERASE_TICKS: u64 = 2;
// Ticks a typed word stays on screen
const HOLD_TICKS: u64 = 30;

// A column row. `right` is the text at the right edge, like a chevron
#[derive(Default)]
struct Item {
    text: String,
    right: String,
    complete: bool,
    next: bool,
}

// A titled list: selected row and focus (focused shows a sky fill)
struct Column {
    title: String,
    items: Vec<Item>,
    selected: Option<usize>,
    focused: bool,
}

pub fn render_columns(
    frame: &mut Frame,
    area: Rect,
    catalog: &Catalog,
    state: ExplorerState,
    course: Option<&Course>,
    marks: &ModuleMarks,
    tick: u64,
) {
    // At the sessions depth the sessions column takes the place of the modules column
    let third = if state.depth == Depth::Sessions {
        sessions_column(state, course, marks, tick)
    } else {
        modules_column(catalog, state, course, marks, tick)
    };
    let columns = [
        sections_column(catalog, state),
        courses_column(catalog, state),
        third,
    ];

    let areas: [Rect; 3] = Layout::horizontal([Constraint::Fill(1); 3]).areas(area);

    for i in 0..3 {
        render_single_column(frame, areas[i], &columns[i], i < 2, tick);
    }
}

fn render_single_column(frame: &mut Frame, area: Rect, column: &Column, divider: bool, tick: u64) {
    let palette = PALETTE.mocha.colors;

    // 1. Render column right border
    let block = if divider {
        Block::new()
            .borders(Borders::RIGHT)
            .border_style(Style::new().fg(palette.overlay0.into()))
    } else {
        Block::new()
    };

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // 2. Render column title
    let [title, _gap, items] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .areas(inner);

    let title_text = format!(" {}", column.title);
    let title_line = if column.focused {
        wave_line(&title_text, tick)
    } else {
        Line::from(title_text).style(Style::new().fg(palette.text.into()).bold())
    };

    frame.render_widget(Paragraph::new(title_line), title);

    // 3. Render column list options
    let shown = column
        .items
        .len()
        .min((tick as usize).saturating_mul(ROWS_PER_TICK));
    let rows = &column.items[..shown];

    let (symbol, style) = if column.focused {
        (
            "   ",
            // Brighter option, the home cards' look:
            // .fg(palette.crust.into())
            // .bg(palette.sky.into())
            Style::new()
                .fg(palette.sky.into())
                .bg(palette.surface1.into())
                .bold(),
        )
    } else {
        (" ▸ ", Style::new().fg(palette.sky.into()))
    };

    let list = List::new(rows.iter().map(|item| {
        let row = ListItem::new(item.text.as_str());

        if item.next {
            row.style(Style::new().fg(palette.sky.into()))
        } else {
            row
        }
    }))
    .style(Style::new().fg(palette.subtext0.into()))
    .highlight_symbol(symbol)
    .highlight_spacing(HighlightSpacing::Always)
    .highlight_style(style);
    let mut state = ListState::default().with_selected(column.selected.filter(|&row| row < shown));

    frame.render_stateful_widget(list, items, &mut state);

    // 4. Render column chevrons
    let right: Vec<Line> = rows
        .iter()
        .map(|item| {
            // The next row types its own cursor into the margin cell
            let text = if item.next {
                item.right.clone()
            } else {
                format!("{} ", item.right)
            };
            // A failed mark is red, and the rest of the text keeps its style
            let line = match text.split_once('✗') {
                Some((before, after)) => Line::from(vec![
                    Span::raw(before.to_string()),
                    Span::styled("✗", Style::new().fg(palette.red.into())),
                    Span::raw(after.to_string()),
                ]),
                None => Line::from(text),
            };

            if item.complete {
                line.style(Style::new().fg(palette.green.into()))
            } else {
                line
            }
        })
        .collect();

    frame.render_widget(Paragraph::new(right).right_aligned(), items);
}

// The text with a color per char, a gradient that slides with the tick

// Each word typed char by char with a cursor, held, and erased, one after the other
fn typed_text(words: [&str; 2], tick: u64) -> String {
    let total: u64 = words.iter().map(|word| cycle_ticks(word)).sum();
    let mut tick = tick % total;

    for word in words {
        let cycle = cycle_ticks(word);

        if tick >= cycle {
            tick -= cycle;
            continue;
        }

        let length = word.chars().count() as u64;
        let typing = length * TYPE_TICKS;
        let held = typing + HOLD_TICKS;

        let typed = if tick < typing {
            tick / TYPE_TICKS
        } else if tick < held {
            length
        } else {
            length - (tick - held) / ERASE_TICKS
        };

        let shown: String = word.chars().take(typed as usize).collect();
        let width = word.chars().count() + 1;
        return format!("{:<width$}", format!("{shown}█"));
    }

    unreachable!("the tick is below the total of the cycles")
}

// Ticks one word takes: typing, holding and erasing
fn cycle_ticks(word: &str) -> u64 {
    let length = word.chars().count() as u64;

    length * TYPE_TICKS + HOLD_TICKS + length * ERASE_TICKS
}
