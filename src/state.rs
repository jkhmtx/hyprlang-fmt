use crate::components::node::Node;
use crate::format::{Sections, Width};

#[derive(PartialEq, Debug)]
pub struct BlockState {
    // The indentation level of the block
    pub level: u8,
    // The longest identifier in the block's length
    lhs_max_length: usize,
    // The longest statement in the block's length
    max_length: usize,
}

impl Width for BlockState {
    fn lhs_width(&self) -> usize {
        self.lhs_max_length
    }

    fn total_width(&self) -> usize {
        self.max_length
    }
}

pub struct LengthsInclusive {
    lhs: usize,
    mid: usize,
    rhs: Option<usize>,
    comment: Option<usize>,
}

impl BlockState {
    pub fn new(nodes: &[Node], level: u8, config: Config) -> Self {
        let indent = usize::from(config.tab_width * level);

        let lhs_max_length = nodes
            .iter()
            .map(|node| node.as_sections().map_or(0, |section| section.lhs.len()))
            .max()
            .unwrap_or(0);

        let max_length = if let Some(max) = nodes
            .iter()
            .map(|node| {
                node.as_sections().map_or(0, |section| {
                    section.lhs.len() + section.mid.len() + section.rhs.map_or(0, str::len)
                })
            })
            .max()
        {
            max + indent
        } else {
            0
        };

        BlockState {
            level,
            lhs_max_length,
            max_length,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Config {
    pub tab_width: u8,
}
