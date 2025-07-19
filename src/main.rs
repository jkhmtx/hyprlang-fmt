#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]

mod state;
mod cli;
mod format;
mod grammar;
mod components;
mod node;

use crate::state::{Config };
use crate::components::block::Block;
use crate::node::Node;
use crate::grammar::{HyprlangParser, Rule};
use pest::iterators::Pair;
use pest::Parser;
use std::io::Read;

fn maybe_node(tag: Option<&Pair<Rule>>, config: Config) -> Option<Node> {
    match tag {
        Some(tag) if tag.as_rule() == Rule::EOI => None,
        Some(tag) => Some(Node::new(tag, config)),
        _ => None,
    }
}

fn get_file_blocks(pair: Pair<Rule>, config: Config) -> Vec<Block> {
    let mut blocks = Vec::new();

    let mut inner = pair.into_inner();
    loop {
        let node = maybe_node(inner.next().as_ref(), config);

        if node.is_none() {
            break;
        }

        let node = node.expect("infallible");

        let mut nodes = vec![node];

        loop {
            let node = maybe_node(inner.next().as_ref(), config);

            if node.is_none() {
                break;
            }

            let node = node.expect("infallible");

            nodes.push(node);

            let mut nodes_iter = nodes.iter().rev();
            if let (Some(last), Some(near_last)) = (nodes_iter.next(), nodes_iter.next()) {
                // Consume until non-newline
                if *last == Node::Newline && *near_last == Node::Newline {
                    for tag in inner.by_ref() {
                        let tag = match tag.as_rule() {
                            Rule::newline => None,
                            _ => Some(tag),
                        };

                        if let Some(tag) = tag {
                            nodes.push(Node::new(&tag, config));
                            break;
                        }
                    }

                    break;
                }
            }
        }

        blocks.push(Block::new(nodes, 0, config));
    }
    blocks
}

struct File {
    blocks: Vec<Block>,
}

fn main() {
    let config = cli::get_config();
    let mut file = String::new();
    std::io::stdin()
        .read_to_string(&mut file)
        .expect("Unable to read stdin.");


    let parse = HyprlangParser::parse(Rule::file, &file).unwrap();

    for pair in parse {
        match pair.as_rule() {
            Rule::file => {
                for block in get_file_blocks(pair, config) {
                    print!("{block}");
                }
            }
            _ => unreachable!(),
        }
    }
}
