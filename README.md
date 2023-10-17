# GNOME Auto Dark Mode Switcher

This Rust utility automatically switches between light and dark GNOME themes based on specified time spans utilizing `gsettings` command-line tool.

[![Build Status](https://github.com/bkobel/gnome-auto-dark/workflows/Rust%20CI/badge.svg)](https://github.com/bkobel/gnome-auto-dark/actions/workflows/rust_ci.yml)

## Features
- Automatically switches between predefined light and dark themes.
- User-configurable switch times via a YAML configuration file.
- Periodically checks the current time to determine the theme to be set.

## Dependencies
- `chrono`: For working with time.
- `serde`: Serialization and deserialization library.
- `serde_yaml`: For parsing and serializing YAML.
- `serde_derive`: Macros for automatically deriving `Serialize` and `Deserialize` traits.

## Configuration

The configuration file is located at `~/.gnome-ad-config.yaml`.

Example configuration:
```yaml
schedule_light_mode: "07:00"
schedule_dark_mode: "19:00"
cycle_rate: 600
```

* `schedule_light_mode`: The time to switch to the light theme.
* `schedule_dark_mode`: The time to switch to the dark theme.
* `cycle_rate`: How often (in seconds) the program checks the current time and updates the theme.


## Usage
```bash
cargo build --release && ./target/release/gnome_auto_dark
```
Simply run the compiled binary. The tool will check the time against the specified schedule in the configuration file and set the GNOME theme accordingly.

## Testing
Tests are included to ensure that the theme switcher works correctly under various scenarios.

To run the tests, use:
```bash
cargo test
```

## Contributing
Please feel free to submit issues or pull requests if you have suggestions or improvements