use ingrain::content::{
    Catalog, Course, Exercise, FillBlank, Intro, Layer, Mode, Module, Quiz, Source, Typing,
};
use ratatui::widgets::{Paragraph, Wrap};
use std::path::{Path, PathBuf};

// The text width of the step card on a 100 column terminal, and its limits in rows (the exercise
// guide is the upper card; a copy chunk counts its guide and its code together)
const STEP_WIDTH: usize = 29;
const MAX_EXERCISE_GUIDE_ROWS: usize = 4;
const MAX_GHOST_GUIDE_ROWS: usize = 9;
const MAX_COPY_ROWS: usize = 6;

// The text width of an intro slide, and its limits: rows of text, and the snippet window
const INTRO_WIDTH: usize = 70;
const MAX_INTRO_TEXT_ROWS: usize = 8;
const MAX_SNIPPET_LINES: usize = 14;
const MAX_SNIPPET_WIDTH: usize = 40;

// The limits of a quiz: its options
const MIN_QUIZ_OPTIONS: usize = 2;
const MAX_QUIZ_OPTIONS: usize = 4;
const MAX_OPTION_CHARS: usize = 50;

// The limits of a fill-blank: its gaps, whose block is 3 cells wide at least
const MIN_GAPS: usize = 1;
const MAX_GAPS: usize = 6;
const GAP_MARKER: &str = "___";

// The code window of a quiz and of a fill-blank does not scroll
const MAX_CODE_LINES: usize = 14;
const MAX_CODE_WIDTH: usize = 50;

// ------------------------------------------------------------
// The checks: small functions, each returns what is wrong as sentences that say where
// ------------------------------------------------------------

// Ids are slugs (lowercase letters, digits and hyphens), and none repeats among its siblings
fn slug_problems(place: &str, ids: &[&str]) -> Vec<String> {
    let mut problems = Vec::new();

    for (index, id) in ids.iter().enumerate() {
        let slug = !id.is_empty()
            && id
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');

        if !slug {
            problems.push(format!(
                "{place}: `{id}` is not a slug (lowercase letters, digits and hyphens)"
            ));
        }
        if ids[..index].contains(id) {
            problems.push(format!("{place}: `{id}` is used twice"));
        }
    }

    problems
}

// A course group has courses, and no other group of the layer has its name
fn group_problems(place: &str, layer: &Layer) -> Vec<String> {
    let mut problems = Vec::new();

    for (index, group) in layer.course_groups.iter().enumerate() {
        if group.courses.is_empty() {
            problems.push(format!(
                "{place}: the group `{}` has no courses",
                group.name
            ));
        }
        if layer.course_groups[..index]
            .iter()
            .any(|other| other.name == group.name)
        {
            problems.push(format!("{place}: the group `{}` is used twice", group.name));
        }
    }

    problems
}

// A module file has the same id and the same number of sessions as its course.json says
fn module_problems(place: &str, id: &str, sessions: usize, module: &Module) -> Vec<String> {
    let mut problems = Vec::new();

    if module.id != id {
        problems.push(format!(
            "{place}: the file has the id `{}`, course.json has `{id}`",
            module.id
        ));
    }
    if module.sessions.len() != sessions {
        problems.push(format!(
            "{place}: the file has {} sessions, course.json says {sessions}",
            module.sessions.len()
        ));
    }

    problems
}

// Every course folder, a folder inside a language folder, has a course.json
fn course_folder_problems(content: &Path) -> Vec<String> {
    let mut problems = Vec::new();

    for language in folders(content) {
        for course in folders(&language) {
            if !course.join("course.json").exists() {
                problems.push(format!(
                    "{}: the course folder has no course.json",
                    course.strip_prefix(content).unwrap().display()
                ));
            }
        }
    }

    problems
}

// The rows a markdown text takes at a width, wrapped as the app wraps it
fn markdown_rows(text: &str, width: usize) -> usize {
    Paragraph::new(tui_markdown::from_str(text))
        .wrap(Wrap { trim: false })
        .line_count(width as u16)
}

// The rows the code of a copy chunk takes: without the indentation its lines share, and a line
// longer than the card wraps onto the next row
fn code_rows(code: &str) -> usize {
    let lines: Vec<&str> = code.split('\n').collect();
    let indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);

    lines
        .iter()
        .map(|line| {
            let chars = line.get(indent..).unwrap_or("").chars().count();

            chars.div_ceil(STEP_WIDTH).max(1)
        })
        .sum()
}

