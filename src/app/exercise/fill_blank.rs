use crate::content::FillBlank;

// The longest word a gap takes, so a long run of keys cannot break the code window
const MAX_WORD_CHARS: usize = 20;

// Progress: the gap under the cursor, the word typed in each gap and whether they were checked
pub struct FillBlankState {
    pub gap: usize,
    pub words: Vec<String>,
    pub checked: bool,
}

impl FillBlankState {
    pub fn new(fill_blank: &FillBlank) -> Self {
        Self {
            gap: 0,
            words: vec![String::new(); fill_blank.gaps.len()],
            checked: false,
        }
    }

    // A word has no spaces, and a gap holds a limited number of chars
    pub fn type_char(&mut self, c: char) {
        if let Some(word) = self.words.get_mut(self.gap)
            && !c.is_whitespace()
            && word.chars().count() < MAX_WORD_CHARS
        {
            word.push(c);
        }
    }

    pub fn backspace(&mut self) {
        if let Some(word) = self.words.get_mut(self.gap) {
            word.pop();
        }
    }

    // Enter: on to the next gap, or check them all on the last one
    pub fn enter(&mut self) {
        if self.gap + 1 < self.words.len() {
            self.gap += 1;
        } else {
            self.checked = true;
        }
    }

    // A gap is right when its word is one of the accepted answers, letter for letter
    pub fn is_right(&self, fill_blank: &FillBlank, gap: usize) -> bool {
        fill_blank.gaps[gap].answers.contains(&self.words[gap])
    }

    pub fn passed(&self, fill_blank: &FillBlank) -> bool {
        (0..self.words.len()).all(|gap| self.is_right(fill_blank, gap))
    }
}
