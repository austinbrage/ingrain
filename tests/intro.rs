use ratatui::crossterm::event::KeyCode;

mod common;

use common::TestApp;

mod slides {
    use super::*;

    fn fresh_module() -> TestApp {
        let mut app = TestApp::with_progress("rust-tools-before-intro");
        app.press(KeyCode::Enter); // Continue opens the module

        app
    }

    fn session_intro() -> TestApp {
        let mut app = fresh_module();
        app.press(KeyCode::Enter);
        app.press(KeyCode::Enter); // past the two slides of the module intro

        app
    }

    fn first_exercise() -> TestApp {
        let mut app = session_intro();
        app.press(KeyCode::Enter);
        app.press(KeyCode::Enter); // past the two slides of the session intro

        app
    }

    #[test]
    fn a_fresh_module_with_an_intro_shows_its_first_slide_before_the_first_exercise() {
        let screen = fresh_module().screen();

        assert!(screen.has_row(&["Rust · Tools · Intro"]));
        assert!(screen.contains("About intros"));
        assert!(screen.contains("First module slide."));
        assert!(screen.has_row(&["Enter next"]));
        assert!(!screen.contains("Intro quiz"));
    }

    #[test]
    fn enter_and_left_move_through_the_slides_and_the_dots_follow() {
        let mut app = fresh_module();

        app.press(KeyCode::Left); // there is no slide before the first
        let screen = app.screen();
        assert!(screen.contains("First module slide."));
        assert!(screen.has_row(&["●  ○"]));
        assert!(!screen.contains("← back"));

        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.contains("Second module slide."));
        assert!(!screen.contains("First module slide."));
        assert!(screen.has_row(&["○  ●"]));
        assert!(screen.has_row(&["← back"]));

        app.press(KeyCode::Left);
        let screen = app.screen();
        assert!(screen.contains("First module slide."));
        assert!(!screen.contains("Second module slide."));
        assert!(screen.has_row(&["●  ○"]));

        app.press(KeyCode::Right);
        assert!(app.screen().contains("Second module slide."));

