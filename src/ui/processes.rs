use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Row, Table},
    Frame,
};
use crate::{app::{AppState, SortColumn}, ui::build_block};

pub fn render(f: &mut Frame, app: &mut AppState, area: Rect) {
    let block = build_block(" Processes ");
    
    let sort_indicator = |col: SortColumn| -> &str {
        if app.sort_column == col {
            if app.sort_ascending { "▲" } else { "▼" }
        } else {
            ""
        }
    };

    let header_cells = vec![
        format!("PID {}", sort_indicator(SortColumn::Pid)),
        format!("Name {}", sort_indicator(SortColumn::Name)),
        format!("CPU% {}", sort_indicator(SortColumn::Cpu)),
        format!("MEM {}", sort_indicator(SortColumn::Memory)),
        "Status".to_string(),
    ];
    
    let header = Row::new(header_cells)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .bottom_margin(1);

    let rows: Vec<Row> = app.processes.iter().map(|p| {
        let mem_mb = p.memory as f64 / 1_048_576.0;
        
        let mut row_style = Style::default();
        if p.cpu_usage > 50.0 {
            row_style = row_style.fg(Color::Rgb(255, 60, 120)); // neon pink
        } else if mem_mb > 1024.0 {
            row_style = row_style.fg(Color::Yellow);
        }

        Row::new(vec![
            p.pid.to_string(),
            p.name.clone(),
            format!("{:.1}%", p.cpu_usage),
            format!("{:.1} MB", mem_mb),
            p.status.clone(),
        ]).style(row_style)
    }).collect();

    let table = Table::new(rows, [
        Constraint::Percentage(10),
        Constraint::Percentage(40),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(20),
    ])
    .header(header)
    .block(block)
    .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::Cyan).add_modifier(Modifier::BOLD))
    .highlight_symbol(">> ");

    f.render_stateful_widget(table, area, &mut app.process_table_state);
}
