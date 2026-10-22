use serde::Deserialize;

use super::Output;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Typing {
    pub id: String,
    pub title: String,
    pub guide: String,
    pub chunks: Vec<Chunk>,
    pub output: Option<Output>,
}

// Code typed at once. `line` is where its first line goes in the file (1-based)
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Chunk {
    pub mode: Mode,
    pub line: usize,
    pub guide: Option<String>,
    #[serde(default)]
    pub notes: Vec<Note>,
    pub text: String,
}

// A hint on a piece of the chunk: `on` is the anchor (occurs once), `text` is plain text
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Note {
    pub on: String,
    pub text: String,
}

// Ghost: dimmed in the editor. Copy: shown in the guide. Auto: inserted by the app
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Ghost,
    Copy,
    Auto,
}
