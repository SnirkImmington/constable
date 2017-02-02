//! Return statement parser

use lex::{tokens, Token, Tokenizer, TokenType};
use parse::{Parser, ParseResult, ParseError, Precedence};
use parse::ast::*;
use parse::symbol::PrefixParser;

/// Parses return statements
///
/// # Examples
/// ```text
/// return x + 1 + 3 * 4
///   ^    ->right:expression
/// ```
#[derive(Debug)]
pub struct ReturnParser { }
impl<T: Tokenizer> PrefixParser<Statement, T> for ReturnParser {
    fn parse(&self, parser: &mut Parser<T>, token: Token) -> ParseResult<Statement> {
        debug_assert!(token.text == tokens::Return,
                      "Return parser called with non-return {:?}", token);
        // If the next statement is on a newline then empty return.
        // Also empty return if next token is deindent
        // Should also check for an indent block to ensure sprious indentation is an error.
        if parser.peek_is_newline(&token) {
            return Ok(Statement::Return(Return::new(token, None)))
        }
        let inner_expr = try!(parser.expression(Precedence::Return));
        let inner = try!(inner_expr.expect_value());
        Ok(Statement::Return(Return::new(token, Box::new(inner))))
    }
}

#[cfg(test)]
mod tests {
    // TODO test
    // - return <expr>
    // - return // no expr
    // - return // expr next line
}
