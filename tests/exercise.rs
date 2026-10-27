use ratatui::crossterm::event::KeyCode;

mod common;

use common::TestApp;

fn many_exercise() -> TestApp {
    let mut app = TestApp::new_user();
    app.press(KeyCode::Enter); // opens the language cards
    app.press(KeyCode::Enter); // opens the Rust explorer
    app.wait(5); // all rows appear
    app.press(KeyCode::Right); // focus the courses column
    app.press(KeyCode::Right); // focus the modules column
    app.press(KeyCode::Down); // Planned
    app.press(KeyCode::Down); // Many
    app.press(KeyCode::Enter); // opens its first exercise

    app
}

mod ghost {
    use super::*;

    #[test]
    fn opens_with_the_exercise_guide_and_the_first_chunk_guide() {
        let screen = many_exercise().screen();

        assert!(screen.has_row(&["Rust · Basics · Many", "session 1/2 · exercise 1/2"]));
        assert!(screen.has_row(&["0/2 chunks", "Esc back", "Ctrl-Q quit"]));
        assert!(screen.has_row(&["[░░░░░░░░░░░░]", "0/2 chunks"]));
        assert!(screen.has_row(&["Rust", "Ln 1, Col 1"]));
        assert!(screen.contains("Session one exercise"));
        assert!(screen.contains("Write main."));
        assert!(screen.contains("Start with main."));
        assert!(screen.contains("fn main() {"));
        assert!(!screen.contains("let a = 1;"));
    }

    #[test]
    fn finishing_a_chunk_changes_the_chunk_guide_but_not_the_exercise_guide() {
        let mut app = many_exercise();

        app.type_chunk("fn main() {");
        let screen = app.screen();
        assert!(screen.contains("0/2 chunks"));
        assert!(screen.contains("Start with main."));
        assert!(!screen.contains("let a = 1;"));
        assert!(screen.has_row(&["[░░░░░░░░░░░░]", "0/2 chunks"]));
        assert!(screen.has_row(&["Rust", "Ln 1, Col 12"]));

        app.type_chunk("\n}");
        let screen = app.screen();
        assert!(screen.contains("1/2 chunks"));
        assert!(screen.contains("Write main."));
        assert!(screen.contains("let names a value."));
        assert!(!screen.contains("Start with main."));
        assert!(screen.contains("let a = 1;"));
        assert!(screen.has_row(&["[▓▓▓▓▓▓░░░░░░]", "1/2 chunks"]));
        assert!(screen.has_row(&["Rust", "Ln 2, Col 5"]));
    }

    #[test]
    fn a_wrong_key_shows_the_expected_key_and_blocks_typing_until_backspace() {
        let mut app = many_exercise();

        app.press(KeyCode::Char('x')); // the chunk starts with an f
        let screen = app.screen();
        assert!(screen.has_row(&["^", "expected f"]));
        assert!(screen.contains("xn main() {"));
        assert!(!screen.contains("fn main() {"));

        app.press(KeyCode::Char('n')); // ignored while a key is wrong
        let screen = app.screen();
        assert!(screen.has_row(&["^", "expected f"]));
        assert!(screen.contains("xn main() {"));
        assert!(screen.contains("0/2 chunks"));
        assert!(screen.contains("Start with main."));

        app.press(KeyCode::Backspace);
        let screen = app.screen();
        assert!(!screen.contains("expected"));
        assert!(screen.contains("fn main() {"));

        app.type_chunk("fn main() {\n}");
        let screen = app.screen();
        assert!(screen.contains("1/2 chunks"));
        assert!(screen.contains("let names a value."));
    }

    #[test]
    fn a_wrong_enter_shows_the_enter_symbol_and_names_the_expected_key() {
        let mut app = many_exercise();
        app.type_chunk("fn");

        app.press(KeyCode::Enter); // a space is expected
        let screen = app.screen();
        assert!(screen.has_row(&["^", "expected space"]));
        assert!(screen.contains("fn↵main() {"));

        app.press(KeyCode::Backspace);
        let screen = app.screen();
        assert!(!screen.contains("expected"));
        assert!(screen.contains("fn main() {"));
    }

