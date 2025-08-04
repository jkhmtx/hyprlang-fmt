use crate::components::node::Node;
use crate::format::{Sections, Width};

#[derive(PartialEq, Debug)]
pub struct LengthsInclusive {
    pub lhs: usize,
    pub mid: usize,
    pub rhs: Option<usize>,
}

#[derive(PartialEq, Debug)]
pub struct BlockState {
    // The indentation level of the block
    pub level: u8,
    pub max_lengths: Option<LengthsInclusive>,
}

impl Width for BlockState {
    fn lhs_width(&self) -> usize {
        match &self.max_lengths {
            Some(lengths) => lengths.lhs,
            _ => 0,
        }
    }

    fn total_width(&self, config: Config) -> usize {
        match &self.max_lengths {
            Some(lengths) => {
                lengths.rhs.unwrap_or(lengths.mid) + usize::from(config.indent_width * self.level)
            }
            _ => 0,
        }
    }
}

impl BlockState {
    pub fn new(nodes: &[Node], level: u8) -> Self {
        let max_lengths = {
            let mut lhs = 0;
            let mut mid = 0;
            let mut rhs = None;

            for section in nodes.iter().filter_map(Sections::as_sections) {
                lhs = std::cmp::max(lhs, section.lhs.len());
                mid = std::cmp::max(mid, lhs + section.mid.len());
                rhs = match (rhs, section.rhs) {
                    (Some(rhs), Some(section_rhs)) => {
                        Some(std::cmp::max(rhs, mid + section_rhs.len()))
                    }
                    (Some(rhs), None) => Some(rhs),
                    (None, Some(section_rhs)) => Some(mid + section_rhs.len()),
                    _ => None,
                };
            }

            if lhs == 0 {
                None
            } else {
                Some(LengthsInclusive { lhs, mid, rhs })
            }
        };

        BlockState { level, max_lengths }
    }
}

#[derive(clap::ValueEnum, PartialEq, Clone, Copy, Debug, Default)]
pub enum IndentMode {
    Tabs,
    #[default]
    Spaces,
}

impl std::fmt::Display for IndentMode {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            Self::Tabs => "tabs",
            Self::Spaces => "spaces",
        })
    }
}

#[derive(clap::ValueEnum, PartialEq, Clone, Copy, Debug, Default)]
pub enum CommentSpacingContext {
    #[default]
    Block,
    Category,
    File,
}

impl std::fmt::Display for CommentSpacingContext {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            Self::Block => "block",
            Self::Category => "category",
            Self::File => "file",
        })
    }
}

#[derive(clap::ValueEnum, PartialEq, Clone, Copy, Debug, Default)]
pub enum CommandRhsSpacingMode {
    #[default]
    Compact,
    Equidistant,
}

impl std::fmt::Display for CommandRhsSpacingMode {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            Self::Compact => "compact",
            Self::Equidistant => "equidistant",
        })
    }
}

#[derive(clap::ValueEnum, PartialEq, Clone, Copy, Debug, Default)]
pub enum CommandRhsSpacingContext {
    #[default]
    Block,
    Category,
}

impl std::fmt::Display for CommandRhsSpacingContext {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        formatter.write_str(match self {
            Self::Block => "block",
            Self::Category => "category",
        })
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Config {
    pub indent_width: u8,
    pub indent_mode: IndentMode,
    pub comment_spacing_context: CommentSpacingContext,
    pub command_rhs_spacing_mode: CommandRhsSpacingMode,
    pub command_rhs_spacing_context: CommandRhsSpacingContext,
}
