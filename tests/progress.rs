use ingrain::progress::Progress;
use ratatui::crossterm::event::KeyCode;

mod common;

use common::TestApp;

const MANY: &str = "rust/basics/many";
const QUIZ: &str = "rust/tools/quiz";

fn last_course(saved: &Progress) -> (&str, &str) {
    let last = saved.last.as_ref().unwrap();

    (last.language.as_str(), last.course.as_str())
}

mod saved {
    use super::*;

    #[test]
    fn opening_an_exercise_saves_the_course_and_finishing_it_saves_the_next_exercise() {
        let mut app = TestApp::new_user();
        app.press(KeyCode::Enter); // opens the language cards
        app.press(KeyCode::Enter); // opens the Rust explorer
        app.wait(5); // all rows appear
        app.press(KeyCode::Right); // focus the courses column
        app.press(KeyCode::Right); // focus the modules column
        app.press(KeyCode::Down); // Planned
        app.press(KeyCode::Down); // Many
        assert!(app.saved_progress().last.is_none());

        app.press(KeyCode::Enter); // opens its first exercise
        let saved = app.saved_progress();
        assert_eq!(last_course(&saved), ("rust", "basics"));
        assert!(saved.modules.is_empty());

        app.type_chunk("fn main() {\n}");
        app.type_chunk("    let a = 1;");

        let saved = app.saved_progress();
        assert_eq!(last_course(&saved), ("rust", "basics"));
        let many = saved
            .modules
            .iter()
            .find(|module| module.id == MANY)
            .unwrap();
        assert_eq!(
            many.in_progress.get("session-1").map(String::as_str),
            Some("a2")
        );
        assert!(many.done.is_empty());
        assert!(many.practiced > 0);
    }

    #[test]
    fn finishing_the_last_exercise_of_a_session_saves_it_as_done_once() {
        let mut app = TestApp::with_progress("rust-many-exercise-2");
        app.press(KeyCode::Enter); // Continue opens the last exercise of session 1
        app.type_chunk("fn main() {\n}");

        let saved = app.saved_progress();
        assert_eq!(last_course(&saved), ("rust", "basics"));
        let many = saved
            .modules
            .iter()
            .find(|module| module.id == MANY)
            .unwrap();
        assert_eq!(many.done, ["session-1"]);
        assert!(many.in_progress.is_empty());
        assert!(many.practiced > 2);

        // Play the same session again: it must stay in done once
        app.press(KeyCode::Esc); // from the complete screen back to the explorer
        app.press(KeyCode::Enter); // Many has a done session, so it opens the sessions column
        app.press(KeyCode::Up); // Session 1, the one just finished
        app.press(KeyCode::Enter); // opens its first exercise again
        app.type_chunk("fn main() {\n}");
        app.type_chunk("    let a = 1;");
        app.press(KeyCode::Enter); // opens the next exercise
        app.type_chunk("fn main() {\n}");

        let saved = app.saved_progress();
        assert_eq!(saved.modules.len(), 2);
        let many = saved
            .modules
            .iter()
            .find(|module| module.id == MANY)
            .unwrap();
        assert_eq!(many.done, ["session-1"]);
        assert!(many.in_progress.is_empty());
    }

    #[test]
    fn finishing_a_session_moves_its_module_to_the_end_of_the_list() {
        let mut app = TestApp::with_progress("rust-many-before-one");
        app.press(KeyCode::Enter); // Continue opens the last exercise of session 1
        app.type_chunk("fn main() {\n}");

        let saved = app.saved_progress();
        let ids: Vec<&str> = saved
            .modules
            .iter()
            .map(|module| module.id.as_str())
            .collect();
        assert_eq!(ids, ["rust/basics/one", MANY]);
    }

    #[test]
    fn finishing_a_quiz_saves_the_next_exercise_and_the_last_one_saves_the_session() {
        let mut app = TestApp::new_user();
        app.press(KeyCode::Enter); // opens the language cards
        app.press(KeyCode::Enter); // opens the Rust explorer
        app.wait(5); // all rows appear
        app.press(KeyCode::Down); // Ecosystem
        app.press(KeyCode::Right); // focus the courses column, on Tools
        app.press(KeyCode::Right); // focus the modules column
        app.press(KeyCode::Down); // Quiz
        app.press(KeyCode::Enter); // opens its first exercise
        let saved = app.saved_progress();
        assert_eq!(last_course(&saved), ("rust", "tools"));
        assert!(saved.modules.is_empty());

        app.press(KeyCode::Enter); // checks the first option
        assert!(app.saved_progress().modules.is_empty());

        app.press(KeyCode::Enter); // goes on, which today opens the complete screen
        let saved = app.saved_progress();
        let quiz = saved
            .modules
            .iter()
            .find(|module| module.id == QUIZ)
            .unwrap();
        assert_eq!(
            quiz.in_progress.get("session-1").map(String::as_str),
            Some("sum")
        );
        assert!(quiz.done.is_empty());
        assert!(quiz.practiced > 0);

        app.press(KeyCode::Enter); // the complete screen opens the second quiz
        app.press(KeyCode::Down); // the output 3
        app.press(KeyCode::Enter); // checks it
        app.press(KeyCode::Enter); // goes on

        let saved = app.saved_progress();
        let quiz = saved
            .modules
            .iter()
            .find(|module| module.id == QUIZ)
            .unwrap();
        assert_eq!(quiz.done, ["session-1"]);
        assert!(quiz.in_progress.is_empty());
    }

