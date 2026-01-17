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
    let default_width = 48;
    let bar_width = 18;

    let name_len = bat.name.chars().count();
    let required_width = if bat.model.is_empty() {
        default_width
    } else {
        let model_label = "Model: ";
        let model_label_len = model_label.chars().count();
        let model_len = bat.model.chars().count();
        let min_spacing = 4;
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

    let status = format!("Status: {}", bat.status);
    lines.push(format!("│ {} │", pad(&status, width)));

    let charge = format!(
        "Charge: {} {:>3}%",
        bar(bat.capacity, bar_width),
        bat.capacity
    );
    lines.push(format!("│ {} │", pad(&charge, width)));

    let manufacturer = format!("Manufacturer: {}", bat.manufacturer);
    lines.push(format!("│ {} │", pad(&manufacturer, width)));

    let cycles = format!(
        "Cycles: {}",
        bat.cycle_count
            .map(|c| c.to_string())
            .unwrap_or_else(|| "N/A".into())
    );
    lines.push(format!("│ {} │", pad(&cycles, width)));

    let voltage_str = format!("Voltage: {:.1} V", bat.voltage().unwrap_or(0.0));
    let power_str = format!("{:>6.2} W", bat.power().unwrap_or(0.0));
    let voltage_power = format!("{:<32}Power: {}", voltage_str, power_str);
    lines.push(format!("│ {} │", pad(&voltage_power, width)));

    let capacity_str = format!(
        "Capacity: {} / {}",
        format_wh(bat.energy_now as f64 / 1_000_000.0),
        format_wh(bat.energy_full as f64 / 1_000_000.0)
    );
    let health_str = format!("{:>4.0}%", bat.health().unwrap_or(0.0));
    let capacity_health = format!("{:<32}Health: {}", capacity_str, health_str);
    lines.push(format!("│ {} │", pad(&capacity_health, width)));

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
