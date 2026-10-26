use crate::app::{
    Action, CompleteState, Content, ExerciseState, ExplorerState, Screen, SessionDone,
};
use crate::content::{Output, Quiz, Typing};
use crate::progress::Progress;

// What Enter does on the complete screen
pub enum Next {
    Exercise,
    Session,
    Finish,
}

// The finished exercise: code and output for the cards, plus its place in the module
pub struct ExerciseDone {
    pub explorer: ExplorerState,
    pub session: usize,
    pub exercise: usize,
    pub session_count: usize,
    pub exercise_count: usize,
    pub retake: bool,
    pub next_failed: Option<usize>,
    pub bar_done: usize,
    pub bar_total: usize,
    pub bar_unit: &'static str,
    pub code: Vec<String>,
    pub output: Option<Output>,
    pub scroll: usize,
}

impl ExerciseDone {
    pub fn new(state: &ExerciseState, typing: &Typing, code: Vec<String>) -> Self {
        Self {
            explorer: state.explorer,
            session: state.session,
            exercise: state.exercise,
            session_count: state.session_count(),
            exercise_count: state.exercise_count(),
            retake: state.retake,
            next_failed: state.next_failed,
            bar_done: typing.chunks.len(),
            bar_total: typing.chunks.len(),
            bar_unit: "chunks",
            code,
            output: typing.output.clone(),
            scroll: 0,
        }
    }

    pub fn from_quiz(state: &ExerciseState, quiz: &Quiz) -> Self {
        Self {
            explorer: state.explorer,
            session: state.session,
            exercise: state.exercise,
            session_count: state.session_count(),
            exercise_count: state.exercise_count(),
            retake: state.retake,
            next_failed: state.next_failed,
            bar_done: state.exercise + 1,
            bar_total: state.exercise_count(),
            bar_unit: "exercises",
            code: quiz.code.lines().map(String::from).collect(),
            output: quiz.output.clone(),
            scroll: 0,
        }
    }

    pub fn next(&self) -> Next {
        if self.retake {
            return if self.next_failed.is_some() {
                Next::Exercise
            } else {
                Next::Finish
            };
        }

        if self.exercise + 1 < self.exercise_count {
            Next::Exercise
        } else if self.session + 1 < self.session_count {
            Next::Session
        } else {
            Next::Finish
        }
    }

    // None means the screen stays (it may have scrolled)
    pub fn handle_key(
        &mut self,
        action: Action,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        let explorer = self.explorer;

        match (action, self.next()) {
            (Action::Up, _) => self.scroll = self.scroll.saturating_sub(1),
            (Action::Down, _) => {
                self.scroll = (self.scroll + 1).min(self.code.len().saturating_sub(1))
            }
            (Action::Select, Next::Exercise) => {
                return ExerciseState::open(
                    explorer,
                    self.session,
                    self.next_failed.unwrap_or(self.exercise + 1),
                    content,
                    progress,
                );
            }
            (Action::Select, Next::Session) => {
                return SessionDone::new(explorer, self.session, content, progress)
                    .map(|done| Screen::Complete(CompleteState::Session(done)));
            }
            (Action::Select | Action::Back, _) => return Some(Screen::Explorer(explorer)),
            _ => {}
        }

        None
    }
}
