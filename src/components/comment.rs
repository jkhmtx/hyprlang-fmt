use crate::format::text;
use crate::format::Format;
use crate::grammar::Rule;
use crate::state::{BlockState, Config};
use pest::iterators::Pair;
use std::fmt;

#[derive(PartialEq, Debug)]
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