// A typing exercise fits a 100x30 terminal without scrolling: its guide, and each chunk's guide
// and code, are within the limits of the step card
fn card_problems(place: &str, typing: &Typing) -> Vec<String> {
    let mut problems = Vec::new();
    let rows = markdown_rows(&typing.guide, STEP_WIDTH);

    if rows > MAX_EXERCISE_GUIDE_ROWS {
        problems.push(format!(
            "{place}: the exercise guide takes {rows} rows, the limit is {MAX_EXERCISE_GUIDE_ROWS}"
        ));
    }

    for (index, chunk) in typing.chunks.iter().enumerate() {
        let place = format!("{place} chunk {}", index + 1);
        let guide = chunk
            .guide
            .as_deref()
            .map_or(0, |guide| markdown_rows(guide, STEP_WIDTH));

        match chunk.mode {
            Mode::Ghost if guide > MAX_GHOST_GUIDE_ROWS => problems.push(format!(
                "{place}: the guide takes {guide} rows, the limit is {MAX_GHOST_GUIDE_ROWS}"
            )),
            Mode::Copy if guide + code_rows(&chunk.text) > MAX_COPY_ROWS => problems.push(format!(
                "{place}: the guide and the code take {} rows, the limit is {MAX_COPY_ROWS}",
                guide + code_rows(&chunk.text)
            )),
            _ => {}
        }
    }

    problems
}

// An intro has slides, and each slide has a text and a snippet that fit the intro screen
fn intro_problems(place: &str, intro: &Intro) -> Vec<String> {
    let mut problems = Vec::new();

    if intro.slides.is_empty() {
        problems.push(format!("{place}: the intro has no slides"));
    }

    for (index, slide) in intro.slides.iter().enumerate() {
        let place = format!("{place} slide {}", index + 1);
        let rows = markdown_rows(&slide.text, INTRO_WIDTH);

        if slide.text.trim().is_empty() {
            problems.push(format!("{place}: the slide has no text"));
        }
        if rows > MAX_INTRO_TEXT_ROWS {
            problems.push(format!(
                "{place}: the text takes {rows} rows, the limit is {MAX_INTRO_TEXT_ROWS}"
            ));
        }

        let Some(code) = &slide.code else {
            continue;
        };
        let lines = code.lines().count();
        let widest = code
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(0);

        if lines > MAX_SNIPPET_LINES {
            problems.push(format!(
                "{place}: the snippet has {lines} lines, the limit is {MAX_SNIPPET_LINES}"
            ));
        }
        if widest > MAX_SNIPPET_WIDTH {
            problems.push(format!(
                "{place}: the snippet has a line of {widest} columns, the limit is {MAX_SNIPPET_WIDTH}"
            ));
        }
    }

    problems
}

// A quiz has 2 to 4 different options of 50 characters or less, an answer that is one of them,
// and code that fits the code window
fn quiz_problems(place: &str, quiz: &Quiz) -> Vec<String> {
    let mut problems = Vec::new();
    let options = quiz.options.len();

    if !(MIN_QUIZ_OPTIONS..=MAX_QUIZ_OPTIONS).contains(&options) {
        problems.push(format!(
            "{place}: a quiz needs {MIN_QUIZ_OPTIONS} to {MAX_QUIZ_OPTIONS} options and has {options}"
        ));
    }
    if quiz.answer >= options {
        problems.push(format!(
            "{place}: the answer is {}, but the options are counted from 0 and there are {options}",
            quiz.answer
        ));
    }

    for (index, option) in quiz.options.iter().enumerate() {
        let chars = option.chars().count();

        if quiz.options[..index].contains(option) {
            problems.push(format!("{place}: the option `{option}` is repeated"));
        }
        if chars > MAX_OPTION_CHARS {
            problems.push(format!(
                "{place}: option {} has {chars} characters, the limit is {MAX_OPTION_CHARS}",
                index + 1
            ));
        }
    }

    let lines = quiz.code.lines().count();
    let widest = quiz
        .code
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    if lines > MAX_CODE_LINES {
        problems.push(format!(
            "{place}: the code has {lines} lines, the limit is {MAX_CODE_LINES}"
        ));
    }
    if widest > MAX_CODE_WIDTH {
        problems.push(format!(
            "{place}: the code has a line of {widest} columns, the limit is {MAX_CODE_WIDTH}"
        ));
    }

    problems
}

