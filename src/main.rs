#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]

use clap::Parser as ClapParser;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::fmt;
use std::fmt::Write as _;
use std::io::Read;

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
    fn as_mid(&self) -> Option<String>;
}

trait Format {
    fn format(&self, config: Config, state: &BlockState) -> Result<String, fmt::Error>;
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
struct CommentNode {
    tokens: String,
}
impl Format for CommentNode {
    fn format(&self, _config: Config, _state: &BlockState) -> Result<String, fmt::Error> {
        Ok(self.tokens.to_string())
    }
}

impl CommentNode {
    fn new(tag: &Pair<Rule>) -> Self {
        CommentNode { tokens: text(tag) }
    }
}
#[derive(PartialEq)]
struct CommandNode {
    comment: Option<String>,
    ident: String,
    parts: Vec<String>,
}

impl Format for CommandNode {
    fn format(&self, _config: Config, state: &BlockState) -> Result<String, fmt::Error> {
        let lhs_pad_right = state.lhs_max_length;

        let lhs = self.as_lhs().expect("infallible");
        let mid = self.as_mid().expect("infallible");
        let rhs = self.as_rhs().expect("infallible");

        let mut s = String::new();
        write!(s, "{lhs:lhs_pad_right$}{mid}{rhs}")?;

        if let Some(c) = &self.comment {
            let comment_gap = state.max_length - s.as_str().len();
            write!(s, " {empty:>comment_gap$}{c}", empty = "")?;
        }
        Ok(s)
    }
}

impl CommandNode {
    fn new(tag: &Pair<Rule>) -> Self {
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

        CommandNode {
            comment,
            ident: ident.expect("command must have an ident"),
            parts,
        }
    }
}
#[derive(PartialEq)]
struct VariableAssignmentNode {
    comment: Option<String>,
    expression: String,
    ident: String,
}

impl Format for VariableAssignmentNode {
    fn format(&self, _config: Config, state: &BlockState) -> Result<String, fmt::Error> {
        let lhs_pad_right = state.lhs_max_length;

        let lhs = self.as_lhs().expect("infallible");
        let mid = self.as_mid().expect("infallible");
        let rhs = self.as_rhs().expect("infallible");

        let mut s = String::new();
        write!(s, "{lhs:lhs_pad_right$}{mid}{rhs}")?;

        if let Some(c) = &self.comment {
            let comment_gap = state.max_length - s.as_str().len();
            write!(s, " {empty:>comment_gap$}{c}", empty = "")?;
        }

        Ok(s)
    }
}

impl VariableAssignmentNode {
    fn new(tag: &Pair<Rule>) -> Self {
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
        VariableAssignmentNode {
            comment,
            expression: expression.expect("variable_assignment must have an expression"),
            ident: ident.expect("variable_assignment must have an ident"),
        }
    }
}
#[derive(PartialEq)]
struct CategoryNode {
    ident: String,
    block: Block,
}

impl Format for CategoryNode {
    fn format(&self, config: Config, state: &BlockState) -> Result<String, fmt::Error> {
        let CategoryNode { ident, block } = self;
        let mut s = String::new();
        write!(s, "{ident} {{")?;
        write!(s, "{}", &block.to_string())?;
        let leading_spaces = usize::from(config.tab_width * state.level);
        write!(s, "{empty:>leading_spaces$}", empty = "")?;
        write!(s, "}}")?;
        Ok(s)
    }
}

impl CategoryNode {
    fn new(tag: &Pair<Rule>, level: u8, config: Config) -> Self {
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

#[derive(PartialEq)]
enum Node {
    Comment(CommentNode),
    Command(CommandNode),
    VariableAssignment(VariableAssignmentNode),
    Category(CategoryNode),
    Newline,
    EndOfInput,
}

impl Measure for Node {
    fn as_lhs(&self) -> Option<String> {
        match self {
            Node::Newline | Node::EndOfInput | Node::Comment(_) | Node::Category(_) => None,
            Node::Command(n) => n.as_lhs(),
            Node::VariableAssignment(n) => n.as_lhs(),
        }
    }
    fn as_rhs(&self) -> Option<String> {
        match self {
            Node::Newline | Node::EndOfInput | Node::Comment(_) | Node::Category(_) => None,
            Node::Command(n) => n.as_rhs(),
            Node::VariableAssignment(n) => n.as_rhs(),
        }
    }

    fn as_mid(&self) -> Option<String> {
        match self {
            Node::Newline | Node::EndOfInput | Node::Comment(_) | Node::Category(_) => None,
            Node::Command(n) => n.as_mid(),
            Node::VariableAssignment(n) => n.as_mid(),
        }
    }
}

impl Measure for VariableAssignmentNode {
    fn as_lhs(&self) -> Option<String> {
        Some(self.ident.to_string())
    }
    fn as_rhs(&self) -> Option<String> {
        Some(self.expression.to_string())
    }
    fn as_mid(&self) -> Option<String> {
        let has_lhs = self.as_lhs().map(|side| !side.is_empty());
        let has_rhs = self.as_rhs().map(|side| !side.is_empty());
        match (has_lhs, has_rhs) {
            (Some(l), Some(r)) if l && r => Some(String::from(" = ")),
            (Some(l), Some(r)) if l && !r => Some(String::from(" =")),
            _ => None,
        }
    }
}

impl Measure for CommandNode {
    fn as_lhs(&self) -> Option<String> {
        Some(self.ident.to_string())
    }
    fn as_rhs(&self) -> Option<String> {
        Some(
            self.parts
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
    fn as_mid(&self) -> Option<String> {
        let has_lhs = self.as_lhs().map(|side| !side.is_empty());
        let has_rhs = self.as_rhs().map(|side| !side.is_empty());
        match (has_lhs, has_rhs) {
            (Some(l), Some(r)) if l && r => Some(String::from(" = ")),
            (Some(l), Some(r)) if l && !r => Some(String::from(" =")),
            _ => None,
        }
    }
}

impl Format for Node {
    fn format(&self, config: Config, state: &BlockState) -> Result<String, fmt::Error> {
        match self {
            Node::EndOfInput => Ok(String::new()),
            Node::Newline => Ok("\n".to_string()),
            Node::Comment(n) => n.format(config, state),
            Node::Command(n) => n.format(config, state),
            Node::VariableAssignment(n) => n.format(config, state),
            Node::Category(n) => n.format(config, state),
        }
    }
}

impl Node {
    fn new(tag: &Pair<Rule>, config: Config) -> Node {
        match tag.as_rule() {
            Rule::comment => Node::Comment(CommentNode::new(tag)),
            Rule::newline => Node::Newline,
            Rule::command => Node::Command(CommandNode::new(tag)),
            Rule::variable_assignment => Node::VariableAssignment(VariableAssignmentNode::new(tag)),
            Rule::category => Node::Category(CategoryNode::new(tag, 0, config)),
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

/// A formatter for the hyprlang language.
#[derive(ClapParser, Debug)]
#[command(version)]
struct Args {
    /// How many spaces to use for indentation
    #[arg(short, long, default_value_t = 2)]
    spaces: u8,
}

struct File {
    blocks: Vec<Block>,
}

fn main() {
    let args = Args::parse();

    let mut file = String::new();
    std::io::stdin()
        .read_to_string(&mut file)
        .expect("Unable to read stdin.");

    let config = Config {
        tab_width: args.spaces,
    };

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
