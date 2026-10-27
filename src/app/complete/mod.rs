mod exercise;
mod module;
mod session;

pub use exercise::{ExerciseDone, Next};
pub use module::ModuleDone;
pub use session::{DoneRow, SessionDone};

use crate::app::{Action, Content, Screen};
use crate::progress::Progress;

// The screen that ends something: an exercise, and later a session and a module
pub enum CompleteState {
    Exercise(ExerciseDone),
    Session(SessionDone),
    Module(ModuleDone),
}

impl CompleteState {
    // None means the screen stays
    pub fn handle_key(
        &mut self,
        action: Action,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        match self {
            Self::Exercise(done) => done.handle_key(action, content, progress),
            Self::Session(done) => done.handle_key(action, content, progress),
            Self::Module(done) => done.handle_key(action, content, progress),
        }
    }
}
