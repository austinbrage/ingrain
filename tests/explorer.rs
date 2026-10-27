use catppuccin::PALETTE;
use ratatui::{crossterm::event::KeyCode, style::Color};

mod common;

use common::TestApp;

fn highlighted() -> Color {
    PALETTE.mocha.colors.surface1.into()
}

fn rust_explorer() -> TestApp {
    let mut app = TestApp::new_user();
    app.press(KeyCode::Enter); // opens the language cards
    app.press(KeyCode::Enter); // opens the Rust explorer
    app.wait(5); // all rows appear

    app
}

mod sections {
    use super::*;

    #[test]
    fn down_and_up_swap_the_courses_and_modules_columns() {
        let mut app = rust_explorer();

        let screen = app.screen();
        assert_eq!(screen.background_of("Core"), Some(highlighted()));
        assert_ne!(screen.background_of("Ecosystem"), Some(highlighted()));
        assert!(screen.has_row(&["RUST", "CORE", "BASICS"]));
        assert!(screen.has_row(&["Core", "Basics", "FOUNDATIONS", "basic"]));
        assert!(!screen.contains("Tools"));
        assert!(!screen.contains("Helpers"));
        assert!(screen.has_row(&["Ingrain", "Rust"]));
        assert!(!screen.has_row(&["Ingrain", "Rust ›"]));
        assert!(screen.has_row(&["↑↓ move", "→ open", "← back", "Ctrl-Q quit"]));

        app.press(KeyCode::Down);
        let screen = app.screen();
        assert_ne!(screen.background_of("Core"), Some(highlighted()));
        assert_eq!(screen.background_of("Ecosystem"), Some(highlighted()));
        assert!(screen.has_row(&["RUST", "ECOSYSTEM", "TOOLS"]));
        assert!(screen.has_row(&["Core", "Tools", "UTILITIES", "medium"]));
        assert!(screen.has_row(&["Helpers"]));
        assert!(!screen.contains("Basics"));
        assert!(!screen.contains("FOUNDATIONS"));

        app.press(KeyCode::Up);
        let screen = app.screen();
        assert_eq!(screen.background_of("Core"), Some(highlighted()));
        assert!(screen.has_row(&["RUST", "CORE", "BASICS"]));
        assert!(screen.has_row(&["Core", "Basics", "FOUNDATIONS", "basic"]));
        assert!(!screen.contains("Tools"));
        assert!(!screen.contains("Helpers"));

        app.press(KeyCode::Up); // already at the first section
        let screen = app.screen();
        assert_eq!(screen.background_of("Core"), Some(highlighted()));

        app.press(KeyCode::Down);
        app.press(KeyCode::Down); // already at the last section
        let screen = app.screen();
        assert_eq!(screen.background_of("Ecosystem"), Some(highlighted()));
        assert_ne!(screen.background_of("Core"), Some(highlighted()));

        app.press(KeyCode::Esc);
        let screen = app.screen();
        assert!(!screen.contains("Ingrain"));
        assert_eq!(
            screen.background_of("Rust"),
            Some(PALETTE.mocha.colors.sky.into())
        );
        assert!(screen.has_row(&["←→ move", "Esc back"]));
    }
}

mod courses {
    use super::*;

