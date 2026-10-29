mod catalog;
mod course;
mod module;

pub use catalog::Catalog;
pub use course::Course;
pub use module::{
    Chunk, Exercise, Intro, Mode, Module, Note, Output, Quiz, QuizKind, Session, Slide, Typing,
};

use include_dir::{Dir, include_dir};
use std::collections::HashMap;

// Embedded at compile time, so a content change needs a rebuild
static PROD: Dir = include_dir!("$CARGO_MANIFEST_DIR/content/prod");
static TEST: Dir = include_dir!("$CARGO_MANIFEST_DIR/content/test");

// Which content folder is read: the real courses or the small frozen ones the tests use
#[derive(Clone, Copy)]
pub enum Source {
    Prod,
    Test,
}

impl Source {
    fn dir(self) -> &'static Dir<'static> {
        match self {
            Source::Prod => &PROD,
            Source::Test => &TEST,
        }
    }
}

// A language's catalog and its courses by id
pub struct Library {
    pub source: Source,
    pub catalog: Catalog,
    pub courses: HashMap<String, Course>,
}

impl Library {
    pub fn load(source: Source, language: &str) -> Self {
        let catalog = Catalog::load(source, language);
        let mut courses = HashMap::new();

        for section in &catalog.sections {
            for summary in &section.courses {
                if let Some(course) = Course::load(source, language, &summary.id) {
                    courses.insert(summary.id.clone(), course);
                }
            }
        }

        Self {
            source,
            catalog,
            courses,
        }
    }
}

fn read_content(source: Source, path: &str) -> Option<&'static str> {
    source.dir().get_file(path)?.contents_utf8()
}
