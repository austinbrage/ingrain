use ratatui::crossterm::event::KeyCode;

mod common;

use common::TestApp;

mod exercise_done {
    use super::*;

    fn finished_first_exercise() -> TestApp {
        let mut app = TestApp::new_user();
        app.press(KeyCode::Enter); // opens the language cards
        app.press(KeyCode::Enter); // opens the Rust explorer
        app.wait(5); // all rows appear
        app.press(KeyCode::Right); // focus the courses column
        app.press(KeyCode::Right); // focus the modules column
        app.press(KeyCode::Down); // Planned
        app.press(KeyCode::Down); // Many
        app.press(KeyCode::Enter); // opens its first exercise
        app.type_chunk("fn main() {\n}");
        app.type_chunk("    let a = 1;");

        app
    }

    #[test]
    fn shows_the_complete_screen() {
        let screen = finished_first_exercise().screen();

        assert!(screen.has_row(&["Rust · Basics · Many", "session 1/2 · exercise 1/2"]));
        assert!(screen.contains("▗▛▜▖▗▛▙ █▖▟▌▜▛▜▖▜▛  ▜▛▀▌▛█▜ ▜▛▀▌"));
        assert!(screen.contains("█   █ ▐▌███▌▐▙▟▘▐▌  ▐▙▌  █  ▐▙▌"));
        assert!(screen.contains("▜▖▗▖▜▖▟▘█▝▐▌▐▌  ▐▌▗▌▐▌▘▖ █  ▐▌▘▖"));
        assert!(screen.contains("▀▀  ▀▘ ▀ ▝▘▀▀  ▀▀▀▘▀▀▀▘▝▀▘ ▀▀▀▘"));
        assert!(screen.contains("Exercise 1 of 2 complete"));
        assert!(screen.contains("Press Enter for the next exercise"));
        assert!(!screen.contains("Session one exercise"));
        assert!(screen.has_row(&["╭", "output"]));
        assert!(screen.contains("● ● ●   main.rs"));
        assert_eq!(screen.column_of("main.rs"), Some(10));
        assert_eq!(screen.column_of("output"), Some(78));
        assert!(screen.has_rows_in_order(&["fn main() {", "let a = 1;", "}"]));
        assert!(screen.has_row(&["1  fn main() {", "ok"]));
        assert!(screen.has_row(&["2      let a = 1;"]));
        assert!(screen.has_row(&["[▓▓▓▓▓▓▓▓▓▓▓▓]", "2/2 chunks", "Esc back", "Ctrl-Q quit"]));
        assert!(!screen.contains("↑↓ scroll"));
    }

    #[test]
    fn enter_opens_the_next_exercise() {
        let mut app = finished_first_exercise();

        app.press(KeyCode::Enter);
        let screen = app.screen();

        assert!(screen.has_row(&["Rust · Basics · Many", "session 1/2 · exercise 2/2"]));
        assert!(screen.contains("Session one part two"));
        assert!(screen.contains("Start with main."));
        assert!(screen.contains("fn main() {"));
        assert!(!screen.contains("let a = 1;"));
        assert!(screen.has_row(&["Rust", "Ln 1, Col 1"]));
        assert!(screen.has_row(&["[░░░░░░░░░░░░]", "0/1 chunks"]));
        assert!(!screen.contains("Press Enter for the next exercise"));
    }

    #[test]
    fn esc_goes_back_to_the_explorer() {
        let mut app = finished_first_exercise();

        app.press(KeyCode::Esc);
        let screen = app.screen();

        assert!(screen.has_row(&["Ingrain", "Rust › Core › Basics"]));
        assert!(screen.has_row(&["↑↓ move", "Enter start", "← back", "Ctrl-Q quit"]));
        assert!(!screen.contains("Press Enter for the next exercise"));
    }
}

mod session_done {
    use super::*;

    #[test]
    fn enter_opens_the_next_session() {
        let mut app = TestApp::with_progress("rust-many-exercise-2");
        app.press(KeyCode::Enter); // Continue opens the last exercise of session 1
        app.type_chunk("fn main() {\n}");

        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Basics · Many", "session 1/2 · exercise 2/2"]));
        assert!(screen.contains("Session 1 of 2 complete"));
        assert!(screen.contains("Press Enter for the next session"));
        assert!(screen.has_row(&["[▓▓▓▓▓▓▓▓▓▓▓▓]", "1/1 chunks", "Esc back", "Ctrl-Q quit"]));

        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Basics · Many", "session 2/2 · exercise 1/1"]));
        assert!(screen.contains("Session two exercise"));
        assert!(screen.contains("use std::io;"));
        assert!(screen.has_row(&["[▓▓▓░░░░░░░░░]", "1/4 chunks"]));
        assert!(!screen.contains("Press Enter for the next session"));
    }
}

