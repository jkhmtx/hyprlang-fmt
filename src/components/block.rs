use crate::components::node::Node;
use crate::format::Format;
use crate::state::{BlockState, Config};
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
#[derive(PartialEq, Debug)]
pub struct Block<'a> {
    state: BlockState,

    nodes: Vec<Node<'a>>,

    config: &'a Config,
}

impl<'a> Block<'a> {
    pub fn new(nodes: Vec<Node<'a>>, level: u8, config: &'a Config) -> Self {
        Block {
            state: BlockState::new(&nodes, level),
            nodes,
            config,
        }
    }
}

impl fmt::Display for Block<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for node in &self.nodes {
            if node != &Node::Newline {
                let leading_spaces = usize::from(self.config.tab_width * self.state.level);
                write!(formatter, "{empty:>leading_spaces$}", empty = "")?;
            }

            formatter.write_str(&node.format(self.config, &self.state)?)?;
        }

        Ok(())
    }
}