// A fill-blank has one `___` in its code for each of its 1 to 6 gaps, each gap has different
// single-word answers, and the code fits the code window once the gaps hold their longest answer
fn fill_blank_problems(place: &str, fill_blank: &FillBlank) -> Vec<String> {
    let mut problems = Vec::new();
    let gaps = fill_blank.gaps.len();
    let markers = fill_blank.code.matches(GAP_MARKER).count();

    if !(MIN_GAPS..=MAX_GAPS).contains(&gaps) {
        problems.push(format!(
            "{place}: a fill-blank needs {MIN_GAPS} to {MAX_GAPS} gaps and has {gaps}"
        ));
    }
    if markers != gaps {
        problems.push(format!(
            "{place}: the code has {markers} `{GAP_MARKER}` and there are {gaps} gaps"
        ));
    }

    for (index, gap) in fill_blank.gaps.iter().enumerate() {
        let place = format!("{place} gap {}", index + 1);

        if gap.answers.is_empty() {
            problems.push(format!("{place}: the gap has no answers"));
        }

        for (answer_index, answer) in gap.answers.iter().enumerate() {
            if answer.is_empty() || answer.contains(char::is_whitespace) {
                problems.push(format!(
                    "{place}: the answer `{answer}` is not a single word"
                ));
            }
            if gap.answers[..answer_index].contains(answer) {
                problems.push(format!("{place}: the answer `{answer}` is repeated"));
            }
        }
    }

    let mut longest = fill_blank.gaps.iter().map(|gap| {
        gap.answers
            .iter()
            .map(|answer| answer.chars().count())
            .max()
            .unwrap_or(0)
    });
    let mut shown = String::new();

    for (index, text) in fill_blank.code.split(GAP_MARKER).enumerate() {
        if index > 0 {
            let width = longest.next().unwrap_or(0).max(GAP_MARKER.len());

            shown.push_str(&"x".repeat(width));
        }
        shown.push_str(text);
    }

    let lines = shown.lines().count();
    let widest = shown
        .lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    if lines > MAX_CODE_LINES {
        problems.push(format!(
            "{place}: the code has {lines} lines, the limit is {MAX_CODE_LINES}"
        ));
    }
    if widest > MAX_CODE_WIDTH {
        problems.push(format!(
            "{place}: the code has a line of {widest} columns, the limit is {MAX_CODE_WIDTH}"
        ));
    }

    problems
}

// Brackets are balanced inside a chunk: what opens closes in the same chunk. Brackets inside a
// string do not count. (A quote inside a char literal is not handled: no chunk has one)
fn balanced(text: &str) -> bool {
    let mut open = Vec::new();
    let mut in_string = false;
    let mut chars = text.chars();

    while let Some(c) = chars.next() {
        match c {
            '\\' if in_string => {
                chars.next();
            }
            '"' => in_string = !in_string,
            '(' | '[' | '{' if !in_string => open.push(c),
            ')' | ']' | '}' if !in_string => {
                let opener = match c {
                    ')' => '(',
                    ']' => '[',
                    _ => '{',
                };

                if open.pop() != Some(opener) {
                    return false;
                }
            }
            _ => {}
        }
    }

    open.is_empty()
}

// The file the chunks make, as the editor builds it, has code only inside functions: a line outside
// every bracket starts a declaration or closes one. A chunk whose `line` is one too far leaves a
// statement after the `}` of `main`. Returns the file line of the first such code
fn code_outside_function(typing: &Typing) -> Option<usize> {
    let mut file: Vec<&str> = Vec::new();

    for chunk in &typing.chunks {
        let at = chunk.line.saturating_sub(1).min(file.len());
        file.splice(at..at, chunk.text.split('\n'));
    }

    let mut depth: i32 = 0;

    for (index, line) in file.iter().enumerate() {
        let starts = [
            "package", "import", "func", "var", "const", "type", "}", ")",
        ];
        let text = line.trim();

        if depth == 0 && !text.is_empty() && !starts.iter().any(|word| text.starts_with(word)) {
            return Some(index + 1);
        }

        let mut in_string = false;
        let mut chars = line.chars();

        while let Some(c) = chars.next() {
            match c {
                '\\' if in_string => {
                    chars.next();
                }
                '"' => in_string = !in_string,
                '(' | '[' | '{' if !in_string => depth += 1,
                ')' | ']' | '}' if !in_string => depth -= 1,
                _ => {}
            }
        }
    }

    None
}

// The chunks of a typing exercise are well formed: each `line` is a place in the file as it is
// when the chunk starts, a ghost chunk has a guide, the code is balanced, and each note is plain
// text on a piece of the chunk that occurs once
fn chunk_problems(place: &str, typing: &Typing) -> Vec<String> {
    let mut problems = Vec::new();
    let mut file_lines = 0;

    for (index, chunk) in typing.chunks.iter().enumerate() {
        let place = format!("{place} chunk {}", index + 1);

        if chunk.line == 0 || chunk.line > file_lines + 1 {
            problems.push(format!(
                "{place}: the line {} is not a place in the file, which has {file_lines} lines when the chunk starts",
                chunk.line
            ));
        }
        file_lines += chunk.text.split('\n').count();

        if matches!(chunk.mode, Mode::Ghost) && chunk.guide.is_none() {
            problems.push(format!("{place}: a ghost chunk needs a guide"));
        }
        if !balanced(&chunk.text) {
            problems.push(format!(
                "{place}: the code opens a bracket it does not close, or the other way round"
            ));
        }

        for note in &chunk.notes {
            let times = chunk.text.matches(note.on.as_str()).count();

            if times != 1 {
                problems.push(format!(
                    "{place}: the note anchor `{}` occurs {times} times in the chunk, it must occur once",
                    note.on
                ));
            }
            if note.text.contains('`') || note.text.contains("**") {
                problems.push(format!(
                    "{place}: the note `{}` has markdown, a note is plain text",
                    note.text
                ));
            }
        }
    }

    if place.starts_with("go/")
        && let Some(line) = code_outside_function(typing)
    {
        problems.push(format!(
            "{place}: line {line} of the file is code outside a function"
        ));
    }

    problems
}

