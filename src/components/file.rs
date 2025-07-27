use crate::components::block::Block;
use std::fmt;

use crate::grammar::Rule;
use crate::node::Node;
use crate::state::Config;
use pest::iterators::Pair;

#[derive(PartialEq, Debug)]
pub struct File {
    blocks: Vec<Block>,
}

impl fmt::Display for File {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in &self.blocks {
            write!(formatter, "{block}")?;
        }

        Ok(())
    }
}

impl File {
    pub fn new(pair: Pair<Rule>, config: Config) -> Self {
        let mut blocks = Vec::new();

        let mut inner = pair.into_inner();
        loop {
            let node = Node::maybe(inner.next().as_ref(), config);

            if node.is_none() {
                break;
            }

            let node = node.expect("infallible");

            let mut nodes = vec![node];

            loop {
                let node = Node::maybe(inner.next().as_ref(), config);

                if node.is_none() {
                    break;
                }

                let node = node.expect("infallible");

                nodes.push(node);

                let mut nodes_iter = nodes.iter();
                if let (Some(last), Some(near_last)) =
                    (nodes_iter.next_back(), nodes_iter.next_back())
                {
                    // Consume until non-newline
                    if *last == Node::Newline && *near_last == Node::Newline {
                        for tag in inner.by_ref() {
                            let node = Node::maybe(Some(&tag), config).and_then(|node| {
                                if node == Node::Newline {
                                    None
                                } else {
                                    Some(node)
                                }
                            });

                            if let Some(node) = node {
                                nodes.push(node);
                                break;
                            }
                        }

                        break;
                    }
                }
            }

            blocks.push(Block::new(nodes, 0, config));
        }

        File { blocks }
    }
}
