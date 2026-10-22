use super::content::Content;
use super::{Action, ContinueLabel, Depth, ExerciseState, ExplorerState, Screen};
use crate::progress::Progress;

#[derive(Clone, Copy, PartialEq)]
pub enum HomeButton {
    Continue,
    ChooseLanguage,
}

#[derive(Clone, Copy, PartialEq)]
pub enum HomeLanguage {
    Rust,
    Go,
}

#[derive(Clone, Copy)]
pub enum HomeState {
    Buttons(HomeButton),
    Languages(HomeLanguage),
}

impl HomeButton {
    pub fn toggle(self) -> Self {
        match self {
            Self::Continue => Self::ChooseLanguage,
            Self::ChooseLanguage => Self::Continue,
        }
    }
}

impl HomeLanguage {
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "rust" => Some(Self::Rust),
            "go" => Some(Self::Go),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::Go => "Go",
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Go => "go",
        }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Rust => Self::Go,
            Self::Go => Self::Rust,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::Rust => Self::Go,
            Self::Go => Self::Rust,
        }
    }
}

impl HomeState {
    // The language the Home demo types: the card under the cursor, else the one practiced last
    pub fn language(self, last: Option<HomeLanguage>) -> HomeLanguage {
        match self {
            Self::Buttons(_) => last.unwrap_or(HomeLanguage::Rust),
            Self::Languages(selected) => selected,
        }
    }

    // The language practiced last, None for a new user
    pub fn last_language(progress: &Progress) -> Option<HomeLanguage> {
        HomeLanguage::from_id(&progress.last.as_ref()?.language)
    }

    pub fn continue_label(progress: &Progress, content: &Content) -> Option<ContinueLabel> {
        let language = Self::last_language(progress)?;

        progress.continue_label(content.library(language))
    }

    fn continue_target(
        progress: &Progress,
        content: &Content,
    ) -> Option<(ExplorerState, usize, usize)> {
        let language = Self::last_language(progress)?;
        let at = progress.continue_at(content.library(language))?;

        let mut explorer = ExplorerState::new(language);
        explorer.depth = Depth::Modules;
        explorer.section = at.section;
        explorer.course = at.course;
        explorer.module = at.module;

        Some((explorer, at.session, at.exercise))
    }

    // None means the screen stays. It changes itself when the buttons or the cards move
    pub fn handle_key(
        &mut self,
        action: Action,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        match *self {
            Self::Buttons(selected) => self.handle_buttons_key(action, selected, content, progress),
            Self::Languages(selected) => self.handle_cards_key(action, selected),
        }
    }

    fn handle_buttons_key(
        &mut self,
        action: Action,
        selected: HomeButton,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        match action {
            Action::Up | Action::Down if Self::continue_label(progress, content).is_some() => {
                *self = Self::Buttons(selected.toggle());
            }
            Action::Select if selected == HomeButton::Continue => {
                let (explorer, session, exercise) = Self::continue_target(progress, content)?;
                return ExerciseState::open(explorer, session, exercise, content, progress);
            }
            Action::Select if selected == HomeButton::ChooseLanguage => {
                *self = Self::Languages(HomeLanguage::Rust);
            }
            _ => {}
        }

        None
    }

    fn handle_cards_key(&mut self, action: Action, selected: HomeLanguage) -> Option<Screen> {
        match action {
            Action::Left => *self = Self::Languages(selected.previous()),
            Action::Right => *self = Self::Languages(selected.next()),
            Action::Select => return Some(Screen::Explorer(ExplorerState::new(selected))),
            Action::Back => *self = Self::Buttons(HomeButton::ChooseLanguage),
            _ => {}
        }

        None
    }
}
