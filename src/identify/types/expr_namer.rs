use ast::*;

use parse::ScopedId;

use check::{CheckerError, ErrorCollector};
use identify::NameScopeBuilder;
use visit;
use visit::visitor::*;

/// Does the second pass of scope checking
/// to identify item `Identifier`s in expressions.
#[derive(Debug, PartialEq)]
pub struct ExpressionTypeIdentifier<'err, 'builder> {
    // Only need to look at type defs
    builder: &'builder NameScopeBuilder,
    errors: &'err mut ErrorCollector
}

impl<'err, 'builder> ExpressionTypeIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder NameScopeBuilder)
               -> ExpressionTypeIdentifier<'err, 'builder> {
        ExpressionTypeIdentifier { errors, builder }
    }
}

impl<'err, 'builder> DefaultUnitVisitor
    for ExpressionTypeIdentifier<'err, 'builder> { }

impl<'err, 'builder> DefaultItemVisitor
    for ExpressionTypeIdentifier<'err, 'builder> { }

impl<'err, 'builder> DefaultBlockVisitor
    for ExpressionTypeIdentifier<'err, 'builder> { }

impl<'err, 'builder> DefaultStmtVisitor
    for ExpressionTypeIdentifier<'err, 'builder> { }

impl<'err, 'builder> ExpressionVisitor
    for ExpressionTypeIdentifier<'err, 'builder> {

    fn visit_literal_expr(&mut self, literal: &Literal) { }

    fn visit_var_ref(&mut self, ident: &Identifier) { }

    fn visit_unary_op(&mut self, unary_op: &UnaryOperation) {
        // Only interested in declarations...
    }

    fn visit_binary_op(&mut self, binary_op: &BinaryOperation) {
        // Only interested in declarations...
    }

    fn visit_fn_call(&mut self, fn_call: &FnCall) {
        // In the future, it would be possible to declare types here
        // similar to Rust's turbofish but for now we skip.
    }

    fn visit_assignment(&mut self, assign: &Assignment) { }

    fn visit_declaration(&mut self, declaration: &Declaration) {
        if let Some(type_decl) = declaration.get_type_decl() {
            self.visit_type_expr(type_decl);
        }
    }

    fn visit_if_expr(&mut self, if_expr: &IfExpression) { }
}

impl <'err, 'builder> TypeVisitor
    for ExpressionTypeIdentifier<'err, 'builder> {

    fn visit_named_type_expr(&mut self, named_ty: &NamedTypeExpression) {
        // Code here is basically duplicated from `item_namer`.
        // They'll diverge once the item namer also identifies new types.
        if let Some(type_id) = self.builder.get(named_ty.get_ident().get_name()) {
            named_ty.get_ident().set_id(type_id.clone());
        }
        else {
            trace!("Encountered unexpected type name {}",
                named_ty.get_ident().get_name());
            let err_text = format!("Unknown type {}",
            named_ty.get_ident().get_name());
            self.errors.add_error(CheckerError::new(
                named_ty.get_ident().get_token().clone(), vec![], err_text
            ));
        }
    }

    fn visit_primitive_type_expr(&mut self, prim: &Primitive) {

    }

    fn visit_fn_type_expr(&mut self, fn_type: &FnTypeExpression) {
        visit::walk_fn_type(self, fn_type);
    }
}
