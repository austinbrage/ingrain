use crate::app::{ContinueLabel, HomeButton, HomeLanguage, HomeState};

use catppuccin::PALETTE;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Padding, Paragraph},
};

const CONTINUE: &str = "Continue";
const CHOOSE_LANGUAGE: &str = "Choose a language";

// Ticks each dot of the review button stays
const DOT_TICKS: u64 = 10;

const CARD_WIDTH: u16 = 21;
const CARD_GAP: u16 = 2;

enum ButtonState {
    Selected,
    Normal,
}

pub fn render_buttons(
    frame: &mut Frame,
    state: HomeState,
    continue_label: Option<&ContinueLabel>,
    tick: u64,
    continue_area: Rect,
    choose_area: Rect,
) {
    match state {
        HomeState::Buttons(selected) => render_menu(
            frame,
            continue_label,
            tick,
            continue_area,
            choose_area,
            selected,
        ),
        HomeState::Languages(selected) => render_cards(frame, choose_area, selected),
    }
}

fn render_menu(
    frame: &mut Frame,
    continue_label: Option<&ContinueLabel>,
    tick: u64,
    continue_area: Rect,
    choose_area: Rect,
    selected: HomeButton,
) {
    let (continue_state, choose_state) = match selected {
        HomeButton::Continue => (ButtonState::Selected, ButtonState::Normal),
        HomeButton::ChooseLanguage => (ButtonState::Normal, ButtonState::Selected),
    };

    // Only render Continue when there is something to offer
    match continue_label {
        Some(ContinueLabel::Course(text)) => {
            let text = format!("{CONTINUE}   {text}");
            render_button(frame, continue_area, &text, continue_state);
        }
        Some(ContinueLabel::Review(language)) => {
            let text = review_text(language, tick);
            render_button(frame, continue_area, &text, continue_state);
        }
        None => {}
    }
    render_button(frame, choose_area, CHOOSE_LANGUAGE, choose_state);
}

fn render_cards(frame: &mut Frame, area: Rect, selected: HomeLanguage) {
    let [rust_area, _gap, go_area] = Layout::horizontal([
        Constraint::Length(CARD_WIDTH),
        Constraint::Length(CARD_GAP),
        Constraint::Length(CARD_WIDTH),
    ])
    .areas(area);

    let (rust_state, go_state) = match selected {
        HomeLanguage::Rust => (ButtonState::Selected, ButtonState::Normal),
        HomeLanguage::Go => (ButtonState::Normal, ButtonState::Selected),
    };

    render_button(frame, rust_area, HomeLanguage::Rust.name(), rust_state);
    render_button(frame, go_area, HomeLanguage::Go.name(), go_state);
}

fn render_button(frame: &mut Frame, area: Rect, text: &str, state: ButtonState) {
    frame.render_widget(
        Paragraph::new(text)
            .centered()
            .block(button_block(&state))
            .style(selection_style(state)),
        area,
    );
}

// A border only when selected; the padding keeps the text in the middle row when not
fn button_block(state: &ButtonState) -> Block<'static> {
    match state {
        ButtonState::Selected => Block::bordered(),
        ButtonState::Normal => Block::new().padding(Padding::top(1)),
    }
}

fn selection_style(state: ButtonState) -> Style {
    let palette = PALETTE.mocha.colors;

    match state {
        ButtonState::Selected => Style::new()
            .fg(palette.crust.into())
            .bg(palette.sky.into())
            .bold(),
        ButtonState::Normal => Style::new().fg(palette.overlay1.into()),
    }
}

// "Rust completed · Let's review", with dots that come and go slowly
fn review_text(language: &str, tick: u64) -> String {
    let dots = (tick / DOT_TICKS % 4) as usize;

    format!(
        "{language} completed · Let's review{}{}",
        ".".repeat(dots),
        " ".repeat(3 - dots)
    )
}
