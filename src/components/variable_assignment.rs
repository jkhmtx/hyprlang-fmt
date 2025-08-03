use crate::format::{text, Format, Sections, SectionsView, Width};
use crate::grammar::Rule;
use crate::state::{BlockState, Config};
use pest::iterators::Pair;
use std::fmt;
use std::fmt::Write as _;

#[derive(PartialEq, Debug)]
pub struct VariableAssignmentNode {
    comment: Option<String>,
    eq: String,
    expression: Option<String>,
    ident: String,
}

impl Format for VariableAssignmentNode {
    fn format(&self, _config: Config, state: &BlockState) -> Result<String, fmt::Error> {
        let lhs_pad_right = state.lhs_width();

        let Some(SectionsView { lhs, mid, rhs }) = self.as_sections() else {
            unreachable!()
        };

        let rhs = rhs.unwrap_or("");

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

impl Sections for VariableAssignmentNode {
    fn as_sections(&self) -> Option<SectionsView<'_>> {
        Some(SectionsView {
            lhs: &self.ident,
            mid: &self.eq,
            rhs: self.expression.as_deref(),
        })
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

        let ident = ident.expect("variable_assignment must have an ident");
        let has_lhs = !ident.is_empty();
        let has_rhs = if let Some(ref expression) = expression {
            !expression.is_empty()
        } else {
            false
        };

        let eq = String::from(if has_lhs && has_rhs { " = " } else { " =" });

        VariableAssignmentNode {
            comment,
            eq,
            expression,
            ident,
        }
    }
}
