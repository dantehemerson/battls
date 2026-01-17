use crate::battery::model::Battery;
use std::io::{self, Write};

fn bar(percent: u8, width: usize) -> String {
    let filled = percent as usize * width / 100;

    format!(
        "{}{}",
        "█".repeat(filled),
        "░".repeat(width - filled),
    )
}

fn pad(s: &str, width: usize) -> String {
    let len = s.chars().count();
    if len >= width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(width - len))
    }
}

pub fn render(batteries: &[Battery]) {
    render_to(batteries, &mut io::stdout());
}

pub fn render_to(batteries: &[Battery], writer: &mut dyn Write) {
    for bat in batteries {
        render_battery_to(bat, writer);
        writeln!(writer).unwrap();
    }
}

fn render_battery_to(bat: &Battery, writer: &mut dyn Write) {
    let width = 40;
    let bar_width = 18;

    let title = format!("{}", bat.name);

    writeln!(writer, "┌{}┐", "─".repeat(width + 2)).unwrap();
    writeln!(writer, "│ {} │", pad(&title, width)).unwrap();
    writeln!(writer, "├{}┤", "─".repeat(width + 2)).unwrap();

    let status_health = format!(
        "Status: {:<14} Health: {:>3.0}%",
        bat.status,
        bat.health().unwrap_or(0.0)
    );
    writeln!(writer, "│ {} │", pad(&status_health, width)).unwrap();


    let manufacturer = format!(
        "Manufacturer: {}",
        bat.manufacturer,
    );
    writeln!(writer, "│ {} │", pad(&manufacturer, width)).unwrap();

    let charge = format!(
        "Charge: {} {:>3}%",
        bar(bat.capacity, bar_width),
        bat.capacity
    );
    writeln!(writer, "│ {} │", pad(&charge, width)).unwrap();

    let cycles_power = format!(
        "Cycles: {:<5}          Power: {}",
        bat.cycle_count
            .map(|c| c.to_string())
            .unwrap_or_else(|| "N/A".into()),
        bat.power()
            .map(|p| format!("{:>6.2} W", p))
            .unwrap_or_else(|| "N/A".into())
    );
    writeln!(writer, "│ {} │", pad(&cycles_power, width)).unwrap();

    let energy = format!(
        "Energy: {:.1} / {:.1} Wh",
        bat.energy_full as f64 / 1_000_000.0,
        bat.energy_full_design as f64 / 1_000_000.0,
    );
    writeln!(writer, "│ {} │", pad(&energy, width)).unwrap();

    let voltage = format!(
        "Voltage: {:.1} V",
        bat.voltage().unwrap_or(0.0)
    );
    writeln!(writer, "│ {} │", pad(&voltage, width)).unwrap();

    writeln!(writer, "└{}┘", "─".repeat(width + 2)).unwrap();
}
