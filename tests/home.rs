use catppuccin::PALETTE;
use ratatui::{crossterm::event::KeyCode, style::Color};

mod common;

use common::TestApp;

fn selected() -> Color {
    PALETTE.mocha.colors.sky.into()
}

mod new_user {
    use super::*;

    #[test]
    fn shows_the_title() {
        let screen = TestApp::new_user().screen();

        assert!(screen.contains("▀█▀  █▄ █  ▄▀▀▀  █▀▀▄  ▄▀▀▄  ▀█▀  █▄ █"));
        assert!(screen.contains(" █   █ ▀█  █ ▀█  █▀█▀  █▀▀█   █   █ ▀█"));
        assert!(screen.contains("▄█▄  █  █  ▀▄▄▀  █ ▀▄  █  █  ▄█▄  █  █"));
    }

    #[test]
    fn shows_the_tagline() {
        let screen = TestApp::new_user().screen();

        assert!(screen.contains("No AI, Just you and the keys"));
    }

    #[test]
    fn shows_only_the_choose_language_button() {
        let mut app = TestApp::new_user();
        let screen = app.screen();

        assert!(screen.contains("Choose a language"));
        assert!(!screen.contains("Continue"));
        assert!(screen.has_row(&["Enter select", "Ctrl-Q quit"]));
        assert!(!screen.contains("↑↓"));

        app.press(KeyCode::Up);
        app.press(KeyCode::Down);
        app.press(KeyCode::Up);
        let screen = app.screen();
        assert!(!screen.contains("Continue"));
        assert_eq!(screen.background_of("Choose a language"), Some(selected()));
    }

    #[test]
    fn treats_a_missing_last_course_like_a_new_user() {
        let screen = TestApp::with_progress("last-missing-course").screen();

        assert!(screen.contains("Choose a language"));
        assert!(!screen.contains("Continue"));
        assert!(!screen.contains("Let's review"));
    }
}

mod continue_button {
    use super::*;

    #[test]
    fn offers_continue_or_review_by_progress() {
        let screen = TestApp::with_progress("last-rust-basics").screen();
        assert!(screen.contains("Continue   Rust · Basics"));
        assert!(screen.contains("Choose a language"));

        let mut app = TestApp::with_progress("rust-completed");
        let screen = app.screen();
        assert!(screen.contains("Rust completed · Let's review"));
        assert!(screen.contains("Choose a language"));
        assert!(!screen.contains("Continue"));
        assert_eq!(screen.background_of("Rust completed"), Some(selected()));

        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Basics · One", "session 1/1 · exercise 1/1"]));
    }

    #[test]
    fn moves_the_selection_with_down_and_up() {
        let mut app = TestApp::with_progress("last-rust-basics");

        let screen = app.screen();
        assert_eq!(screen.background_of("Continue"), Some(selected()));
        assert_ne!(screen.background_of("Choose a language"), Some(selected()));
        assert!(screen.has_row(&["↑↓ move", "Enter select", "Ctrl-Q quit"]));

        app.press(KeyCode::Down);
        let screen = app.screen();
        assert_ne!(screen.background_of("Continue"), Some(selected()));
        assert_eq!(screen.background_of("Choose a language"), Some(selected()));

        app.press(KeyCode::Up);
        let screen = app.screen();
        assert_eq!(screen.background_of("Continue"), Some(selected()));
        assert_ne!(screen.background_of("Choose a language"), Some(selected()));
    }
}

mod exercise_on_continue {
    use super::*;

    #[test]
    fn opens_the_exercise() {
        let mut app = TestApp::with_progress("last-rust-basics");
        assert_eq!(app.screen().background_of("Continue"), Some(selected()));

        app.press(KeyCode::Enter);
        let screen = app.screen();

        assert!(!screen.contains("Choose a language"));
        assert!(screen.has_row(&["Rust · Basics · One", "session 1/1 · exercise 1/1"]));
        assert!(screen.contains("First exercise"));
        assert!(screen.contains("Print the value of a."));
        assert!(screen.contains("Start with main."));
        assert!(screen.contains("fn main() {"));
        assert!(screen.has_row(&["0/3 chunks", "Esc back", "Ctrl-Q quit"]));
    }

    #[test]
    fn opens_the_session_after_the_done_one() {
        let mut app = TestApp::with_progress("rust-many-session-1-done");
        assert_eq!(app.screen().background_of("Continue"), Some(selected()));

        app.press(KeyCode::Enter);
        let screen = app.screen();

        assert!(screen.has_row(&["Rust · Basics · Many", "session 2/2 · exercise 1/1"]));
        assert!(screen.contains("Session two exercise"));
        assert!(!screen.contains("Session one exercise"));
    }

    #[test]
    fn opens_the_second_quiz_when_the_first_one_is_saved() {
        let mut app = TestApp::with_progress("rust-quiz-exercise-2");
        assert_eq!(app.screen().background_of("Continue"), Some(selected()));

        app.press(KeyCode::Enter);
        let screen = app.screen();

        assert!(screen.has_row(&["Rust · Tools · Quiz", "session 1/1 · exercise 2/2"]));
        assert!(screen.has_row(&["1/2 exercises", "Enter check"]));
        assert!(screen.contains("What does it print?"));
        assert!(screen.contains("12"));
        assert!(!screen.contains("What does total do?"));
    }

