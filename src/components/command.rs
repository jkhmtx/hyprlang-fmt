use crate::format::{text, Format, Sections, SectionsView, Width};
use crate::grammar::Rule;
use crate::state::{BlockState, Config};
use pest::iterators::Pair;
use std::fmt;
use std::fmt::Write as _;

#[derive(PartialEq, Debug)]
pub struct CommandNode {
    comment: Option<String>,
    eq: String,
    joined_parts: Option<String>,
    ident: String,
    parts: Vec<String>,
}

impl Format for CommandNode {
    fn format(&self, config: &Config, state: &BlockState) -> Result<String, fmt::Error> {
        let lhs_pad_right = state.lhs_width();

        let Some(SectionsView { lhs, mid, rhs }) = self.as_sections() else {
            unreachable!()
        };

        let rhs = rhs.unwrap_or("");

        let mut s = String::new();
        write!(s, "{lhs:lhs_pad_right$}{mid}{rhs}")?;

        if let Some(c) = &self.comment {
            let sizes = [2_usize, state.total_width(config) - s.as_str().len()];
            let comment_gap = sizes.iter().max().unwrap();

            write!(s, " {empty:>comment_gap$}{c}", empty = "")?;
        }
        Ok(s)
    }
}

impl Sections for CommandNode {
    fn as_sections(&self) -> Option<SectionsView<'_>> {
        Some(SectionsView {
            lhs: &self.ident,
            mid: &self.eq,
            rhs: self.joined_parts.as_deref(),
        })
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

        let ident = ident.expect("command must have an ident");

        let has_lhs = !ident.is_empty();
        let has_rhs = parts.iter().any(|part| !part.is_empty());

        let eq = String::from(if has_lhs && has_rhs { " = " } else { " =" });

        let joined_parts = if parts.is_empty() {
            None
        } else {
            Some(
                parts
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        };

        CommandNode {
            comment,
            eq,
            joined_parts,
            ident,
            parts,
        }
    }
}