    #[test]
    fn a_file_with_no_failed_exercises_reads_as_nothing_failed() {
        let app = TestApp::with_progress("rust-quiz-exercise-2");

        let saved = app.saved_progress();
        let quiz = saved
            .modules
            .iter()
            .find(|module| module.id == QUIZ)
            .unwrap();
        assert!(quiz.failed.is_empty());
    }

    #[test]
    fn the_failed_exercises_stay_in_the_file_when_the_app_saves() {
        let mut app = TestApp::with_progress("rust-quiz-failed");
        app.press(KeyCode::Enter); // Continue opens the second quiz, which saves

        let saved = app.saved_progress();
        let quiz = saved
            .modules
            .iter()
            .find(|module| module.id == QUIZ)
            .unwrap();
        assert_eq!(quiz.failed["session-1"], ["total"]);
    }

    #[test]
    fn a_wrong_quiz_answer_is_saved_as_failed() {
        let mut app = TestApp::with_progress("last-rust-tools");
        app.press(KeyCode::Enter); // Continue opens the first quiz
        assert!(app.screen().contains("What does total do?"));

        app.press(KeyCode::Down); // a wrong option
        app.press(KeyCode::Enter); // checks it
        app.press(KeyCode::Enter); // goes on

        let saved = app.saved_progress();
        let quiz = saved
            .modules
            .iter()
            .find(|module| module.id == QUIZ)
            .unwrap();
        assert_eq!(quiz.failed["session-1"], ["total"]);
    }

    #[test]
    fn a_wrong_answer_on_the_last_exercise_still_saves_the_session_as_done() {
        let mut app = TestApp::with_progress("rust-quiz-exercise-2");
        app.press(KeyCode::Enter); // Continue opens the second quiz
        assert!(app.screen().contains("What does it print?"));

        app.press(KeyCode::Enter); // checks the first option, which is wrong
        app.press(KeyCode::Enter); // goes on

        let saved = app.saved_progress();
        let quiz = saved
            .modules
            .iter()
            .find(|module| module.id == QUIZ)
            .unwrap();
        assert_eq!(quiz.done, ["session-1"]);
        assert_eq!(quiz.failed["session-1"], ["sum"]);
    }

    #[test]
    fn a_right_answer_clears_the_failed_exercise() {
        let mut app = TestApp::with_progress("rust-quiz-total-failed");
        app.press(KeyCode::Enter); // Continue opens the first quiz again
        assert!(app.screen().contains("What does total do?"));

        app.press(KeyCode::Enter); // checks the first option, which is right
        app.press(KeyCode::Enter); // goes on

        let saved = app.saved_progress();
        let quiz = saved
            .modules
            .iter()
            .find(|module| module.id == QUIZ)
            .unwrap();
        assert!(quiz.failed.is_empty());
    }

    #[test]
    fn a_broken_file_is_kept_as_a_backup_and_the_app_starts_like_a_new_user() {
        let app = TestApp::with_progress("broken");
        let screen = app.screen();

        assert!(screen.contains("Choose a language"));
        assert!(!screen.contains("Continue"));
        assert!(app.has_backup());
        assert!(app.saved_progress().last.is_none());
    }
}

mod retake {
    use super::*;

    #[test]
    fn continue_retakes_the_failed_exercise_of_a_module_played_through() {
        let mut app = TestApp::with_progress("rust-quiz-done-failed");
        app.press(KeyCode::Enter); // Continue

        let screen = app.screen();
        assert!(screen.contains("What does it print?"));
        assert!(screen.contains("exercise 2/2"));
    }

    #[test]
    fn enter_on_a_module_row_retakes_its_failed_exercise() {
        let mut app = TestApp::with_progress("rust-quiz-done-failed");
        app.press(KeyCode::Down); // selects "Choose a language", Continue is first
        app.press(KeyCode::Enter); // opens the language cards
        app.press(KeyCode::Enter); // opens the Rust explorer
        app.wait(5); // all rows appear
        app.press(KeyCode::Down); // Ecosystem
        app.press(KeyCode::Right); // focus the courses column, on Tools
        app.press(KeyCode::Right); // focus the modules column
        app.press(KeyCode::Down); // Quiz
        app.press(KeyCode::Enter); // opens the failed exercise

        let screen = app.screen();
        assert!(screen.contains("What does it print?"));
        assert!(screen.contains("exercise 2/2"));
    }

