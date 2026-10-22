use crate::content::Quiz;

// Progress: the option under the selection and whether it was checked
#[derive(Default)]
pub struct QuizState {
    pub selected: usize,
    pub answered: bool,
}

impl QuizState {
    pub fn up(&mut self) {
        if !self.answered {
            self.selected = self.selected.saturating_sub(1);
        }
    }

    pub fn down(&mut self, quiz: &Quiz) {
        if !self.answered {
            self.selected = (self.selected + 1).min(quiz.options.len().saturating_sub(1));
        }
    }
}
