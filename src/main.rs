#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]

mod cli;
mod components;
mod format;
mod grammar;
mod node;
mod state;

fn main() {
    let config = cli::get_config();

    let file = cli::get_file();

    cli::run(config, &file);
}