    #[test]
    fn esc_goes_back_to_the_explorer() {
        let mut app = many_exercise();
        app.type_chunk("fn");

        app.press(KeyCode::Esc);
        let screen = app.screen();

        assert!(screen.has_row(&["Ingrain", "Rust › Core › Basics"]));
        assert!(screen.has_row(&["↑↓ move", "Enter start", "← back", "Ctrl-Q quit"]));
        assert!(!screen.contains("fn main() {"));
    }

    #[test]
    fn tab_types_spaces_so_it_goes_wrong_where_fewer_are_expected() {
        let mut app = many_exercise();
        app.type_chunk("fn");

        app.press(KeyCode::Tab); // four spaces, but only one is expected
        let screen = app.screen();

        assert!(screen.has_row(&["^", "expected m"]));
    }

    #[test]
    fn a_note_appears_under_its_anchor_in_a_ghost_chunk() {
        let mut app = many_exercise();
        app.type_chunk("fn");
        assert!(!app.screen().contains("main is where the program starts."));

        app.press(KeyCode::Char(' ')); // the anchor starts right after this space
        let screen = app.screen();
        assert!(screen.has_row(&["╰─ ", "main is where the program starts."]));
        assert_eq!(
            screen.column_of("╰─ "),
            screen.column_of("fn main() {").map(|x| x + 3)
        );
    }
}

mod auto {
    use super::*;

    #[test]
    fn an_auto_chunk_at_the_start_is_already_in_the_file() {
        let mut app = TestApp::with_progress("rust-many-session-1-done");
        app.press(KeyCode::Enter); // Continue opens session 2

        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Basics · Many", "session 2/2 · exercise 1/1"]));
        assert!(screen.contains("Session two exercise"));
        assert!(screen.contains("use std::io;"));
        assert!(screen.has_row(&["[▓▓▓░░░░░░░░░]", "1/4 chunks"]));
        assert!(screen.contains("Start with main."));
        assert!(screen.has_row(&["Rust", "Ln 2, Col 1"]));
        assert!(!screen.contains("println!"));
    }

    #[test]
    fn an_auto_chunk_in_the_middle_appears_when_the_chunk_before_is_finished() {
        let mut app = TestApp::with_progress("rust-many-session-1-done");
        app.press(KeyCode::Enter); // Continue opens session 2

        app.type_chunk("fn main() {\n}");
        let screen = app.screen();
        assert!(screen.contains("println!(\"hi\");"));
        assert!(screen.has_row(&["[▓▓▓▓▓▓▓▓▓░░░]", "3/4 chunks"]));
        assert!(screen.contains("let names a value."));
        assert!(!screen.contains("Start with main."));
        assert!(screen.has_row(&["Rust", "Ln 3, Col 5"]));

        app.type_chunk("    let a = 1;");
        let screen = app.screen();
        assert!(screen.contains("Enter finish module"));
    }
}

mod copy {
    use super::*;

    fn first_copy_chunk() -> TestApp {
        let mut app = TestApp::new_user();
        app.press(KeyCode::Enter); // opens the language cards
        app.press(KeyCode::Enter); // opens the Rust explorer
        app.wait(5); // all rows appear
        app.press(KeyCode::Right); // focus the courses column
        app.press(KeyCode::Right); // focus the modules column, One is selected
        app.press(KeyCode::Enter); // opens its exercise
        app.type_chunk("fn main() {\n}"); // the ghost chunk comes first

        app
    }

    #[test]
    fn shows_the_guide_and_the_code_to_copy_with_the_next_mark_under_its_first_char() {
        let screen = first_copy_chunk().screen();

        assert!(screen.has_row(&["[▓▓▓▓░░░░░░░░]", "1/3 chunks"]));
        assert!(screen.contains("Print the value of a."));
        assert!(screen.contains("let names a value."));
        assert!(screen.contains("let a = 1;"));
        assert!(!screen.has_row(&["▶", "let a = 1;"]));
        assert!(screen.has_row(&["Rust", "Ln 2, Col 5"]));
        assert_eq!(screen.column_of("^  next"), screen.column_of("let a = 1;"));
    }

