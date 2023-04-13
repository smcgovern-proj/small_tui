use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use chrono;
use std::{io, thread, time::{Duration, SystemTime}};
use tui::{
    backend::{CrosstermBackend, Backend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
    Frame,
};

// Dummy App for sample data

struct App<'a> {
    data: Vec<(&'a str, i32)>
}
impl<'a> App<'a> {
    fn new() -> App<'a> {
        let mut data: Vec<(&String, i32)> = vec![];
        for n in 0..10 {
            let time = SystemTime::now().into();
            data.push((time, n));
        }
        App {
            data
        }
    }
}
fn main() -> Result<(), io::Error> {
    //setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(ui)?;

    thread::sleep(Duration::from_millis(5000));

    //restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;


    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint:: Percentage(10),
            Constraint:: Percentage(80),
            Constraint:: Percentage(10),
        ])
        .split(f.size());

    let block = Block::default()
        .title("Block 1")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let block2 = Block::default()
        .title("Block 2")
        .borders(Borders::ALL);
    f.render_widget(block2, chunks[1]);

    let block3 = Block::default()
        .title("final block")
        .borders(Borders::ALL);
    f.render_widget(block3, chunks[2]);

}

