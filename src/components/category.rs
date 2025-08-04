use crate::components::block::Block;
use crate::components::command::CommandNode;
use crate::components::comment::CommentNode;
use crate::components::node::Node;
use crate::config::Config;
use crate::format::{text, Format, FormatStrategy};
use crate::grammar::Rule;
use crate::state::BlockState;
use pest::iterators::Pair;
use std::fmt;
use std::fmt::Write as _;

#[derive(PartialEq, Debug)]
pub struct CategoryNode {
    ident: String,
    block: Block,
}

impl Format for CategoryNode {
    fn format(&self, strategy: FormatStrategy, state: &BlockState) -> Result<String, fmt::Error> {
        let CategoryNode { ident, block } = self;
        let mut s = String::new();
        write!(s, "{ident} {{")?;
        write!(s, "{}", &block.to_string())?;

        let leading_whitespace = (strategy.get_leading_whitespace)(state);

        write!(s, "{leading_whitespace}")?;
        write!(s, "}}")?;
        Ok(s)
    }
}

impl CategoryNode {
    pub fn new(tag: &Pair<Rule>, level: u8, config: Config) -> Self {
        let mut ident = None;
        let mut nodes = Vec::new();

        for pair in tag.clone().into_inner() {
            match pair.as_rule() {
                Rule::category_ident => {
                    ident = Some(text(&pair));
                }
                Rule::category_inner => {
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::command => {
                                nodes.push(Node::Command(CommandNode::new(&inner_pair)));
                            }
                            Rule::comment => {
                                nodes.push(Node::Comment(CommentNode::new(&inner_pair)));
                            }
                            Rule::newline => {
                                let mut nodes_iter = nodes.iter().rev();
                                match (nodes_iter.next(), nodes_iter.next()) {
                                    // Don't add newlines if the previous two nodes were also newlines
                                    (Some(last), Some(near_last))
                                        if *last == Node::Newline
                                            && *near_last == Node::Newline => {}
                                    _ => nodes.push(Node::Newline),
                                }
                            }
                            Rule::category => {
                                nodes.push(Node::Category(CategoryNode::new(
                                    &inner_pair,
                                    level + 1,
                                    config,
                                )));
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        CategoryNode {
            ident: ident.expect("category must have an ident"),
            block: Block::new(nodes, level + 1, config),
        }
    }
}
