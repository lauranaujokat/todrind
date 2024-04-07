use crossterm::{
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};

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

    terminal.clear()?;
    stdout().execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
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
}