    #[test]
    fn typing_moves_the_next_mark_along_the_code() {
        let mut app = first_copy_chunk();
        let start = app.screen().column_of("let a = 1;").unwrap();

        app.type_chunk("let ");
        let screen = app.screen();

        assert!(screen.has_row(&["▶", "let a"]));
        assert!(!screen.has_row(&["▶", "= 1;"]));
        assert_eq!(screen.column_of("^  next"), Some(start + 4));
    }

    #[test]
    fn a_wrong_key_turns_the_mark_into_wrong_key_until_backspace() {
        let mut app = first_copy_chunk();
        let start = app.screen().column_of("let a = 1;").unwrap();
        app.type_chunk("let ");

        app.press(KeyCode::Char('x')); // an a is expected
        let screen = app.screen();
        assert_eq!(screen.column_of("^  wrong key"), Some(start + 4));
        assert!(screen.has_row(&["^", "expected a"]));
        assert!(!screen.contains("^  next"));

        app.press(KeyCode::Backspace);
        let screen = app.screen();
        assert_eq!(screen.column_of("^  next"), Some(start + 4));
        assert!(!screen.contains("wrong key"));
        assert!(!screen.contains("expected"));
    }

    #[test]
    fn a_note_appears_under_its_anchor_when_the_typing_reaches_it() {
        let mut app = first_copy_chunk();
        assert!(!app.screen().contains("The literal 1 is the value."));

        app.type_chunk("let a");
        assert!(!app.screen().contains("The literal 1 is the value."));

        app.press(KeyCode::Char(' ')); // the anchor starts right after this space
        let screen = app.screen();
        assert!(screen.has_row(&["╰─ ", "The literal 1 is the value."]));
        assert_eq!(
            screen.column_of("╰─ "),
            screen.column_of("let a ").map(|x| x + 6)
        );
    }

    #[test]
    fn a_note_goes_away_when_its_chunk_is_finished() {
        let mut app = first_copy_chunk();
        app.type_chunk("let a = 1;");

        let screen = app.screen();

        assert!(!screen.contains("The literal 1 is the value."));
        assert!(!screen.contains("╰─ "));
    }

    #[test]
    fn finishing_a_copy_chunk_changes_the_guide_and_the_code_to_copy() {
        let mut app = first_copy_chunk();

        app.type_chunk("let a = 1;");
        let screen = app.screen();

        assert!(screen.has_row(&["[▓▓▓▓▓▓▓▓░░░░]", "2/3 chunks"]));
        assert!(screen.contains("Print the value of a."));
        assert!(screen.contains("println prints it."));
        assert!(!screen.contains("let names a value."));
        assert!(screen.contains("println!(\"{a}\");"));
        assert_eq!(
            screen.column_of("^  next"),
            screen.column_of("println!(\"{a}\");")
        );
    }
}

mod indent {
    use super::*;

    fn alone_exercise() -> TestApp {
        let mut app = TestApp::with_progress("rust-basics-done");
        app.press(KeyCode::Enter); // Continue opens Alone, the first module of Second

        app
    }

    #[test]
    fn the_indentation_after_an_enter_is_filled_by_the_app() {
        let mut app = alone_exercise();

        app.type_chunk("fn main() {");
        app.press(KeyCode::Enter); // the next line starts with four spaces
        let screen = app.screen();
        assert!(screen.has_row(&["Rust", "Ln 2, Col 5"]));
        assert!(!screen.contains("expected"));

        app.type_chunk("let a = 1;");
        app.press(KeyCode::Enter); // the closing brace has no indentation
        let screen = app.screen();
        assert!(screen.has_row(&["Rust", "Ln 3, Col 1"]));
        assert!(!screen.contains("expected"));

        app.type_chunk("}");
        let screen = app.screen();
        assert!(screen.contains("    let a = 1;"));
        assert!(screen.has_rows_in_order(&["fn main() {", "let a = 1;", "}"]));
        assert!(screen.contains("Enter finish module"));
    }

