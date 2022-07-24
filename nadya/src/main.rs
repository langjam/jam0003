use std::{
    fs, io,
    time::{Duration, Instant},
};

use clap::Parser;
use crossterm::{
    event,
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use prelude::*;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

mod lexer;
mod parser;
mod place;
mod program;
mod simulation;
mod syntax;

mod prelude {
    pub use crate::{lexer::*, parser::*, place::*, program::*, simulation::*, syntax::*};
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Which example to use
    #[clap(short, long, value_parser, required = true)]
    example: String,
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    let folder = args.example;

    // Parse the language
    // Load the file in the folder provided
    let contents = fs::read_to_string(format!("examples/{}/program.nya", folder))
        .expect(&format!("Could not find an example called '{}'!", folder));

    // Parse the file
    let mut program = parse(&contents, folder);

    // Lex the file
    lexer(&mut program);

    setup_terminal(program)?;

    Ok(())
}

fn setup_terminal(program: Program) -> Result<(), io::Error> {
    // Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set up the app
    let tick_rate = Duration::from_millis(250);
    let app = App::new(program);
    let _res = run_app(&mut terminal, app, tick_rate);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Exit the program
    Ok(())
}

struct App {
    simulation: Simulation,
}

impl App {
    fn new(program: Program) -> App {
        App {
            simulation: Simulation::new(program),
        }
    }

    fn on_tick(&mut self) {
        // Run a simulation tick
        self.simulation.simulate();
    }
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.size());

    let (map_string, x, y) = app.simulation.map_string();

    let paragraph = Paragraph::new(map_string)
        .style(Style::default().bg(Color::White).fg(Color::Black))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::White).fg(Color::Black))
                .title(Span::styled(
                    "Execution",
                    Style::default().add_modifier(Modifier::BOLD),
                )),
        )
        .alignment(Alignment::Center);

    f.render_widget(paragraph, chunks[0]);

    let events: Vec<ListItem> = app
        .simulation
        .outputs
        .iter()
        .rev()
        .map(|output| ListItem::new(vec![Spans::from(output.clone())]))
        .collect();

    let events_list = List::new(events)
        .block(Block::default().borders(Borders::ALL).title("Outputs"))
        .start_corner(Corner::TopLeft);

    f.render_widget(events_list, chunks[1]);
}