    #[test]
    fn opens_the_next_course_when_the_last_one_is_finished() {
        let mut app = TestApp::with_progress("rust-basics-done");
        let screen = app.screen();
        assert!(screen.contains("Continue   Rust · Second"));
        assert_eq!(screen.background_of("Continue"), Some(selected()));

        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Second · Alone", "session 1/1 · exercise 1/1"]));
        assert!(screen.contains("Alone exercise"));
    }

    #[test]
    fn opens_the_exercise_where_the_user_stopped() {
        let mut app = TestApp::with_progress("rust-many-exercise-2");
        assert_eq!(app.screen().background_of("Continue"), Some(selected()));

        app.press(KeyCode::Enter);
        let screen = app.screen();

        assert!(screen.has_row(&["Rust · Basics · Many", "session 1/2 · exercise 2/2"]));
        assert!(screen.contains("Session one part two"));
        assert!(!screen.contains("Session one exercise"));
    }
}

mod cards {
    use super::*;

    #[test]
    fn open_move_and_go_back() {
        let mut app = TestApp::new_user();

        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(!screen.contains("Choose a language"));
        assert_eq!(screen.background_of("Rust"), Some(selected()));
        assert_ne!(screen.background_of("Go"), Some(selected()));
        assert!(screen.has_row(&["←→ move", "Enter select", "Esc back", "Ctrl-Q quit"]));

        app.press(KeyCode::Right);
        let screen = app.screen();
        assert_ne!(screen.background_of("Rust"), Some(selected()));
        assert_eq!(screen.background_of("Go"), Some(selected()));

        app.press(KeyCode::Left);
        let screen = app.screen();
        assert_eq!(screen.background_of("Rust"), Some(selected()));
        assert_ne!(screen.background_of("Go"), Some(selected()));

        app.press(KeyCode::Esc);
        let screen = app.screen();
        assert!(!screen.contains("Rust"));
        assert!(!screen.contains("Go"));
        assert_eq!(screen.background_of("Choose a language"), Some(selected()));
    }
}

mod explorer_from_cards {
    use super::*;

    #[test]
    fn opens_the_rust_explorer() {
        let mut app = TestApp::new_user();
        assert_eq!(
            app.screen().background_of("Choose a language"),
            Some(selected())
        );

        app.press(KeyCode::Enter);
        assert_eq!(app.screen().background_of("Rust"), Some(selected()));

        app.press(KeyCode::Enter);
        app.wait(5);
        let screen = app.screen();

        assert!(screen.has_row(&["Ingrain", "Rust"]));
        assert!(screen.has_row(&["RUST", "CORE", "BASICS"]));
        assert!(screen.has_row(&["Core", "Basics", "FOUNDATIONS", "basic"]));
        assert!(screen.has_row(&["One"]));
    }

    #[test]
    fn opens_the_go_explorer() {
        let mut app = TestApp::new_user();

        app.press(KeyCode::Enter);
        assert_eq!(app.screen().background_of("Rust"), Some(selected()));

        app.press(KeyCode::Right);
        assert_eq!(app.screen().background_of("Go"), Some(selected()));

        app.press(KeyCode::Enter);
        app.wait(5);
        let screen = app.screen();

        assert!(screen.has_row(&["Ingrain", "Go"]));
        assert!(screen.has_row(&["GO", "CORE", "BASICS"]));
        assert!(screen.has_row(&["Core", "Basics"]));

        app.press(KeyCode::Esc);
        let screen = app.screen();
        assert_eq!(screen.background_of("Go"), Some(selected()));
        assert_ne!(screen.background_of("Rust"), Some(selected()));
        assert!(screen.has_row(&["←→ move", "Esc back"]));
    }
}

mod demo {
    use super::*;

    #[test]
    fn follows_the_language_cards() {
        let mut app = TestApp::new_user();
        assert!(app.screen().contains("main.rs"));

        app.press(KeyCode::Enter);
        app.press(KeyCode::Right);
        let screen = app.screen();
        assert!(screen.contains("main.go"));
        assert!(!screen.contains("main.rs"));

        app.press(KeyCode::Left);
        let screen = app.screen();
        assert!(screen.contains("main.rs"));
        assert!(!screen.contains("main.go"));
    }

    #[test]
    fn types_go_for_a_returning_go_user() {
        let screen = TestApp::with_progress("last-go-basics").screen();

        assert!(screen.contains("main.go"));
        assert!(!screen.contains("main.rs"));
    }
}

mod quit {
    use super::*;

    #[test]
    fn ctrl_q_quits() {
        let mut app = TestApp::new_user();
        assert!(!app.has_quit());

        app.press_ctrl('q');

        assert!(app.has_quit());
    }
}
