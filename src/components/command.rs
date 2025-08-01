use crate::format::{text, Format, Sections, Width};
use crate::grammar::Rule;
use crate::state::{BlockState, Config};
use pest::iterators::Pair;
use std::fmt;
use std::fmt::Write as _;

#[derive(PartialEq, Debug)]
pub struct CommandNode {
    comment: Option<String>,
    ident: String,
    parts: Vec<String>,
}

impl Format for CommandNode {
    fn format(&self, _config: Config, state: &BlockState) -> Result<String, fmt::Error> {
        let lhs_pad_right = state.lhs_width();

        let lhs = self.as_lhs().expect("infallible");
        let mid = self.as_mid().expect("infallible");
        let rhs = self.as_rhs().expect("infallible");

        let mut s = String::new();
        write!(s, "{lhs:lhs_pad_right$}{mid}{rhs}")?;

        if let Some(c) = &self.comment {
            let sizes = [2_usize, state.total_width() - s.as_str().len()];
            let comment_gap = sizes.iter().max().unwrap();

            write!(s, " {empty:>comment_gap$}{c}", empty = "")?;
        }
        Ok(s)
    }
}

impl Sections for CommandNode {
    fn as_lhs(&self) -> Option<String> {
        Some(self.ident.to_string())
    }
    fn as_rhs(&self) -> Option<String> {
        Some(
            self.parts
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
    fn as_mid(&self) -> Option<String> {
        let has_lhs = self.as_lhs().map(|side| !side.is_empty());
        let has_rhs = self.as_rhs().map(|side| !side.is_empty());
        match (has_lhs, has_rhs) {
            (Some(l), Some(r)) if l && r => Some(String::from(" = ")),
            (Some(l), Some(r)) if l && !r => Some(String::from(" =")),
            _ => None,
        }
    }
}

impl CommandNode {
    pub fn new(tag: &Pair<Rule>) -> Self {
        let mut ident = None;
        let mut parts = Vec::new();
        let mut comment = None;

        for part in tag.clone().into_inner() {
            match part.as_rule() {
                Rule::command_ident => {
                    ident = Some(text(&part));
                }
                Rule::command_expression | Rule::command_rule => parts.push(text(&part)),
                Rule::comment => {
                    comment = Some(text(&part));
                }
                _ => unreachable!(),
            }
        }

        CommandNode {
            comment,
            ident: ident.expect("command must have an ident"),
            parts,
        }
    }
}
