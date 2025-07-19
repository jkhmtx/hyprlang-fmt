use crate::state::{Config, BlockState};
use std::fmt;
use crate::format::text;
use crate::grammar::Rule;
use pest::iterators::Pair;
use crate::format::Format;

#[derive(PartialEq)]
pub struct CommentNode {
    tokens: String,
}

impl Format for CommentNode {
    fn format(&self, _config: Config, _state: &BlockState) -> Result<String, fmt::Error> {
        Ok(self.tokens.to_string())
    }
}

impl CommentNode {
    pub fn new(tag: &Pair<Rule>) -> Self {
        CommentNode { tokens: text(tag) }
    }
}