    #[test]
    fn typing_the_indentation_yourself_is_a_wrong_key() {
        let mut app = alone_exercise();
        app.type_chunk("fn main() {");
        app.press(KeyCode::Enter);

        app.press(KeyCode::Char(' ')); // the app already typed the spaces
        let screen = app.screen();

        assert!(screen.has_row(&["^", "expected l"]));
    }
}

mod scroll {
    use super::*;

    fn lets(from: u32, to: u32) -> String {
        let lines: Vec<String> = (from..=to).map(|i| format!("let a{i} = {i};")).collect();

        lines.join("\n")
    }

    #[test]
    fn the_editor_scrolls_when_the_finished_chunks_accumulate() {
        let mut app = TestApp::with_progress("rust-alone-done");
        app.press(KeyCode::Enter); // Continue opens Long, the next module of Second

        let screen = app.screen();
        assert!(screen.has_row(&["Rust · Second · Long", "session 1/1 · exercise 1/1"]));
        assert!(screen.contains("fn main() {"));
        assert!(!screen.contains("let a1 = 1;"));
        assert!(screen.has_row(&["Rust", "Ln 1, Col 1"]));

        app.type_chunk("fn main() {\n}");
        app.type_chunk(&lets(1, 6));
        let screen = app.screen(); // 14 lines still fit in the editor
        assert!(screen.contains("fn main() {"));
        assert!(screen.has_row(&["▶", "let a7 = 7;"]));
        assert!(screen.has_row(&["Rust", "Ln 8, Col 5"]));
        assert!(screen.contains("let a12 = 12;"));

        app.type_chunk(&lets(7, 12));
        let screen = app.screen(); // the cursor reached line 14, so the first lines scroll off
        assert!(screen.has_row(&["Rust", "Ln 14, Col 5"]));
        assert!(screen.has_row(&["▶", "let a13 = 13;"]));
        assert!(!screen.contains("fn main() {"));
        assert!(!screen.contains("let a3 = 3;"));
        assert!(screen.contains("let a4 = 4;"));
        assert!(screen.contains("let a18 = 18;"));
    }
}

mod wide {
    use super::*;

    fn wide_exercise() -> TestApp {
        let mut app = TestApp::with_progress("rust-long-done");
        app.press(KeyCode::Enter); // Continue opens Wide, the next module of Second
        app.type_chunk("fn main() {\n}"); // the ghost chunk comes first

        app
    }

    #[test]
    fn a_wide_copy_line_wraps_and_the_mark_follows_the_piece_that_holds_the_next_char() {
        let mut app = wide_exercise();

        let screen = app.screen();
        assert!(screen.has_rows_in_order(&[
            "let total = compute_total(fir",
            "^  next",
            "st, second);",
        ]));

        app.type_chunk("let total = compute_to"); // no room left for the label
        let screen = app.screen();
        assert!(!screen.contains("^  next"));
        assert!(screen.has_rows_in_order(&["let total = compute_total(fir", "^", "st, second);",]));

        app.type_chunk("tal(fir"); // the next char is now in the second piece
        let screen = app.screen();
        assert!(screen.has_rows_in_order(&[
            "let total = compute_total(fir",
            "st, second);",
            "^  next",
        ]));

        app.type_chunk("st");
        let screen = app.screen();
        assert_eq!(
            screen.column_of("^  next"),
            screen.column_of("st, second);").map(|x| x + 2)
        );
    }

    #[test]
    fn a_long_note_wraps_into_a_second_line_aligned_with_the_first() {
        let mut app = wide_exercise();
        assert!(!app.screen().contains("It adds the two values"));

        app.type_chunk("let total = "); // the anchor starts right after this
        let screen = app.screen();

        assert!(screen.has_row(&["╰─ ", "It adds the two values and returns"]));
        assert!(screen.has_rows_in_order(&["It adds the two values and returns", "the sum."]));
        assert_eq!(
            screen.column_of("the sum."),
            screen.column_of("It adds the two values and returns")
        );
    }