    #[test]
    fn enter_on_a_failed_session_row_retakes_only_its_failed_exercise() {
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
        app.press(KeyCode::Up); // Session 1, the failed one
        app.press(KeyCode::Enter); // opens its failed exercise

        let screen = app.screen();
        assert!(screen.contains("Session one part two"));
        assert!(!screen.contains("Session one exercise"));
    }

    #[test]
    fn finishing_a_retaken_exercise_saves_no_exercise_to_resume() {
        let mut app = TestApp::with_progress("rust-quiz-done-total-failed");
        app.press(KeyCode::Enter); // Continue opens the failed exercise
        app.press(KeyCode::Enter); // checks the first option, which is right
        app.press(KeyCode::Enter); // goes on

        let saved = app.saved_progress();
        let quiz = saved
            .modules
            .iter()
            .find(|module| module.id == QUIZ)
            .unwrap();
        assert!(quiz.in_progress.is_empty());
        assert_eq!(quiz.done, ["session-1"]);
    }

    #[test]
    fn a_retake_goes_through_every_failed_exercise_in_content_order() {
        let mut app = TestApp::with_progress("rust-quiz-done-both-failed");
        app.press(KeyCode::Enter); // Continue opens the first failed exercise
        let screen = app.screen();
        assert!(screen.contains("What does total do?"));
        assert!(screen.contains("exercise 1/2"));

        app.press(KeyCode::Enter); // checks the first option, which is right
        app.press(KeyCode::Enter); // goes on, which today opens the complete screen
        app.press(KeyCode::Enter); // the complete screen opens the next failed exercise
        let screen = app.screen();
        assert!(screen.contains("What does it print?"));
        assert!(screen.contains("exercise 2/2"));

        app.press(KeyCode::Down); // the output 3
        app.press(KeyCode::Enter); // checks it
        app.press(KeyCode::Enter); // goes on
        app.press(KeyCode::Enter); // the complete screen ends the retake
        app.wait(5); // all rows appear

        assert!(app.screen().has_row(&["Quiz", "✓ 1/1"]));
    }

    #[test]
    fn a_retake_skips_the_passed_exercise_between_two_failed_ones() {
        let mut app = TestApp::with_progress("rust-retake-first-and-third-failed");
        app.press(KeyCode::Enter); // Continue opens the first failed exercise
        let screen = app.screen();
        assert!(screen.contains("First question"));
        assert!(screen.contains("exercise 1/3"));

        app.press(KeyCode::Enter); // checks the first option, which is right
        app.press(KeyCode::Enter); // goes on, which today opens the complete screen
        app.press(KeyCode::Enter); // the complete screen opens the next failed exercise
        let screen = app.screen();
        assert!(screen.contains("Third question"));
        assert!(!screen.contains("Second question"));
        assert!(screen.contains("exercise 3/3"));

        app.press(KeyCode::Enter); // checks the first option, which is right
        app.press(KeyCode::Enter); // goes on
        app.press(KeyCode::Enter); // the complete screen ends the retake
        app.wait(5); // all rows appear

        assert!(app.screen().has_row(&["Retake", "✓ 1/1"]));
    }

    #[test]
    fn a_retake_ends_in_the_explorer_after_its_last_failed_exercise() {
        let mut app = TestApp::with_progress("rust-quiz-done-total-failed");
        app.press(KeyCode::Enter); // Continue opens the failed exercise
        assert!(app.screen().contains("What does total do?"));

        app.press(KeyCode::Enter); // checks the first option, which is right
        app.press(KeyCode::Enter); // goes on, which today opens the complete screen
        app.press(KeyCode::Enter); // the complete screen ends the retake
        app.wait(5); // all rows appear

        let screen = app.screen();
        assert!(screen.has_row(&["Quiz", "✓ 1/1"]));
        assert!(!screen.contains("What does it print?"));
    }

    #[test]
    fn a_wrong_answer_in_the_first_pass_still_goes_to_the_next_exercise() {
        let mut app = TestApp::with_progress("last-rust-tools");
        app.press(KeyCode::Enter); // Continue opens the first quiz
        app.press(KeyCode::Down); // a wrong option
        app.press(KeyCode::Enter); // checks it
        app.press(KeyCode::Enter); // goes on, which today opens the complete screen
        app.press(KeyCode::Enter); // the complete screen opens the next exercise

        let screen = app.screen();
        assert!(screen.contains("What does it print?"));
        assert!(screen.contains("exercise 2/2"));
    }
}
