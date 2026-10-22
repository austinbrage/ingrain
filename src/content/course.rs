use serde::Deserialize;

use super::{Module, Source, read_content};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Course {
    pub id: String,
    pub module_sections: Vec<ModuleSection>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModuleSection {
    pub name: String,
    pub tier: String,
    pub modules: Vec<ModuleSummary>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModuleSummary {
    pub id: String,
    pub name: String,
    pub sessions: u32,
    // Whether the module file exists yet, set when the course is loaded
    #[serde(skip)]
    pub written: bool,
}

impl Course {
    pub fn load(source: Source, language: &str, course_id: &str) -> Option<Self> {
        let path = format!("{language}/{course_id}/course.json");
        let json = read_content(source, &path)?;

        let mut course: Self =
            serde_json::from_str(json).unwrap_or_else(|error| panic!("{path} is invalid: {error}"));

        for module_section in &mut course.module_sections {
            for module in &mut module_section.modules {
                module.written = Module::exists(source, language, course_id, &module.id);
            }
        }

        Some(course)
    }

    pub fn module_count(&self) -> usize {
        self.module_sections
            .iter()
            .map(|section| section.modules.len())
            .sum()
    }

    pub fn module_at(&self, index: usize) -> Option<&ModuleSummary> {
        self.module_sections
            .iter()
            .flat_map(|section| &section.modules)
            .nth(index)
    }

    pub fn module_index(&self, id: &str) -> Option<usize> {
        self.module_sections
            .iter()
            .flat_map(|section| &section.modules)
            .position(|module| module.id == id)
    }
}
