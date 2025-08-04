use crate::config::{Config, IndentMode};
use crate::grammar::Rule;
use crate::state::BlockState;
use pest::iterators::Pair;
use std::fmt;

type GetLeadingWhitespace<'a> = dyn Fn(&BlockState) -> String + 'a;
type GetCommentOffset<'a> = dyn Fn(&BlockState, &str) -> usize + 'a;
type GetLhsPadRight<'a> = dyn Fn(&BlockState) -> usize + 'a;

pub struct FormatStrategy<'a> {
    phantom: std::marker::PhantomData<&'a Config>,
    pub get_leading_whitespace: Box<GetLeadingWhitespace<'a>>,
    pub get_comment_offset: Box<GetCommentOffset<'a>>,
    pub get_lhs_pad_right: Box<GetLhsPadRight<'a>>,
}

impl FormatStrategy<'_> {
    pub fn new(config: Config) -> Self {
        let indent_width = move |state: &BlockState| usize::from(config.indent_width * state.level);
        let get_leading_whitespace = Box::new(move |state: &BlockState| {
            (match config.indent_mode {
                IndentMode::Tabs => "\t",
                IndentMode::Spaces => " ",
            })
            .repeat(indent_width(state))
        });

        let get_comment_offset = Box::new(move |state: &BlockState, line: &str| {
            let total_width = match &state.max_lengths {
                Some(lengths) => lengths.rhs.unwrap_or(lengths.mid),
                _ => 0,
            };
            let sizes = [2_usize, total_width + indent_width(state) - line.len()];
            *unsafe { sizes.iter().max().unwrap_unchecked() }
        });

        let get_lhs_pad_right = Box::new(move |state: &BlockState| match &state.max_lengths {
            Some(lengths) => lengths.lhs,
            _ => 0,
        });

        Self {
            phantom: std::marker::PhantomData,
            get_leading_whitespace,
            get_comment_offset,
            get_lhs_pad_right,
        }
    }
}

pub trait Format {
    fn format(&self, strategy: FormatStrategy, state: &BlockState) -> Result<String, fmt::Error>;
}

pub fn text(tag: &Pair<Rule>) -> String {
    tag.as_span().as_str().trim_end_matches(' ').to_string()
}
