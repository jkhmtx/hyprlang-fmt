use crate::state::Config;
use clap::Parser;
use std::io::Read;

/// A formatter for the hyprlang language.
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// How many spaces to use for indentation
    #[arg(short, long, default_value_t = 2)]
    pub spaces: u8,
}

pub fn get_config() -> Config {
    let args = Args::parse();

    Config {
        tab_width: args.spaces,
    }
}

pub fn get_file() -> String {
    let mut file = String::new();
    std::io::stdin()
        .read_to_string(&mut file)
        .expect("Unable to read stdin.");

    file
}
