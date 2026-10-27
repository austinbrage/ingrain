pub mod complete;
pub mod exercise;
pub mod explorer;
pub mod home;
mod wave;

use crate::app::{App, HomeState, Screen};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::Home(state) => {
            let continue_label = HomeState::continue_label(&app.progress, &app.content);
            let language = state.language(HomeState::last_language(&app.progress));

            home::home(frame, state, app.tick, continue_label.as_ref(), language)
        }
        Screen::Explorer(state) => {
            let catalog = app.content.catalog(state.language);
            let course = app.content.course(state);
            let marks = state.module_marks(app);
            let breadcrumb = state.explorer_breadcrumb(app);

            explorer::explorer(frame, catalog, state, course, &marks, breadcrumb, app.tick)
        }
        Screen::Exercise(ref state) => {
            exercise::exercise(frame, app.content.breadcrumb(state.explorer), state)
        }
        Screen::Complete(ref state) => complete::complete(frame, &app.content, state, app.tick),
    }
}
