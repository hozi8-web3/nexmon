use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Sparkline},
    Frame,
};
use crate::{app::AppState, ui::build_block};

pub fn render(f: &mut Frame, app: &AppState, area: Rect) {
    let block = build_block(" Network ");
    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if app.network_history.is_empty() { return; }

    let rows_count = app.network_history.len() as u16;
    let constraints: Vec<_> = (0..rows_count).map(|_| Constraint::Length(4)).collect();
    
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner_area);

    for (i, net) in app.network_history.iter().enumerate() {
        if i >= layout.len() { break; }
        let chunk = layout[i];
        
        let sub_chunks = Layout::default()
             .direction(Direction::Vertical)
             .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Length(1), Constraint::Length(1)])
             .split(chunk);

        let rx_mb = net.rx_bytes as f64 / 1_048_576.0;
        let tx_mb = net.tx_bytes as f64 / 1_048_576.0;
        
        let header = Paragraph::new(format!("↓ {}  RX: {:.2} MB/s  TX: {:.2} MB/s", net.interface_name, rx_mb, tx_mb));
        f.render_widget(header, sub_chunks[0]);

        let rx_data: Vec<u64> = net.rx_history.iter().copied().collect();
        let rx_spark = Sparkline::default()
            .data(&rx_data)
            .style(Style::default().fg(Color::Green));
        f.render_widget(rx_spark, sub_chunks[1]);

        let tx_data: Vec<u64> = net.tx_history.iter().copied().collect();
        let tx_spark = Sparkline::default()
            .data(&tx_data)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(tx_spark, sub_chunks[2]);
    }
}
