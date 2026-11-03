use super::{Column, Item};
use crate::app::{Depth, ExplorerState};
use crate::content::Catalog;

pub fn courses_column(catalog: &Catalog, state: ExplorerState) -> Column {
    let layer = &catalog.layers[state.layer];

    Column {
        title: layer.name.to_uppercase(),
        items: layer
            .courses
            .iter()
            .map(|course| Item {
                text: course.name.clone(),
                right: "›".to_string(),
                ..Item::default()
            })
            .collect(),
        selected: if state.depth == Depth::Layers {
            None
        } else {
            Some(state.course)
        },
        focused: state.depth == Depth::Courses,
    }
}
