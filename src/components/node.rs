use crate::components::category::CategoryNode;
use crate::components::command::CommandNode;
use crate::components::comment::CommentNode;
use crate::components::variable_assignment::VariableAssignmentNode;
use crate::format::{Format, Sections, SectionsView};
use crate::grammar::Rule;
use crate::state::{BlockState, Config};
use pest::iterators::Pair;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Node<'a> {
    Comment(CommentNode),
    Command(CommandNode),
    VariableAssignment(VariableAssignmentNode),
    Category(CategoryNode<'a>),
    Newline,
}

impl Sections for Node<'_> {
    fn as_sections(&self) -> Option<SectionsView<'_>> {
        match self {
            Node::Newline | Node::Comment(_) | Node::Category(_) => None,
            Node::Command(n) => n.as_sections(),
            Node::VariableAssignment(n) => n.as_sections(),
        }
    }
}

impl Format for Node<'_> {
    fn format(&self, config: &Config, state: &BlockState) -> Result<String, fmt::Error> {
        match self {
            Node::Newline => Ok("\n".to_string()),
            Node::Comment(n) => n.format(config, state),
            Node::Command(n) => n.format(config, state),
            Node::VariableAssignment(n) => n.format(config, state),
            Node::Category(n) => n.format(config, state),
        }
    }
}

impl Node<'_> {
    pub fn maybe<'a>(tag: Option<&Pair<Rule>>, config: &'a Config) -> Option<Node<'a>> {
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
