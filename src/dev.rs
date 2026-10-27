use crate::app::{
    App, Body, CompleteState, Depth, DoneRow, ExerciseDone, ExerciseState, ExplorerState,
    HomeLanguage, ModuleDone, ModuleFailed, Screen, SessionDone, SessionRow,
};
use crate::content::{Exercise, Mode, Output};

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
        _ => {}
    }
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
