use serde::Deserialize;

// What teaches a concept before a module or a session, in slides passed with Enter
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Intro {
    pub title: String,
    pub slides: Vec<Slide>,
}

// `text` is markdown (the subset of the guide). `code` is a read-only snippet
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Slide {
    pub text: String,
    pub code: Option<String>,
}
