use ingrain::{
    app::{App, Content},
    content::Source,
    progress::Progress,
    ui,
};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{Terminal, backend::TestBackend};
use tempfile::TempDir;

use super::screen::Screen;

const WIDTH: u16 = 100;
const HEIGHT: u16 = 30;

// An app on the test content, with its progress in a temp folder that lives as long as it
pub struct TestApp {
    app: App,
    dir: TempDir,
}

impl TestApp {
    // A user with no saved progress
    pub fn new_user() -> Self {
        Self::in_dir(tempfile::tempdir().unwrap())
    }

    // A user whose saved progress is tests/fixtures/progress/<fixture>.json
    pub fn with_progress(fixture: &str) -> Self {
        let dir = tempfile::tempdir().unwrap();
        let path = format!(
            "{}/tests/fixtures/progress/{fixture}.json",
            env!("CARGO_MANIFEST_DIR")
        );
        std::fs::copy(path, dir.path().join("progress.json")).unwrap();

        Self::in_dir(dir)
    }

    // The app on the test content, saving its progress in that folder
    fn in_dir(dir: TempDir) -> Self {
        let progress = Progress::load_from(dir.path().to_path_buf());
        let app = App::with(progress, Content::load(Source::Test));

        Self { app, dir }
    }

    // What the app has saved to progress.json so far
    pub fn saved_progress(&self) -> Progress {
        Progress::load_from(self.dir.path().to_path_buf())
    }

    // Whether a broken progress file was kept as progress.json.bak
    pub fn has_backup(&self) -> bool {
        self.dir.path().join("progress.json.bak").exists()
    }

    // Send Ctrl plus a letter, as the user would type it
    pub fn press_ctrl(&mut self, letter: char) {
        self.app
            .handle_key(KeyEvent::new(KeyCode::Char(letter), KeyModifiers::CONTROL));
    }

    // Whether the app asked to quit, which ends its main loop
    pub fn has_quit(&self) -> bool {
        self.app.quit
    }

    // Send one key, with no modifiers, as the user would type it
    pub fn press(&mut self, code: KeyCode) {
        self.app.handle_key(KeyEvent::new(code, KeyModifiers::NONE));
    }

    // Types the text as the user would: the app fills the indentation by itself
    pub fn type_chunk(&mut self, text: &str) {
        let mut line_start = true;

        for c in text.chars() {
            match c {
                '\n' => {
                    self.press(KeyCode::Enter);
                    line_start = true;
                }
                ' ' if line_start => {}
                _ => {
                    self.press(KeyCode::Char(c));
                    line_start = false;
                }
            }
        }
    }

    // Let the animations run for some ticks, as the app does when no key arrives
    pub fn wait(&mut self, ticks: u64) {
        for _ in 0..ticks {
            self.app.tick();
        }
    }

    // Draw the current screen into memory
    pub fn screen(&self) -> Screen {
        let mut terminal = Terminal::new(TestBackend::new(WIDTH, HEIGHT)).unwrap();
        terminal.draw(|frame| ui::draw(frame, &self.app)).unwrap();

        Screen::new(terminal.backend().buffer().clone())
    }
}
