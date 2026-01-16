use std::{fs, path::Path};
use anyhow::Result;
use super::model::Battery;


fn read<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(fs::read_to_string(path)?.trim().to_string())
}

pub fn read_batteries() -> Result<Vec<Battery>> {
    let mut batteries = Vec::new();

    for entry in fs::read_dir("/sys/class/power_supply")? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();

        if !name.starts_with("BAT") {
            continue;
        }

        let base = entry.path();

        let capacity = read(base.join("capacity"))?.parse()?;
        let status = read(base.join("status"))?;
        let energy_full = read(base.join("energy_full"))?.parse().unwrap_or(0);
        let energy_full_design = read(base.join("energy_full_design"))?.parse().unwrap_or(0);

        let cycle_count = base.join("cycle_count")
            .exists()
            .then(|| read(base.join("cycle_count")).ok()?.parse().ok())
            .flatten();

        batteries.push(Battery {
            name,
            capacity,
            status,
            cycle_count,
            energy_full,
            energy_full_design
        })
    }

    Ok(batteries)
}

