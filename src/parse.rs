use crate::grammar::{HyprlangParser, Rule};
use pest::iterators::FlatPairs;
use pest::Parser;
use std::fmt;

pub type ParseIterator<'a> = FlatPairs<'a, Rule>;

#[derive(Debug)]
pub enum ParseError {
    InvalidInput(Box<dyn std::error::Error>),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidInput(e) => write!(f, "invalid input: {e}"),
        }
    }
}

pub fn get_file_tokens_iterator(file: &str) -> Result<ParseIterator<'_>, ParseError> {
    let pair = HyprlangParser::parse(Rule::file, file)
        .map_err(|e| ParseError::InvalidInput(Box::new(e)))
        .map(|mut parsed| unsafe { parsed.next().unwrap_unchecked() })?;

    Ok(pair.into_inner().flatten())
}
