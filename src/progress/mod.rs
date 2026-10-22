mod resume;
mod storage;

use std::{
    collections::HashMap,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::content::{Course, Library, Module};
use resume::{next_course, next_exercise, next_review, next_session};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Progress {
    pub last: Option<Last>,
    pub modules: Vec<ModuleProgress>,
    // The folder it is saved in, None when there is no place to save
    #[serde(skip)]
    dir: Option<PathBuf>,
}

#[derive(Serialize, Deserialize)]
pub struct Last {
    pub language: String,
    pub course: String,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleProgress {
    pub id: String,
    #[serde(default)]
    pub done: Vec<String>,
    #[serde(default)]
    pub in_progress: HashMap<String, String>,
    // Unix seconds of the last exercise finished in this module, 0 if unknown
    #[serde(default)]
    pub practiced: u64,
}

// Where Continue opens, as positions in the catalog and in the module file
pub struct ContinueAt {
    pub section: usize,
    pub course: usize,
    pub module: usize,
    pub session: usize,
    pub exercise: usize,
}

// What the Continue button offers
pub enum ContinueLabel {
    // The course it opens, as "Rust · Syntax"
    Course(String),
    // Nothing is left to study in the language: its name, as "Rust"
    Review(String),
}

impl Progress {
    // ------------------------------------------------------------
    // Loading and saving
    // ------------------------------------------------------------

    // Read the saved file of the user's data folder, or start empty if there is none
    pub fn load() -> Self {
        match storage::data_dir() {
            Some(dir) => Self::load_from(dir),
            None => {
                eprintln!("no data directory found, progress will not be saved");
                Self::default()
            }
        }
    }

    // Read the saved file of a folder, which is also where save writes
    pub fn load_from(dir: PathBuf) -> Self {
        let mut progress = storage::read(&dir);
        progress.dir = Some(dir);
        progress
    }

    // Write the file. A failure is only printed
    pub fn save(&self) {
        let Some(dir) = &self.dir else {
            return;
        };

        if let Err(error) = storage::write(self, dir) {
            eprintln!("could not save progress: {error}");
        }
    }

    // ------------------------------------------------------------
    // Reading
    // ------------------------------------------------------------

    // The saved entry of a module, if it has one
    fn find_module(&self, id: &str) -> Option<&ModuleProgress> {
        self.modules.iter().find(|entry| entry.id == id)
    }

    // Sessions done in a module, 0 if it has no entry yet
    pub fn done_count(&self, module: &str) -> usize {
        self.find_module(module).map_or(0, |entry| entry.done.len())
    }

    // When a module was last practiced, 0 if it never was
    fn practiced(&self, module: &str) -> u64 {
        self.find_module(module).map_or(0, |entry| entry.practiced)
    }

    // Sessions done in each module of a course, in course order
    pub fn done_counts(&self, language: &str, course_id: &str, course: &Course) -> Vec<usize> {
        course
            .module_sections
            .iter()
            .flat_map(|section| &section.modules)
            .map(|module| self.done_count(&format!("{language}/{course_id}/{}", module.id)))
            .collect()
    }

    // Which sessions of a module are done, in module order
    pub fn sessions_done(&self, module_path: &str, module: &Module) -> Vec<bool> {
        module
            .sessions
            .iter()
            .map(|session| self.is_session_done(module_path, &session.id))
            .collect()
    }

    // The index of the module to study next in a course, None when all are complete
    pub fn next_module(&self, language: &str, course_id: &str, course: &Course) -> Option<usize> {
        resume::next_module(self, language, course_id, course)
    }

    fn is_session_done(&self, module: &str, session: &str) -> bool {
        self.find_module(module)
            .is_some_and(|entry| entry.done.iter().any(|id| id == session))
    }

    // The exercise saved as next in a session, None if there is none
    fn in_progress_exercise(&self, module: &str, session: &str) -> Option<&str> {
        self.find_module(module)?
            .in_progress
            .get(session)
            .map(String::as_str)
    }

    // ------------------------------------------------------------
    // Continuing
    // ------------------------------------------------------------

    // The exercise to open in a session: the one saved for it, or the first
    pub fn resume_exercise(&self, module_path: &str, module: &Module, session: usize) -> usize {
        next_exercise(self, module_path, module, session)
    }

    // Where to open a module: its first unfinished session, at the exercise left off
    pub fn resume_point(&self, module_path: &str, module: &Module) -> (usize, usize) {
        let session = next_session(self, module_path, module);
        let exercise = self.resume_exercise(module_path, module, session);

        (session, exercise)
    }

    // What the Continue button offers, None for a new user
    pub fn continue_label(&self, library: &Library) -> Option<ContinueLabel> {
        let last = self.last.as_ref()?;
        let catalog = &library.catalog;
        // A course that no longer exists is treated like a new user
        catalog.course_index(&last.course)?;

        Some(
            match next_course(self, &last.language, library, &last.course) {
                Some((section, course, _)) => ContinueLabel::Course(format!(
                    "{} · {}",
                    catalog.name, catalog.sections[section].courses[course].name
                )),
                None => ContinueLabel::Review(catalog.name.clone()),
            },
        )
    }

    // Where Continue opens: a module of the last course, at its session and exercise
    pub fn continue_at(&self, library: &Library) -> Option<ContinueAt> {
        let last = self.last.as_ref()?;
        // With nothing left to study, review the module practiced longest ago
        let (section, course, module) = next_course(self, &last.language, library, &last.course)
            .or_else(|| next_review(self, &last.language, library))?;

        let course_id = &library.catalog.sections[section].courses[course].id;
        let module_id = &library.courses.get(course_id)?.module_at(module)?.id;
        let module_path = format!("{}/{course_id}/{module_id}", last.language);
        let data = Module::load(library.source, &last.language, course_id, module_id)?;

        let (session, exercise) = self.resume_point(&module_path, &data);

        Some(ContinueAt {
            section,
            course,
            module,
            session,
            exercise,
        })
    }

    // ------------------------------------------------------------
    // Recording
    // ------------------------------------------------------------

    // `next_exercise` is None when the finished exercise was the last one of its session
    pub fn complete_exercise(&mut self, module: &str, session: &str, next_exercise: Option<&str>) {
        // Find the module's entry, or create it the first time
        let index = match self.modules.iter().position(|entry| entry.id == module) {
            Some(index) => index,
            None => {
                self.modules.push(ModuleProgress {
                    id: module.to_string(),
                    done: Vec::new(),
                    in_progress: HashMap::new(),
                    practiced: 0,
                });
                self.modules.len() - 1
            }
        };
        let entry = &mut self.modules[index];
        entry.practiced = Self::now();

        match next_exercise {
            // More exercises to do: remember the next one
            Some(next) => {
                entry
                    .in_progress
                    .insert(session.to_string(), next.to_string());
            }
            // Session finished: it is no longer in progress and is saved as done
            None => {
                entry.in_progress.remove(session);

                if !entry.done.iter().any(|id| id == session) {
                    entry.done.push(session.to_string());
                    // Move the module to the end, so the list ends with the last one worked on
                    let entry = self.modules.remove(index);
                    self.modules.push(entry);
                }
            }
        }
    }

    // Remember the course the user is practicing, for the Continue button
    pub fn set_last(&mut self, language: &str, course: &str) {
        self.last = Some(Last {
            language: language.to_string(),
            course: course.to_string(),
        });
    }

    // The time now in Unix seconds, 0 if the clock is before 1970
    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |time| time.as_secs())
    }
}
