use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Gauge, Sparkline},
    Frame,
};
use crate::{app::AppState, ui::build_block};

pub fn render(f: &mut Frame, app: &AppState, area: Rect) {
    let block = build_block(" CPU ");
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Overall CPU
            Constraint::Length(1), // Spacer
            Constraint::Min(0),    // Cores
            Constraint::Length(2), // Overall Sparkline
        ])
        .split(inner_area);

    let overall_color = get_color(app.overall_cpu);
    let overall_gauge = Gauge::default()
        .gauge_style(Style::default().fg(overall_color).add_modifier(Modifier::BOLD))
        .percent((app.overall_cpu).clamp(0.0, 100.0) as u16)
        .label(format!("Overall CPU [{:.1}%]", app.overall_cpu));
    f.render_widget(overall_gauge, layout[0]);

    if app.cpu_history.is_empty() { return; }

    let core_constraints = vec![Constraint::Length(1); app.cpu_history.len()];
    let cores_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(core_constraints)
        .split(layout[2]);

    for (i, core) in app.cpu_history.iter().enumerate() {
        if i >= cores_layout.len() { break; }
        let color = get_color(core.usage);
        let g = Gauge::default()
            .gauge_style(Style::default().fg(color))
            .percent((core.usage).clamp(0.0, 100.0) as u16)
            .label(format!("{} [{:.1}%]", core.core_name, core.usage));
        f.render_widget(g, cores_layout[i]);
    }
    
    // overall sparkline mapping first core's history as rough average, to avoid creating new state var
    if let Some(first_core) = app.cpu_history.first() {
        let history_data: Vec<u64> = first_core.history.iter().copied().collect();
        let spark = Sparkline::default()
            .data(&history_data)
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(spark, layout[3]);
    }
}

fn get_color(usage: f32) -> Color {
    if usage > 80.0 {
        Color::Red
    } else if usage > 50.0 {
        Color::Yellow
    } else {
        Color::Green
    }
}
