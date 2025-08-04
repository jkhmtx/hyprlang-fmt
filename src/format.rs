use crate::grammar::Rule;
use crate::state::{BlockState, Config};
use pest::iterators::Pair;
use std::fmt;

pub trait Format {
    fn format(&self, config: Config, state: &BlockState) -> Result<String, fmt::Error>;
}

pub struct SectionsView<'a> {
    pub lhs: &'a str,
    pub mid: &'a str,
    pub rhs: Option<&'a str>,
}

pub trait Sections {
    fn as_sections(&self) -> Option<SectionsView<'_>>;
}

pub trait Width {
    fn lhs_width(&self) -> usize;
    fn total_width(&self, config: Config) -> usize;
}

pub fn text(tag: &Pair<Rule>) -> String {
    tag.as_span().as_str().trim_end_matches(' ').to_string()
}