        app.press(KeyCode::Right); // the last slide has no next one
        assert!(app.screen().contains("Second module slide."));
    }

    #[test]
    fn the_session_intro_follows_the_module_intro_and_then_the_first_exercise() {
        let mut app = fresh_module();

        app.press(KeyCode::Enter); // the last slide of the module intro
        let screen = app.screen();
        assert!(screen.contains("Second module slide."));
        assert!(screen.has_row(&["Enter next"])); // a session intro follows, not the exercise
        assert!(!screen.contains("Enter start"));

        app.press(KeyCode::Enter); // goes on, to the session intro
        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Tools · Intro", "session 1/1"]));
        assert!(screen.contains("About the session"));
        assert!(screen.contains("First session slide."));
        assert!(!screen.contains("module slide"));
        assert!(screen.has_row(&["●  ○"])); // its own dots, from its first slide
        assert!(!screen.contains("← back"));

        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.contains("Second session slide."));
        assert!(screen.has_row(&["Enter start"])); // the first exercise follows

        app.press(KeyCode::Enter); // starts the first exercise
        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Tools · Intro", "session 1/1 · exercise 1/2"]));
        assert!(screen.contains("Intro quiz"));
    }

    #[test]
    fn esc_goes_back_to_the_explorer_from_any_intro_and_saves_nothing() {
        let mut app = fresh_module();

        app.press(KeyCode::Esc); // from the module intro
        assert!(
            app.screen()
                .has_row(&["Ingrain", "Rust › Ecosystem › Tools"])
        );
        assert!(
            !app.saved_progress()
                .modules
                .iter()
                .any(|module| module.id == "rust/tools/intro")
        );

        let mut app = session_intro();
        assert!(app.screen().contains("About the session"));

        app.press(KeyCode::Esc); // from the session intro
        assert!(
            app.screen()
                .has_row(&["Ingrain", "Rust › Ecosystem › Tools"])
        );
        assert!(
            !app.saved_progress()
                .modules
                .iter()
                .any(|module| module.id == "rust/tools/intro")
        );
    }

    #[test]
    fn a_slide_with_code_shows_it_in_a_window_and_a_slide_without_code_shows_none() {
        let app = fresh_module();
        assert!(!app.screen().contains("╭ Rust")); // a module slide with no code

        let mut app = session_intro();
        let screen = app.screen();
        assert!(screen.contains("First session slide."));
        assert!(screen.has_row(&["╭ Rust"])); // the window, labeled with the language
        assert!(screen.contains("let x = 1;"));

        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.contains("Second session slide."));
        assert!(screen.has_row(&["╭ Rust"])); // the second slide has its own code and window
        assert!(screen.contains("let y = 2;"));
        assert!(!screen.contains("let x = 1;"));
    }

    #[test]
    fn a_long_slide_scrolls_its_text_and_keeps_the_rest_in_place() {
        let mut app = session_intro();
        assert!(!app.screen().contains("↑↓ scroll")); // the first slide fits

        app.press(KeyCode::Enter); // the long slide
        let screen = app.screen();
        assert!(screen.contains("Second session slide."));
        assert!(!screen.contains("End of the long slide."));
        assert!(screen.has_row(&["↑↓ scroll"]));
        assert!(screen.contains("let y = 2;")); // the code is there from the start

        for _ in 0..30 {
            app.press(KeyCode::Down); // past the end, where it stops
        }
        let screen = app.screen();
        assert!(screen.contains("End of the long slide."));
        assert!(!screen.contains("Second session slide."));
        assert!(screen.contains("About the session")); // the title, the dots and the footer stay
        assert!(screen.has_row(&["○  ●"]));
        assert!(screen.has_row(&["↑↓ scroll"]));
        assert!(screen.has_row(&["╭ Rust"])); // and the code stays while the text scrolls
        assert!(screen.contains("let y = 2;"));

        for _ in 0..30 {
            app.press(KeyCode::Up);
        }
        let screen = app.screen();
        assert!(screen.contains("Second session slide."));
        assert!(!screen.contains("End of the long slide."));
        assert!(screen.contains("let y = 2;"));

        app.press(KeyCode::Down);
        app.press(KeyCode::Left);
        app.press(KeyCode::Enter); // a slide opens at its top
        assert!(app.screen().contains("Second session slide."));
    }

    #[test]
    fn a_session_resumed_halfway_or_retaken_shows_no_intro() {
        let mut app = first_exercise();
        app.press(KeyCode::Enter); // checks the first option, which is right
        app.press(KeyCode::Enter); // goes on, to the second exercise
        app.press(KeyCode::Esc);
        app.press(KeyCode::Enter); // opens the module again, where the user stopped
        let screen = app.screen();
        assert!(screen.contains("Another quiz"));
        assert!(!screen.contains("About intros"));
        assert!(!screen.contains("About the session"));

        let mut app = first_exercise();
        app.press(KeyCode::Down);
        app.press(KeyCode::Enter); // a wrong answer
        app.press(KeyCode::Enter); // goes on, to the second exercise
        app.press(KeyCode::Enter); // checks the right option
        app.press(KeyCode::Enter); // the module is played through with a failed exercise
        app.press(KeyCode::Enter); // retakes it
        let screen = app.screen();
        assert!(screen.contains("Intro quiz"));
        assert!(!screen.contains("About the session"));
    }

    #[test]
    fn a_finished_module_started_over_shows_its_intro_again() {
        let mut app = first_exercise();
        app.press(KeyCode::Enter); // checks the right option
        app.press(KeyCode::Enter); // goes on, to the second exercise
        app.press(KeyCode::Enter); // checks the right option
        app.press(KeyCode::Enter); // the module is done, and so is its course
        app.press(KeyCode::Enter); // back to the explorer
        app.press(KeyCode::Enter); // starts the module over
        let screen = app.screen();
        assert!(screen.contains("About intros"));
        assert!(screen.contains("First module slide."));
    }
}
