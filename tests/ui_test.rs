use battery::battery::model::Battery;
use battery::ui::simple::render_to;
use insta::assert_snapshot;

#[test]
fn test_render_battery_snapshot() {
    let battery = Battery {
        name: "BAT0".to_string(),
        manufacturer: "Dante's Co".to_string(),
        capacity: 75,
        status: "Discharging".to_string(),
        cycle_count: Some(100),
        energy_full: 4000000,
        energy_full_design: 5000000,
        voltage_now: 12000000,
        power_now: 15000000, // 15W in microwatts
    };

    let mut output = Vec::new();
    render_to(&[battery], &mut output);

    let output_str = String::from_utf8(output).unwrap();
    assert_snapshot!(output_str);
}