    #[test]
    fn down_and_up_swap_the_modules_column() {
        let mut app = rust_explorer();
        app.press(KeyCode::Right); // focus the courses column

        let screen = app.screen();
        assert_eq!(screen.background_of("Basics"), Some(highlighted()));
        assert_ne!(screen.background_of("Second"), Some(highlighted()));
        assert!(screen.has_row(&["RUST", "CORE", "BASICS"]));
        assert!(screen.has_row(&["FOUNDATIONS", "basic"]));
        assert!(!screen.contains("Alone"));
        assert!(screen.has_row(&["Ingrain", "Rust › Core"]));
        assert!(!screen.has_row(&["Ingrain", "Rust › Core ›"]));
        assert!(screen.has_row(&["↑↓ move", "→ open", "← back", "Ctrl-Q quit"]));

        app.press(KeyCode::Down);
        let screen = app.screen();
        assert_ne!(screen.background_of("Basics"), Some(highlighted()));
        assert_eq!(screen.background_of("Second"), Some(highlighted()));
        assert!(screen.has_row(&["RUST", "CORE", "SECOND"]));
        assert!(screen.has_row(&["EXTRAS", "advanced"]));
        assert!(screen.has_row(&["Alone"]));
        assert!(!screen.contains("FOUNDATIONS"));

        app.press(KeyCode::Up);
        let screen = app.screen();
        assert_eq!(screen.background_of("Basics"), Some(highlighted()));
        assert_ne!(screen.background_of("Second"), Some(highlighted()));
        assert!(screen.has_row(&["RUST", "CORE", "BASICS"]));
        assert!(screen.has_row(&["FOUNDATIONS", "basic"]));
        assert!(!screen.contains("Alone"));

        app.press(KeyCode::Up); // already at the first course
        let screen = app.screen();
        assert_eq!(screen.background_of("Basics"), Some(highlighted()));

        app.press(KeyCode::Down);
        app.press(KeyCode::Down); // already at the last course
        let screen = app.screen();
        assert_eq!(screen.background_of("Second"), Some(highlighted()));
        assert_ne!(screen.background_of("Basics"), Some(highlighted()));

        app.press(KeyCode::Left);
        let screen = app.screen();
        assert!(screen.has_row(&["Ingrain", "Rust"]));
        assert!(!screen.has_row(&["Ingrain", "Rust ›"]));
        assert_eq!(screen.background_of("Core"), Some(highlighted()));
        assert_ne!(screen.background_of("Basics"), Some(highlighted()));
    }

    #[test]
    fn a_course_without_modules_keeps_the_modules_column_empty_and_does_not_open() {
        let mut app = rust_explorer();
        app.press(KeyCode::Down); // Ecosystem
        app.press(KeyCode::Right); // focus the courses column
        app.press(KeyCode::Down); // Bare, a course with no course.json

        let screen = app.screen();
        assert!(screen.has_row(&["RUST", "ECOSYSTEM", "BARE"]));
        assert!(screen.has_row(&["Ecosystem", "Bare"]));
        assert!(!screen.contains("UTILITIES"));
        assert!(!screen.contains("Helpers"));

        app.press(KeyCode::Right); // there is nothing to open
        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.has_row(&["Ingrain", "Rust › Ecosystem"]));
        assert!(!screen.has_row(&["Ingrain", "Rust › Ecosystem ›"]));
        assert!(screen.has_row(&["↑↓ move", "→ open", "← back", "Ctrl-Q quit"]));
        assert_eq!(screen.background_of("Bare"), Some(highlighted()));
    }
}

mod modules {
    use super::*;

    fn rust_modules() -> TestApp {
        let mut app = rust_explorer();
        app.press(KeyCode::Right); // focus the courses column
        app.press(KeyCode::Right); // focus the modules column

        app
    }

    #[test]
    fn down_and_up_move_the_selection_and_stay_inside_the_list() {
        let mut app = rust_modules();

        let screen = app.screen();
        assert_eq!(screen.background_of("One"), Some(highlighted()));
        assert_ne!(screen.background_of("Planned"), Some(highlighted()));
        assert!(screen.has_row(&["↑↓ move", "Enter start", "← back", "Ctrl-Q quit"]));
        assert!(screen.has_row(&["Ingrain", "Rust › Core › Basics"]));

        app.press(KeyCode::Down);
        let screen = app.screen();
        assert_ne!(screen.background_of("One"), Some(highlighted()));
        assert_eq!(screen.background_of("Planned"), Some(highlighted()));

        app.press(KeyCode::Down);
        let screen = app.screen();
        assert_eq!(screen.background_of("Many"), Some(highlighted()));

        app.press(KeyCode::Down);
        let screen = app.screen();
        assert_eq!(screen.background_of("Many"), Some(highlighted()));

        app.press(KeyCode::Up);
        app.press(KeyCode::Up);
        let screen = app.screen();
        assert_eq!(screen.background_of("One"), Some(highlighted()));
        assert_ne!(screen.background_of("Many"), Some(highlighted()));

        app.press(KeyCode::Left);
        let screen = app.screen();
        assert!(screen.has_row(&["Ingrain", "Rust › Core"]));
        assert!(!screen.has_row(&["Ingrain", "Rust › Core ›"]));
        assert_eq!(screen.background_of("Basics"), Some(highlighted()));
        assert_ne!(screen.background_of("One"), Some(highlighted()));
        assert!(screen.has_row(&["↑↓ move", "→ open", "← back", "Ctrl-Q quit"]));
    }

