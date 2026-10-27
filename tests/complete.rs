use catppuccin::PALETTE;
use ratatui::crossterm::event::KeyCode;

mod common;

use common::TestApp;

mod exercise_done {
    use super::*;

    // The first row of the word DONE, as `tui_big_text` draws it in `Sextant`
    const DONE: &str = "🬨🬕🬪🬏🬵🬆🬪🬏█🬱▐▌🬨🬕🬒🬄";

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
        assert!(screen.contains(DONE));
        assert!(screen.contains("✓ Session one exercise"));
        assert!(screen.contains("Next: Exercise 2 of 2"));
        assert!(screen.has_row(&["╭", "output"]));
        assert!(screen.contains("● ● ●   main.rs"));
        assert_eq!(screen.column_of("main.rs"), Some(10));
        assert_eq!(screen.column_of("output"), Some(78));
        assert!(screen.has_rows_in_order(&["fn main() {", "let a = 1;", "}"]));
        assert!(screen.has_row(&["1  fn main() {", "ok"]));
        assert!(screen.has_row(&["2      let a = 1;"]));
        assert!(screen.has_row(&["Enter next exercise", "Esc back", "Ctrl-Q quit"]));
        assert!(!screen.contains("2/2 chunks"));
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
        assert!(!screen.contains(DONE));
    }

    #[test]
    fn esc_goes_back_to_the_explorer() {
        let mut app = finished_first_exercise();

        app.press(KeyCode::Esc);
        let screen = app.screen();

        assert!(screen.has_row(&["Ingrain", "Rust › Core › Basics"]));
        assert!(screen.has_row(&["↑↓ move", "Enter start", "← back", "Ctrl-Q quit"]));
        assert!(!screen.contains(DONE));
    }
}

mod session_done {
    use super::*;

    #[test]
    fn the_last_exercise_of_a_session_points_to_the_next_session() {
        let mut app = TestApp::with_progress("rust-many-exercise-2");
        app.press(KeyCode::Enter); // Continue opens the last exercise of session 1
        app.type_chunk("fn main() {\n}");

        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Basics · Many", "session 1/2 · exercise 2/2"]));
        assert!(screen.contains("Next: Session recap"));
        assert!(screen.has_row(&["Enter session recap", "Esc back", "Ctrl-Q quit"]));
    }

    #[test]
    fn shows_the_exercises_of_the_session_and_not_the_next_one() {
        let mut app = TestApp::with_progress("rust-many-exercise-2");
        app.press(KeyCode::Enter); // Continue opens the last exercise of session 1
        app.type_chunk("fn main() {\n}");
        app.press(KeyCode::Enter); // the complete screen goes on, to the session screen

        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Basics · Many", "session 1/2 · exercise 2/2"]));
        assert!(screen.contains("🬻🬒🬌 🬨🬕🬒🬄🬻🬒🬌 🬻🬒🬌 🬁█🬀 🬵🬆🬪🬏█🬱▐▌    🬨🬕🬪🬏🬵🬆🬪🬏█🬱▐▌🬨🬕🬒🬄"));
        assert!(screen.has_row(&["╭", "Session 1"]));
        assert!(screen.has_row(&["1  ✓  Session one exercise", "typing   passed"]));
        assert!(screen.has_row(&["2  ✓  Session one part two", "typing   passed"]));
        assert!(screen.contains("2 of 2 exercises passed"));
        assert!(screen.has_row(&["Enter next session", "Esc back", "Ctrl-Q quit"]));
        assert!(!screen.contains("Session two exercise"));
    }

    #[test]
    fn a_wrong_quiz_answer_shows_a_red_cross_on_its_row() {
        let mut app = TestApp::with_progress("rust-tools-before-recap");
        app.press(KeyCode::Enter); // Continue opens the first quiz of Recap
        app.press(KeyCode::Down); // a wrong option
        app.press(KeyCode::Enter); // checks it
        app.press(KeyCode::Enter); // goes on, to the second quiz
        app.press(KeyCode::Enter); // checks the first option, which is right
        app.press(KeyCode::Enter); // goes on, to the session screen

        let screen = app.screen();
        assert!(screen.has_row(&["1  ✗  Recap one", "quiz   failed"]));
        assert!(screen.has_row(&["2  ✓  Recap two", "quiz   passed"]));
        assert!(screen.contains("1 of 2 exercises passed"));
        assert!(screen.has_row(&["[▓▓▓▓▓▓░░░░░░] 1/2"]));
        assert_eq!(
            screen.foreground_of("✗"),
            Some(PALETTE.mocha.colors.red.into())
        );
    }

    #[test]
    fn enter_on_the_recap_opens_the_next_session() {
        let mut app = TestApp::with_progress("rust-many-exercise-2");
        app.press(KeyCode::Enter); // Continue opens the last exercise of session 1
        app.type_chunk("fn main() {\n}");
        app.press(KeyCode::Enter); // the complete screen goes on, to the session screen

        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Basics · Many", "session 2/2 · exercise 1/1"]));
        assert!(screen.contains("Session two exercise"));
        assert!(screen.contains("use std::io;"));
        assert!(screen.has_row(&["[▓▓▓░░░░░░░░░]", "1/4 chunks"]));
        assert!(!screen.contains("2 of 2 exercises passed"));
    }

