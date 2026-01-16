use anyhow::Result;
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use std::{io, time::Duration};

mod battery;
mod ui;

fn main() -> Result<()> {
    let batteries = battery::sysfs::read_batteries()?;

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = ratatui::Terminal::new(backend)?;

    terminal.draw(|f| {
        ui::render::render(f, &batteries);
    })?;

    std::thread::sleep(Duration::from_millis(200));

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
