//! Parses variable declarations

// This will become much more complex with tuple declarations
// and other pattern declaration types.

use lex::{tokens, Token, Tokenizer, TokenType, TokenData};
use parse::{Parser, ParseResult, ParseError, Precedence};
use parse::ast::*;
use parse::symbol::PrefixParser;

///
/// # Examples
/// ```text
/// let mut            x          =         6 + 3
/// ^:.  ^:mutable  ->name:name (skip) ->value:expression
/// ```
#[derive(Debug)]
pub struct DeclarationParser { }
impl<T: Tokenizer> PrefixParser<Expression, T> for DeclarationParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        debug_assert!(token.text == tokens::Let,
                      "Let parser called with non-let token {:?}", token);
        trace!("Parsing declaration for {}", token);
        let is_mutable = parser.peek().text == tokens::Mut;
        if is_mutable {
            parser.consume();
        }
        trace!("Found mutability: {}", is_mutable);
        let name = try!(parser.lvalue());
        trace!("Got name {:?}", name);
        try!(parser.consume_name(TokenType::Symbol, tokens::Equals));
        trace!("Consumed =, parsing rvalue");
        // TODO allow for block here
        let value_expr = try!(parser.expression(Precedence::Min));
        let value = try!(value_expr.expect_value());
        println!("Got rvalue {:?}", value);
        Ok(Expression::Declaration(Declaration::new(name.into(), is_mutable, Box::new(value))))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use lex::{Token, TokenData, TokenType};
    use parse::ast::{Declaration, Expression, Statement, Block, Literal};
    use parse::symbol::{PrefixParser, DeclarationParser};
    use parse::tests as parse_tests;

    const LET_TOKEN: Token = Token {
        data: TokenData::Keyword,
        text: Cow::Borrowed("let"),
        .. Default::default()
    };

    const LITERAL_ZERO: Expression = Expression::Literal(Literal {
        token: Token {
            data: TokenData::NumberLiteral(0f64),
            .. Default::default()
        }});

    #[test]
    fn it_parses_let_var_eq_value() {
        let mut parser = parse_tests::parser("x = 0");
        let expected = Declaration::new(LET_TOKEN.clone(), false, Box::new(LITERAL_ZERO.clone()));
        let parsed = DeclarationParser { }.parse(&mut parser, LET_TOKEN.clone()).unwrap();
        parse_tests::expression_eq(Expression::Declaration(expected), parsed);
    }

    #[test]
    fn it_parses_let_mut_var_eq_value() {
        let mut parser = parse_tests::parser("mut x = 0");
        let expected = Declaration::new(LET_TOKEN.clone(), true, Box::new(LITERAL_ZERO.clone()));
        let parsed = DeclarationParser { }.parse(&mut parser, LET_TOKEN.clone()).unwrap();
        parse_tests::expression_eq(Expression::Declaration(expected), parsed);

    }

    /* // TODO: design requires some thinking, not supported right now.
    fn it_parses_let_var_eq_block() {
        let mut parser = parse_tests::parser("x = do\n    0");
        let block = Block::new(vec![Statement::Expression(LITERAL_ZERO.clone())]);
        let expected = Declaration::new(LET_TOKEN.clone(),
                                        true,
                                        Box::new(Statement::DoBlock(block));
        let parsed = DeclarationParser { }.parse(&mut parser, LET_TOKEN.clone()).unwrap();
        parse_tests::expression_eq(Expression::Declaration(expected), parsed);

    }
    */
}