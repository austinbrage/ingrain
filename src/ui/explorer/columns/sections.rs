use super::{Column, Item};
use crate::app::{Depth, ExplorerState};
use crate::content::Catalog;

pub fn sections_column(catalog: &Catalog, state: ExplorerState) -> Column {
    Column {
        title: catalog.name.to_uppercase(),
        items: catalog
            .sections
            .iter()
            .map(|section| Item {
                text: section.name.clone(),
                right: "›".to_string(),
                ..Item::default()
            })
            .collect(),
        selected: Some(state.section),
        focused: state.depth == Depth::Sections,
    }
}
