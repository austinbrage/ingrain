use super::{ExplorerState, HomeLanguage};
use crate::content::{Catalog, Course, Library, Module, Source};

// The loaded courses of every language, and the questions about a position in them
pub struct Content {
    rust: Library,
    go: Library,
}

impl Content {
    pub fn load(source: Source) -> Self {
        Self {
            rust: Library::load(source, "rust"),
            go: Library::load(source, "go"),
        }
    }

    pub fn library(&self, language: HomeLanguage) -> &Library {
        match language {
            HomeLanguage::Rust => &self.rust,
            HomeLanguage::Go => &self.go,
        }
    }

    pub fn catalog(&self, language: HomeLanguage) -> &Catalog {
        &self.library(language).catalog
    }

    pub fn course(&self, state: ExplorerState) -> Option<&Course> {
        let library = self.library(state.language);
        let id = &library.catalog.sections[state.section].courses[state.course].id;

        library.courses.get(id)
    }

    pub fn module(&self, state: ExplorerState) -> Option<Module> {
        let summary = self.course(state)?.module_at(state.module)?;

        Module::load(
            self.library(state.language).source,
            state.language.id(),
            self.course_id(state),
            &summary.id,
        )
    }

    pub fn course_id(&self, state: ExplorerState) -> &str {
        &self.catalog(state.language).sections[state.section].courses[state.course].id
    }

    // "rust/syntax/loops": the id that progress uses for a module
    pub fn module_path(&self, state: ExplorerState) -> Option<String> {
        let module_id = &self.course(state)?.module_at(state.module)?.id;

        Some(format!(
            "{}/{}/{module_id}",
            state.language.id(),
            self.course_id(state)
        ))
    }

    // " Rust · Syntax · Loops": the language, course and module of an explorer position
    pub fn breadcrumb(&self, state: ExplorerState) -> String {
        let catalog = self.catalog(state.language);
        let course_name = &catalog.sections[state.section].courses[state.course].name;
        let module_name = self
            .course(state)
            .and_then(|course| course.module_at(state.module))
            .map_or("", |module| module.name.as_str());

        format!(" {} · {} · {}", catalog.name, course_name, module_name)
    }
}
