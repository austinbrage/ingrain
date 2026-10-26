use super::{Column, Item, NEXT_WORD, typed_text};
use crate::app::{Depth, ExplorerState, ModuleMarks};
use crate::content::{Catalog, Course};

pub fn modules_column(
    catalog: &Catalog,
    state: ExplorerState,
    course: Option<&Course>,
    marks: &ModuleMarks,
    tick: u64,
) -> Column {
    let selected_course = &catalog.sections[state.section].courses[state.course];
    let focused = state.depth == Depth::Modules;

    let mut items = Vec::new();
    let mut selected = None;
    let mut module_index = 0;

    if let Some(course) = course {
        for module_section in &course.module_sections {
            if !items.is_empty() {
                items.push(Item::default());
            }

            items.push(Item {
                text: module_section.name.to_uppercase(),
                right: module_section.tier.clone(),
                ..Item::default()
            });

            for module in &module_section.modules {
                if focused && module_index == state.module {
                    selected = Some(items.len());
                }

                let passed = marks.passed.get(module_index).copied().unwrap_or(0);
                let failed = marks.failed.get(module_index).copied().unwrap_or(false);
                let complete = passed == module.sessions as usize;
                let next = marks.next == Some(module_index);
                let count = if complete {
                    format!("✓ {passed}/{}", module.sessions)
                } else if failed {
                    format!("✗ {passed}/{}", module.sessions)
                } else {
                    format!("{passed}/{}", module.sessions)
                };

                items.push(Item {
                    text: format!("  {}", module.name),
                    right: if next {
                        typed_text([NEXT_WORD, &count], tick)
                    } else {
                        count
                    },
                    complete,
                    next,
                });
                module_index += 1;
            }
        }
    }

    Column {
        title: selected_course.name.to_uppercase(),
        items,
        selected,
        focused,
    }
}
