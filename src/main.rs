#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]

mod state;
mod cli;
mod format;
mod grammar;
mod components;
mod node;

fn main() {
    let config = cli::get_config();

    let file = cli::get_file();

    cli::run(config, &file);
}
