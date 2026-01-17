# Battery Monitor

A simple Rust CLI tool that displays battery information on Linux.

## How It Works

1. Reads battery info from Linux sysfs (`/sys/class/power_supply/BAT*`)
2. Parses status, capacity, manufacturer, cycle count, etc.
3. Renders a box-style UI to the terminal

## Build & Run

```bash
cargo build
cargo run
```

## Test

```bash
cargo test
```

## Dependencies

- `anyhow` - Error handling
