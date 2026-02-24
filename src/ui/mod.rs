pub mod cpu;
pub mod memory;
pub mod network;
pub mod processes;

use crate::app::AppState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(10), // top row (CPU + Memory)
            Constraint::Length(10), // middle (Network)
            Constraint::Min(10),    // bottom (Processes)
            Constraint::Length(3),  // Footer
        ])
        .split(f.size());

    render_header(f, chunks[0]);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    cpu::render(f, app, top_chunks[0]);
    memory::render(f, app, top_chunks[1]);

    network::render(f, app, chunks[2]);
    processes::render(f, app, chunks[3]);

    render_footer(f, app, chunks[4]);
}

pub fn build_block(title: &str) -> Block<'static> {
    Block::default()
        .title(title.to_string())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .border_type(BorderType::Rounded)
}

fn render_header(f: &mut Frame, area: Rect) {
    let host = sysinfo::System::host_name().unwrap_or_else(|| "unknown".to_string());
    let title = format!(" ◈ NEXMON v{} │ host: {} ", env!("CARGO_PKG_VERSION"), host);
    let p = Paragraph::new(title)
        .style(
            Style::default()
                .fg(Color::Rgb(0, 255, 180))
                .add_modifier(Modifier::BOLD),
        )
        .block(build_block(""));
    f.render_widget(p, area);
}

fn render_footer(f: &mut Frame, app: &AppState, area: Rect) {
    let text = if app.search_mode {
        format!("Search: {}_ (Press Enter to exit search)", app.search_query)
    } else {
        "[Q]uit  [J/K/↑/↓]scroll  [C]pu  [M]em  [P]id  [N]ame  [/]search  [R]everse".to_string()
    };

    let p = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .block(build_block(""));
    f.render_widget(p, area);
}
