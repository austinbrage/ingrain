use super::Progress;
use crate::content::{Course, Library, Module};

// Every course of a language: its section, its position and its id, in catalog order
fn course_list(library: &Library) -> Vec<(usize, usize, &str)> {
    library
        .catalog
        .sections
        .iter()
        .enumerate()
        .flat_map(|(section, data)| {
            data.courses
                .iter()
                .enumerate()
                .map(move |(course, summary)| (section, course, summary.id.as_str()))
        })
        .collect()
}

// The first course with a module left to study, starting at the one practiced last:
// its section, course and module. Then the courses after it, wrapping to the start
pub fn next_course(
    progress: &Progress,
    language: &str,
    library: &Library,
    last_course: &str,
) -> Option<(usize, usize, usize)> {
    let courses = course_list(library);
    let start = courses.iter().position(|&(_, _, id)| id == last_course)?;

    courses[start..]
        .iter()
        .chain(&courses[..start])
        .find_map(|&(section, course, id)| {
            let module = progress.next_module(language, id, library.courses.get(id)?)?;
            Some((section, course, module))
        })
}

// The written module practiced longest ago in the whole language, for the review:
// its section, course and module. On a tie, the first in catalog order
pub fn next_review(
    progress: &Progress,
    language: &str,
    library: &Library,
) -> Option<(usize, usize, usize)> {
    let mut oldest = None;

    for (section, course, id) in course_list(library) {
        let Some(course_data) = library.courses.get(id) else {
            continue;
        };
        let modules = course_data.module_sections.iter().flat_map(|s| &s.modules);

        for (module, summary) in modules.enumerate() {
            if !summary.written {
                continue;
            }

            let practiced = progress.practiced(&format!("{language}/{id}/{}", summary.id));

            if oldest.is_none_or(|(best, _)| practiced < best) {
                oldest = Some((practiced, (section, course, module)));
            }
        }
    }

    oldest.map(|(_, position)| position)
}

// The index of the module to study next in a course, None when all are complete
pub fn next_module(
    progress: &Progress,
    language: &str,
    course_id: &str,
    course: &Course,
) -> Option<usize> {
    let prefix = format!("{language}/{course_id}/");
    let is_complete = |index: usize| {
        course.module_at(index).is_some_and(|module| {
            progress.done_count(&format!("{prefix}{}", module.id)) == module.sessions as usize
        })
    };

    // The list ends with the module worked on last, so search it backwards
    let last_complete = progress.modules.iter().rev().find_map(|entry| {
        let index = course.module_index(entry.id.strip_prefix(&prefix)?)?;
        is_complete(index).then_some(index)
    });

    // The first incomplete module after it, then wrapping around to the start.
    // Modules that are not written yet are skipped
    let is_written = |index: usize| course.module_at(index).is_some_and(|module| module.written);
    let start = last_complete.map_or(0, |index| index + 1);
    (start..course.module_count())
        .chain(0..start)
        .find(|&index| !is_complete(index) && is_written(index))
}

// The first session of a module that is not done, the first one if all are
pub fn next_session(progress: &Progress, module_path: &str, module: &Module) -> usize {
    module
        .sessions
        .iter()
        .position(|session| !progress.is_session_done(module_path, &session.id))
        .unwrap_or(0)
}

// The exercise saved for a session, the first one if there is none
pub fn next_exercise(
    progress: &Progress,
    module_path: &str,
    module: &Module,
    session: usize,
) -> usize {
    let session = &module.sessions[session];

    progress
        .in_progress_exercise(module_path, &session.id)
        .and_then(|saved| session.exercise_index(saved))
        .unwrap_or(0)
}