    #[test]
    fn esc_on_the_recap_goes_back_to_the_explorer() {
        let mut app = TestApp::with_progress("rust-many-exercise-2");
        app.press(KeyCode::Enter); // Continue opens the last exercise of session 1
        app.type_chunk("fn main() {\n}");
        app.press(KeyCode::Enter); // the complete screen goes on, to the session screen

        app.press(KeyCode::Esc);
        let screen = app.screen();
        assert!(screen.has_row(&["Ingrain", "Rust › Core › Basics"]));
        assert!(screen.has_row(&["↑↓ move", "Enter start", "← back", "Ctrl-Q quit"]));
        assert!(!screen.contains("2 of 2 exercises passed"));
    }
}

mod module_done {
    use super::*;

    // The first row of the word LEVEL UP, as `tui_big_text` draws it in `Sextant`
    pub const LEVEL_UP: &str = "🬨🬕  🬨🬕🬒🬄█ █ 🬨🬕🬒🬄🬨🬕      █ █ 🬨🬕🬨🬓";

    #[test]
    fn the_last_exercise_of_a_module_points_to_the_module_recap() {
        let mut app = TestApp::with_progress("rust-many-session-1-done");
        app.press(KeyCode::Enter); // Continue opens the last exercise of the module
        app.type_chunk("fn main() {\n}");
        app.type_chunk("    let a = 1;");

        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Basics · Many", "session 2/2 · exercise 1/1"]));
        assert!(screen.contains("Next: Module recap"));
        assert!(screen.has_row(&["Enter finish module", "Esc back", "Ctrl-Q quit"]));
    }

    #[test]
    fn shows_the_celebration_with_the_module_and_its_numbers() {
        let mut app = TestApp::with_progress("rust-many-session-1-done");
        app.press(KeyCode::Enter); // Continue opens the last exercise of the module
        app.type_chunk("fn main() {\n}");
        app.type_chunk("    let a = 1;");
        app.press(KeyCode::Enter); // the complete screen goes on, to the module screen

        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Basics · Many", "session 2/2 · exercise 1/1"]));
        assert!(screen.contains(LEVEL_UP));
        assert!(screen.has_rows_in_order(&["module complete", "Many", "2 sessions · 3 exercises"]));
        assert!(screen.has_row(&["Enter explorer", "Esc back", "Ctrl-Q quit"]));
        assert!(!screen.contains("Press Enter to finish"));
    }

    #[test]
    fn the_end_of_a_course_names_it_and_goes_to_the_explorer() {
        let mut app = TestApp::with_progress("rust-many-session-1-done");
        app.press(KeyCode::Enter); // Continue opens the last exercise of Many, the end of Basics
        app.type_chunk("fn main() {\n}");
        app.type_chunk("    let a = 1;");
        app.press(KeyCode::Enter); // the complete screen goes on, to the module screen

        let screen = app.screen();
        assert!(screen.contains(LEVEL_UP));
        assert!(screen.contains("You completed the Basics course"));
        assert!(screen.has_row(&["Enter explorer", "Esc back", "Ctrl-Q quit"]));

        app.press(KeyCode::Enter);
        app.wait(5); // all rows appear
        let screen = app.screen();
        assert!(screen.has_row(&["Ingrain", "Rust › Core › Basics"]));
        assert!(screen.has_row(&["Many", "✓ 2/2"]));
        assert!(!screen.contains(LEVEL_UP));
    }

    #[test]
    fn enter_on_the_celebration_opens_the_next_module() {
        let mut app = TestApp::with_progress("rust-basics-done");
        app.press(KeyCode::Enter); // Continue opens Alone, the first module of Second
        app.type_chunk("fn main() {\n    let a = 1;\n}");
        app.press(KeyCode::Enter); // the complete screen goes on, to the module screen
        let screen = app.screen();
        assert!(screen.contains(LEVEL_UP));
        assert!(!screen.contains("You completed"));
        assert!(screen.has_row(&["Enter next module", "Esc back", "Ctrl-Q quit"]));

        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Second · Long", "session 1/1 · exercise 1/1"]));
        assert!(screen.contains("Long exercise"));
        assert!(!screen.contains(LEVEL_UP));
    }

