use crate::app::{Action, Content, ExerciseState, ExplorerState, Screen};
use crate::progress::Progress;

// A module just finished: its name and its numbers
pub struct ModuleDone {
    pub explorer: ExplorerState,
    pub name: String,
    pub session_count: usize,
    pub exercise_count: usize,
    // The exercises of the last session, for the header
    pub last_exercises: usize,
    // The course just completed, when this module was the last one left in it
    pub finished_course: Option<String>,
}

impl ModuleDone {
    pub fn new(explorer: ExplorerState, content: &Content, progress: &Progress) -> Option<Self> {
        let module = content.module(explorer)?;
        let course = content.course(explorer)?;
        let name = course.module_at(explorer.module)?.name.clone();

        let course_id = content.course_id(explorer);
        let course_done = progress
            .next_module(explorer.language.id(), course_id, course)
            .is_none();
        let finished_course = course_done.then(|| {
            content.catalog(explorer.language).sections[explorer.section].courses[explorer.course]
                .name
                .clone()
        });

        Some(Self {
            explorer,
            name,
            session_count: module.sessions.len(),
            exercise_count: module.sessions.iter().map(|s| s.exercises.len()).sum(),
            last_exercises: module.sessions.last()?.exercises.len(),
            finished_course,
        })
    }

    // None means the screen stays
    pub fn handle_key(
        &mut self,
        action: Action,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        match action {
            // The end of a course goes back to the explorer
            Action::Select if self.finished_course.is_some() => {
                Some(Screen::Explorer(self.explorer))
            }
            // In the middle of a course: the same module Continue opens, now that this one is done
            Action::Select => {
                let (explorer, session, exercise) =
                    ExplorerState::continue_target(progress, content)?;
                ExerciseState::open(explorer, session, exercise, content, progress)
            }
            Action::Back => Some(Screen::Explorer(self.explorer)),
            _ => None,
        }
    }
}