    #[test]
    fn a_note_stays_under_the_wrong_key_badge_until_backspace() {
        let mut app = wide_exercise();
        app.type_chunk("let total = ");

        app.press(KeyCode::Char('x')); // a c is expected
        let screen = app.screen();
        assert!(screen.has_rows_in_order(&["let total = x", "^  expected c", "╰─ ", "the sum.",]));
        assert_eq!(screen.column_of("^  expected c"), screen.column_of("╰─ "));

        app.press(KeyCode::Backspace);
        let screen = app.screen();
        assert!(!screen.contains("expected"));
        assert!(screen.contains("╰─ "));
    }

    #[test]
    fn a_tall_copy_chunk_scrolls_the_card_to_keep_the_next_mark_in_view() {
        let mut app = wide_exercise();
        app.type_chunk("let total = compute_total(first, second);"); // the tall chunk comes next

        let screen = app.screen();
        assert!(screen.contains("A tall chunk."));
        assert_eq!(screen.column_of("^  next"), screen.column_of("let b1 = 1;"));

        let lines: Vec<String> = (1..=9).map(|i| format!("let b{i} = {i};")).collect();
        app.type_chunk(&lines.join("\n"));
        let screen = app.screen();
        assert!(!screen.contains("A tall chunk."));
        assert!(screen.contains("^  next"));
    }
}

mod quiz {
    use super::*;

    fn quiz_exercise() -> TestApp {
        let mut app = TestApp::new_user();
        app.press(KeyCode::Enter); // opens the language cards
        app.press(KeyCode::Enter); // opens the Rust explorer
        app.wait(5); // all rows appear
        app.press(KeyCode::Down); // Ecosystem
        app.press(KeyCode::Right); // focus the courses column, on Tools
        app.press(KeyCode::Right); // focus the modules column
        app.press(KeyCode::Down); // Quiz
        app.press(KeyCode::Enter); // opens its first exercise

        app
    }

    #[test]
    fn opens_with_the_guide_the_code_and_the_options() {
        let screen = quiz_exercise().screen();

        assert!(screen.has_row(&["Rust · Tools · Quiz", "session 1/1 · exercise 1/2"]));
        assert!(screen.has_row(&["[░░░░░░░░░░░░]", "0/2 exercises"]));
        assert!(screen.has_row(&["↑↓ choose", "Enter check", "Esc back", "Ctrl-Q quit"]));
        assert!(screen.contains("What does total do?"));
        assert!(screen.contains("Pick what the function does."));
        assert!(screen.contains("fn total(prices: &[u32]) -> u32 {"));
        assert!(screen.has_row(&["Rust", "read only"]));
        assert!(screen.contains("purpose"));
        assert!(screen.has_row(&["▶ 1", "Adds up the prices"]));
        assert!(screen.has_row(&["2", "Finds the highest price"]));
        assert!(screen.has_row(&["3", "Counts the prices"]));
        assert!(!screen.contains("explanation"));
    }

    #[test]
    fn down_and_up_move_the_selected_option_and_stay_inside_the_list() {
        let mut app = quiz_exercise();

        app.press(KeyCode::Up); // already on the first option
        assert!(app.screen().has_row(&["▶ 1", "Adds up the prices"]));

        app.press(KeyCode::Down);
        let screen = app.screen();
        assert!(screen.has_row(&["▶ 2", "Finds the highest price"]));
        assert!(!screen.has_row(&["▶ 1"]));
        assert!(screen.has_row(&["1", "Adds up the prices"]));

        app.press(KeyCode::Down);
        app.press(KeyCode::Down); // already on the last option
        let screen = app.screen();
        assert!(screen.has_row(&["▶ 3", "Counts the prices"]));
        assert!(!screen.has_row(&["▶ 2"]));

        app.press(KeyCode::Up);
        assert!(app.screen().has_row(&["▶ 2", "Finds the highest price"]));
    }

