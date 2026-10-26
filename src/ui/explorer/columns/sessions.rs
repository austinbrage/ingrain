use super::{Column, Item, NEXT_WORD, typed_text};
use crate::app::{ExplorerState, ModuleMarks};
use crate::content::Course;

// The sessions of the selected module: `✓` in green when done, `✗` in red when failed, `○` when not
pub fn sessions_column(
    state: ExplorerState,
    course: Option<&Course>,
    marks: &ModuleMarks,
    tick: u64,
) -> Column {
    let module_name = course
        .and_then(|course| course.module_at(state.module))
        .map_or("", |module| module.name.as_str());

    // The first session not done is the next one, like the next module
    let next_session = marks.sessions.iter().position(|&done| !done);

    let items = marks
        .sessions
        .iter()
        .enumerate()
        .map(|(index, &done)| {
            let failed = marks.sessions_failed.get(index).copied().unwrap_or(false);
            let mark = if failed {
                "✗"
            } else if done {
                "✓"
            } else {
                "○"
            };
            let next = next_session == Some(index);

            Item {
                text: format!("  Session {}", index + 1),
                right: if next {
                    typed_text([NEXT_WORD, mark], tick)
                } else {
                    mark.to_string()
                },
                complete: done && !failed,
                next,
            }
        })
        .collect();

    Column {
        title: module_name.to_uppercase(),
        items,
        selected: Some(state.session),
        focused: true,
    }
}