mod module_done {
    use super::*;

    #[test]
    fn enter_goes_back_to_the_explorer() {
        let mut app = TestApp::with_progress("rust-many-session-1-done");
        app.press(KeyCode::Enter); // Continue opens the last exercise of the module
        app.wait(5); // the explorer will show all its rows
        app.type_chunk("fn main() {\n}");
        app.type_chunk("    let a = 1;");

        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Basics · Many", "session 2/2 · exercise 1/1"]));
        assert!(screen.contains("Module complete"));
        assert!(screen.contains("Press Enter to finish"));

        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.has_row(&["Ingrain", "Rust › Core › Basics"]));
        assert!(screen.has_row(&["↑↓ move", "Enter start", "← back", "Ctrl-Q quit"]));
        assert!(screen.has_row(&["One", "✓ 1/1"]));
        assert!(screen.has_row(&["Many", "✓ 2/2"]));
        assert!(!screen.contains("Press Enter to finish"));
    }
}

mod scroll {
    use super::*;

    const LONG_COMMENT: &str = "// this comment is long enough to wrap, so its tail wraps";

    fn finished_long_exercise() -> TestApp {
        let mut app = TestApp::with_progress("rust-alone-done");
        app.press(KeyCode::Enter); // Continue opens Long, the next module of Second
        app.type_chunk("fn main() {\n}");
        for lines in [1..=6, 7..=12] {
            let text: Vec<String> = lines.map(|i| format!("let a{i} = {i};")).collect();
            app.type_chunk(&text.join("\n"));
        }
        let mut last: Vec<String> = (13..=17).map(|i| format!("let a{i} = {i};")).collect();
        last.push(format!("let a18 = 18; {LONG_COMMENT}"));
        app.type_chunk(&last.join("\n"));

        app
    }

    #[test]
    fn down_and_up_scroll_the_code_card() {
        let mut app = finished_long_exercise();

        let screen = app.screen();
        assert!(screen.has_row(&["↑↓ scroll", "Esc back", "Ctrl-Q quit"]));
        assert!(screen.contains("fn main() {"));
        assert!(screen.contains("let a8 = 8;"));
        assert!(!screen.contains("let a9 = 9;"));

        for _ in 0..3 {
            app.press(KeyCode::Down);
        }
        let screen = app.screen();
        assert!(!screen.contains("fn main() {"));
        assert!(!screen.contains("let a2 = 2;"));
        assert!(screen.contains("let a3 = 3;"));
        assert!(screen.contains("let a11 = 11;"));
        assert!(!screen.contains("let a12 = 12;"));

        for _ in 0..3 {
            app.press(KeyCode::Up);
        }
        let screen = app.screen();
        assert!(screen.contains("fn main() {"));
        assert!(!screen.contains("let a9 = 9;"));
    }

    #[test]
    fn a_long_line_wraps_onto_the_next_row_without_a_number() {
        let mut app = finished_long_exercise();

        for _ in 0..15 {
            app.press(KeyCode::Down);
        }
        let screen = app.screen();

        assert!(screen.has_rows_in_order(&[
            "19      let a18 = 18; // this comment",
            "tail wraps",
            "}"
        ]));
        assert!(screen.has_row(&["       tail wraps"]));
    }
}

mod no_output {
    use super::*;

    #[test]
    fn only_the_code_card_shows_centered() {
        let mut app = TestApp::with_progress("rust-many-exercise-2");
        app.press(KeyCode::Enter); // Continue opens the last exercise of session 1
        app.type_chunk("fn main() {\n}");

        let screen = app.screen();
        assert!(screen.contains("Session 1 of 2 complete"));
        assert!(screen.contains("● ● ●   main.rs"));
        assert_eq!(screen.column_of("main.rs"), Some(24));
        assert!(screen.has_rows_in_order(&["fn main() {", "}"]));
        assert!(!screen.contains("output"));
        assert!(!screen.contains("$ cargo run"));
    }
}

mod quit {
    use super::*;

    #[test]
    fn ctrl_q_quits() {
        let mut app = TestApp::with_progress("rust-basics-done");
        app.press(KeyCode::Enter); // Continue opens Alone, the first module of Second
        app.type_chunk("fn main() {\n    let a = 1;\n}");
        assert!(app.screen().contains("Press Enter to finish"));
        assert!(!app.has_quit());

        app.press_ctrl('q');

        assert!(app.has_quit());
    }
}
