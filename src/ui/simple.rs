use crate::battery::model::Battery;
use crate::formatter::format_wh;
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
    let default_width = 40;
    let bar_width = 18;

    // Calculate required width to fit name + model
    let name_len = bat.name.chars().count();
    let required_width = if bat.model.is_empty() {
        default_width
    } else {
        let model_label = "Model: ";
        let model_label_len = model_label.chars().count();
        let model_len = bat.model.chars().count();
        let min_spacing = 4; // Minimum spacing between name and model
        let required = name_len + min_spacing + model_label_len + model_len;
        default_width.max(required)
    };
    let width = required_width;

    let mut lines: Vec<String> = Vec::new();

    lines.push(format!("╭{}╮", "─".repeat(width + 2)));
    let name_mode = if bat.model.is_empty() {
        bat.name.clone()
    } else {
        let model_label = format!("Model: {}", bat.model);
        let mode_len = model_label.chars().count();
        let spacing = width.saturating_sub(name_len + mode_len);
        format!("{}{}{}", bat.name, " ".repeat(spacing), model_label)
    };
    lines.push(format!("│ {} │", pad(&name_mode, width)));
    lines.push(format!("├{}┤", "─".repeat(width + 2)));

    let status_health = format!(
        "Status: {:<14} Health: {:>3.0}%",
        bat.status,
        bat.health().unwrap_or(0.0)
    );
    lines.push(format!("│ {} │", pad(&status_health, width)));

    let charge = format!(
        "Charge: {} {:>3}%",
        bar(bat.capacity, bar_width),
        bat.capacity
    );
    lines.push(format!("│ {} │", pad(&charge, width)));

    let manufacturer = format!("Manufacturer: {}", bat.manufacturer);
    lines.push(format!("│ {} │", pad(&manufacturer, width)));

    let cycles_power = format!(
        "Cycles: {:<5}          Power: {}",
        bat.cycle_count
            .map(|c| c.to_string())
            .unwrap_or_else(|| "N/A".into()),
        bat.power()
            .map(|p| format!("{:>6.2} W", p))
            .unwrap_or_else(|| "N/A".into())
    );
    lines.push(format!("│ {} │", pad(&cycles_power, width)));

    let energy = format!(
        "Capacity: {} / {}",
        format_wh(bat.energy_now as f64 / 1_000_000.0),
        format_wh(bat.energy_full as f64 / 1_000_000.0),
    );
    lines.push(format!("│ {} │", pad(&energy, width)));

    let voltage = format!("Voltage: {:.1} V", bat.voltage().unwrap_or(0.0));
    lines.push(format!("│ {} │", pad(&voltage, width)));

    let full_charge_capacity = format!("Full charge capacity: {}", format_wh(bat.energy_full as f64 / 1_000_000.0));
    lines.push(format!("│ {} │", pad(&full_charge_capacity, width)));

    let design_capacity = format!("Design capacity: {}", format_wh(bat.energy_full_design as f64 / 1_000_000.0));
    lines.push(format!("│ {} │", pad(&design_capacity, width)));

    lines.push(format!("╰{}╯", "─".repeat(width + 2)));

    let total_lines = lines.len();
    let nub_height = 5;
    let nub_start = (total_lines - nub_height) / 2 + 1;

    // Draw battery head
    for (i, line) in lines.iter().enumerate() {
        if i == nub_start {
            let trimmed: String = line.chars().take(line.chars().count() - 1).collect();
            writeln!(writer, "{}├──╮", trimmed).unwrap();
        } else if i > nub_start && i < nub_start + nub_height - 1 {
            writeln!(writer, "{}  │", line).unwrap();
        } else if i == nub_start + nub_height - 1 {
            let trimmed: String = line.chars().take(line.chars().count() - 1).collect();
            writeln!(writer, "{}├──╯", trimmed).unwrap();
        } else {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}
