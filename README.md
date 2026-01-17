# battls 🔋
[![Latest Release](https://img.shields.io/github/release/dantehemerson/battls.svg?style=for-the-badge)](https://github.com/dantehemerson/battls/releases)
[![Build Status](https://img.shields.io/github/actions/workflow/status/dantehemerson/battls/test.yml?style=for-the-badge&branch=master)](https://github.com/dantehemerson/battls/actions)
[![Crates.io](https://img.shields.io/crates/v/battls.svg?style=for-the-badge)](https://crates.io/crates/battls)
[![License](https://img.shields.io/badge/license-GPLv3-blue.svg?style=for-the-badge)](/LICENSE)

A simple CLI utility that lists your battery information.

<p align="center">
  <img src="https://raw.githubusercontent.com/dantehemerson/files/refs/heads/master/screenshot-2026-01-17_17-22-09%20(1).png" alt="battls screenshot">
</p>

## Features ✨

- 📦 Beautiful box-style terminal UI
- 📊 Battery info clearly formatted for quick reading
- 🔋 Multiple battery support (e.g., ThinkPad T480 with dual batteries)
- 🦀 Built with Rust for better performance

## Requirements

- Linux system with battery support
- Rust (latest stable version)

## Installation

### Packages

#### Linux
- Arch Linux: `yay -S battls`
- [Packages](https://github.com/dantehemerson/battls/releases) in Alpine, Debian & RPM formats

### Binaries
- [Binaries](https://github.com/dantehemerson/battls/releases) for Linux, FreeBSD, OpenBSD

### From crates.io

If you have Rust installed, you can install battls directly from [crates.io](https://crates.io/crates/battls):

```bash
cargo install battls
```

### From source

Make sure you have a working Rust environment (Rust 1.70 or higher is recommended).
See the [install instructions](https://www.rust-lang.org/tools/install).

Compiling battls is easy, simply run:
```bash
git clone https://github.com/dantehemerson/battls.git
cd battls
cargo build --release
```

## Usage

Simply run the executable:

```bash
battls
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

## Feedback

Got some feedback or suggestions? Please open an issue or drop me a note!

* [Twitter](https://twitter.com/dantehemerson)
