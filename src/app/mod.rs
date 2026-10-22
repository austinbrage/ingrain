mod complete;
mod content;
mod exercise;
mod explorer;
mod home;
mod input;

use crate::content::Source;
pub use crate::progress::ContinueLabel;
use crate::progress::Progress;
pub use content::Content;

pub use complete::{CompleteState, Next};
pub use exercise::{Body, EditorView, ExerciseState, Kind, QuizState, Row, TypingState};
pub use explorer::{Depth, ExplorerState, ModuleMarks};
pub use home::{HomeButton, HomeLanguage, HomeState};
pub use input::{Action, TypingKey, to_action, to_typing_key};

use ratatui::crossterm::event::KeyEvent;

pub enum Screen {
    Home(HomeState),
    Explorer(ExplorerState),
    Exercise(ExerciseState),
    Complete(CompleteState),
}

pub struct App {
    pub screen: Screen,
    pub tick: u64,
    pub quit: bool,
    pub progress: Progress,
    pub content: Content,
}

impl App {
    pub fn new() -> Self {
        Self::with(Progress::load(), Content::load(Source::Prod))
    }

    pub fn with(progress: Progress, content: Content) -> Self {
        // Only a user with progress has something to continue
        let has_continue = HomeState::continue_label(&progress, &content).is_some();

        let button = if has_continue {
            HomeButton::Continue
        } else {
            HomeButton::ChooseLanguage
        };

        App {
            screen: Screen::Home(HomeState::Buttons(button)),
            tick: 0,
            quit: false,
            progress,
            content,
        }
    }

    // ------------------------------------------------------------
    // Animation
    // ------------------------------------------------------------

    pub fn animating(&self) -> bool {
        matches!(self.screen, Screen::Home(_) | Screen::Explorer(_))
    }

    pub fn tick(&mut self) {
        self.tick += 1;
    }

    // ------------------------------------------------------------
    // Input handling
    // ------------------------------------------------------------

    pub fn handle_key(&mut self, key: KeyEvent) {
        let action = to_action(key);

        if action == Some(Action::Quit) {
            self.quit = true;
            return;
        }

        // Handle Exercise: typing needs raw keys, a quiz needs actions
        if let Screen::Exercise(state) = &mut self.screen {
            let next = match state.body {
                Body::Typing(_) => to_typing_key(key).and_then(|typing_key| {
                    state.handle_typing_key(typing_key, &self.content, &mut self.progress)
                }),
                Body::Quiz(_) => action.and_then(|action| {
                    state.handle_quiz_action(action, &self.content, &mut self.progress)
                }),
            };

            if let Some(screen) = next {
                self.screen = screen;
            }
            return;
        }

        let Some(action) = action else {
            return;
        };

        // 2. Handle Screen States
        let next = match &mut self.screen {
            // Handle Home Navigation
            Screen::Home(state) => state.handle_key(action, &self.content, &mut self.progress),
            // Handle Explorer Navigation
            Screen::Explorer(state) => state.handle_key(action, &self.content, &mut self.progress),
            // Exercise has NO NAVIGATION (except for esc)
            Screen::Exercise(_) => unreachable!("exercise keys are handled above"),
            // Handle Complete Navigation
            Screen::Complete(state) => state.handle_key(action, &self.content, &mut self.progress),
        };

        if let Some(screen) = next {
            // The explorer opens from the language cards: its rows show again from the top
            if matches!(self.screen, Screen::Home(_)) && matches!(screen, Screen::Explorer(_)) {
                self.tick = 0;
            }
            self.screen = screen;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
