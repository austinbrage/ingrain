use serde::Deserialize;

use super::{Source, read_content};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub name: String,
    pub layers: Vec<Layer>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub courses: Vec<CourseSummary>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CourseSummary {
    pub id: String,
    pub name: String,
}

impl Catalog {
    pub fn load(source: Source, language: &str) -> Self {
        let path = format!("{language}/catalog.json");
        let json =
            read_content(source, &path).unwrap_or_else(|| panic!("missing content file {path}"));

        serde_json::from_str(json).unwrap_or_else(|error| panic!("{path} is invalid: {error}"))
    }

    // The layer and course positions of a course id
    pub fn course_index(&self, id: &str) -> Option<(usize, usize)> {
        self.layers.iter().enumerate().find_map(|(layer, data)| {
            let course = data.courses.iter().position(|course| course.id == id)?;
            Some((layer, course))
        })
    }
}
