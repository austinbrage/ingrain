use super::{Column, Item};
use crate::app::{Depth, ExplorerState};
use crate::content::Catalog;

pub fn courses_column(catalog: &Catalog, state: ExplorerState) -> Column {
    let section = &catalog.sections[state.section];

    Column {
        title: section.name.to_uppercase(),
        items: section
            .courses
            .iter()
            .map(|course| Item {
                text: course.name.clone(),
                right: "›".to_string(),
                ..Item::default()
            })
            .collect(),
        selected: if state.depth == Depth::Sections {
            None
        } else {
            Some(state.course)
        },
        focused: state.depth == Depth::Courses,
    }
}
