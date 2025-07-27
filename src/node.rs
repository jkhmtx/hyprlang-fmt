use crate::components::category::CategoryNode;
use crate::components::command::CommandNode;
use crate::components::comment::CommentNode;
use crate::components::variable_assignment::VariableAssignmentNode;
use crate::format::{Format, Sections};
use crate::grammar::Rule;
use crate::state::{BlockState, Config};
use pest::iterators::Pair;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Node {
    Comment(CommentNode),
    Command(CommandNode),
    VariableAssignment(VariableAssignmentNode),
    Category(CategoryNode),
    Newline,
}

impl Sections for Node {
    fn as_lhs(&self) -> Option<String> {
        match self {
            Node::Newline | Node::Comment(_) | Node::Category(_) => None,
            Node::Command(n) => n.as_lhs(),
            Node::VariableAssignment(n) => n.as_lhs(),
        }
    }
    fn as_rhs(&self) -> Option<String> {
        match self {
            Node::Newline | Node::Comment(_) | Node::Category(_) => None,
            Node::Command(n) => n.as_rhs(),
            Node::VariableAssignment(n) => n.as_rhs(),
        }
    }

    fn as_mid(&self) -> Option<String> {
        match self {
            Node::Newline | Node::Comment(_) | Node::Category(_) => None,
            Node::Command(n) => n.as_mid(),
            Node::VariableAssignment(n) => n.as_mid(),
        }
    }
}

impl Format for Node {
    fn format(&self, config: Config, state: &BlockState) -> Result<String, fmt::Error> {
        match self {
            Node::Newline => Ok("\n".to_string()),
            Node::Comment(n) => n.format(config, state),
            Node::Command(n) => n.format(config, state),
            Node::VariableAssignment(n) => n.format(config, state),
            Node::Category(n) => n.format(config, state),
        }
    }
}

impl Node {
    pub fn maybe(tag: Option<&Pair<Rule>>, config: Config) -> Option<Node> {
        tag.and_then(|tag| {
            if tag.as_rule() == Rule::EOI {
                return None;
            }

            Some(match tag.as_rule() {
                Rule::comment => Node::Comment(CommentNode::new(tag)),
                Rule::newline => Node::Newline,
                Rule::command => Node::Command(CommandNode::new(tag)),
                Rule::variable_assignment => {
                    Node::VariableAssignment(VariableAssignmentNode::new(tag))
                }
                Rule::category => Node::Category(CategoryNode::new(tag, 0, config)),
                _ => unreachable!(),
            })
        })
    }
}
