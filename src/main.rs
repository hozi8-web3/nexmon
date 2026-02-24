pub mod app;
pub mod system;
pub mod ui;

use app::AppState;
use clap::Parser;
use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::{Duration, Instant}};

#[derive(Parser)]
#[command(name = "nexmon", about = "Futuristic system monitor", version)]
struct Args {
    #[arg(short, long, default_value_t = 500)]
    interval: u64,
    #[arg(long)]
    show_loopback: bool,
    #[arg(short, long, default_value_t = 100)]
    processes: usize,
    #[arg(short, long, default_value = "cpu")]
    sort: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new(args.interval, args.show_loopback, args.processes, args.sort);

    let tick_rate = Duration::from_millis(args.interval);
    let mut last_tick = Instant::now();

    let res = run_app(&mut terminal, &mut app, tick_rate, &mut last_tick);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
         println!("{:?}", err);
    }
    
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut AppState,
    tick_rate: Duration,
    last_tick: &mut Instant,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if app.search_mode {
                    match key.code {
                        KeyCode::Enter => app.search_mode = false,
                        KeyCode::Esc => {
                            app.search_mode = false;
                            app.search_query.clear();
                        }
                        KeyCode::Backspace => {
                            app.search_query.pop();
                        }
                        KeyCode::Char(c) => {
                            app.search_query.push(c);
                        }
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                        KeyCode::Char('c') => {
                            if key.modifiers.contains(KeyModifiers::CONTROL) {
                                return Ok(());
                            }
                        }
                        KeyCode::Char('j') | KeyCode::Down => app.next_process(),
                        KeyCode::Char('k') | KeyCode::Up => app.previous_process(),
                        KeyCode::Char('C') | KeyCode::Char('c') if !key.modifiers.contains(KeyModifiers::CONTROL) => app.sort_column = app::SortColumn::Cpu,
                        KeyCode::Char('M') | KeyCode::Char('m') => app.sort_column = app::SortColumn::Memory,
                        KeyCode::Char('P') | KeyCode::Char('p') => app.sort_column = app::SortColumn::Pid,
                        KeyCode::Char('N') | KeyCode::Char('n') => app.sort_column = app::SortColumn::Name,
                        KeyCode::Char('R') | KeyCode::Char('r') => app.sort_ascending = !app.sort_ascending,
                        KeyCode::Char('/') => app.search_mode = true,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            system::collector::refresh(app);
            *last_tick = Instant::now();
        }
    }
}
