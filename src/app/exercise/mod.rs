mod quiz;
mod typing;

pub use quiz::QuizState;
pub use typing::{EditorView, Kind, Row, ShortcutUses, TypingState};

use super::content::Content;
use super::{Action, CompleteState, ExerciseDone, ExplorerState, IntroState, Screen, TypingKey};
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

        if IntroState::applies(&module, session, exercise, retake) {
            return Some(Screen::Intro(IntroState::new(explorer, module, session)));
        }

        let next_failed = if retake {
            progress.next_failed_exercise(&path, &module, session, exercise)
        } else {
            None
        };

        Some(Self::build(
            explorer,
            module,
            session,
            exercise,
            retake,
            next_failed,
        ))
    }

    // The first exercise of a session once its intros are read, which is never a retake
    pub fn start(explorer: ExplorerState, module: Module, session: usize) -> Screen {
        Self::build(explorer, module, session, 0, false, None)
    }

    fn build(
        explorer: ExplorerState,
        module: Module,
        session: usize,
        exercise: usize,
        retake: bool,
        next_failed: Option<usize>,
    ) -> Screen {
        let body = match &module.sessions[session].exercises[exercise] {
            Exercise::Typing(typing) => Body::Typing(TypingState::new(typing)),
            Exercise::Quiz(_) => Body::Quiz(QuizState::default()),
        };

        Screen::Exercise(Self {
            explorer,
            module,
            session,
            exercise,
            retake,
            next_failed,
            body,
        })
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
        shortcut_uses: &mut ShortcutUses,
    ) -> Option<Screen> {
        let (Exercise::Typing(typing), Body::Typing(typing_state)) = (
            &self.module.sessions[self.session].exercises[self.exercise],
            &mut self.body,
        ) else {
            return None;
        };

        // The modal takes the keys: Enter finishes the chunk, Esc leaves it to the user
        if typing_state.modal_open {
            match key {
                TypingKey::Enter => {
                    typing_state.modal_open = false;
                    typing_state.start_autotype(typing);
                }
                TypingKey::Back => typing_state.modal_open = false,
                _ => {}
            }
            return None;
        }

        // A chunk finished with the keys, and not with the shortcut, starts the count over
        let by_hand = typing_state.autotype.is_none() && key != TypingKey::Finish;
        let chunk = typing_state.chunk;

        match key {
            TypingKey::Back => return Some(Screen::Explorer(self.explorer)),
            // While the shortcut types, any other key stops it where it reached and is not typed
            _ if typing_state.autotype.is_some() => typing_state.autotype = None,
            TypingKey::Backspace => typing_state.backspace(),
            TypingKey::Finish => {
                shortcut_uses.add();
                if shortcut_uses.shows_modal() {
                    typing_state.modal_open = true;
                } else {
                    typing_state.start_autotype(typing);
                }
            }
            TypingKey::Enter => typing_state.type_char(typing, '\n'),
            TypingKey::Tab => {
                for _ in 0..4 {
                    typing_state.type_char(typing, ' ');
                }
            }
            TypingKey::Char(c) => typing_state.type_char(typing, c),
        }

        if by_hand && typing_state.chunk > chunk {
            shortcut_uses.reset();
        }

        self.complete_if_finished(content, progress)
    }

    // One tick of the shortcut, which may type the last char of the exercise
    pub fn tick(&mut self, content: &Content, progress: &mut Progress) -> Option<Screen> {
        if let (Exercise::Typing(typing), Body::Typing(typing_state)) = (
            &self.module.sessions[self.session].exercises[self.exercise],
            &mut self.body,
        ) {
            typing_state.tick_autotype(typing);
        }

        self.complete_if_finished(content, progress)
    }

    // Whether the shortcut is typing, so the screen needs the tick
    pub fn autotyping(&self) -> bool {
        matches!(&self.body, Body::Typing(typing_state) if typing_state.autotype.is_some())
    }

    // The complete screen when the last chunk of a typing exercise is done
    fn complete_if_finished(
        &mut self,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        let (Exercise::Typing(typing), Body::Typing(typing_state)) = (
            &self.module.sessions[self.session].exercises[self.exercise],
            &self.body,
        ) else {
            return None;
        };

        if typing_state.current(typing).is_some() {
            return None;
        }

        let code = typing_state.file.clone();
        let complete = CompleteState::Exercise(ExerciseDone::new(self, typing, code));
        self.record_completion(content, progress, true);
        Some(Screen::Complete(complete))
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
                // A quiz has no complete screen. `done` is the state that screen would have, and
                // it is only used to decide where to go: its Enter handling returns the screen
                // that follows (the next exercise, the session recap, the module screen or the
                // explorer). `done` itself is never put on screen
                let mut done = ExerciseDone::from_quiz(self, quiz);
                self.record_completion(content, progress, passed);
                return done.handle_key(Action::Select, content, progress);

                // Before, this Enter opened that state as a complete screen, and the user had
                // to press Enter again to go on:
                // let complete = CompleteState::Exercise(ExerciseDone::from_quiz(self, quiz));
                // return Some(Screen::Complete(complete));
            }
            (Action::Left | Action::Right | Action::Quit, _) => {}
        }

        None
    }
}
