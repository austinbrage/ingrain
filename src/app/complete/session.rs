use crate::app::{Action, Content, ExerciseState, ExplorerState, Screen};
use crate::progress::Progress;

// One exercise of the finished session, as the recap lists it
pub struct DoneRow {
    pub title: String,
    pub kind: &'static str,
    pub passed: bool,
}

// A session just finished: its exercises, and how each went
pub struct SessionDone {
    pub explorer: ExplorerState,
    pub session: usize,
    pub session_count: usize,
    pub rows: Vec<DoneRow>,
}

impl SessionDone {
    pub fn new(
        explorer: ExplorerState,
        session: usize,
        content: &Content,
        progress: &Progress,
    ) -> Option<Self> {
        let module = content.module(explorer)?;
        let path = content.module_path(explorer)?;
        let failed = progress.exercises_failed(&path, &module, session);

        let rows = module.sessions[session]
            .exercises
            .iter()
            .zip(failed)
            .map(|(exercise, failed)| DoneRow {
                title: exercise.title().to_string(),
                kind: exercise.kind(),
                passed: !failed,
            })
            .collect();

        Some(Self {
            explorer,
            session,
            session_count: module.sessions.len(),
            rows,
        })
    }

    pub fn passed(&self) -> usize {
        self.rows.iter().filter(|row| row.passed).count()
    }

    // None means the screen stays
    pub fn handle_key(
        &mut self,
        action: Action,
        content: &Content,
        progress: &mut Progress,
    ) -> Option<Screen> {
        match action {
            Action::Select => {
                ExerciseState::open(self.explorer, self.session + 1, 0, content, progress)
            }
            Action::Back => Some(Screen::Explorer(self.explorer)),
            _ => None,
        }
    }
}
