use battery::battery::model::Battery;
use battery::ui::simple::render_to;
use insta::assert_snapshot;

#[test]
fn test_render_battery_snapshot() {
    let battery = Battery {
        name: "BAT0".to_string(),
        manufacturer: "Dante's Co".to_string(),
        model: "BAT-12345".to_string(),
        capacity: 75,
        status: "Discharging".to_string(),
        cycle_count: Some(100),
        energy_now: 30000000,
        energy_full: 40000000,
        energy_full_design: 5000000,
        voltage_now: 12000000,
        power_now: 15000000, // 15W in microwatts
    };

    let mut output = Vec::new();
    render_to(&[battery], &mut output);

    let output_str = String::from_utf8(output).unwrap();
    assert_snapshot!(output_str);
}

#[test]
fn test_render_battery_with_long_model() {
    let battery = Battery {
        name: "BAT0".to_string(),
        manufacturer: "Dante's Co".to_string(),
        model: "This is a very long model name that should be truncated with ellipsis".to_string(),
        capacity: 50,
        status: "Charging".to_string(),
        cycle_count: Some(50),
        energy_now: 2000000,
        energy_full: 4000000,
        energy_full_design: 5000000,
        voltage_now: 12000000,
        power_now: 10000000,
    };

    let mut output = Vec::new();
    render_to(&[battery], &mut output);

    let output_str = String::from_utf8(output).unwrap();
    // Verify the full model name is shown (no truncation)
    let name_line = output_str.lines()
        .find(|line| line.contains("BAT0") && line.contains("Model:"))
        .expect("Should find name line with Model");
    assert!(!name_line.contains("..."), "Long model should NOT be truncated");
    // Verify the full model name is present
    assert!(name_line.contains("This is a very long model name that should be truncated with ellipsis"),
            "Full model name should be shown: {}", name_line);
    // Verify the width expanded to fit the full model name
    // The name line should be: │ {width chars} │ where width >= required width
    let expected_min_width = 4 + 2 + 7 + 62; // "BAT0" + spacing + "Model: " + model length
    assert!(name_line.chars().count() >= expected_min_width + 4, // +4 for borders and padding
            "Name line should expand to fit full model: {}", name_line);
}

