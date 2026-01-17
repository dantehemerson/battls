fn format_wh(value: f64) -> String {
    let s = format!("{:.2}", value);
    let s = s.trim_end_matches('0').trim_end_matches('.');
    format!("{} Wh", s)
}
