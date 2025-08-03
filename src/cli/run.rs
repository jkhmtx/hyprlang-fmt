use crate::components::file::File;
use crate::grammar::{HyprlangParser, Rule};
use crate::state::Config;
use pest::Parser;

#[derive(Debug)]
pub enum RunError {
    InvalidInput,
}

pub fn run(config: &Config, file: &str) {
    let pair = HyprlangParser::parse(Rule::file, file)
        .map_err(|_| RunError::InvalidInput)
        .map(|mut parsed| unsafe { parsed.next().unwrap_unchecked() })
        .expect("failed to parse file");

    let file = File::new(pair, config);

    print!("{file}");
}
