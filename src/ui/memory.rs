use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    style::Style,
    widgets::{Gauge, Row, Table},
    Frame,
};
use crate::{app::AppState, ui::build_block};

pub fn render(f: &mut Frame, app: &AppState, area: Rect) {
    let block = build_block(" Memory ");
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // RAM Gauge
            Constraint::Length(2), // Swap Gauge
            Constraint::Min(0),    // Table
        ])
        .split(inner_area);

    let ram_used = app.system.used_memory() as f64 / 1_073_741_824.0;
    let ram_total = app.system.total_memory() as f64 / 1_073_741_824.0;
    let ram_percent = if ram_total > 0.0 { (ram_used / ram_total * 100.0) as u16 } else { 0 };

    let ram_gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(ram_percent.clamp(0, 100))
        .label(format!("RAM [{:.1}GB / {:.1}GB] ({}%)", ram_used, ram_total, ram_percent));
    f.render_widget(ram_gauge, layout[0]);

    let swap_used = app.system.used_swap() as f64 / 1_073_741_824.0;
    let swap_total = app.system.total_swap() as f64 / 1_073_741_824.0;
    let swap_percent = if swap_total > 0.0 { (swap_used / swap_total * 100.0) as u16 } else { 0 };

    let swap_gauge = Gauge::default()
        .gauge_style(Style::default().fg(Color::Magenta))
        .percent(swap_percent.clamp(0, 100))
        .label(format!("SWAP [{:.1}GB / {:.1}GB] ({}%)", swap_used, swap_total, swap_percent));
    f.render_widget(swap_gauge, layout[1]);

    let rows = vec![
        Row::new(vec!["Type", "Used", "Total", "Free"]).style(Style::default().fg(Color::DarkGray)),
        Row::new(vec![
            "RAM".to_string(),
            format!("{:.1} GB", ram_used),
            format!("{:.1} GB", ram_total),
            format!("{:.1} GB", app.system.available_memory() as f64 / 1_073_741_824.0),
        ]),
        Row::new(vec![
            "SWAP".to_string(),
            format!("{:.1} GB", swap_used),
            format!("{:.1} GB", swap_total),
            format!("{:.1} GB", app.system.free_swap() as f64 / 1_073_741_824.0),
        ]),
    ];

    let table = Table::new(rows, [Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25)]);
    f.render_widget(table, layout[2]);
}
