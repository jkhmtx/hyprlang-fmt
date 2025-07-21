use crate::node::Node;
use crate::state::{Config, BlockState};
use crate::format::{Measure, Format};
use std::fmt;

// Blocks are lines of code localized by either:
//   1. Contiguous spacing
//   2. Category enclosure
//
// A Block is used for ensuring that the infix spacing
// between identifiers and '=' on the left-hand side,
// and '=' and expressions on the right-hand side is
// consistent.
//
// This spacing also applies to trailing comments,
// which are aligned by the longest statement in a
// block.
//
// Example:
// ident = foo # trailing 1
// another_ident = much_longer_bar # trailing 2
//
// ident         = foo             # trailing 1
// another_ident = much_longer_bar # trailing 2
#[derive(PartialEq)]
pub struct Block {
    state: BlockState,

    nodes: Vec<Node>,

    config: Config,
}

impl Block {
    pub fn new(nodes: Vec<Node>, level: u8, config: Config) -> Block {
        let indent = config.tab_width * level;

        let lhs_max_length = nodes
            .iter()
            .filter_map(|node| node.as_lhs().as_deref().map(str::len))
            .max()
            .unwrap_or(0);

        let max_length = nodes
            .iter()
            .filter_map(|node| node.as_rhs().as_deref().map(str::len))
            .max()
            .unwrap_or(0)
            + usize::from(indent)
            + lhs_max_length
            + 3;

        Block {
            state: BlockState {
                level,
                lhs_max_length,
                max_length,
            },
            nodes,
            config,
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in &self.nodes {
            let s = node.format(self.config, &self.state)?;

            if node != &Node::Newline {
                let leading_spaces = usize::from(self.config.tab_width * self.state.level);
                write!(formatter, "{empty:>leading_spaces$}", empty = "")?;
            }

            formatter.write_str(&s)?;
        }

        Ok(())
    }
}


