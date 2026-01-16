use crate::battery::model::Battery;

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
    for bat in batteries {
        render_battery(bat);
        println!();
    } 
}


fn render_battery(bat: &Battery) {
    let width = 40;
    let bar_width = 18;

    let title = format!("{}", bat.name);
    
    println!("┌{}┐", "─".repeat(width + 2));
    println!("│ {} │", pad(&title, width));
    println!("├{}┤", "─".repeat(width + 2));

    let status_health = format!(
        "Status: {:<14} Health: {:>3.0}%",
        bat.status,
        bat.health().unwrap_or(0.0)
    );

    println!("| {} |", pad(&status_health, width));

    let charge = format!(
        "Charge: {} {:>3}%",
        bar(bat.capacity, bar_width),
        bat.capacity
    );
    println!("│ {} │", pad(&charge, width));

    let cycles_power = format!(
        "Cycles: {:<5}          Power: {}",
        bat.cycle_count
            .map(|c| c.to_string())
            .unwrap_or_else(|| "N/A".into()),
        "—"
    );
    println!("│ {} │", pad(&cycles_power, width));

    let energy = format!(
        "Energy: {:.1} / {:.1} Wh",
        bat.energy_full as f64 / 1_000_000.0,
        bat.energy_full_design as f64 / 1_000_000.0,
    );
    println!("│ {} │", pad(&energy, width));

    println!("│ {} │", pad("Time left: —", width));
    println!("└{}┘", "─".repeat(width + 2));
}

