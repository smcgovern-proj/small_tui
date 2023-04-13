use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::{Duration, Instant}};
use tui::{
    backend::{CrosstermBackend, Backend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{BarChart, Block, Borders, Widget},
    Terminal,
    Frame,
};
use rand::prelude::*;

// Dummy App for sample data
#[derive(Debug)]
struct App<'a> {
    data: Vec<(&'a str, u64)>
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        let mut data: Vec<(&str, u64)> = vec![];
        for _ in 0..10 {
            let mut range = thread_rng();
            let random_num = range.gen_range(1..20);
            data.push(("service", random_num));
        }
        App {
            data
        }
    }

    fn on_tick(&mut self) {
        let value = self.data.pop().unwrap();
        self.data.insert(0, value);
    }
}

fn main() -> Result<(), io::Error> {
    //setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // setup app
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);
    if let Err(err) = res {
        println!("{:?}", err);
    };

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

//runner
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // chunk the display
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint:: Percentage(50),
            Constraint:: Percentage(50),
        ])
        .split(f.size());
    
    let barchart = BarChart::default()
        .block(Block::default().title("Sample Latencies").borders(Borders::ALL))
        .data(&app.data)
        .bar_width(9)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black));

    f.render_widget(barchart, chunks[0]);

}

