use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Quiz {
    pub id: String,
    pub kind: Option<QuizKind>,
    pub title: String,
    pub guide: String,
    pub code: String,
    pub options: Vec<String>,
    pub answer: usize,
    pub explanation: String,
}

// What the quiz asks: the purpose of the code, its output, the cause of a compiler message, or the kind of error
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuizKind {
    Purpose,
    Output,
    Compiler,
    Error,
}