// Everything on disk is listed: a course folder in its catalog, a module file in its course.json.
// Taking a name off the list does not remove its content, it only hides it from the app
fn unlisted_problems(place: &str, list: &str, listed: &[&str], found: &[String]) -> Vec<String> {
    found
        .iter()
        .filter(|name| !listed.contains(&name.as_str()))
        .map(|name| format!("{place}: `{name}` exists but is not in {list}"))
        .collect()
}

// The names of the folders inside a folder
fn folder_names(dir: &Path) -> Vec<String> {
    folders(dir)
        .iter()
        .map(|folder| folder.file_name().unwrap().to_str().unwrap().to_string())
        .collect()
}

// The names of the module files of a course folder: its .json files, but course.json
fn module_file_names(course: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(course)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .map(|path| path.file_stem().unwrap().to_str().unwrap().to_string())
        .filter(|name| name != "course")
        .collect();

    names.sort();
    names
}

// The folders inside a folder, in a stable order
fn folders(dir: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .collect();

    found.sort();
    found
}

// The real test: every check on the actual content in content/prod. It fails with the full list of
// problems, each saying where. The tests of the other module do not look at content/prod
mod real_content {
    use super::*;

    // Everything that is wrong in content/prod, as sentences that say where
    fn prod_problems() -> Vec<String> {
        let content = Path::new(env!("CARGO_MANIFEST_DIR")).join("content/prod");
        let mut problems = course_folder_problems(&content);

        for folder in folders(&content) {
            let language = folder.file_name().unwrap().to_str().unwrap();
            let catalog = Catalog::load(Source::Prod, language);
            let layers: Vec<&str> = catalog.layers.iter().map(|l| l.id.as_str()).collect();
            let courses: Vec<&str> = catalog
                .layers
                .iter()
                .flat_map(|layer| layer.courses())
                .map(|course| course.id.as_str())
                .collect();

            problems.extend(slug_problems(language, &layers));
            problems.extend(slug_problems(language, &courses));

            for layer in &catalog.layers {
                problems.extend(group_problems(&format!("{language}/{}", layer.id), layer));
            }

            problems.extend(unlisted_problems(
                language,
                "catalog.json",
                &courses,
                &folder_names(&folder),
            ));

            for course_id in courses {
                // A course of the catalog that is not written yet has no folder
                let Some(course) = Course::load(Source::Prod, language, course_id) else {
                    continue;
                };
                let place = format!("{language}/{course_id}");
                let modules: Vec<_> = course
                    .module_sections
                    .iter()
                    .flat_map(|section| &section.modules)
                    .collect();
                let module_ids: Vec<&str> = modules.iter().map(|m| m.id.as_str()).collect();

                problems.extend(slug_problems(&place, &module_ids));
                problems.extend(unlisted_problems(
                    &place,
                    "course.json",
                    &module_ids,
                    &module_file_names(&content.join(language).join(course_id)),
                ));

                for summary in modules.iter().filter(|summary| summary.written) {
                    let place = format!("{place}/{}", summary.id);
                    let module =
                        Module::load(Source::Prod, language, course_id, &summary.id).unwrap();
                    let sessions: Vec<&str> =
                        module.sessions.iter().map(|s| s.id.as_str()).collect();

                    problems.extend(module_problems(
                        &place,
                        &summary.id,
                        summary.sessions as usize,
                        &module,
                    ));
                    problems.extend(slug_problems(&place, &sessions));

                    if let Some(intro) = &module.intro {
                        problems.extend(intro_problems(&format!("{place} intro"), intro));
                    }

                    for session in &module.sessions {
                        let exercises: Vec<&str> =
                            session.exercises.iter().map(|e| e.id()).collect();
                        let place = format!("{place}/{}", session.id);

                        problems.extend(slug_problems(&place, &exercises));

                        if let Some(intro) = &session.intro {
                            problems.extend(intro_problems(&format!("{place} intro"), intro));
                        }

                        for exercise in &session.exercises {
                            let place = format!("{place}/{}", exercise.id());

                            match exercise {
                                Exercise::Typing(typing) => {
                                    problems.extend(card_problems(&place, typing));
                                    problems.extend(chunk_problems(&place, typing));
                                }
                                Exercise::Quiz(quiz) => {
                                    problems.extend(quiz_problems(&place, quiz))
                                }
                                Exercise::FillBlank(fill_blank) => {
                                    problems.extend(fill_blank_problems(&place, fill_blank))
                                }
                            }
                        }
                    }
                }
            }
        }

        problems
    }

    #[test]
    fn prod_content_has_no_problems() {
        let problems = prod_problems();

        assert!(problems.is_empty(), "\n{}\n", problems.join("\n"));
    }
}

