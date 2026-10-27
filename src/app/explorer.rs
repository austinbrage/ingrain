use super::content::Content;
use super::{Action, App, ExerciseState, HomeLanguage, HomeState, Screen};
use crate::progress::Progress;

#[derive(Clone, Copy, PartialEq)]
pub enum Depth {
    Sections,
    Courses,
    Modules,
    Sessions,
}

// Where the user is: the focused column (`depth`) and the selected item of each column
#[derive(Clone, Copy)]
pub struct ExplorerState {
    pub language: HomeLanguage,
    pub depth: Depth,
    pub section: usize,
    pub course: usize,
    pub module: usize,
    pub session: usize,
}

// What the explorer marks on the modules of a course: sessions passed, which have a failed
// session, and the next module
#[derive(Default)]
pub struct ModuleMarks {
    pub passed: Vec<usize>,
    pub failed: Vec<bool>,
    pub next: Option<usize>,
    // Which sessions of the selected module are done, only while the sessions column is open
    pub sessions: Vec<bool>,
    // And which of them are failed
    pub sessions_failed: Vec<bool>,
}

impl ExplorerState {
    pub fn new(language: HomeLanguage) -> Self {
        Self {
            language,
            depth: Depth::Sections,
            section: 0,
            course: 0,
            module: 0,
            session: 0,
        }
    }

    // Where Continue opens: the module of the last course, with its session and exercise
    pub fn continue_target(progress: &Progress, content: &Content) -> Option<(Self, usize, usize)> {
        let language = HomeState::last_language(progress)?;
        let at = progress.continue_at(content.library(language))?;

        let mut explorer = Self::new(language);
        explorer.depth = Depth::Modules;
        explorer.section = at.section;
        explorer.course = at.course;
        explorer.module = at.module;

        Some((explorer, at.session, at.exercise))
    }

    // What the explorer marks on the modules of the course on screen
    pub fn module_marks(self, app: &App) -> ModuleMarks {
        let Some(course) = app.content.course(self) else {
            return ModuleMarks::default();
        };
        let (language, course_id) = (self.language.id(), app.content.course_id(self));

        // The module file is read on every frame the sessions column is open. It is embedded
        // and small, so nothing is cached, and the marks can never be stale
        let mut sessions = Vec::new();
        let mut sessions_failed = Vec::new();
        if self.depth == Depth::Sessions
            && let (Some(path), Some(module)) =
                (app.content.module_path(self), app.content.module(self))
        {
            sessions = app.progress.sessions_done(&path, &module);
            sessions_failed = app.progress.sessions_failed(&path, &module);
        }

        ModuleMarks {
            passed: app.progress.passed_counts(language, course_id, course),
            failed: app.progress.modules_failed(language, course_id, course),
            next: app.progress.next_module(language, course_id, course),
            sessions,
            sessions_failed,
        }
    }

    // "Rust", "Rust › Core", "Rust › Core › Syntax" or with the module: the path down to the focused column
    pub fn explorer_breadcrumb(self, app: &App) -> String {
        let catalog = app.content.catalog(self.language);
        let section = &catalog.sections[self.section];
        let course = &section.courses[self.course];
        let module = app
            .content
            .course(self)
            .and_then(|course| course.module_at(self.module))
            .map_or("", |module| module.name.as_str());

        match self.depth {
            Depth::Sections => catalog.name.clone(),
            Depth::Courses => format!("{} › {}", catalog.name, section.name),
            Depth::Modules => format!("{} › {} › {}", catalog.name, section.name, course.name),
            Depth::Sessions => format!(
                "{} › {} › {} › {module}",
                catalog.name, section.name, course.name
            ),
        }
    }

    // One step up or down, staying inside the list
    fn step(index: usize, len: usize, action: Action) -> usize {
        match action {
            Action::Down => (index + 1).min(len - 1),
            Action::Up => index.saturating_sub(1),
            _ => index,
        }
    }

    pub(super) fn handle_key(
        &mut self,
        action: Action,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        match self.depth {
            Depth::Sections => self.handle_sections_key(action, content),
            Depth::Courses => self.handle_courses_key(action, content),
            Depth::Modules => self.handle_modules_key(action, content, progress),
            Depth::Sessions => self.handle_sessions_key(action, content, progress),
        }
    }

    // Handle 2nd Column on Explorer
    fn handle_sections_key(&mut self, action: Action, content: &Content) -> Option<Screen> {
        let section_count = content.catalog(self.language).sections.len();

        match action {
            Action::Up | Action::Down => {
                self.section = Self::step(self.section, section_count, action);
                self.course = 0;
                self.module = 0;
            }
            Action::Right | Action::Select => self.depth = Depth::Courses,
            Action::Left | Action::Back => {
                return Some(Screen::Home(HomeState::Languages(self.language)));
            }
            _ => {}
        }

        None
    }

    // Handle 3nd Column on Explorer
    fn handle_courses_key(&mut self, action: Action, content: &Content) -> Option<Screen> {
        let course_count = content.catalog(self.language).sections[self.section]
            .courses
            .len();
        let module_count = content
            .course(*self)
            .map_or(0, |course| course.module_count());

        match action {
            Action::Up | Action::Down => {
                self.course = Self::step(self.course, course_count, action);
                self.module = 0;
            }
            Action::Right | Action::Select if module_count > 0 => self.depth = Depth::Modules,
            Action::Left | Action::Back => self.depth = Depth::Sections,
            _ => {}
        }

        None
    }

    // Handle 4th Column on Explorer
    fn handle_modules_key(
        &mut self,
        action: Action,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        let module_count = content
            .course(*self)
            .map_or(0, |course| course.module_count());

        match action {
            Action::Up | Action::Down => {
                self.module = Self::step(self.module, module_count, action)
            }
            Action::Select => {
                let path = content.module_path(*self)?;
                let module = content.module(*self)?;
                let (session, exercise) = progress.resume_point(&path, &module);

                // A module with a finished session and more than one session lets the user pick
                if progress.done_count(&path) > 0 && module.sessions.len() > 1 {
                    self.depth = Depth::Sessions;
                    self.session = session;
                } else {
                    return ExerciseState::open(*self, session, exercise, content, progress);
                }
            }
            Action::Left | Action::Back => self.depth = Depth::Courses,
            _ => {}
        }

        None
    }

    // Handle the sessions column of a module with a finished session
    fn handle_sessions_key(
        &mut self,
        action: Action,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        let path = content.module_path(*self)?;
        let module = content.module(*self)?;

        match action {
            Action::Up | Action::Down => {
                self.session = Self::step(self.session, module.sessions.len(), action)
            }
            Action::Select => {
                let exercise = progress.resume_exercise(&path, &module, self.session);
                return ExerciseState::open(*self, self.session, exercise, content, progress);
            }
            Action::Left | Action::Back => self.depth = Depth::Modules,
            _ => {}
        }

        None
    }
}
