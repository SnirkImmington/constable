//! Abstract syntax tree definitions.
//!
//! This module contains definitions of all of the
//! AST node types used to parse a protosnirk program,
//! with a `Unit` being the root of the syntax tree.
//!
//! Currently, the parser and checkers do not run
//! transformative passes to the AST. Instead, many
//! nodes contain `parse::Id`s which point to data
//! tables collected in various passes, such as
//! symbol or type information.

mod expression;
mod item;
mod stmt;
mod operator;
pub mod types;

pub use self::expression::*;
pub use self::item::*;
pub use self::stmt::*;
pub use self::types::*;
pub use self::operator::Operator;

use std::cell::{Cell, RefCell, Ref};

use lex::Token;
use parse::{TypeId, ScopedId};

/// Basic identifier type
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub token: Token,
    id: RefCell<ScopedId>,
    type_id: Cell<TypeId>
}
impl Identifier {
    pub fn new(token: Token) -> Self {
        Identifier {
            token,
            id: RefCell::default(),
            type_id: Cell::default()
        }
    }
    pub fn get_name(&self) -> &str {
        &self.token.text
    }
    pub fn get_token(&self) -> &Token {
        &self.token
    }

    pub fn get_id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.id.borrow()
    }

    pub fn set_id(&self, index: ScopedId) {
        *self.id.borrow_mut() = index;
    }

    pub fn get_type_id(&self) -> TypeId {
        self.type_id.get()
    }

    pub fn set_type_id(&self, id: TypeId) {
        self.type_id.set(id);
    }

    pub fn get_type_id(&self) -> TypeId {
        self.type_id.get()
    }

    pub fn set_type_id(&self, id: TypeId) {
        self.type_id.set(id);
    }
}
impl Into<Token> for Identifier {
    fn into(self) -> Token {
        self.token
    }
}

/// Collection of statements which may have an expression value
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    /// Statements in the block
    pub statements: Vec<Statement>,
    /// Identifier used for typechecking.
    scope_id: RefCell<ScopedId>,
    /// `TypeId` used for typechecking.
    type_id: Cell<TypeId>
}
impl Block {
    /// Create a new block from the given statements and scope id.
    pub fn new(statements: Vec<Statement>) -> Block {
        Block {
            statements,
            scope_id: RefCell::default(),
            type_id: Cell::default()
        }
    }
    pub fn has_value(&self) -> bool {
        if self.statements.len() == 0 {
            return false
        }
        let last_ix = self.statements.len() - 1;
        // TODO actual analysis
        for (ix, statement) in self.statements.iter().enumerate() {
            if ix == last_ix {
                return statement.has_value()
            }
            // else if stmt == return {
            //     return stmt.has_value()
            // }
        }
        return false
    }
    pub fn get_stmts(&self) -> &[Statement] {
        &self.statements
    }
    pub fn get_id<'a>(&'a self) -> Ref<'a, ScopedId> {
        self.scope_id.borrow()
    }
    pub fn set_id(&self, id: ScopedId) {
        *self.scope_id.borrow_mut() = id;
    }
    pub fn get_type_id(&self) -> TypeId {
        self.type_id.get()
    }
    pub fn set_type_id(&self, value: TypeId) {
        self.type_id.set(value);
    }
}
