extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parser;

use pest::error::Error;
// use pest::Parser;

use parser::ast::*;
use parser::parser::*;

#[test]
fn test1() {
    parse("114514");
    assert!(false);
}
