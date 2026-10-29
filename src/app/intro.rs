use crate::app::{Action, Content, ExerciseState, ExplorerState, Screen};
use crate::content::{Intro, Module};

// The slides taught before a module or a session: the module's own first, then the session's
pub struct IntroState {
    pub explorer: ExplorerState,
    pub module: Module,
    pub session: usize,
    // Whether the slides are the module's or the session's
    pub of_module: bool,
    pub slide: usize,
    // How many rows of the slide text are scrolled off the top
    pub scroll: usize,
}

impl IntroState {
    // A fresh start of a session teaches before the first exercise, and a retake never does
    pub fn applies(module: &Module, session: usize, exercise: usize, retake: bool) -> bool {
        let module_intro = session == 0 && module.intro.is_some();

        exercise == 0 && !retake && (module_intro || module.sessions[session].intro.is_some())
    }

    pub fn new(explorer: ExplorerState, module: Module, session: usize) -> Self {
        let of_module = session == 0 && module.intro.is_some();

        Self {
            explorer,
            module,
            session,
            of_module,
            slide: 0,
            scroll: 0,
        }
    }

    // None means the screen stays
    pub fn handle_key(&mut self, action: Action, content: &Content) -> Option<Screen> {
        let last_slide = self.intro().slides.len() - 1;

        match action {
            Action::Select | Action::Right if self.slide < last_slide => self.show(self.slide + 1),
            Action::Select => return self.go_on(content),
            Action::Left if self.slide > 0 => self.show(self.slide - 1),
            Action::Up => self.scroll = self.scroll.saturating_sub(1),
            Action::Down => {
                let last_line = self.intro().slides[self.slide].text.lines().count() - 1;

                self.scroll = (self.scroll + 1).min(last_line);
            }
            Action::Back => return Some(Screen::Explorer(self.explorer)),
            _ => {}
        }

        None
    }

    // After the last slide: the session intro when it follows the module's, else the exercise
    fn go_on(&mut self, content: &Content) -> Option<Screen> {
        if self.session_intro_follows() {
            self.of_module = false;
            self.show(0);
            return None;
        }

        let module = content.module(self.explorer)?;

        Some(ExerciseState::start(self.explorer, module, self.session))
    }

    // A slide opens at its top
    fn show(&mut self, slide: usize) {
        self.slide = slide;
        self.scroll = 0;
    }

    // Whether Enter on the last slide starts the exercise, for the footer
    pub fn starts_exercise(&self) -> bool {
        self.slide + 1 == self.intro().slides.len() && !self.session_intro_follows()
    }

    fn session_intro_follows(&self) -> bool {
        self.of_module && self.module.sessions[self.session].intro.is_some()
    }

    pub fn intro(&self) -> &Intro {
        let intro = if self.of_module {
            &self.module.intro
        } else {
            &self.module.sessions[self.session].intro
        };

        intro.as_ref().expect("an intro screen has its intro")
    }
}
