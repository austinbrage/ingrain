use crate::app::{
    App, Body, CompleteState, Depth, DoneRow, ExerciseDone, ExerciseState, ExplorerState,
    HomeLanguage, IntroState, ModuleDone, ModuleFailed, Screen, SessionDone, SessionRow,
};
use crate::content::{Exercise, Intro, Mode, Module, Output, Session, Slide};

pub fn start_at(app: &mut App, screen: &str, target: Option<&str>) {
    let mut explorer = ExplorerState::new(HomeLanguage::Rust);
    explorer.depth = Depth::Modules;
    explorer.module = 2;

    match screen {
        // An exercise by path: language/course/module[/session[/exercise[/chunk]]]
        // chunk is an index or "last"
        "exercise" => {
            if let Some((explorer, session, exercise, chunk)) =
                target.and_then(|path| locate(app, path))
            {
                let screen = ExerciseState::open(
                    explorer,
                    session,
                    exercise,
                    &app.content,
                    &mut app.progress,
                );

                if let Some(screen) = screen {
                    app.screen = screen;
                }

                if let Some(chunk) = chunk {
                    skip_to_chunk(app, chunk);
                }
            }
        }
        // Short code: fits the cards
        "complete" => {
            let code = [
                "fn main() {",
                "    for i in 0..3 {",
                "        println!(\"{i}\");",
                "    }",
                "}",
            ];
            let output = Output {
                command: "cargo run".to_string(),
                text: "0\n1\n2".to_string(),
            };

            app.screen = complete(explorer, &code, output);
        }
        // Long code: overflows the cards, with a line that needs to wrap
        "complete-long" => {
            let code = [
                "use ratatui::crossterm::event::{self, Event, KeyCode};",
                "use ratatui::{DefaultTerminal, Frame};",
                "",
                "fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {",
                "    let mut presses = 0;",
                "",
                "    loop {",
                "        terminal.draw(|frame| render(frame, presses))?;",
                "",
                "        if let Event::Key(key) = event::read()? {",
                "            if key.code == KeyCode::Char('q') {",
                "                return Ok(());",
                "            }",
                "            presses += 1;",
                "        }",
                "    }",
                "}",
                "",
                "fn render(frame: &mut Frame, presses: u32) {",
                "    frame.render_widget(format!(\"Key presses: {presses}\"), frame.area());",
                "}",
            ];
            let output = Output {
                command: "cargo run".to_string(),
                text: "Key presses: 3".to_string(),
            };

            app.screen = complete(explorer, &code, output);
        }
        // The module celebration, at the end of a course
        "module-done" => {
            app.screen = Screen::Complete(CompleteState::Module(ModuleDone {
                explorer,
                name: "Loops".to_string(),
                session_count: 3,
                exercise_count: 9,
                last_exercises: 2,
                finished_course: Some("Syntax".to_string()),
                // finished_course: None,
            }));
        }
        // A module with a failed session: the sessions table and the retake
        "module-failed" => {
            let row = |passed, total| SessionRow { passed, total };

            app.screen = Screen::Complete(CompleteState::ModuleFailed(ModuleFailed {
                explorer,
                name: "Loops".to_string(),
                rows: vec![row(3, 3), row(2, 3), row(3, 3)],
                last_exercises: 2,
            }));
        }
        // The session recap, with one failed exercise
        "session-done" => {
            let row = |title: &str, kind, passed| DoneRow {
                title: title.to_string(),
                kind,
                passed,
            };

            app.screen = Screen::Complete(CompleteState::Session(SessionDone {
                explorer,
                session: 1,
                session_count: 3,
                rows: vec![
                    row("Move a value", "typing", true),
                    row("Who owns it?", "quiz", false),
                    row("Borrow it", "typing", true),
                ],
            }));
        }
        // An intro of two slides: a short one, and one whose text overflows next to its snippet
        "intro" => {
            let shape = "match value {\n    pattern_a => do_this,\n    pattern_b => do_that,\n    _ => do_otherwise,\n}";
            let patterns = [
                "`1`, `'a'`, `true`: a literal fits only itself",
                "`n`: a name fits anything and binds it",
                "`_`: fits anything and binds nothing",
                "`0..=9`: a range fits the values between its ends",
                "`10..`: an open range fits everything from 10 up",
                "`1 | 2 | 3`: an alternative fits any of its patterns",
                "`(x, y)`: a tuple fits a pair and names both",
                "`(x, _)`: a tuple can ignore one of its parts",
                "`[a, b, c]`: an array fits exactly three items",
                "`[first, .., last]`: a slice fits any list and names its ends",
                "`[head, tail @ ..]`: a slice can name the rest as well",
                "`None`: a variant with no data fits itself",
                "`Some(n)`: a variant with data fits and names its content",
                "`Ok(Some(n))`: variants nest as deep as the type does",
                "`Point { x, y }`: a struct fits and names its fields",
                "`Point { x: px, .. }`: a field can be renamed and the rest ignored",
                "`&n`: a reference fits and names what it points to",
                "`n @ 1..=9`: a name and a range together",
                "`Light::Green`: a path fits one variant of an enum",
                "`n if n > 5`: a guard adds a condition to any arm",
                "`[]`: an empty slice fits only an empty list",
                "`[_]`: a slice of one item, whatever it is",
                "`'a'..='z'`: a range works on chars too",
                "`1..=5 | 10..=15`: alternatives can be ranges",
                "`(0, y) | (y, 0)`: alternatives can bind the same name",
                "`Shape::Circle { r }`: a variant can have named fields",
                "`x @ Some(_)`: a name can bind the whole value while it is checked inside",
                "`\"yes\"`: a string literal fits only that text",
            ];
            let list = patterns.map(|pattern| format!("- {pattern}")).join("\n\n");
            let slides = vec![
                slide(
                    "A `match` compares a value against **patterns**, from the first arm to the last.\n\nThe first arm that fits runs.",
                    Some(shape),
                ),
                slide(
                    &list,
                    Some("if let Some(n) = value {\n    println!(\"{n}\");\n}"),
                ),
            ];

            app.screen = intro(explorer, slides);
        }
        _ => {}
    }
}