// The checks of the checks: tests on tiny inline examples, not on the content. Each runs one check
// on a tiny piece of data that is wrong on purpose, and expects the check to find it, so a check
// that is broken and always says "no problems" fails here instead of hiding in the real test
mod checks_of_the_checks {
    use super::*;

    #[test]
    fn the_slug_check_catches_a_bad_id_and_a_repeated_one() {
        let problems = slug_problems("rust/syntax", &["loops", "Loops", "loops"]);

        assert_eq!(problems.len(), 2);
        assert!(problems[0].contains("`Loops` is not a slug"));
        assert!(problems[1].contains("`loops` is used twice"));
        assert!(slug_problems("rust/syntax", &["loops", "pattern-matching"]).is_empty());
    }

    fn layer(groups: &str) -> Layer {
        serde_json::from_str(&format!(
            r#"{{ "id": "core", "name": "Core", "course_groups": {groups} }}"#
        ))
        .unwrap()
    }

    #[test]
    fn the_group_check_catches_an_empty_group_and_a_repeated_name() {
        let bad = layer(
            r#"[
                { "name": "Basics", "courses": [{ "id": "syntax", "name": "Syntax" }] },
                { "name": "Empty", "courses": [] },
                { "name": "Basics", "courses": [{ "id": "loops", "name": "Loops" }] }
            ]"#,
        );
        let good = layer(
            r#"[
                { "name": "Basics", "courses": [{ "id": "syntax", "name": "Syntax" }] },
                { "name": "Flow", "courses": [{ "id": "loops", "name": "Loops" }] }
            ]"#,
        );

        let problems = group_problems("rust/core", &bad);

        assert_eq!(problems.len(), 2);
        assert!(problems[0].contains("the group `Empty` has no courses"));
        assert!(problems[1].contains("the group `Basics` is used twice"));
        assert!(group_problems("rust/core", &good).is_empty());
    }

    #[test]
    fn the_module_check_catches_a_wrong_id_and_a_wrong_session_count() {
        let module: Module = serde_json::from_str(
            r#"{ "id": "while", "sessions": [{ "id": "session-1", "exercises": [] }] }"#,
        )
        .unwrap();

        let problems = module_problems("rust/syntax/loops", "loops", 2, &module);

        assert_eq!(problems.len(), 2);
        assert!(problems[0].contains("the file has the id `while`, course.json has `loops`"));
        assert!(problems[1].contains("the file has 1 sessions, course.json says 2"));
        assert!(module_problems("rust/syntax/while", "while", 1, &module).is_empty());
    }

    #[test]
    fn the_listing_check_catches_a_course_or_a_module_that_is_not_in_its_list() {
        let found = ["first-frame".to_string(), "event-loop".to_string()];

        let problems = unlisted_problems("rust/ratatui", "course.json", &["first-frame"], &found);

        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("`event-loop` exists but is not in course.json"));
        assert!(
            unlisted_problems(
                "rust/ratatui",
                "course.json",
                &["first-frame", "event-loop"],
                &found
            )
            .is_empty()
        );
    }

    #[test]
    fn the_folder_check_catches_a_course_with_no_course_json() {
        let content = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(content.path().join("rust/orphan")).unwrap();
        std::fs::create_dir_all(content.path().join("rust/written")).unwrap();
        std::fs::write(content.path().join("rust/written/course.json"), "{}").unwrap();

        let problems = course_folder_problems(content.path());

        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("rust/orphan"));
    }

    // A typing exercise with a guide and one chunk, built from JSON
    fn typing(guide: &str, chunk: &str) -> Typing {
        serde_json::from_str(&format!(
            r#"{{ "id": "x", "title": "X", "guide": "{guide}", "chunks": [{chunk}] }}"#
        ))
        .unwrap()
    }

    #[test]
    fn the_card_check_catches_an_exercise_guide_that_is_too_long() {
        let auto = r#"{ "mode": "auto", "line": 1, "text": "x" }"#;

        let problems = card_problems("rust/x/y", &typing(&"word ".repeat(30), auto));

        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("the exercise guide takes"));
        assert!(card_problems("rust/x/y", &typing("Short.", auto)).is_empty());
    }

    #[test]
    fn the_card_check_catches_a_ghost_guide_that_is_too_long() {
        let ghost = |guide: &str| {
            format!(r#"{{ "mode": "ghost", "line": 1, "guide": "{guide}", "text": "x" }}"#)
        };

        let problems = card_problems("rust/x/y", &typing("Short.", &ghost(&"word ".repeat(60))));

        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("chunk 1: the guide takes"));
        assert!(card_problems("rust/x/y", &typing("Short.", &ghost("Short."))).is_empty());
    }

    #[test]
    fn the_card_check_catches_a_copy_chunk_whose_guide_and_code_are_too_tall() {
        let copy = |guide: &str, text: &str| {
            format!(r#"{{ "mode": "copy", "line": 1, "guide": "{guide}", "text": "{text}" }}"#)
        };
        let two_rows = "Two short lines of guide text here, nothing more.";

        // 2 rows of guide and 5 lines of code
        let many_lines = card_problems(
            "rust/x/y",
            &typing("Short.", &copy(two_rows, r"a\nb\nc\nd\ne")),
        );
        // 2 rows of guide and one line of 130 chars, which wraps into 5 rows
        let wide_line = card_problems(
            "rust/x/y",
            &typing("Short.", &copy(two_rows, &"x".repeat(130))),
        );

        assert_eq!(many_lines.len(), 1);
        assert!(many_lines[0].contains("chunk 1: the guide and the code take 7 rows"));
        assert_eq!(wide_line.len(), 1);
        assert!(card_problems("rust/x/y", &typing("Short.", &copy("Short.", "ab"))).is_empty());
    }

    // An intro with the given slides, built from JSON
    fn intro(slides: &str) -> Intro {
        serde_json::from_str(&format!(r#"{{ "title": "T", "slides": [{slides}] }}"#)).unwrap()
    }

    #[test]
    fn the_intro_check_catches_no_slides_and_a_slide_with_no_text() {
        let none = intro_problems("rust/x/y intro", &intro(""));
        let blank = intro_problems("rust/x/y intro", &intro(r#"{ "text": "  " }"#));

        assert_eq!(none.len(), 1);
        assert!(none[0].contains("the intro has no slides"));
        assert_eq!(blank.len(), 1);
        assert!(blank[0].contains("slide 1: the slide has no text"));
        assert!(intro_problems("rust/x/y intro", &intro(r#"{ "text": "A text." }"#)).is_empty());
    }

    #[test]
    fn the_intro_check_catches_a_text_that_is_too_tall() {
        let long = format!(r#"{{ "text": "{}" }}"#, "word ".repeat(120));

        let problems = intro_problems("rust/x/y intro", &intro(&long));

        assert_eq!(problems.len(), 1);
        assert!(problems[0].contains("slide 1: the text takes"));
    }

    #[test]
    fn the_intro_check_catches_a_snippet_that_is_too_tall_or_too_wide() {
        let snippet = |code: &str| format!(r#"{{ "text": "A text.", "code": "{code}" }}"#);

        let tall = intro_problems("rust/x/y intro", &intro(&snippet(&r"x\n".repeat(15))));
        let wide = intro_problems("rust/x/y intro", &intro(&snippet(&"x".repeat(41))));

        assert_eq!(tall.len(), 1);
        assert!(tall[0].contains("the snippet has 15 lines"));
        assert_eq!(wide.len(), 1);
        assert!(wide[0].contains("a line of 41 columns"));
        assert!(intro_problems("rust/x/y intro", &intro(&snippet(&"x".repeat(40)))).is_empty());
    }

    // A quiz with the given options (as JSON), answer and code, built from JSON
    fn quiz(options: &str, answer: usize, code: &str) -> Quiz {
        serde_json::from_str(&format!(
            r#"{{ "id": "q", "title": "T", "guide": "G", "code": "{code}", "options": [{options}],
                  "answer": {answer}, "explanation": "E" }}"#
        ))
        .unwrap()
    }

    #[test]
    fn the_quiz_check_catches_too_few_or_too_many_options() {
        let one = quiz_problems("rust/x/y/q", &quiz(r#""a""#, 0, "c"));
        let five = quiz_problems("rust/x/y/q", &quiz(r#""a", "b", "c", "d", "e""#, 0, "c"));

        assert_eq!(one.len(), 1);
        assert!(one[0].contains("needs 2 to 4 options and has 1"));
        assert_eq!(five.len(), 1);
        assert!(quiz_problems("rust/x/y/q", &quiz(r#""a", "b""#, 0, "c")).is_empty());
        assert!(quiz_problems("rust/x/y/q", &quiz(r#""a", "b", "c", "d""#, 3, "c")).is_empty());
    }

    #[test]
    fn the_quiz_check_catches_a_repeated_option_and_an_answer_outside_the_options() {
        let repeated = quiz_problems("rust/x/y/q", &quiz(r#""a", "a""#, 0, "c"));
        let outside = quiz_problems("rust/x/y/q", &quiz(r#""a", "b""#, 2, "c"));

        assert_eq!(repeated.len(), 1);
        assert!(repeated[0].contains("the option `a` is repeated"));
        assert_eq!(outside.len(), 1);
        assert!(outside[0].contains("the answer is 2"));
    }

    #[test]
    fn the_quiz_check_catches_a_long_option_and_code_that_is_too_tall_or_too_wide() {
        let options = |last: &str| format!(r#""a", "{last}""#);
        let long = quiz_problems("rust/x/y/q", &quiz(&options(&"x".repeat(51)), 0, "c"));
        let tall = quiz_problems("rust/x/y/q", &quiz(r#""a", "b""#, 0, &r"x\n".repeat(15)));
        let wide = quiz_problems("rust/x/y/q", &quiz(r#""a", "b""#, 0, &"x".repeat(51)));
        let fits = quiz(&options(&"x".repeat(50)), 0, &"x".repeat(50));

        assert_eq!(long.len(), 1);
        assert!(long[0].contains("option 2 has 51 characters"));
        assert_eq!(tall.len(), 1);
        assert!(tall[0].contains("the code has 15 lines"));
        assert_eq!(wide.len(), 1);
        assert!(wide[0].contains("a line of 51 columns"));
        assert!(quiz_problems("rust/x/y/q", &fits).is_empty());
    }

    // A fill-blank with the given code and gaps (as JSON), built from JSON
    fn fill_blank(code: &str, gaps: &str) -> FillBlank {
        serde_json::from_str(&format!(
            r#"{{ "id": "f", "title": "T", "guide": "G", "code": "{code}", "gaps": [{gaps}] }}"#
        ))
        .unwrap()
    }

    #[test]
    fn the_fill_blank_check_catches_markers_that_do_not_match_the_gaps() {
        let gap = r#"{ "answers": ["a"] }"#;
        let more_markers = fill_blank_problems("rust/x/y/f", &fill_blank("___ ___", gap));
        let fewer_markers = fill_blank_problems("rust/x/y/f", &fill_blank("c", gap));

        assert_eq!(more_markers.len(), 1);
        assert!(more_markers[0].contains("the code has 2 `___` and there are 1 gaps"));
        assert_eq!(fewer_markers.len(), 1);
        assert!(fill_blank_problems("rust/x/y/f", &fill_blank("___", gap)).is_empty());
    }

    #[test]
    fn the_fill_blank_check_catches_no_gaps_and_too_many_gaps() {
        let none = fill_blank_problems("rust/x/y/f", &fill_blank("c", ""));
        let gaps = [r#"{ "answers": ["a"] }"#; 7].join(",");
        let seven = fill_blank_problems("rust/x/y/f", &fill_blank(&"___ ".repeat(7), &gaps));

        assert_eq!(none.len(), 1);
        assert!(none[0].contains("needs 1 to 6 gaps and has 0"));
        assert_eq!(seven.len(), 1);
        assert!(seven[0].contains("has 7"));
    }

    #[test]
    fn the_fill_blank_check_catches_a_gap_with_no_answer_a_phrase_and_a_repeat() {
        let check = |gap: &str| fill_blank_problems("rust/x/y/f", &fill_blank("___", gap));

        let none = check(r#"{ "answers": [] }"#);
        let empty = check(r#"{ "answers": [""] }"#);
        let phrase = check(r#"{ "answers": ["a b"] }"#);
        let repeated = check(r#"{ "answers": ["a", "a"] }"#);

        assert_eq!(none.len(), 1);
        assert!(none[0].contains("rust/x/y/f gap 1: the gap has no answers"));
        assert_eq!(empty.len(), 1);
        assert!(empty[0].contains("is not a single word"));
        assert_eq!(phrase.len(), 1);
        assert!(phrase[0].contains("the answer `a b` is not a single word"));
        assert_eq!(repeated.len(), 1);
        assert!(repeated[0].contains("the answer `a` is repeated"));
        assert!(check(r#"{ "answers": ["a", "b"] }"#).is_empty());
    }

    #[test]
    fn the_fill_blank_check_catches_code_that_is_too_tall_or_too_wide_with_the_longest_answer() {
        let gap = |answer: &str| format!(r#"{{ "answers": ["{answer}"] }}"#);
        let tall = fill_blank_problems(
            "rust/x/y/f",
            &fill_blank(&format!("___{}", r"\nx".repeat(14)), &gap("a")),
        );
        let wide = fill_blank_problems(
            "rust/x/y/f",
            &fill_blank(&format!("{}___", "x".repeat(47)), &gap("a")),
        );
        let long_answer = fill_blank_problems(
            "rust/x/y/f",
            &fill_blank(&format!("{}___", "x".repeat(47)), &gap("abcdef")),
        );

        assert_eq!(tall.len(), 1);
        assert!(tall[0].contains("the code has 15 lines"));
        assert!(wide.is_empty()); // 47 columns and the block of 3 make exactly 50
        assert_eq!(long_answer.len(), 1);
        assert!(long_answer[0].contains("a line of 53 columns"));
    }

    #[test]
    fn the_chunk_check_catches_a_line_that_is_not_in_the_file() {
        let first = r#"{ "mode": "ghost", "line": 1, "guide": "G", "text": "a\nb" }"#;
        let second = |line: u32| {
            format!(r#"{{ "mode": "ghost", "line": {line}, "guide": "G", "text": "c" }}"#)
        };
        let check = |chunks: &str| chunk_problems("rust/x/y", &typing("Short.", chunks));

        // the file has 0 lines when the first chunk starts, and 2 when the second does
        let first_too_far = check(&first.replace(r#""line": 1"#, r#""line": 2"#));
        let second_too_far = check(&format!("{first}, {}", second(4)));

        assert_eq!(first_too_far.len(), 1);
        assert!(first_too_far[0].contains("chunk 1: the line 2 is not a place in the file"));
        assert_eq!(second_too_far.len(), 1);
        assert!(second_too_far[0].contains("chunk 2: the line 4"));
        assert!(check(&format!("{first}, {}", second(3))).is_empty());
    }

    #[test]
    fn the_chunk_check_catches_code_after_the_end_of_main() {
        let main = r#"{ "mode": "auto", "line": 1, "text": "func main() {\n}" }"#;
        let body = |line: u32| {
            format!(r#"{{ "mode": "ghost", "line": {line}, "guide": "G", "text": "    x := 1" }}"#)
        };
        let check = |chunks: &str| chunk_problems("go/x/y", &typing("Short.", chunks));

        let after_main = check(&format!("{main}, {}", body(3)));

        assert_eq!(after_main.len(), 1);
        assert!(after_main[0].contains("line 3 of the file is code outside a function"));
        assert!(check(&format!("{main}, {}", body(2))).is_empty());
    }

    #[test]
    fn the_chunk_check_catches_a_ghost_chunk_with_no_guide_and_unbalanced_code() {
        let check = |chunk: &str| chunk_problems("rust/x/y", &typing("Short.", chunk));

        let no_guide = check(r#"{ "mode": "ghost", "line": 1, "text": "x" }"#);
        let unclosed = check(r#"{ "mode": "copy", "line": 1, "text": "fn main() {" }"#);
        let in_string = check(r#"{ "mode": "copy", "line": 1, "text": "s = \"(\"" }"#);

        assert_eq!(no_guide.len(), 1);
        assert!(no_guide[0].contains("a ghost chunk needs a guide"));
        assert_eq!(unclosed.len(), 1);
        assert!(unclosed[0].contains("opens a bracket it does not close"));
        assert!(in_string.is_empty());
    }

    #[test]
    fn the_chunk_check_catches_a_note_anchor_that_does_not_occur_once_and_markdown_in_a_note() {
        let chunk = |on: &str, text: &str| {
            format!(
                r#"{{ "mode": "ghost", "line": 1, "guide": "G", "text": "aa b",
                      "notes": [{{ "on": "{on}", "text": "{text}" }}] }}"#
            )
        };
        let check = |chunk: &str| chunk_problems("rust/x/y", &typing("Short.", chunk));

        let twice = check(&chunk("a", "plain"));
        let never = check(&chunk("z", "plain"));
        let markdown = check(&chunk("b", "uses `b`"));

        assert_eq!(twice.len(), 1);
        assert!(twice[0].contains("occurs 2 times"));
        assert_eq!(never.len(), 1);
        assert!(never[0].contains("occurs 0 times"));
        assert_eq!(markdown.len(), 1);
        assert!(markdown[0].contains("has markdown"));
        assert!(check(&chunk("b", "plain")).is_empty());
    }
}

mod course_groups {
    use super::*;

    #[test]
    fn course_at_counts_the_courses_across_the_groups_of_a_layer() {
        let catalog = Catalog::load(Source::Test, "rust");

        assert_eq!(catalog.course_count(0), 2);
        assert_eq!(catalog.course_count(1), 3);
        assert_eq!(catalog.course_at(0, 1).id, "second");
        assert_eq!(catalog.course_at(1, 1).id, "bare");
        assert_eq!(catalog.course_at(1, 2).id, "extra");
    }

    #[test]
    fn course_index_finds_a_course_in_a_later_group() {
        let catalog = Catalog::load(Source::Test, "rust");

        assert_eq!(catalog.course_index("second"), Some((0, 1)));
        assert_eq!(catalog.course_index("bare"), Some((1, 1)));
        assert_eq!(catalog.course_index("extra"), Some((1, 2)));
        assert_eq!(catalog.course_index("missing"), None);
    }

    #[test]
    fn a_layer_keeps_its_groups_with_their_names_and_courses() {
        let catalog = Catalog::load(Source::Test, "rust");

        let groups: Vec<(&str, Vec<&str>)> = catalog.layers[1]
            .course_groups
            .iter()
            .map(|group| {
                let courses = group.courses.iter().map(|c| c.id.as_str()).collect();
                (group.name.as_str(), courses)
            })
            .collect();

        assert_eq!(
            groups,
            vec![
                ("Tooling", vec!["tools"]),
                ("Libraries", vec!["bare", "extra"])
            ]
        );
        assert_eq!(catalog.layers[0].course_groups[0].name, "Fundamentals");
        assert_eq!(catalog.layers[0].course_groups[0].courses.len(), 2);
    }
}
