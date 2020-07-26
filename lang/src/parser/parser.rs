use super::ast::*;
use crate::parser::utils::ParserError;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "parser/octopus.pest"]
pub enum Octopus {}

impl ParseFrom<Rule> for Literal {
    fn parse_from(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::unit_lit => Literal::Unit,
            Rule::bool_lit => Literal::Bool(pair.as_str().parse().unwrap()),
            Rule::char_lit => Literal::Char(pair.as_str().parse().unwrap()),
            Rule::string_lit => Literal::Char(pair.as_str().parse().unwrap()),
            Rule::number_lit => Literal::Char(pair.as_str().parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

pub fn parse(inp: &str) {
    let ts = Octopus::parse(Rule::literal, inp).unwrap().next().unwrap();
    let ast = Literal::parse_from(ts);
    eprintln!("out: {:?}", ast);
}

// impl ParseFrom<Rule> for