    #[test]
    fn enter_on_a_new_one_session_module_opens_its_exercise() {
        let mut app = rust_modules();

        app.press(KeyCode::Enter);
        let screen = app.screen();

        assert!(!screen.contains("Ingrain"));
        assert!(screen.has_row(&["Rust · Basics · One", "session 1/1 · exercise 1/1"]));
        assert!(screen.contains("First exercise"));
    }

    #[test]
    fn enter_on_a_new_two_session_module_opens_its_first_session() {
        let mut app = rust_modules();
        app.press(KeyCode::Down);
        app.press(KeyCode::Down);

        app.press(KeyCode::Enter);
        let screen = app.screen();

        assert!(!screen.contains("Ingrain"));
        assert!(screen.has_row(&["Rust · Basics · Many", "session 1/2 · exercise 1/2"]));
        assert!(screen.contains("Session one exercise"));
    }

    #[test]
    fn enter_on_an_unwritten_module_does_nothing() {
        let mut app = rust_modules();
        app.press(KeyCode::Down);
        assert_eq!(app.screen().background_of("Planned"), Some(highlighted()));

        app.press(KeyCode::Enter);
        let screen = app.screen();

        assert!(screen.contains("Ingrain"));
        assert!(screen.has_row(&["↑↓ move", "Enter start", "← back", "Ctrl-Q quit"]));
        assert_eq!(screen.background_of("Planned"), Some(highlighted()));
    }
}

mod sessions {
    use super::*;

    fn rust_many_selected() -> TestApp {
        let mut app = TestApp::with_progress("rust-many-session-1-done");
        app.press(KeyCode::Down); // selects "Choose a language", Continue is first
        app.press(KeyCode::Enter); // opens the language cards
        app.press(KeyCode::Enter); // opens the Rust explorer
        app.wait(5); // all rows appear
        app.press(KeyCode::Right); // focus the courses column
        app.press(KeyCode::Right); // focus the modules column
        app.press(KeyCode::Down); // Planned
        app.press(KeyCode::Down); // Many

        app
    }

    #[test]
    fn enter_on_a_module_with_a_done_session_opens_the_sessions_column() {
        let mut app = rust_many_selected();
        assert_eq!(app.screen().background_of("Many"), Some(highlighted()));

        app.press(KeyCode::Enter);
        let screen = app.screen();

        assert!(screen.has_row(&["Ingrain", "Rust › Core › Basics › Many"]));
        assert!(screen.has_row(&["RUST", "CORE", "MANY"]));
        assert!(screen.has_row(&["Session 1", "✓"]));
        assert!(screen.has_row(&["Session 2"]));
        assert_ne!(screen.background_of("Session 1"), Some(highlighted()));
        assert_eq!(screen.background_of("Session 2"), Some(highlighted()));
        assert!(!screen.contains("FOUNDATIONS"));
        assert!(!screen.contains("Planned"));
        assert!(screen.has_row(&["↑↓ move", "Enter start", "← back", "Ctrl-Q quit"]));
    }

    #[test]
    fn down_and_up_move_between_sessions() {
        let mut app = rust_many_selected();
        app.press(KeyCode::Enter); // opens its sessions column

        app.press(KeyCode::Up);
        let screen = app.screen();
        assert_eq!(screen.background_of("Session 1"), Some(highlighted()));
        assert_ne!(screen.background_of("Session 2"), Some(highlighted()));

        app.press(KeyCode::Down);
        app.press(KeyCode::Down);
        let screen = app.screen();
        assert_ne!(screen.background_of("Session 1"), Some(highlighted()));
        assert_eq!(screen.background_of("Session 2"), Some(highlighted()));
    }

    #[test]
    fn enter_opens_the_selected_session() {
        let mut app = rust_many_selected();
        app.press(KeyCode::Enter); // opens its sessions column
        app.press(KeyCode::Enter); // opens Session 2, the next one
        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Basics · Many", "session 2/2 · exercise 1/1"]));
        assert!(screen.contains("Session two exercise"));

        let mut app = rust_many_selected();
        app.press(KeyCode::Enter); // opens its sessions column
        app.press(KeyCode::Up); // Session 1
        app.press(KeyCode::Enter); // opens Session 1
        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Basics · Many", "session 1/2 · exercise 1/2"]));
        assert!(screen.contains("Session one exercise"));
    }

