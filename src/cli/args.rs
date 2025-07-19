use clap::Parser as ClapParser;

/// A formatter for the hyprlang language.
#[derive(ClapParser, Debug)]
#[command(version)]
pub struct Args {
    /// How many spaces to use for indentation
    #[arg(short, long, default_value_t = 2)]
    pub spaces: u8,
}



