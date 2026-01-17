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

        let status = read(base.join("status"))?;
        let capacity = read(base.join("capacity"))?.parse()?;
        let manufacturer = read(base.join("manufacturer"))?;
        let energy_full = read(base.join("energy_full"))?.parse().unwrap_or(0);
        let energy_full_design = read(base.join("energy_full_design"))?.parse().unwrap_or(0);
        let voltage_now = read(base.join("voltage_now"))?.parse().unwrap_or(0);

        let power_now = if base.join("power_now").exists() {
            read(base.join("power_now"))
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(0)
        } else {
            read(base.join("current_now"))
                .ok()
                .and_then(|c| c.parse::<u64>().ok())
                .map(|current| current * voltage_now / 1_000_000)
                .unwrap_or(0)
        };

        let cycle_count = base.join("cycle_count")
            .exists()
            .then(|| read(base.join("cycle_count")).ok()?.parse().ok())
            .flatten();

        batteries.push(Battery {
            name,
            manufacturer,
            capacity,
            status,
            cycle_count,
            energy_full,
            energy_full_design,
            voltage_now,
            power_now,
        })
    }

    batteries.sort_by_key(|b| b.name.clone());

    Ok(batteries)
}