    #[test]
    fn esc_comes_back_to_the_modules() {
        let mut app = rust_many_selected();
        app.press(KeyCode::Enter); // opens its sessions column

        app.press(KeyCode::Esc);
        let screen = app.screen();

        assert!(!screen.contains("Session 1"));
        assert!(screen.has_row(&["FOUNDATIONS", "basic"]));
        assert_eq!(screen.background_of("Many"), Some(highlighted()));
    }

    #[test]
    fn marks_show_a_check_on_done_sessions_and_the_next_one_alternates_with_a_circle() {
        let mut app = rust_many_selected();
        app.press(KeyCode::Enter); // opens its sessions column

        let mut saw_next = false;
        let mut saw_circle = false;
        for _ in 0..120 {
            // longer than a full cycle of the typing animation
            let screen = app.screen();
            saw_next |= screen.has_row(&["Session 2", "next"]);
            saw_circle |= screen.has_row(&["Session 2", "○"]);
            assert!(screen.has_row(&["Session 1", "✓"]));
            assert!(!screen.has_row(&["Session 1", "next"]));
            assert!(!screen.has_row(&["Session 1", "○"]));
            app.wait(1);
        }
        assert!(saw_next);
        assert!(saw_circle);
    }

    #[test]
    fn a_failed_session_shows_a_red_cross_instead_of_a_check() {
        let mut app = TestApp::with_progress("rust-many-session-1-failed");
        app.press(KeyCode::Down); // selects "Choose a language", Continue is first
        app.press(KeyCode::Enter); // opens the language cards
        app.press(KeyCode::Enter); // opens the Rust explorer
        app.wait(5); // all rows appear
        app.press(KeyCode::Right); // focus the courses column
        app.press(KeyCode::Right); // focus the modules column
        app.press(KeyCode::Down); // Planned
        app.press(KeyCode::Down); // Many
        app.press(KeyCode::Enter); // opens its sessions column

        let screen = app.screen();
        assert!(screen.has_row(&["Session 1", "✗"]));
        assert!(!screen.has_row(&["Session 1", "✓"]));
        assert_eq!(
            screen.foreground_of("✗"),
            Some(PALETTE.mocha.colors.red.into())
        );
    }

    #[test]
    fn the_hint_says_retake_only_on_a_failed_session() {
        let mut app = TestApp::with_progress("rust-many-session-1-failed");
        app.press(KeyCode::Down); // selects "Choose a language", Continue is first
        app.press(KeyCode::Enter); // opens the language cards
        app.press(KeyCode::Enter); // opens the Rust explorer
        app.wait(5); // all rows appear
        app.press(KeyCode::Right); // focus the courses column
        app.press(KeyCode::Right); // focus the modules column
        app.press(KeyCode::Down); // Planned
        app.press(KeyCode::Down); // Many
        app.press(KeyCode::Enter); // opens its sessions column, on Session 2
        let screen = app.screen();
        assert!(screen.has_row(&["Enter start"]));
        assert!(!screen.contains("Enter retake"));

        app.press(KeyCode::Up); // Session 1, the failed one
        let screen = app.screen();
        assert!(screen.has_row(&["↑↓ move", "Enter retake", "← back", "Ctrl-Q quit"]));
        assert!(!screen.contains("Enter start"));

        app.press(KeyCode::Down); // Session 2
        assert!(app.screen().has_row(&["Enter start"]));
    }
}

mod marks {
    use super::*;

    #[test]
    fn a_new_user_sees_zero_counts() {
        let screen = rust_explorer().screen();

        assert!(screen.has_row(&["Planned", "0/1"]));
        assert!(screen.has_row(&["Many", "0/2"]));
    }

    #[test]
    fn done_modules_show_a_check_and_the_next_marker_alternates_with_the_count() {
        let mut app = TestApp::with_progress("rust-many-session-1-done");
        app.press(KeyCode::Down); // selects "Choose a language", Continue is first
        app.press(KeyCode::Enter); // opens the language cards
        app.press(KeyCode::Enter); // opens the Rust explorer
        app.wait(5); // all rows appear

        let screen = app.screen();
        assert!(screen.has_row(&["One", "✓ 1/1"]));
        assert!(screen.has_row(&["Planned", "0/1"]));

        let mut saw_next = false;
        let mut saw_count = false;
        for _ in 0..120 {
            // longer than a full cycle of the typing animation
            app.wait(1);
            let screen = app.screen();
            saw_next |= screen.has_row(&["Many", "next"]);
            saw_count |= screen.has_row(&["Many", "1/2"]);
            assert!(!screen.has_row(&["One", "next"]));
            assert!(!screen.has_row(&["Planned", "next"]));
        }
        assert!(saw_next);
        assert!(saw_count);
    }

