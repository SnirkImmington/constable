//! If expression parser.

use lex::{tokens, Token, Tokenizer, TokenType, TokenData};
use parse::ast::*;
use parse::{Parser, ParseError, ParseResult};
use parse::symbol::{PrefixParser, Precedence};

/// Parses block and inline forms of prefix expr/block `if`.
///
/// # Examples
/// Inline if expression:
/// ```text
/// if expr => expr else expr
/// ```
///
/// This parser may have been called from an `IfBlockParser`
/// in order to parse the inline if when it was in the expression form.
/// However, block if form is not allowed in all expression places.
#[derive(Debug)]
pub struct IfExpressionParser { }
impl<T: Tokenizer> PrefixParser<Expression, T> for IfExpressionParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Expression> {
        debug_assert!(token.text == "if",
            "Invlaid token {:?} in IfExpressionParser", token);
        trace!("Parsing conditional of if expression");
        let condition = try!(parser.expression(Precedence::Min));
        trace!("Parsed if conditional");
        try!(parser.consume_name(TokenType::Symbol, tokens::InlineArrow));
        trace!("Consumed inline arrow token");
        let true_expr = try!(parser.expression(Precedence::Min));
        trace!("Parsed sucess half of conditional");
        try!(parser.consume_name(TokenType::Keyword, tokens::Else));
        trace!("Parsing else half of conditional");
        let else_expr = try!(parser.expression(Precedence::Min));
        let if_expr = IfExpression::new(token,
                                        Box::new(condition),
                                        Box::new(true_expr),
                                        Box::new(else_expr));
        Ok(Expression::IfExpression(if_expr))
    }
}
