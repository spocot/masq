mod parser;
mod targets {
    pub mod target;
}

use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use targets::target::{Gtk3Target, MasqTarget, SwayTarget};

#[derive(Deserialize, Debug)]
struct Config {
    theme: Theme,
}

#[derive(Deserialize, Debug)]
struct Theme {
    accent: u32,
    accent_deep: u32,
    foreground: u32,
    complement: u32,
    dark: u32,
    light_dark: u32,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Config {{ theme: {} }}", self.theme)
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Theme {{ \
                   accent: {:#x} \
                   accent_deep: {:#x} \
                   foreground: {:#x} \
                   complement: {:#x} \
                   dark: {:#x} \
                   light_dark: {:#x} }}",
            self.accent,
            self.accent_deep,
            self.foreground,
            self.complement,
            self.dark,
            self.light_dark
        )
    }
}

fn main() {
    let args = parser::Cli::parse();
    println!("args:\n{:?}", args);

    let available_targets: HashMap<&str, &dyn MasqTarget> = HashMap::from([
        ("sway", &SwayTarget as &dyn MasqTarget),
        ("gtk3", &Gtk3Target as &dyn MasqTarget),
    ]);

    match &args.commands {
        parser::Commands::Apply { file, targets } => {
            println!("Applying theme from file: {}", file);
            println!("Targets: {:?}", targets);

            let config_file = fs::read_to_string(file);
            match config_file {
                Ok(config_content) => {
                    let config: Config =
                        toml::from_str(&config_content).expect("Failed to parse config");
                    println!("Parsed config: {}", config);
                    let theme = config.theme;

                    for target_name in targets {
                        let target = available_targets.get(target_name.to_lowercase().as_str());
                        match target {
                            Some(t) => {
                                println!("Applying theme to target: {}", t.get_name());
                                if let Err(e) = t.apply(&theme) {
                                    eprintln!("Error applying theme to {}: {}", t.get_name(), e);
                                }
                            }
                            None => {
                                eprintln!("Target '{}' not found", target_name);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file {}: {}", file, e);
                }
            }
        }
        parser::Commands::Generate { output, colors } => {
            println!("Generating masq file: {}", output);
            println!("Colors: {:?}", colors);
        }
    }
}
