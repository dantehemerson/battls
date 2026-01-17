# battls 🔋

A simple CLI utility that lists your battery information.

## Features ✨

- 📦 Beautiful box-style terminal UI
- 📊 Battery info clearly formatted for quick reading
- 🔋 Multiple battery support (e.g., ThinkPad T480 with dual batteries)
- 🦀 Built with Rust for better performance

## Requirements

- Linux system with battery support
- Rust (latest stable version)

## Installation

```bash
# Build the project
cargo build --release

# Run it
cargo run
```

## Usage

Simply run the executable:

```bash
cargo run
```

## How It Works

1. Reads battery data from Linux sysfs (`/sys/class/power_supply/BAT*`)
2. Parses status, capacity, manufacturer, cycle count, and other metrics
3. Renders a formatted box-style UI to the terminal

## Testing

Run the test suite:

```bash
cargo test
```

## License

- [GPL v3.0](./LICENSE)

