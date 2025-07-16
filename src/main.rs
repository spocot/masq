use clap::Parser;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    theme: Theme,
}

#[derive(Deserialize, Debug)]
struct Theme {
    accent: u16,
}

/// Masq is a command-line tool for applying a shared base colorscheme theme to multiple target applications, toolkits, or libraries.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Apply a theme to one or more targets
    Apply {
        /// The masq file of the theme to apply
        #[arg(short, long)]
        file: String,

        /// The list of targets to apply the theme to
        #[arg(required = true, short, long, num_args = 1.., value_delimiter = ',')]
        targets: Vec<String>,
    },
    /// Generate a masq theming file based on the passed theme colors
    Generate {
        /// The name of the masq file to generate
        #[arg(short, long)]
        output: String,

        /// The color settings to save to the generated masq file
        #[arg(required = true, short, long, num_args = 6, value_names = &[
            "ACCENT",
            "ACCENT_DEEP",
            "FOREGROUND",
            "COMPLEMENT",
            "DARK",
            "LIGHT_DARK"
        ])]
        colors: Vec<String>,
    },
}

fn main() {
    let args = Cli::parse();
    match &args.commands {
        Commands::Apply { file, targets } => {
            println!("Applying theme from file: {}", file);
            println!("Targets: {:?}", targets);
        }
        Commands::Generate { output, colors } => {
            println!("Generating masq file: {}", output);
            println!("Colors: {:?}", colors);
        }
    }

    println!("config:\n{:?}", args)
}
