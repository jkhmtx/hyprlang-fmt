use crate::format::{Sections, Width};
use crate::node::Node;

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

impl BlockState {
    pub fn new(nodes: &[Node], level: u8, config: Config) -> Self {
        let indent = usize::from(config.tab_width * level);

        let lhs_max_length = nodes
            .iter()
            .filter_map(|node| node.as_lhs().as_deref().map(str::len))
            .max()
            .unwrap_or(0);

        let max_length = nodes
            .iter()
            .map(|node| {
                [node.as_lhs(), node.as_mid(), node.as_rhs()]
                    .map(|n| n.as_deref().map_or(0, str::len))
                    .iter()
                    .sum()
            })
            .max()
            .unwrap_or(0)
            + indent;

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
