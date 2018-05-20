//! Parses variable declarations

// This will become more complex with tuple declarations
// and other pattern declaration types.

use lex::{Token, Tokenizer, TokenType, TokenData};
use ast::*;
use parse::{Parser, ParseResult, ParseError};
use parse::symbol::{PrefixParser, Precedence};

///
/// # Examples
/// ```text
/// let mut            x        :      type?   =         6 + 3
/// ^:.  ^:mutable  ->name:name ^check ^opt   (skip) ->value:expression
/// ```
#[derive(Debug)]
pub struct DeclarationParser { }
impl<T: Tokenizer> PrefixParser<Expression, T> for DeclarationParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        debug_assert!(token.get_type() == TokenType::Let,
                      "Let parser called with non-let token {:?}", token);
        trace!("Parsing declaration for {}", token);
        let is_mutable = parser.next_type() == TokenType::Mut;
        if is_mutable {
            parser.consume();
        }
        trace!("Found mutability: {}", is_mutable);
        let name = try!(parser.lvalue());
        trace!("Got name {:?}", name);
        let decl_type = if parser.next_type() == TokenType::Colon {
            trace!("Found type declaration");
            parser.consume();
            Some(try!(parser.type_expr()))
        }
        else {
            trace!("No type declaration");
            None
        };
        try!(parser.consume_type(TokenType::Equals));
        trace!("Consumed =, parsing rvalue");
        let value_expr = try!(parser.expression(Precedence::Min));
        let value = try!(value_expr.expect_value());
        trace!("Got rvalue {:?}", value);
        Ok(Expression::Declaration(Declaration::new(
            name, is_mutable, decl_type, Box::new(value)
        )))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::cell::RefCell;

    use lex::{Token, TokenData, TokenType, TextLocation};
    use ast::{Declaration, Expression, Statement, Block, Literal, Identifier};
    use parse::symbol::{PrefixParser, DeclarationParser};
    use parse::ScopedId;
    use parse::tests as parse_tests;

    const LET_TOKEN: Token = Token {
        data: TokenData::Keyword,
        text: Cow::Borrowed("let"),
        location: TextLocation {
            column: 0, line: 0, index: 0
        }
    };

    const X_TOKEN: Token = Token {
        data: TokenData::Ident,
        text: Cow::Borrowed("x"),
        location: TextLocation {
            column: 0, line: 0, index: 0
        }
    };


    const LITERAL_ZERO: Expression = Expression::Literal(Literal {
        token: Token {
            data: TokenData::NumberLiteral(0f64),
            text: Cow::Borrowed("0"),
            location: TextLocation {
                column: 0, line: 0, index: 0
            }
        }
    });

    #[test]
    fn it_parses_let_var_eq_value() {
        let mut parser = parse_tests::parser("x = 0");
        let ident = Identifier {
            index: RefCell::new(ScopedId::default()),
            token: X_TOKEN.clone() // Not looking at token here?
        };
        let expected = Declaration::new(LET_TOKEN.clone(), false, ident, Box::new(LITERAL_ZERO.clone()));
        let parsed = DeclarationParser { }.parse(&mut parser, LET_TOKEN.clone()).unwrap();
        parse_tests::expression_match(&Expression::Declaration(expected), &parsed);
    }

    #[test]
    fn it_parses_let_mut_var_eq_value() {
        let mut parser = parse_tests::parser("mut x = 0");
        let ident = Identifier {
            index: RefCell::new(ScopedId::default()),
            token: X_TOKEN.clone() // Not looking at token here?
        };
        let expected = Declaration::new(LET_TOKEN.clone(), true, ident, Box::new(LITERAL_ZERO.clone()));
        let parsed = DeclarationParser { }.parse(&mut parser, LET_TOKEN.clone()).unwrap();
        parse_tests::expression_match(&Expression::Declaration(expected), &parsed);
    }
}
