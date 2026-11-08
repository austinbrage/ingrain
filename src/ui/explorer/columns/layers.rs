use super::{Column, Item};
use crate::app::{Depth, ExplorerState};
use crate::content::Catalog;

pub fn layers_column(catalog: &Catalog, state: ExplorerState) -> Column {
    Column {
        title: catalog.name.to_uppercase(),
        items: catalog
            .layers
            .iter()
            .map(|layer| Item {
                text: layer.name.clone(),
                right: "›".to_string(),
                ..Item::default()
            })
            .collect(),
        selected: Some(state.layer),
        focused: state.depth == Depth::Layers,
        coming_soon: false,
    }
}
