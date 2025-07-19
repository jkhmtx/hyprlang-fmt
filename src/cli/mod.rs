mod args;

use clap::Parser;
use crate::state::config::Config;
use args::Args;

pub fn get_config() -> Config {
    let args = Args::parse();
        
    Config {
        tab_width: args.spaces,
    }

}

