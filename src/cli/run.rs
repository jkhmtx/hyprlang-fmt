use crate::components::file::File;
use crate::grammar::{HyprlangParser, Rule};
use crate::state::Config;
use pest::Parser;

pub fn run(config: Config, file: &str) {
    let parse = HyprlangParser::parse(Rule::file, file).unwrap();

    for pair in parse {
        let file = match pair.as_rule() {
            Rule::file => File::new(pair, config),
            _ => unreachable!(),
        };

        print!("{file}");
    }
}
