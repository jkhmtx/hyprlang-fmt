use crate::format::{text, Format, Measure};
use crate::grammar::Rule;
use crate::state::{BlockState, Config};
use pest::iterators::Pair;
use std::fmt;
use std::fmt::Write as _;

#[derive(PartialEq)]
pub struct VariableAssignmentNode {
    comment: Option<String>,
    expression: String,
    ident: String,
}

impl Format for VariableAssignmentNode {
    fn format(&self, _config: Config, state: &BlockState) -> Result<String, fmt::Error> {
        let lhs_pad_right = state.lhs_max_length;

        let lhs = self.as_lhs().expect("infallible");
        let mid = self.as_mid().expect("infallible");
        let rhs = self.as_rhs().expect("infallible");

        let mut s = String::new();
        write!(s, "{lhs:lhs_pad_right$}{mid}{rhs}")?;

        if let Some(c) = &self.comment {
            let comment_gap = state.max_length - s.as_str().len();
            write!(s, " {empty:>comment_gap$}{c}", empty = "")?;
        }

        Ok(s)
    }
}

impl Measure for VariableAssignmentNode {
    fn as_lhs(&self) -> Option<String> {
        Some(self.ident.to_string())
    }
    fn as_rhs(&self) -> Option<String> {
        Some(self.expression.to_string())
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

impl VariableAssignmentNode {
    pub fn new(tag: &Pair<Rule>) -> Self {
        let mut ident = None;
        let mut expression = None;
        let mut comment = None;

        for part in tag.clone().into_inner() {
            match part.as_rule() {
                Rule::variable_ident => {
                    ident = Some(text(&part));
                }
                Rule::variable_expression => {
                    expression = Some(text(&part));
                }
                Rule::comment => {
                    comment = Some(text(&part));
                }
                _ => unreachable!(),
            }
        }
        VariableAssignmentNode {
            comment,
            expression: expression.expect("variable_assignment must have an expression"),
            ident: ident.expect("variable_assignment must have an ident"),
        }
    }
}
