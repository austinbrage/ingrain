use ingrain::{app::App, dev, ui};

use std::time::Duration;

use ratatui::crossterm::{
    cursor::SetCursorStyle,
    event::{self, Event},
    execute,
};

// How often the animated screens redraw without a key
const TICK: Duration = Duration::from_millis(50);

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    // Ask for a blinking block cursor
    execute!(std::io::stdout(), SetCursorStyle::BlinkingBlock)?;

    let mut app = App::new();

    // Render screens on demand for DEV args
    let mut args = std::env::args().skip(1);
    if let Some(screen) = args.next() {
        dev::start_at(&mut app, &screen, args.next().as_deref());
    }

    while !app.quit {
        // Draw the TUI
        terminal.draw(|frame| ui::draw(frame, &app))?;

        // Redraw on a tick when no key arrives, only on animated screens
        if app.animating() && !event::poll(TICK)? {
            app.tick();
            continue;
        }

        // Handle Keys
        if let Event::Key(key) = event::read()? {
            app.handle_key(key);
        }
    }

    // Back to the user's cursor style
    execute!(std::io::stdout(), SetCursorStyle::DefaultUserShape)?;
    ratatui::restore();
    Ok(())
}
