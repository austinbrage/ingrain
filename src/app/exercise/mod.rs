mod quiz;
mod typing;

pub use quiz::QuizState;
pub use typing::{EditorView, Kind, Row, TypingState};

use super::content::Content;
use super::{Action, CompleteState, ExerciseDone, ExplorerState, Screen, TypingKey};
use crate::content::{Exercise, Module};
use crate::progress::Progress;

pub enum Body {
    Typing(TypingState),
    Quiz(QuizState),
}

pub struct ExerciseState {
    pub explorer: ExplorerState,
    pub module: Module,
    pub session: usize,
    pub exercise: usize,
    // A retake plays only the failed exercises of a session that was played through,
    // and `next_failed` is the one after this
    pub retake: bool,
    pub next_failed: Option<usize>,
    pub body: Body,
}

impl ExerciseState {
    // The screen of an exercise at a place of a module, None when the module file is missing.
    // It also remembers the course, for the Continue button
    pub fn open(
        explorer: ExplorerState,
        session: usize,
        exercise: usize,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        let module = content.module(explorer)?;
        let path = content.module_path(explorer)?;

        Self::remember_course(explorer, content, progress);

        let retake = progress.is_retake(&path, &module.sessions[session].id);
        let next_failed = if retake {
            progress.next_failed_exercise(&path, &module, session, exercise)
        } else {
            None
        };

        let body = match &module.sessions[session].exercises[exercise] {
            Exercise::Typing(typing) => Body::Typing(TypingState::new(typing)),
            Exercise::Quiz(_) => Body::Quiz(QuizState::default()),
        };

        Some(Screen::Exercise(Self {
            explorer,
            module,
            session,
            exercise,
            retake,
            next_failed,
            body,
        }))
    }

    pub fn session_count(&self) -> usize {
        self.module.sessions.len()
    }

    pub fn exercise_count(&self) -> usize {
        self.module.sessions[self.session].exercises.len()
    }

    // Save which course the user is practicing, so Continue can come back to it
    fn remember_course(explorer: ExplorerState, content: &Content, progress: &mut Progress) {
        progress.set_last(explorer.language.id(), content.course_id(explorer));
        progress.save();
    }

    // Save the finished exercise in progress.json, with whether it was passed
    fn record_completion(&self, content: &Content, progress: &mut Progress, passed: bool) {
        let Some(module) = content.module_path(self.explorer) else {
            return;
        };

        let session = &self.module.sessions[self.session];
        let exercise = session.exercises[self.exercise].id();
        let next_exercise = if self.retake {
            None
        } else {
            session.exercises.get(self.exercise + 1).map(Exercise::id)
        };

        progress.complete_exercise(&module, &session.id, exercise, passed, next_exercise);
        Self::remember_course(self.explorer, content, progress);
    }

    // Raw keys: a typing exercise needs letters, not actions. None means the screen stays
    pub fn handle_typing_key(
        &mut self,
        key: TypingKey,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        let (Exercise::Typing(typing), Body::Typing(typing_state)) = (
            &self.module.sessions[self.session].exercises[self.exercise],
            &mut self.body,
        ) else {
            return None;
        };

        match key {
            TypingKey::Back => return Some(Screen::Explorer(self.explorer)),
            TypingKey::Backspace => typing_state.backspace(),
            TypingKey::Enter => typing_state.type_char(typing, '\n'),
            TypingKey::Tab => {
                for _ in 0..4 {
                    typing_state.type_char(typing, ' ');
                }
            }
            TypingKey::Char(c) => typing_state.type_char(typing, c),
        }

        if typing_state.current(typing).is_none() {
            let code = typing_state.file.clone();
            let complete = CompleteState::Exercise(ExerciseDone::new(self, typing, code));
            self.record_completion(content, progress, true);
            return Some(Screen::Complete(complete));
        }

        None
    }

    // Actions: a quiz moves with Up and Down, Enter checks the option and then goes on
    pub fn handle_quiz_action(
        &mut self,
        action: Action,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        let (Exercise::Quiz(quiz), Body::Quiz(quiz_state)) = (
            &self.module.sessions[self.session].exercises[self.exercise],
            &mut self.body,
        ) else {
            return None;
        };

        match (action, quiz_state.answered) {
            (Action::Back, _) => return Some(Screen::Explorer(self.explorer)),
            (Action::Up, _) => quiz_state.up(),
            (Action::Down, _) => quiz_state.down(quiz),
            (Action::Select, false) => quiz_state.answered = true,
            (Action::Select, true) => {
                let passed = quiz_state.selected == quiz.answer;
                let complete = CompleteState::Exercise(ExerciseDone::from_quiz(self, quiz));
                self.record_completion(content, progress, passed);
                return Some(Screen::Complete(complete));
            }
            (Action::Left | Action::Right | Action::Quit, _) => {}
        }

        None
    }
}
