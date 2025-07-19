use pest::iterators::Pair;
use crate::state::{Config, BlockState};
use crate::grammar::Rule;
use std::fmt;

pub trait Format {
    fn format(&self, config: Config, state: &BlockState) -> Result<String, fmt::Error>;
}

pub fn text(tag: &Pair<Rule>) -> String {
    tag.as_span().as_str().trim_end_matches(' ').to_string()
}

