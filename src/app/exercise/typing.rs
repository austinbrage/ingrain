use crate::content::{Chunk, Mode, Note, Typing};

// The most ticks the shortcut takes to type the rest of a chunk: 2 seconds
const AUTOTYPE_TICKS: usize = 40;

// How many times in a row the shortcut was used. The modal shows at the 4th use, then at the
// 9th, the 16th...: the perfect squares from 4 up
#[derive(Default)]
pub struct ShortcutUses(usize);

impl ShortcutUses {
    pub fn add(&mut self) {
        self.0 += 1;
    }

    pub fn shows_modal(&self) -> bool {
        self.0 >= 4 && self.0.isqrt().pow(2) == self.0
    }

    // A chunk typed by hand starts the count over
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

// Progress: the current chunk, what was typed of it (auto-indent included), the finished file
pub struct TypingState {
    pub chunk: usize,
    pub typed: String,
    pub file: Vec<String>,
    pub autotype: Option<Autotype>,
    // The modal is open and takes the keys until it is answered
    pub modal_open: bool,
}

// The shortcut is typing the rest of a chunk: which one, and how many chars it types per tick
#[derive(Clone, Copy)]
pub struct Autotype {
    pub chunk: usize,
    pub per_tick: usize,
}

// What a piece is: typed right, wrong key, cursor (next char) or ghost (not typed yet)
#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    Typed,
    Wrong,
    Cursor,
    Ghost,
}

// A run of text of one kind
pub struct Piece {
    pub kind: Kind,
    pub text: String,
}

// A file line cut in pieces by kind. `line` is 0-based in the whole file
pub struct Row {
    pub line: usize,
    pub pieces: Vec<Piece>,
}

// What the editor draws: rows, cursor row, expected text (for colors), expected key if wrong
pub struct EditorView {
    pub rows: Vec<Row>,
    pub cursor_row: Option<usize>,
    pub text: String,
    pub wrong_expected: Option<char>,
}

impl TypingState {
    // ------------------------------------------------------------
    // Typing rules
    // ------------------------------------------------------------

    pub fn new(typing: &Typing) -> Self {
        let mut state = Self {
            chunk: 0,
            typed: String::new(),
            file: Vec::new(),
            autotype: None,
            modal_open: false,
        };

        state.skip_auto(typing);
        state.indent(typing);
        state
    }

