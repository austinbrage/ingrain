use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone, Copy, PartialEq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
    Quit,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TypingKey {
    Char(char),
    Enter,
    Tab,
    Backspace,
    Back,
}

pub fn to_action(key: KeyEvent) -> Option<Action> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Char('q') if ctrl => Some(Action::Quit),
        KeyCode::Up | KeyCode::Char('k') => Some(Action::Up),
        KeyCode::Down | KeyCode::Char('j') => Some(Action::Down),
        KeyCode::Left | KeyCode::Char('h') => Some(Action::Left),
        KeyCode::Right | KeyCode::Char('l') => Some(Action::Right),
        KeyCode::Enter | KeyCode::Tab => Some(Action::Select),
        KeyCode::Esc => Some(Action::Back),
        _ => None,
    }
}

pub fn to_typing_key(key: KeyEvent) -> Option<TypingKey> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
        KeyCode::Char(c) if !ctrl => Some(TypingKey::Char(c)),
        KeyCode::Enter => Some(TypingKey::Enter),
        KeyCode::Tab => Some(TypingKey::Tab),
        KeyCode::Backspace => Some(TypingKey::Backspace),
        KeyCode::Esc => Some(TypingKey::Back),
        _ => None,
    }
}