    #[test]
    fn enter_on_the_right_option_shows_the_result_and_the_explanation() {
        let mut app = quiz_exercise();

        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.contains("right"));
        assert!(screen.has_row(&["✓ 1", "Adds up the prices"]));
        assert!(!screen.contains("✗"));
        assert!(screen.contains("explanation"));
        assert!(screen.contains("It sums every price in the list."));
        assert!(!screen.contains("Pick what the function does."));
        assert!(!screen.contains("purpose"));
        assert!(screen.has_row(&["0/2 exercises", "Enter continue", "Esc back", "Ctrl-Q quit"]));
        assert!(!screen.contains("Enter check"));
    }

    #[test]
    fn enter_on_a_wrong_option_marks_it_and_shows_the_right_one() {
        let mut app = quiz_exercise();

        app.press(KeyCode::Down); // Finds the highest price
        app.press(KeyCode::Enter);
        let screen = app.screen();
        assert!(screen.contains("wrong"));
        assert!(screen.has_row(&["✓ 1", "Adds up the prices"]));
        assert!(screen.has_row(&["✗ 2", "Finds the highest price"]));
        assert!(screen.contains("It sums every price in the list."));
        assert!(!screen.contains("purpose"));
        assert!(screen.has_row(&["0/2 exercises", "Enter continue"]));
    }

    #[test]
    fn after_checking_up_and_down_do_not_move_the_selection() {
        let mut app = quiz_exercise();

        app.press(KeyCode::Down); // Finds the highest price
        app.press(KeyCode::Enter);
        app.press(KeyCode::Down);
        app.press(KeyCode::Up);
        app.press(KeyCode::Up);
        let screen = app.screen();
        assert!(screen.has_row(&["✓ 1", "Adds up the prices"]));
        assert!(screen.has_row(&["✗ 2", "Finds the highest price"]));
        assert!(!screen.has_row(&["▶"]));
    }

    #[test]
    fn esc_goes_back_to_the_explorer_before_and_after_checking() {
        let mut app = quiz_exercise();

        app.press(KeyCode::Esc);
        let screen = app.screen();
        assert!(screen.has_row(&["Ingrain", "Rust › Ecosystem › Tools"]));
        assert!(screen.has_row(&["↑↓ move", "Enter start", "← back", "Ctrl-Q quit"]));
        assert!(!screen.contains("fn total(prices: &[u32]) -> u32 {"));

        let mut app = quiz_exercise();

        app.press(KeyCode::Enter); // checks the first option
        app.press(KeyCode::Esc);
        let screen = app.screen();
        assert!(screen.has_row(&["Ingrain", "Rust › Ecosystem › Tools"]));
        assert!(!screen.contains("explanation"));
    }

    #[test]
    fn the_options_card_names_what_each_kind_asks() {
        let mut app = TestApp::with_progress("last-rust-tools");
        app.press(KeyCode::Enter); // Continue opens the purpose quiz
        assert!(app.screen().has_row(&["╭ purpose"]));

        let mut app = TestApp::with_progress("rust-quiz-exercise-2");
        app.press(KeyCode::Enter); // Continue opens the output quiz
        assert!(app.screen().has_row(&["╭ output"]));

        let mut app = TestApp::with_progress("rust-tools-before-kinds");
        app.press(KeyCode::Enter); // Continue opens the compiler quiz
        assert!(app.screen().has_row(&["╭ cause"]));

        app.press(KeyCode::Enter); // checks the first option, which is right
        app.press(KeyCode::Enter); // goes on, to the error quiz
        assert!(app.screen().has_row(&["╭ error kind"]));
    }
}

mod quit {
    use super::*;

    #[test]
    fn ctrl_q_quits_and_a_plain_q_is_only_a_letter() {
        let mut app = many_exercise();

        app.press(KeyCode::Char('q')); // the chunk starts with an f
        let screen = app.screen();
        assert!(!app.has_quit());
        assert!(screen.has_row(&["^", "expected f"]));

        app.press(KeyCode::Backspace);
        app.press_ctrl('q');
        let screen = app.screen();
        assert!(app.has_quit());
        assert!(!screen.contains("expected"));
    }
}
