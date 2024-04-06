use crossterm::{
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{calendar::CalendarEventStore, *},
    style::Stylize,
};
use std::io::{self, stdout};
use time::Date;

// The update loop returns this struct
// It contains data on what to do
struct Command {
    quit: bool,
}
impl Default for Command {
    fn default() -> Self {
        Command { quit: false }
    }
}

pub fn termui() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    stdout().execute(terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut quit = false;
    while !quit {
        terminal.draw(ui)?;
        let cmd = handle_events()?;
        quit = cmd.quit;
    }

    Ok(())
}

fn handle_events() -> io::Result<Command> {
    let mut cmd = Command::default();

    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                cmd.quit = true;
            }
        }
    }

    Ok(cmd)
}

fn ui(frame: &mut Frame) {
    let calendar_date = Date::from_calendar_date(2024, time::Month::April, 1).unwrap();
    let events = CalendarEventStore::default();

    let calendar = calendar::Monthly::new(calendar_date, events)
        .show_month_header(Style::new().bold());

    frame.render_widget(calendar, frame.size());
}
