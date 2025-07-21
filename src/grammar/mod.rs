use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct HyprlangParser;
