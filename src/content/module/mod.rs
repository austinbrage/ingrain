mod intro;
mod quiz;
mod typing;

pub use intro::{Intro, Slide};
pub use quiz::{Quiz, QuizKind};
pub use typing::{Chunk, Mode, Note, Typing};

use serde::Deserialize;

use super::{Source, read_content};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Module {
    pub id: String,
    pub intro: Option<Intro>,
    pub sessions: Vec<Session>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Session {
    pub id: String,
    pub intro: Option<Intro>,
    pub exercises: Vec<Exercise>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Exercise {
    Typing(Typing),
    Quiz(Quiz),
}

// Used on inner modules (exercise types)
#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Output {
    pub command: String,
    pub text: String,
}

impl Exercise {
    pub fn id(&self) -> &str {
        match self {
            Exercise::Typing(typing) => &typing.id,
            Exercise::Quiz(quiz) => &quiz.id,
        }
    }

    pub fn title(&self) -> &str {
        match self {
            Exercise::Typing(typing) => &typing.title,
            Exercise::Quiz(quiz) => &quiz.title,
        }
    }

    // What the exercise is, as the session recap names it
    pub fn kind(&self) -> &'static str {
        match self {
            Exercise::Typing(_) => "typing",
            Exercise::Quiz(_) => "quiz",
        }
    }
}

impl Module {
    pub fn load(source: Source, language: &str, course_id: &str, module_id: &str) -> Option<Self> {
        let path = Self::module_file(language, course_id, module_id);
        let json = read_content(source, &path)?;

        Some(
            serde_json::from_str(json).unwrap_or_else(|error| panic!("{path} is invalid: {error}")),
        )
    }

    // Whether the module file is written yet
    pub fn exists(source: Source, language: &str, course_id: &str, module_id: &str) -> bool {
        read_content(source, &Self::module_file(language, course_id, module_id)).is_some()
    }

    // The position of a session id in this module
    pub fn session_index(&self, id: &str) -> Option<usize> {
        self.sessions.iter().position(|session| session.id == id)
    }

    fn module_file(language: &str, course_id: &str, module_id: &str) -> String {
        format!("{language}/{course_id}/{module_id}.json")
    }
}

impl Session {
    // The position of an exercise id in this session
    pub fn exercise_index(&self, id: &str) -> Option<usize> {
        self.exercises
            .iter()
            .position(|exercise| exercise.id() == id)
    }
}
