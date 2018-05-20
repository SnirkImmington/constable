//! Assignment parser

use lex::{tokens, Token, Tokenizer, TokenType, TokenData};
use ast::*;
use parse::{Parser, ParseResult, ParseError};
use parse::symbol::{InfixParser, Precedence};

/// Parses an assignment expresion.
///
/// # Examples
/// ```text
///   x    =   y + 2
/// (left) ^ ->right:expression
/// ```
#[derive(Debug)]
pub struct AssignmentParser { }
impl<T: Tokenizer> InfixParser<Expression, T> for AssignmentParser {
    fn parse(&self, parser: &mut Parser<T>,
             left: Expression, _token: Token) -> ParseResult<Expression> {
        debug_assert!(_token.text == tokens::Equals,
                      "Assign parser called with non-assign token {:?}", _token);
        let ident = try!(left.expect_identifier());
        let right_expr = try!(parser.expression(Precedence::Assign));
        let right = try!(right_expr.expect_value());
        Ok(Expression::Assignment(Assignment::new(ident, Box::new(right))))
    }
    fn get_precedence(&self) -> Precedence {
        Precedence::Assign
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use lex::{Token, TokenData, TokenType};
    use ast::{Expression, Assignment, Literal, Identifier};
    use parse::symbol::{InfixParser, AssignmentParser};
    use parse::tests as parse_tests;

    // TODO test
    // - var = expr
    // - var = block?
    // - expr = var

    #[test]
    fn it_parses_lvalue_eq_expr() {
        let mut parser = parse_tests::parser("5");
        let lvalue_ident = Identifier::new(Token {
            data: TokenData::Ident,
            text: Cow::Borrowed("x"),
            .. Default::default()
        });
        let assign_token = Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("="),
            .. Default::default()
        };
        let five_token = Token {
            data: TokenData::NumberLiteral(5f64),
            .. Default::default()
        };
        let lvalue = Expression::VariableRef(lvalue_ident.clone());
        let expr = AssignmentParser { }.parse(&mut parser, lvalue, assign_token);
        let expected = Expression::Assignment(Assignment::new(lvalue_ident,
            Box::new(Expression::Literal(Literal::new(five_token)))));
        parse_tests::expression_match(&expected, &expr.unwrap());
    }

    #[test]
    fn it_fails_lvalue_eq_block() {
        let mut parser = parse_tests::parser("do\n    return x");
        let lvalue = Expression::Literal(Literal::new(Token {
            data: TokenData::NumberLiteral(5f64),
            .. Default::default()
        }));
        let assign_token = Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("="),
            .. Default::default()
        };
        let expr = AssignmentParser { }.parse(&mut parser, lvalue, assign_token);
        assert!(expr.is_err());

    }

    #[test]
    fn it_fails_for_bad_lvalue() {
        let mut parser = parse_tests::parser("5");
        let lvalue = Expression::Literal(Literal::new(Token {
            data: TokenData::NumberLiteral(5f64),
            .. Default::default()
        }));
        let assign_token = Token {
            data: TokenData::Symbol,
            text: Cow::Borrowed("="),
            .. Default::default()
        };
        let expr = AssignmentParser { }.parse(&mut parser, lvalue, assign_token);
        assert!(expr.is_err());
    }
}
