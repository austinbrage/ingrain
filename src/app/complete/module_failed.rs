use crate::app::{Action, Content, ExerciseState, ExplorerState, Screen};
use crate::progress::Progress;

// One session of the finished module: the exercises passed out of its total
pub struct SessionRow {
    pub passed: usize,
    pub total: usize,
}

// A module just finished with a failed session: how each of its sessions went
pub struct ModuleFailed {
    pub explorer: ExplorerState,
    pub name: String,
    pub rows: Vec<SessionRow>,
    // The exercises of the last session, for the header
    pub last_exercises: usize,
}

impl ModuleFailed {
    pub fn new(explorer: ExplorerState, content: &Content, progress: &Progress) -> Option<Self> {
        let module = content.module(explorer)?;
        let path = content.module_path(explorer)?;
        let name = content
            .course(explorer)?
            .module_at(explorer.module)?
            .name
            .clone();

        let rows = (0..module.sessions.len())
            .map(|session| {
                let failed = progress.exercises_failed(&path, &module, session);

                SessionRow {
                    passed: failed.iter().filter(|failed| !**failed).count(),
                    total: failed.len(),
                }
            })
            .collect();

        Some(Self {
            explorer,
            name,
            rows,
            last_exercises: module.sessions.last()?.exercises.len(),
        })
    }

    // The sessions with every exercise passed
    pub fn passed_sessions(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.passed == row.total)
            .count()
    }

    // None means the screen stays
    pub fn handle_key(
        &mut self,
        action: Action,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        match action {
            // The first failed exercise of the module, as Continue would pick it
            Action::Select => {
                let path = content.module_path(self.explorer)?;
                let module = content.module(self.explorer)?;
                let (session, exercise) = progress.resume_point(&path, &module);

                ExerciseState::open(self.explorer, session, exercise, content, progress)
            }
            Action::Back => Some(Screen::Explorer(self.explorer)),
            _ => None,
        }
    }
}
