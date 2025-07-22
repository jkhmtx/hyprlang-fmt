use crate::grammar::Rule;
use crate::state::{BlockState, Config};
use pest::iterators::Pair;
use std::fmt;

pub trait Format {
    fn format(&self, config: Config, state: &BlockState) -> Result<String, fmt::Error>;
}

pub trait Sections {
    fn as_lhs(&self) -> Option<String>;
    fn as_rhs(&self) -> Option<String>;
    fn as_mid(&self) -> Option<String>;
}

pub trait Width {
    fn lhs_width(&self) -> usize;
    fn total_width(&self) -> usize;
}

pub fn text(tag: &Pair<Rule>) -> String {
    tag.as_span().as_str().trim_end_matches(' ').to_string()
}
