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
use std::fs::read_to_string;

const TAB_WIDTH: u8 = 4;

#[derive(Parser)]
#[grammar = "pest/grammar.pest"]
pub struct HyprlangParser;

#[derive(PartialEq)]
struct Comment {
    level: u8,
    comment: String,
}

impl Comment {
    fn new(tag: &Pair<Rule>, level: u8) -> Self {
        Comment {
            comment: text(tag),
            level,
        }
    }
}

impl fmt::Display for Comment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&" ".repeat((TAB_WIDTH * self.level).into()))?;
        formatter.write_str(&self.comment)
    }
}

#[derive(PartialEq)]
enum CommandExpression {
    Expression(String),
    Rule(String),
}

fn text(tag: &Pair<Rule>) -> String {
    tag.as_span().as_str().trim_end_matches(' ').to_string()
}

impl fmt::Display for CommandExpression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandExpression::Expression(e) => write!(formatter, "{e}"),
            CommandExpression::Rule(r) => write!(formatter, "{r}"),
        }
    }
}

#[derive(PartialEq)]
struct Command {
    ident: String,
    level: u8,
    parts: Vec<CommandExpression>,
    comment: Option<String>,
}

impl Command {
    fn new(tag: &Pair<Rule>, level: u8) -> Self {
        let mut ident = None;
        let mut parts = Vec::new();
        let mut comment = None;

        for part in tag.clone().into_inner() {
            match part.as_rule() {
                Rule::command_ident => {
                    ident = Some(text(&part));
                }
                Rule::command_expression => parts.push(CommandExpression::Expression(text(&part))),
                Rule::command_rule => parts.push(CommandExpression::Rule(text(&part))),
                Rule::comment => {
                    comment = Some(text(&part));
                }
                _ => unreachable!(),
            }
        }
        Command {
            ident: ident.expect("command must have an ident"),
            level,
            parts,
            comment,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&" ".repeat((TAB_WIDTH * self.level).into()))?;
        write!(formatter, "{} =", &self.ident)?;
        let full_expression = self
            .parts
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");

        if !full_expression.is_empty() {
            formatter.write_str(&(" ".to_string() + &full_expression))?;
        }

        if let Some(comment) = &self.comment {
            write!(formatter, " {}", &comment)?;
        }
        formatter.write_str("")
    }
}

#[derive(PartialEq)]
struct VariableAssignment {
    ident: String,
    expression: String,
    comment: Option<String>,
}

impl fmt::Display for VariableAssignment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} = {}", &self.ident, &self.expression)?;
        if let Some(comment) = &self.comment {
            write!(formatter, " {}", &comment)?;
        }

        formatter.write_str("")
    }
}

#[derive(PartialEq)]
enum CategoryInner {
    Command(Command),
    Category(Category),
    Comment(Comment),
    Newline,
}

impl fmt::Display for CategoryInner {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CategoryInner::Command(n) => formatter.write_str(&n.to_string()),
            CategoryInner::Category(n) => formatter.write_str(&n.to_string()),
            CategoryInner::Comment(n) => formatter.write_str(&n.to_string()),
            CategoryInner::Newline => formatter.write_str("\n"),
        }
    }
}

#[derive(PartialEq)]
struct Category {
    ident: String,
    level: u8,
    inner: Vec<CategoryInner>,
}

impl Category {
    fn new(tag: &Pair<Rule>, level: u8) -> Self {
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
                            Rule::command => inner
                                .push(CategoryInner::Command(Command::new(&inner_pair, level + 1))),
                            Rule::comment => inner
                                .push(CategoryInner::Comment(Comment::new(&inner_pair, level + 1))),
                            Rule::newline => inner.push(CategoryInner::Newline),
                            Rule::category => inner.push(CategoryInner::Category(Category::new(
                                &inner_pair,
                                level + 1,
                            ))),
                            _ => unreachable!(),
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        Category {
            ident: ident.expect("category must have an ident"),
            level,
            inner,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&" ".repeat((TAB_WIDTH * self.level).into()))?;

        write!(formatter, "{} {{", self.ident)?;
        for inner in &self.inner {
            formatter.write_str(&inner.to_string())?;
        }

        formatter.write_str(&" ".repeat((TAB_WIDTH * self.level).into()))?;
        formatter.write_str("}")
    }
}

#[derive(PartialEq)]
enum PrintableNode {
    Comment(Comment),
    Command(Command),
    VariableAssignment(VariableAssignment),
    Category(Category),
    Newline,
    EndOfInput,
}

impl fmt::Display for PrintableNode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == &PrintableNode::EndOfInput {
            return formatter.write_str("");
        }
        match self {
            PrintableNode::Comment(n) => formatter.write_str(&n.to_string()),
            PrintableNode::Command(n) => formatter.write_str(&n.to_string()),
            PrintableNode::VariableAssignment(n) => formatter.write_str(&n.to_string()),
            PrintableNode::Category(n) => formatter.write_str(&n.to_string()),
            PrintableNode::Newline => formatter.write_str("\n"),
            PrintableNode::EndOfInput => unreachable!(),
        }
    }
}

impl PrintableNode {
    fn new_comment(tag: &Pair<Rule>, level: u8) -> PrintableNode {
        PrintableNode::Comment(Comment::new(tag, level))
    }

    fn new_command(tag: &Pair<Rule>, level: u8) -> PrintableNode {
        PrintableNode::Command(Command::new(tag, level))
    }

    fn new_variable_assignment(tag: &Pair<Rule>) -> PrintableNode {
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
        PrintableNode::VariableAssignment(VariableAssignment {
            ident: ident.expect("variable_assignment  must have an ident"),
            expression: expression.expect("variable_assignment must have an expression"),
            comment,
        })
    }

    fn new_category(tag: &Pair<Rule>, level: u8) -> PrintableNode {
        PrintableNode::Category(Category::new(tag, level))
    }
}

fn get_file_nodes(pair: Pair<Rule>) -> Vec<PrintableNode> {
    let mut nodes = Vec::new();
    for tag in pair.into_inner() {
        match tag.as_rule() {
            Rule::EOI => {
                nodes.push(PrintableNode::EndOfInput);
            }
            Rule::comment => {
                nodes.push(PrintableNode::new_comment(&tag, 0));
            }
            Rule::newline => {
                nodes.push(PrintableNode::Newline);
            }
            Rule::command => {
                nodes.push(PrintableNode::new_command(&tag, 0));
            }
            Rule::variable_assignment => {
                nodes.push(PrintableNode::new_variable_assignment(&tag));
            }
            Rule::category => {
                nodes.push(PrintableNode::new_category(&tag, 0));
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
