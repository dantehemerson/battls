use anyhow::Result;

mod battery;
mod ui;

fn main() -> Result<()> {
    let batteries = battery::sysfs::read_batteries()?;

    let backend = ratatui::backend::CrosstermBackend::new(std::io::stdout());
    let mut terminal = ratatui::Terminal::new(backend)?;

    terminal.draw(|f| {
        ui::render::render(f, &batteries);
    })?;

    Ok(())
}
