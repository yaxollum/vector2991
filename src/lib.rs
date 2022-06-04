mod ast;
mod vector;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

use crate::ast::Ast;
use lalrpop_util::{lexer::Token, ParseError};

pub fn run(input: &str) -> Result<Ast, ParseError<usize, Token, &str>> {
    parser::AstParser::new().parse(input)
}