fn slide(text: &str, code: Option<&str>) -> Slide {
    Slide {
        text: text.to_string(),
        code: code.map(str::to_string),
    }
}

// A module of one session whose intro is the slides
fn intro(explorer: ExplorerState, slides: Vec<Slide>) -> Screen {
    let module = Module {
        id: "dev".to_string(),
        intro: Some(Intro {
            title: "The match expression".to_string(),
            slides,
        }),
        sessions: vec![Session {
            id: "session-1".to_string(),
            intro: None,
            exercises: vec![],
        }],
    };

    Screen::Intro(IntroState::new(explorer, module, 0))
}

// Ids of an exercise (language/course/module[/session[/exercise[/chunk]]]) as positions
fn locate<'a>(app: &App, path: &'a str) -> Option<(ExplorerState, usize, usize, Option<&'a str>)> {
    let mut ids = path.split('/');
    let language = HomeLanguage::from_id(ids.next()?)?;
    let (course_id, module_id) = (ids.next()?, ids.next()?);

    let (section, course) = app.content.catalog(language).course_index(course_id)?;

    let mut explorer = ExplorerState::new(language);
    explorer.depth = Depth::Modules;
    explorer.section = section;
    explorer.course = course;
    explorer.module = app.content.course(explorer)?.module_index(module_id)?;

    let module = app.content.module(explorer)?;
    let session = match ids.next() {
        Some(id) => module.session_index(id)?,
        None => 0,
    };
    let exercise = match ids.next() {
        Some(id) => module.sessions[session].exercise_index(id)?,
        None => 0,
    };

    Some((explorer, session, exercise, ids.next()))
}

fn skip_to_chunk(app: &mut App, chunk: &str) {
    let Screen::Exercise(state) = &mut app.screen else {
        return;
    };
    let (Exercise::Typing(typing), Body::Typing(typing_state)) = (
        &state.module.sessions[state.session].exercises[state.exercise],
        &mut state.body,
    ) else {
        return;
    };
    let target = match chunk {
        "last" => typing
            .chunks
            .iter()
            .rposition(|chunk| !matches!(chunk.mode, Mode::Auto))
            .unwrap_or(0),
        index => index.parse().unwrap_or(0),
    };

    typing_state.skip_to(typing, target);
}

fn complete(explorer: ExplorerState, code: &[&str], output: Output) -> Screen {
    Screen::Complete(CompleteState::Exercise(ExerciseDone {
        explorer,
        session: 0,
        exercise: 0,
        session_count: 4,
        exercise_count: 1,
        title: "Count to three".to_string(),
        retake: false,
        next_failed: None,
        code: code.iter().map(|line| line.to_string()).collect(),
        output: Some(output),
        scroll: 0,
    }))
}
