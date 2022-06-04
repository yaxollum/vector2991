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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::run;
    use crate::ast::Ast;
    use crate::vector::Vector;

    fn check_expr_vector(expr: &str, v: Vector) {
        assert_eq!(run(expr), Ok(Ast::ReprCoord(v)));
    }

    #[test]
    fn test_sub() {
        check_expr_vector("(2,3)-(3,4)", Vector::new(-1.0, -1.0));
    }

    #[test]
    fn test_add() {
        check_expr_vector("(2,3)+(3,4)", Vector::new(5.0, 7.0));
    }

    #[test]
    fn test_sub_add() {
        check_expr_vector("(2,3)-(3,4)+(1,1)", Vector::new(0.0, 0.0));
    }

    #[test]
    fn test_mul_l() {
        check_expr_vector("2*(2,3)", Vector::new(4.0, 6.0));
    }

    #[test]
    fn test_mul_r() {
        check_expr_vector("(2,3)*3", Vector::new(6.0, 9.0));
    }

    #[test]
    fn test_mul_lr() {
        check_expr_vector("2*(2,3)*3", Vector::new(12.0, 18.0));
    }
}
