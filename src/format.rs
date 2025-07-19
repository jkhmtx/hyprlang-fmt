use pest::iterators::Pair;
use crate::state::{Config, BlockState};
use crate::grammar::Rule;
use std::fmt;

pub trait Format {
    fn format(&self, config: Config, state: &BlockState) -> Result<String, fmt::Error>;
}

pub trait Measure {
    fn as_lhs(&self) -> Option<String>;
    fn as_rhs(&self) -> Option<String>;
    fn as_mid(&self) -> Option<String>;
}


pub fn text(tag: &Pair<Rule>) -> String {
    tag.as_span().as_str().trim_end_matches(' ').to_string()
}

