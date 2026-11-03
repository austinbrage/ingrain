use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FillBlank {
    pub id: String,
    pub title: String,
    pub guide: String,
    pub code: String,
    pub gaps: Vec<Gap>,
}

// The words a gap accepts. The first one is the one shown when the user is wrong
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    pub answers: Vec<String>,
}
