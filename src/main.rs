#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]

use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::fmt;
use std::fs::read_to_string;

const TAB_WIDTH: u8 = 4;

#[derive(Parser)]
#[grammar = "pest/grammar.pest"]
pub struct HyprlangParser;

fn text(tag: &Pair<Rule>) -> String {
    tag.as_span().as_str().trim_end_matches(' ').to_string()
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
struct Block {
    // The longest identifier in the block's length
    lhs_max_length: u8,
    // The longest statement in the block's length
    max_length: u8,
}

#[derive(PartialEq)]
enum Node {
    Comment {
        block: Block,
        level: u8,
        tokens: String,
    },
    Command {
        block: Block,
        comment: Option<String>,
        ident: String,
        level: u8,
        parts: Vec<String>,
    },
    VariableAssignment {
        block: Block,
        comment: Option<String>,
        expression: String,
        ident: String,
    },
    Category {
        ident: String,
        level: u8,
        inner: Vec<Node>,
    },
    Newline,
    EndOfInput,
}

impl fmt::Display for Node {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == &Node::EndOfInput {
            return formatter.write_str("");
        }
        match self {
            Node::Comment {
                block: _,
                level,
                tokens,
            } => {
                formatter.write_str(&" ".repeat((TAB_WIDTH * level).into()))?;
                formatter.write_str(tokens)
            }
            Node::Command {
                block: _,
                comment,
                ident,
                level,
                parts,
            } => {
                formatter.write_str(&" ".repeat((TAB_WIDTH * level).into()))?;
                write!(formatter, "{} =", &ident)?;
                let full_expression = parts
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ");

                if !full_expression.is_empty() {
                    formatter.write_str(&(" ".to_string() + &full_expression))?;
                }

                if let Some(c) = &comment {
                    write!(formatter, " {}", &c)?;
                }
                formatter.write_str("")
            }
            Node::VariableAssignment {
                block: _,
                comment,
                ident,
                expression,
            } => {
                write!(formatter, "{} = {}", &ident, &expression)?;
                if let Some(c) = &comment {
                    write!(formatter, " {}", &c)?;
                }

                formatter.write_str("")
            }
            Node::Category {
                ident,
                level,
                inner,
            } => {
                formatter.write_str(&" ".repeat((TAB_WIDTH * level).into()))?;

                write!(formatter, "{ident} {{")?;
                for item in inner {
                    formatter.write_str(&item.to_string())?;
                }

                formatter.write_str(&" ".repeat((TAB_WIDTH * level).into()))?;
                formatter.write_str("}")
            }
            Node::Newline => formatter.write_str("\n"),
            Node::EndOfInput => unreachable!(),
        }
    }
}

impl Node {
    fn new_comment(tag: &Pair<Rule>, level: u8) -> Node {
        Node::Comment {
            block: Block {
                lhs_max_length: 0,
                max_length: 0,
            },
            level,
            tokens: text(tag),
        }
    }

    fn new_command(tag: &Pair<Rule>, level: u8) -> Node {
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
            block: Block {
                lhs_max_length: 0,
                max_length: 0,
            },
            comment,
            ident: ident.expect("command must have an ident"),
            level,
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
            block: Block {
                lhs_max_length: 0,
                max_length: 0,
            },
            comment,
            expression: expression.expect("variable_assignment must have an expression"),
            ident: ident.expect("variable_assignment must have an ident"),
        }
    }

    fn new_category(tag: &Pair<Rule>, level: u8) -> Node {
        let mut ident = None;
        let mut inner = Vec::new();

        for pair in tag.clone().into_inner() {
            match pair.as_rule() {
                Rule::category_ident => {
                    ident = Some(text(&pair));
                }
                Rule::category_inner => {
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::command => inner.push(Node::new_command(&inner_pair, level + 1)),
                            Rule::comment => inner.push(Node::new_comment(&inner_pair, level + 1)),
                            Rule::newline => inner.push(Node::Newline),
                            Rule::category => {
                                inner.push(Node::new_category(&inner_pair, level + 1));
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
            level,
            inner,
        }
    }
}

fn get_file_nodes(pair: Pair<Rule>) -> Vec<Node> {
    let mut nodes = Vec::new();
    for tag in pair.into_inner() {
        match tag.as_rule() {
            Rule::EOI => {
                nodes.push(Node::EndOfInput);
            }
            Rule::comment => {
                nodes.push(Node::new_comment(&tag, 0));
            }
            Rule::newline => {
                nodes.push(Node::Newline);
            }
            Rule::command => {
                nodes.push(Node::new_command(&tag, 0));
            }
            Rule::variable_assignment => {
                nodes.push(Node::new_variable_assignment(&tag));
            }
            Rule::category => {
                nodes.push(Node::new_category(&tag, 0));
            }
            _ => {
                unreachable!()
            }
        }
    }

    nodes
}

fn main() {
    let hypr_conf = read_to_string("testbed/hypr/hyprland.conf").unwrap();

    let parse = HyprlangParser::parse(Rule::file, &hypr_conf).unwrap();

    for pair in parse {
        match pair.as_rule() {
            Rule::file => {
                for node in get_file_nodes(pair) {
                    print!("{node}");
                }
            }
            _ => unreachable!(),
        }
    }
}