    #[test]
    fn esc_on_the_celebration_goes_back_to_the_explorer() {
        let mut app = TestApp::with_progress("rust-many-session-1-done");
        app.press(KeyCode::Enter); // Continue opens the last exercise of the module
        app.type_chunk("fn main() {\n}");
        app.type_chunk("    let a = 1;");
        app.press(KeyCode::Enter); // the complete screen goes on, to the module screen
        assert!(app.screen().contains(LEVEL_UP));

        app.press(KeyCode::Esc);
        app.wait(5); // all rows appear
        let screen = app.screen();
        assert!(screen.has_row(&["Ingrain", "Rust › Core › Basics"]));
        assert!(screen.has_row(&["↑↓ move", "Enter start", "← back", "Ctrl-Q quit"]));
        assert!(screen.has_row(&["One", "✓ 1/1"]));
        assert!(screen.has_row(&["Many", "✓ 2/2"]));
        assert!(!screen.contains(LEVEL_UP));
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
        assert!(screen.contains("let a10 = 10;"));
        assert!(!screen.contains("let a11 = 11;"));

        for _ in 0..3 {
            app.press(KeyCode::Down);
        }
        let screen = app.screen();
        assert!(!screen.contains("fn main() {"));
        assert!(!screen.contains("let a2 = 2;"));
        assert!(screen.contains("let a3 = 3;"));
        assert!(screen.contains("let a13 = 13;"));
        assert!(!screen.contains("let a14 = 14;"));

        for _ in 0..3 {
            app.press(KeyCode::Up);
        }
        let screen = app.screen();
        assert!(screen.contains("fn main() {"));
        assert!(!screen.contains("let a11 = 11;"));
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
        assert!(screen.contains("Next: Session recap"));
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
        assert!(app.screen().contains("Enter finish module"));
        assert!(!app.has_quit());

        app.press_ctrl('q');

        assert!(app.has_quit());
    }
}

mod module_failed {
    use super::*;

    use super::module_done::LEVEL_UP;

    // The first row of the word ALMOST THERE, as `tui_big_text` draws it in `Sextant`
    const ALMOST_THERE: &str = "🬵🬎🬱 🬨🬕  █🬱🬻▌🬵🬆🬪🬏🬻🬒🬌 🬆█🬊     🬆█🬊 █ █ 🬨🬕🬒🬄🬨🬕🬨🬓🬨🬕🬒🬄";

    // Recap with its first session failed: finishing the second session shows the failed module
    fn finished_the_module_with_a_failed_session() -> TestApp {
        let mut app = TestApp::with_progress("rust-recap-session-1-failed");
        app.press(KeyCode::Enter); // Continue opens the quiz of the second session
        app.press(KeyCode::Enter); // checks the first option, which is right
        app.press(KeyCode::Enter); // goes on, to the failed module screen

        app
    }

    #[test]
    fn shows_the_sessions_of_a_module_with_a_failed_one() {
        let app = finished_the_module_with_a_failed_session();

        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Tools · Recap", "session 2/2 · exercise 1/1"]));
        assert!(screen.contains(ALMOST_THERE));
        assert!(screen.has_row(&["╭", "Recap"]));
        assert!(screen.has_row(&["1  ✗  Session 1", "1 of 2   failed"]));
        assert!(screen.has_row(&["2  ✓  Session 2", "1 of 1   passed"]));
        assert!(screen.contains("1 of 2 sessions passed"));
        assert!(screen.has_row(&["[▓▓▓▓▓▓░░░░░░] 1/2"]));
        assert!(screen.has_row(&["Enter retake failed", "Esc back", "Ctrl-Q quit"]));
        assert!(!screen.contains("You completed"));
    }

    #[test]
    fn enter_retakes_the_failed_exercise() {
        let mut app = finished_the_module_with_a_failed_session();
        assert!(app.screen().contains(ALMOST_THERE));

        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Tools · Recap", "session 1/2 · exercise 1/2"]));
        assert!(screen.contains("Recap one"));
        assert!(!screen.contains(ALMOST_THERE));
    }

    #[test]
    fn esc_goes_back_to_the_explorer() {
        let mut app = finished_the_module_with_a_failed_session();
        assert!(app.screen().contains(ALMOST_THERE));

        app.press(KeyCode::Esc);
        app.wait(5); // all rows appear
        let screen = app.screen();
        assert!(screen.has_row(&["Ingrain", "Rust › Ecosystem › Tools"]));
        assert!(screen.has_row(&["↑↓ move", "Enter start", "← back", "Ctrl-Q quit"]));
        assert!(!screen.contains(ALMOST_THERE));
    }

    #[test]
    fn passing_the_retake_shows_the_celebration() {
        let mut app = finished_the_module_with_a_failed_session();
        app.press(KeyCode::Enter); // retakes the failed quiz
        app.press(KeyCode::Enter); // checks the first option, which is right
        app.press(KeyCode::Enter); // goes on, which ends the retake: the module passes now

        let screen = app.screen();
        assert!(screen.contains(LEVEL_UP));
        assert!(screen.contains("You completed the Tools course"));
        assert!(!screen.contains(ALMOST_THERE));
    }

    #[test]
    fn failing_the_retake_again_shows_the_failed_module_again() {
        let mut app = finished_the_module_with_a_failed_session();
        app.press(KeyCode::Enter); // retakes the failed quiz
        app.press(KeyCode::Down); // a wrong option
        app.press(KeyCode::Enter); // checks it
        app.press(KeyCode::Enter); // goes on, which ends the retake: the module still has a failed session

        let screen = app.screen();
        assert!(screen.contains(ALMOST_THERE));
        assert!(screen.contains("1 of 2 sessions passed"));
        assert!(!screen.contains(LEVEL_UP));
    }
}