    #[test]
    fn an_exercise_left_with_esc_is_not_failed() {
        let mut app = TestApp::with_progress("last-rust-tools");
        app.press(KeyCode::Enter); // Continue opens the first quiz
        app.press(KeyCode::Esc); // leaves it unanswered
        app.wait(5); // all rows appear

        let screen = app.screen();
        assert!(screen.has_row(&["Quiz"]));
        assert!(!screen.contains("✗"));
        assert!(app.saved_progress().modules.is_empty());
    }

    #[test]
    fn a_wrong_quiz_answer_leaves_a_cross_on_its_module() {
        let mut app = TestApp::with_progress("last-rust-tools");
        app.press(KeyCode::Enter); // Continue opens the first quiz
        app.press(KeyCode::Down); // a wrong option
        app.press(KeyCode::Enter); // checks it
        app.press(KeyCode::Enter); // goes on, to the next quiz
        app.press(KeyCode::Esc); // back to the explorer

        let mut saw_cross = false;
        for _ in 0..120 {
            // longer than a full cycle of the typing animation
            app.wait(1);
            saw_cross |= app.screen().has_row(&["Quiz", "✗ 0/1"]);
        }
        assert!(saw_cross);
    }

    #[test]
    fn a_right_quiz_answer_leaves_no_cross_on_its_module() {
        let mut app = TestApp::with_progress("last-rust-tools");
        app.press(KeyCode::Enter); // Continue opens the first quiz
        app.press(KeyCode::Enter); // checks the first option, which is right
        app.press(KeyCode::Enter); // goes on, to the next quiz
        app.press(KeyCode::Esc); // back to the explorer

        let mut saw_count = false;
        for _ in 0..120 {
            // longer than a full cycle of the typing animation
            app.wait(1);
            let screen = app.screen();
            saw_count |= screen.has_row(&["Quiz", "0/1"]);
            assert!(!screen.contains("✗"));
        }
        assert!(saw_count);
    }

    #[test]
    fn a_module_with_a_failed_session_shows_a_red_cross_before_its_count() {
        let mut app = TestApp::with_progress("rust-quiz-done-failed");
        app.press(KeyCode::Down); // selects "Choose a language", Continue is first
        app.press(KeyCode::Enter); // opens the language cards
        app.press(KeyCode::Enter); // opens the Rust explorer
        app.wait(5); // all rows appear
        app.press(KeyCode::Down); // Ecosystem
        app.press(KeyCode::Right); // focus the courses column, on Tools

        let mut saw_cross = false;
        for _ in 0..120 {
            // longer than a full cycle of the typing animation
            app.wait(1);
            let screen = app.screen();
            if screen.has_row(&["Quiz", "✗ 0/1"]) {
                saw_cross = true;
                assert_eq!(
                    screen.foreground_of("✗"),
                    Some(PALETTE.mocha.colors.red.into())
                );
            }
        }
        assert!(saw_cross);
    }

    #[test]
    fn a_failed_session_is_not_counted_as_done() {
        let mut app = TestApp::with_progress("rust-quiz-done-failed");
        app.press(KeyCode::Down); // selects "Choose a language", Continue is first
        app.press(KeyCode::Enter); // opens the language cards
        app.press(KeyCode::Enter); // opens the Rust explorer
        app.wait(5); // all rows appear
        app.press(KeyCode::Down); // Ecosystem
        app.press(KeyCode::Right); // focus the courses column, on Tools

        let mut saw_count = false;
        for _ in 0..120 {
            // longer than a full cycle of the typing animation
            app.wait(1);
            let screen = app.screen();
            saw_count |= screen.has_row(&["Quiz", "0/1"]);
            assert!(!screen.has_row(&["Quiz", "✓"]));
        }
        assert!(saw_count);
    }
}

mod quit {
    use super::*;

    #[test]
    fn ctrl_q_quits() {
        let mut app = rust_explorer();
        assert!(!app.has_quit());

        app.press_ctrl('q');

        assert!(app.has_quit());
    }
}