    pub fn current<'a>(&self, typing: &'a Typing) -> Option<&'a Chunk> {
        typing.chunks.get(self.chunk)
    }

    // The notes whose anchor the user has reached while typing
    pub fn notes<'a>(&'a self, chunk: &'a Chunk) -> impl Iterator<Item = &'a Note> {
        let typed = self.typed.chars().count();

        chunk.notes.iter().filter(move |note| {
            let start = chunk
                .text
                .find(note.on.as_str())
                .map_or(0, |byte| chunk.text[..byte].chars().count());

            typed >= start
        })
    }

    // Index of the first typed char that differs from the chunk
    pub fn wrong_at(&self, chunk: &Chunk) -> Option<usize> {
        self.typed
            .chars()
            .zip(chunk.text.chars())
            .position(|(typed, expected)| typed != expected)
    }

    // Index in the chunk of the char to type next: the wrong one, or the first not typed yet
    pub fn next_at(&self, chunk: &Chunk) -> usize {
        self.wrong_at(chunk)
            .unwrap_or_else(|| self.typed.chars().count())
    }

    pub fn type_char(&mut self, typing: &Typing, c: char) {
        let Some(chunk) = self.current(typing) else {
            return;
        };

        if self.wrong_at(chunk).is_some() {
            return;
        }

        self.typed.push(c);

        if self.typed == chunk.text {
            self.finish_chunk(chunk);
        }

        self.skip_auto(typing);
        self.indent(typing);
    }

    pub fn backspace(&mut self) {
        self.typed.pop();
    }

    // Goes back to the last chunk the user typed before this one: what is typed of this chunk is
    // dropped, every chunk from there on comes out of the file, and that chunk comes back
    // without its last char
    pub fn go_back(&mut self, typing: &Typing) {
        let Some(target) = self.previous_typed(typing) else {
            return;
        };

        // The latest first, so each chunk comes out of the file just as it went in
        for chunk in typing.chunks[target..self.chunk].iter().rev() {
            let at = chunk.line - 1;
            let lines = chunk.text.split('\n').count();

            self.file.drain(at..at + lines);
        }

        self.chunk = target;
        self.typed = typing.chunks[target].text.clone();
        self.typed.pop();
        self.indent(typing);
    }

    // Whether there is a chunk before this one to go back to
    pub fn can_go_back(&self, typing: &Typing) -> bool {
        self.previous_typed(typing).is_some()
    }

    // The last chunk before this one that the user types, so not an `auto` one
    fn previous_typed(&self, typing: &Typing) -> Option<usize> {
        typing.chunks[..self.chunk]
            .iter()
            .rposition(|chunk| !matches!(chunk.mode, Mode::Auto))
    }

    // Starts typing the rest of the chunk, a few chars on each tick. A wrong tail goes first
    pub fn start_autotype(&mut self, typing: &Typing) {
        let Some(chunk) = self.current(typing) else {
            return;
        };

        // Back to the last right char: next_at is the first wrong char, or the typed length
        self.typed = self.typed.chars().take(self.next_at(chunk)).collect();
        self.indent(typing);

        let remaining = chunk.text.chars().count() - self.typed.chars().count();
        self.autotype = Some(Autotype {
            chunk: self.chunk,
            // One char per tick, or more when the rest would take over 40 ticks
            per_tick: remaining.div_ceil(AUTOTYPE_TICKS).max(1),
        });
    }

    // One tick of the shortcut: types its chars, and stops when the chunk is finished
    pub fn tick_autotype(&mut self, typing: &Typing) {
        let Some(autotype) = self.autotype else {
            return;
        };

        for _ in 0..autotype.per_tick {
            // The last char finishes the chunk and moves `chunk` on, so stop there
            if self.chunk != autotype.chunk {
                break;
            }
            self.type_next(typing);
        }

        // The chunk is done, so the shortcut is done with it
        if self.chunk != autotype.chunk {
            self.autotype = None;
        }
    }

    // Types the char the chunk expects next, as the user would
    fn type_next(&mut self, typing: &Typing) {
        let Some(chunk) = self.current(typing) else {
            return;
        };

        // `typed` already holds the indentation `type_char` fills, so its length is the next char
        if let Some(c) = chunk.text.chars().nth(self.typed.chars().count()) {
            self.type_char(typing, c);
        }
    }

    // Fill the indentation at a line start, as if typed
    fn indent(&mut self, typing: &Typing) {
        let Some(chunk) = self.current(typing) else {
            return;
        };
        let at_line_start = self.typed.is_empty() || self.typed.ends_with('\n');

        if !at_line_start || self.wrong_at(chunk).is_some() {
            return;
        }

        let typed_len = self.typed.chars().count();
        let spaces = chunk
            .text
            .chars()
            .skip(typed_len)
            .take_while(|c| *c == ' ')
            .count();

        self.typed.push_str(&" ".repeat(spaces));
    }

    // Insert the auto chunks that come next, so the user never types them
    fn skip_auto(&mut self, typing: &Typing) {
        while let Some(chunk) = self.current(typing)
            && matches!(chunk.mode, Mode::Auto)
        {
            self.finish_chunk(chunk);
        }
    }

    // Finish every chunk before the target, as if typed
    pub fn skip_to(&mut self, typing: &Typing, target: usize) {
        while self.chunk < target
            && let Some(chunk) = typing.chunks.get(self.chunk)
        {
            self.finish_chunk(chunk);
        }

        self.indent(typing);
    }

    // Put the chunk in the file at its line and move to the next
    fn finish_chunk(&mut self, chunk: &Chunk) {
        let at = chunk.line - 1;
        let lines = chunk.text.split('\n').map(String::from);

        self.file.splice(at..at, lines);
        self.chunk += 1;
        self.typed.clear();
    }

    // ------------------------------------------------------------
    // Editor view
    // ------------------------------------------------------------

    // What the editor shows: rows, cursor row and the full expected text
    pub fn editor(&self, chunk: Option<&Chunk>) -> EditorView {
        let at = chunk.map_or(self.file.len(), |chunk| chunk.line - 1);
        let (before, after) = self.file.split_at(at);
        let chunk_lines: Vec<&str> =
            chunk.map_or(Vec::new(), |chunk| chunk.text.split('\n').collect());

        let mut rows = Self::typed_rows(before, 0);
        let mut cursor_row = None;

        if let Some(chunk) = chunk {
            let chunk_rows = Self::split_rows(self.pieces(chunk), at);

            cursor_row = chunk_rows
                .iter()
                .position(Self::has_cursor)
                .map(|row| rows.len() + row);
            rows.extend(chunk_rows);
        }

        rows.extend(Self::typed_rows(after, at + chunk_lines.len()));

        // The key that was expected where the user typed a wrong one
        let wrong_expected = chunk.and_then(|chunk| {
            let wrong = self.wrong_at(chunk)?;

            chunk.text.chars().nth(wrong)
        });

        EditorView {
            rows,
            cursor_row,
            text: Self::document(before, &chunk_lines, after),
            wrong_expected,
        }
    }

    // The chunk split in typed, wrong, cursor and ghost parts
    fn pieces(&self, chunk: &Chunk) -> Vec<Piece> {
        let typed: Vec<char> = self.typed.chars().collect();
        let wrong = self.wrong_at(chunk);
        let ok = wrong.unwrap_or(typed.len());

        let mut pieces = vec![Self::piece(
            Kind::Typed,
            typed[..ok].iter().collect::<String>(),
        )];

        if let Some(w) = wrong {
            // A wrong Enter shows as ↵ so it is visible
            let shown = if typed[w] == '\n' { '↵' } else { typed[w] };
            pieces.push(Self::piece(Kind::Wrong, shown));
        }

        let rest: String = chunk.text.chars().skip(typed.len()).collect();
        // The cursor is the next char (a newline shows as a space); the rest is the ghost tail
        let (next, tail) = match rest.chars().next() {
            Some(c) if c != '\n' => rest.split_at(c.len_utf8()),
            _ => (" ", rest.as_str()),
        };

        pieces.push(Self::piece(Kind::Cursor, next));

        if matches!(chunk.mode, Mode::Ghost) {
            pieces.push(Self::piece(Kind::Ghost, tail));
        }

        pieces
    }

    // Cut the pieces at every newline into numbered rows
    fn split_rows(pieces: Vec<Piece>, first_line: usize) -> Vec<Row> {
        let mut rows = Vec::new();
        let mut row = Vec::new();

        for piece in pieces {
            for (i, text) in piece.text.split('\n').enumerate() {
                if i > 0 {
                    rows.push(std::mem::take(&mut row));
                }
                if !text.is_empty() {
                    row.push(Piece {
                        kind: piece.kind,
                        text: text.to_string(),
                    });
                }
            }
        }

        rows.push(row);

        rows.into_iter()
            .enumerate()
            .map(|(i, pieces)| Row {
                line: first_line + i,
                pieces,
            })
            .collect()
    }

    fn has_cursor(row: &Row) -> bool {
        row.pieces.iter().any(|piece| piece.kind == Kind::Cursor)
    }

    fn typed_rows(lines: &[String], first_line: usize) -> Vec<Row> {
        lines
            .iter()
            .enumerate()
            .map(|(i, text)| Row {
                line: first_line + i,
                pieces: vec![Self::piece(Kind::Typed, text)],
            })
            .collect()
    }

    // The full expected file as one text, for the highlighter
    fn document(before: &[String], chunk: &[&str], after: &[String]) -> String {
        before
            .iter()
            .map(String::as_str)
            .chain(chunk.iter().copied())
            .chain(after.iter().map(String::as_str))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn piece(kind: Kind, text: impl ToString) -> Piece {
        Piece {
            kind,
            text: text.to_string(),
        }
    }
}
