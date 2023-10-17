extern crate chrono;
extern crate serde;
extern crate serde_yaml;
#[macro_use]
extern crate serde_derive;

use chrono::{Local, NaiveTime};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

const CONFIG_PATH: &str = "~/.gnome-ad-config.yaml";

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    schedule_light_mode: String,
    schedule_dark_mode: String,
    cycle_rate: u64
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            schedule_light_mode: "07:00".to_string(),
            schedule_dark_mode: "19:00".to_string(),
            cycle_rate: 600 // 10 minutes
        }
    }
}

fn main() {
    let config_path = shellexpand::tilde(CONFIG_PATH).to_string();

    loop {
        let settings = bootstrap_settings(&config_path);

        let loop_duration = Duration::from_secs(settings.cycle_rate);

        let current_time = Local::now().time();
        println!("{:?}", current_time);

        let light_time = NaiveTime::parse_from_str(&settings.schedule_light_mode, "%H:%M").unwrap();
        let dark_time = NaiveTime::parse_from_str(&settings.schedule_dark_mode, "%H:%M").unwrap();

        let theme_preference = determine_theme(current_time, light_time, dark_time);
        set_gnome_theme(theme_preference);

        sleep(loop_duration);
    }
}

fn determine_theme(current_time: NaiveTime, light_time: NaiveTime, dark_time: NaiveTime) -> (&'static str, &'static str) {
    let light_theme = ("prefer-light", "Flat-Remix-GTK-Blue-Light-Solid");
    let dark_theme = ("prefer-dark", "Flat-Remix-GTK-Blue-Dark-Solid");

    if (light_time < dark_time && current_time > light_time && current_time < dark_time) 
        || (dark_time < light_time && !(current_time > dark_time && current_time < light_time)) {
        light_theme
    } else {
        dark_theme
    }
}

fn bootstrap_settings(path: &str) -> Settings {
    return if Path::new(&path).exists() {
        read_settings(&path)
    } else {
        let default_settings = Settings::default();
        write_settings(&path, &default_settings);
        default_settings
    };
}

fn set_gnome_theme(theme: (&str, &str)) {
    let (color_theme, gtk_theme) = theme;

    println!("Setting Gnome theme to {:?}", theme);

    // Set the color-scheme
    std::process::Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.interface", "color-scheme", color_theme])
        .output()
        .expect("Failed to set GNOME color-scheme");

    // Set the gtk-theme
    std::process::Command::new("gsettings")
        .args(&["set", "org.gnome.desktop.interface", "gtk-theme", gtk_theme])
        .output()
        .expect("Failed to set GNOME gtk-theme");
}

fn read_settings<P: AsRef<Path>>(path: P) -> Settings {
    let mut file = File::open(path).expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read config file");
    serde_yaml::from_str(&contents).expect("Failed to parse config file")
}

fn write_settings<P: AsRef<Path>>(path: P, settings: &Settings) {
    println!("{:?}", path.as_ref());

    // Ensure the parent directory exists
    if let Some(parent_dir) = path.as_ref().parent() {
        std::fs::create_dir_all(parent_dir).expect("Failed to create directory");
    }

    let mut file = OpenOptions::new().create_new(true).write(true).open(path).expect("Failed to open config file");
    let contents = serde_yaml::to_string(settings).expect("Failed to serialize settings");
    write!(file, "{}", contents).expect("Failed to write to config file");
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    fn naive_time_or_panic(h: u32, m: u32, s: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, s).expect("Invalid time provided")
    }

    // light before dark
    #[test]
    fn test_light_before_dark_in_range() {
        let current = naive_time_or_panic(12, 0, 0);
        let light = naive_time_or_panic(7, 0, 0);
        let dark = naive_time_or_panic(19, 0, 0);
        assert_eq!(determine_theme(current, light, dark), ("prefer-light", "Flat-Remix-GTK-Blue-Light-Solid"));
    }

    #[test]
    fn test_light_before_dark_out_of_range() {
        let current = naive_time_or_panic(20, 0, 0);
        let light = naive_time_or_panic(7, 0, 0);
        let dark = naive_time_or_panic(19, 0, 0);
        assert_eq!(determine_theme(current, light, dark), ("prefer-dark", "Flat-Remix-GTK-Blue-Dark-Solid"));
    }

    #[test]
    fn test_light_before_dark_out_of_range_almost_midnight() {
        let current = naive_time_or_panic(23, 59, 59);
        let light = naive_time_or_panic(7, 0, 0);
        let dark = naive_time_or_panic(23, 0, 0);
        assert_eq!(determine_theme(current, light, dark), ("prefer-dark", "Flat-Remix-GTK-Blue-Dark-Solid"));
    }

    #[test]
    fn test_light_before_dark_out_of_range_midnight() {
        let current = naive_time_or_panic(0, 0, 0);
        let light = naive_time_or_panic(7, 0, 0);
        let dark = naive_time_or_panic(23, 0, 0);
        assert_eq!(determine_theme(current, light, dark), ("prefer-dark", "Flat-Remix-GTK-Blue-Dark-Solid"));
    }

    // dark before light
    #[test]
    fn test_dark_before_light_out_of_range() {
        let current = naive_time_or_panic(6, 0, 0);
        let light = naive_time_or_panic(19, 0, 0);
        let dark = naive_time_or_panic(7, 0, 0);
        assert_eq!(determine_theme(current, light, dark), ("prefer-light", "Flat-Remix-GTK-Blue-Light-Solid"));
    }

    #[test]
    fn test_dark_before_light_in_range() {
        let current = naive_time_or_panic(18, 0, 0);
        let light = naive_time_or_panic(19, 0, 0);
        let dark = naive_time_or_panic(7, 0, 0);
        assert_eq!(determine_theme(current, light, dark), ("prefer-dark", "Flat-Remix-GTK-Blue-Dark-Solid"));
    }

    #[test]
    fn test_dark_before_light_out_of_range_almost_midnight() {
        let current = naive_time_or_panic(23, 59, 59);
        let light = naive_time_or_panic(19, 0, 0);
        let dark = naive_time_or_panic(7, 0, 0);
        assert_eq!(determine_theme(current, light, dark), ("prefer-light", "Flat-Remix-GTK-Blue-Light-Solid"));
    }

    #[test]
    fn test_dark_before_light_out_of_range_midnight() {
        let current = naive_time_or_panic(0, 0, 0);
        let light = naive_time_or_panic(19, 0, 0);
        let dark = naive_time_or_panic(7, 0, 0);
        assert_eq!(determine_theme(current, light, dark), ("prefer-light", "Flat-Remix-GTK-Blue-Light-Solid"));
    }
}