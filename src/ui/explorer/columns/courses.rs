use super::{Column, Item};
use crate::app::{Depth, ExplorerState};
use crate::content::Catalog;

pub fn courses_column(catalog: &Catalog, state: ExplorerState) -> Column {
    let layer = &catalog.layers[state.layer];

    let mut items = Vec::new();
    let mut selected = None;
    let mut course_index = 0;

    for group in &layer.course_groups {
        if !items.is_empty() {
            items.push(Item::default());
        }

        items.push(Item {
            text: group.name.to_uppercase(),
            ..Item::default()
        });

        for course in &group.courses {
            if state.depth != Depth::Layers && course_index == state.course {
                selected = Some(items.len());
            }

            items.push(Item {
                text: format!("  {}", course.name),
                right: "›".to_string(),
                ..Item::default()
            });
            course_index += 1;
        }
    }

    Column {
        title: layer.name.to_uppercase(),
        items,
        selected,
        focused: state.depth == Depth::Courses,
        coming_soon: false,
    }
}
