use pest::iterators::Pair;
use pest::Parser;

use super::utils;

pub trait ParseFrom<T>
where
    Self: std::marker::Sized,
{
    fn parse_from(pair: Pair<T>) -> Self;
}

#[derive(Debug)]
pub struct Module(pub Vec<TopLevel>);

#[derive(Debug)]
pub enum TopLevel {
    TraitDef(TraitDef),
    ImplDef(ImplDef),
    TypeDef(TypeDef),
    FunDef(FunDef),
    Constexpr(Constexpr),
}

#[derive(Debug)]
pub struct TraitDef {
    pub name: String,
}

#[derive(Debug)]
pub struct ImplDef {}

#[derive(Debug)]
pub enum TypeDef {}

#[derive(Debug)]
pub struct FunDef {}

#[derive(Debug)]
pub struct Constexpr {}

#[derive(Debug)]
pub enum Literal {
    Unit,
    Bool(bool),
    Char(String),
    String_(String),
    Number(String),
}
