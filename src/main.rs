#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::fmt;
use std::fmt::Write as _;
use std::fs::read_to_string;

#[derive(Parser)]
#[grammar = "pest/grammar.pest"]
pub struct HyprlangParser;

fn text(tag: &Pair<Rule>) -> String {
    tag.as_span().as_str().trim_end_matches(' ').to_string()
}

#[derive(PartialEq)]
struct BlockState {
    // The indentation level of the block
    level: u8,
    // The longest identifier in the block's length
    lhs_max_length: usize,
    // The longest statement in the block's length
    max_length: usize,
}

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
struct Block {
    state: BlockState,

    nodes: Vec<Node>,

    config: Config,
}

trait Measure {
    fn as_lhs(&self) -> Option<String>;
    fn as_rhs(&self) -> Option<String>;
}

impl Block {
    fn new(nodes: Vec<Node>, level: u8, config: Config) -> Block {
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

#[derive(PartialEq)]
enum Node {
    Comment {
        tokens: String,
    },
    Command {
        comment: Option<String>,
        ident: String,
        parts: Vec<String>,
    },
    VariableAssignment {
        comment: Option<String>,
        expression: String,
        ident: String,
    },
    Category {
        ident: String,
        block: Block,
    },
    Newline,
    EndOfInput,
}

impl Measure for Node {
    fn as_lhs(&self) -> Option<String> {
        match self {
            Node::Newline
            | Node::EndOfInput
            | Node::Comment { tokens: _ }
            | Node::Category { block: _, ident: _ } => None,
            Node::Command {
                ident,
                comment: _,
                parts: _,
            }
            | Node::VariableAssignment {
                ident,
                expression: _,
                comment: _,
            } => Some(ident.to_string()),
        }
    }
    fn as_rhs(&self) -> Option<String> {
        match self {
            Node::Newline
            | Node::EndOfInput
            | Node::Comment { tokens: _ }
            | Node::Category { block: _, ident: _ } => None,
            Node::Command {
                ident: _,
                comment: _,
                parts,
            } => Some(
                parts
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
            Node::VariableAssignment {
                ident: _,
                expression,
                comment: _,
            } => Some(expression.to_string()),
        }
    }
}

impl Node {
    fn new(tag: &Pair<Rule>, config: Config) -> Node {
        match tag.as_rule() {
            Rule::comment => Node::new_comment(tag),
            Rule::newline => Node::Newline,
            Rule::command => Node::new_command(tag),
            Rule::variable_assignment => Node::new_variable_assignment(tag),
            Rule::category => Node::new_category(tag, 0, config),
            Rule::EOI => Node::EndOfInput,
            _ => unreachable!(),
        }
    }

    fn maybe(tag: Option<&Pair<Rule>>, config: Config) -> Option<Node> {
        match tag {
            Some(tag) if tag.as_rule() == Rule::EOI => None,
            Some(tag) => Some(Node::new(tag, config)),
            _ => None,
        }
    }

    fn new_comment(tag: &Pair<Rule>) -> Node {
        Node::Comment { tokens: text(tag) }
    }

    fn new_command(tag: &Pair<Rule>) -> Node {
        let mut ident = None;
        let mut parts = Vec::new();
        let mut comment = None;

        for part in tag.clone().into_inner() {
            match part.as_rule() {
                Rule::command_ident => {
                    ident = Some(text(&part));
                }
                Rule::command_expression | Rule::command_rule => parts.push(text(&part)),
                Rule::comment => {
                    comment = Some(text(&part));
                }
                _ => unreachable!(),
            }
        }

        Node::Command {
            comment,
            ident: ident.expect("command must have an ident"),
            parts,
        }
    }

    fn new_variable_assignment(tag: &Pair<Rule>) -> Node {
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
        Node::VariableAssignment {
            comment,
            expression: expression.expect("variable_assignment must have an expression"),
            ident: ident.expect("variable_assignment must have an ident"),
        }
    }

    fn new_category(tag: &Pair<Rule>, level: u8, config: Config) -> Node {
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
                            Rule::command => nodes.push(Node::new_command(&inner_pair)),
                            Rule::comment => nodes.push(Node::new_comment(&inner_pair)),
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
                                nodes.push(Node::new_category(&inner_pair, level + 1, config));
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        Node::Category {
            ident: ident.expect("category must have an ident"),
            block: Block::new(nodes, level + 1, config),
        }
    }

    fn format(&self, config: Config, state: &BlockState) -> Result<String, fmt::Error> {
        if self == &Node::EndOfInput {
            return Ok(String::new());
        }

        match self {
            Node::Comment { tokens } => Ok(tokens.to_string()),
            Node::Command {
                comment,
                ident: _,
                parts: _,
            } => {
                let lhs_pad_right = state.lhs_max_length;
                let lhs = self.as_lhs().expect("infallible");

                let mut s = String::new();
                write!(s, "{lhs:lhs_pad_right$} =")?;
                let rhs = self.as_rhs().expect("infallible");

                if !rhs.is_empty() {
                    write!(s, " {rhs}")?;
                }

                if let Some(c) = &comment {
                    let comment_gap = state.max_length - s.as_str().len();
                    write!(s, " {empty:>comment_gap$}{c}", empty = "")?;
                }
                Ok(s)
            }
            Node::VariableAssignment {
                comment,
                ident: _,
                expression: _,
            } => {
                let lhs_pad_right = state.lhs_max_length;
                let lhs = self.as_lhs().expect("infallible");
                let rhs = self.as_rhs().expect("infallible");

                let mut s = String::new();
                write!(s, "{lhs:lhs_pad_right$} = {rhs}")?;

                if let Some(c) = &comment {
                    let comment_gap = state.max_length - s.as_str().len();
                    write!(s, " {empty:>comment_gap$}{c}", empty = "")?;
                }

                Ok(s)
            }
            Node::Category { ident, block } => {
                let mut s = String::new();
                write!(s, "{ident} {{")?;
                write!(s, "{}", &block.to_string())?;
                let leading_spaces = usize::from(config.tab_width * state.level);
                write!(s, "{empty:>leading_spaces$}", empty = "")?;
                write!(s, "}}")?;
                Ok(s)
            }
            Node::Newline => Ok("\n".to_string()),
            Node::EndOfInput => unreachable!(),
        }
    }
}

fn get_file_blocks(pair: Pair<Rule>, config: Config) -> Vec<Block> {
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

#[derive(PartialEq, Clone, Copy)]
struct Config {
    pub tab_width: u8,
}

fn main() {
    let hypr_conf = read_to_string("testbed/hypr/hyprland.conf").unwrap();
    let config = Config { tab_width: 4 };

    let parse = HyprlangParser::parse(Rule::file, &hypr_conf).unwrap();

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
