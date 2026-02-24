use crate::{app::AppState, ui::build_block};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    style::Style,
    widgets::{Gauge, Paragraph, Sparkline},
    Frame,
};

pub fn render(f: &mut Frame, app: &AppState, area: Rect) {
    let block = build_block(" GPU ");
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if let Some(gpus) = &app.gpus {
        if gpus.is_empty() {
            return;
        }

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Name and Temp
                Constraint::Length(2), // Usage Gauge
                Constraint::Length(2), // Mem Gauge
                Constraint::Min(0),    // Sparkline
            ])
            .split(inner_area);

        // Assuming 1st GPU for now
        let gpu = &gpus[0];

        let header = Paragraph::new(format!("{} ({}°C)", gpu.name, gpu.temp_c))
            .style(Style::default().fg(Color::Cyan));
        f.render_widget(header, layout[0]);

        let usage_color = match gpu.usage {
            u if u > 80.0 => Color::Red,
            u if u > 50.0 => Color::Yellow,
            _ => Color::Green,
        };

        let usage_gauge = Gauge::default()
            .gauge_style(Style::default().fg(usage_color))
            .percent((gpu.usage).clamp(0.0, 100.0) as u16)
            .label(format!("Util [{:.1}%]", gpu.usage));
        f.render_widget(usage_gauge, layout[1]);

        let mem_percent = if gpu.mem_total_mb > 0.0 {
            (gpu.mem_used_mb / gpu.mem_total_mb * 100.0) as u16
        } else {
            0
        };

        let mem_gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Magenta))
            .percent(mem_percent.clamp(0, 100))
            .label(format!(
                "VRAM [{:.0}MB / {:.0}MB]",
                gpu.mem_used_mb, gpu.mem_total_mb
            ));
        f.render_widget(mem_gauge, layout[2]);

        let history_data: Vec<u64> = gpu.history.iter().copied().collect();
        let spark = Sparkline::default()
            .data(&history_data)
            .style(Style::default().fg(usage_color));
        f.render_widget(spark, layout[3]);
    } else {
        let p = Paragraph::new("N/A - nvidia-smi not available")
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(p, inner_area);
    }
}
