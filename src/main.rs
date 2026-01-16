use anyhow::Result;

use battery::sysfs::read_batteries;

mod battery;
mod ui;

fn main() -> Result<()> {
    let batteries = read_batteries()?;

    ui::simple::render(&batteries);

    Ok(())
}
