

use ast::*;
use check::{CheckerError, ErrorCollector};
use identify::NameScopeBuilder;
use visit::visitor::*;
use visit::*;

/// Identifies names of items that can be used in expressions,
/// namely function definitions.
pub struct ItemVarIdentifier<'err, 'builder> {
    errors: &'err mut ErrorCollector,
    builder: &'builder mut NameScopeBuilder,
    current_id: ScopedId
}

impl<'err, 'builder> ItemVarIdentifier<'err, 'builder> {
    pub fn new(errors: &'err mut ErrorCollector,
               builder: &'builder mut NameScopeBuilder,
               scoped_id: ScopedId)
               -> ItemVarIdentifier<'err, 'builder> {
        ItemVarIdentifier {
            errors,
            builder,
            current_id: scoped_id
        }
    }

    pub fn into_last_id(self) -> ScopedId {
        self.current_id
    }
}

impl<'err, 'builder> DefaultUnitVisitor
    for ItemVarIdentifier<'err, 'builder> { }

impl<'err, 'builder> ItemVisitor for ItemVarIdentifier<'err, 'builder> {
    fn visit_block_fn_decl(&mut self, block_fn: &BlockFnDeclaration) {
        if let Some(previous_def) = self.builder.get(block_fn.get_name()) {
            // fn has been previously defined
            let error_text = format!("Function {} is already declared",
                block_fn.get_name());
            self.errors.add_error(CheckerError::new(
                block_fn.get_token().clone(), vec![], error_text
            ));
            return
        }
        debug_assert!(block_fn.get_ident().get_id().is_default(),
            "Block fn {:?} already had an ID", block_fn);
        self.current_id.increment();
        let fn_id = self.current_id.clone();
        trace!("Created id {:?} for block fn {}",
            fn_id, block_fn.get_name());
        self.builder.define_local(block_fn.get_name().to_string(),
                                 fn_id.clone());
        block_fn.get_ident().set_id(fn_id);

        // Also name the params here
        let mut param_id = self.current_id.pushed();
        //self.builder.new_scope()
        for &(ref param, ref _param_type) in block_fn.get_params() {
            // Identify params internally with {fn_name}:{param_name}.
            let param_name = format!("{}:{}",
                block_fn.get_name(), param.get_name());

            if let Some(previous_def) = self.builder.get(&param_name) {
                let error_text = format!(
                    "Parameter {} of function {} is already declared",
                    param.get_name(), block_fn.get_name());
                self.errors.add_error(CheckerError::new(
                    block_fn.get_token().clone(), vec![], error_text
                ));
                return // Stop checking params if there's a dupe.
            }

            param_id.increment();
            trace!("Created id {:?} for {} param {}",
                param_id, block_fn.get_name(), param.get_name());
            self.builder.define_local(param_name, param_id.clone());
            param.set_id(param_id.clone());
        }
    }
}
