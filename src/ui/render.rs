use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
};
use crate::battery::model::Battery;

pub fn render(frame: &mut Frame, batteries: &[Battery]) {
    let chunks = Layout::vertical(vec![Constraint::Length(7); batteries.len()])
    .split(frame.area());

    for (bat, area) in batteries.iter().zip(chunks.iter()) {
        let health = bat.health().unwrap_or(0.0);

        let block = Block::default()
            .title(format!(" {} ", bat.name))
            .borders(Borders::ALL);

        let gauge = Gauge::default()
            .label(format!("{}%", bat.capacity))
            .ratio(bat.capacity as f64 / 100.0)
            .block(block);

        frame.render_widget(gauge, *area);

        let text = format!(
            "Status: {}\nCycles: {}\nHealth: {:.1}%",
            bat.status,
            bat.cycle_count.map(|c| c.to_string()).unwrap_or("N/A".into()),
            health
        );

        frame.render_widget(
            Paragraph::new(text).wrap(Wrap { trim: true }),
            area.inner(Margin { vertical: 1, horizontal: 2 }),
        );
    }
}
