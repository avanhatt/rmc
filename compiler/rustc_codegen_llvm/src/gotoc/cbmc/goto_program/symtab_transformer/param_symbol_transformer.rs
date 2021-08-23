// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::super::SymbolTable;
use super::Transformer;

use super::super::{Parameter, Type};

/// Struct for performing the identity transformation on a symbol table.
/// Mainly used as a demo/for testing.
pub struct ParameterSymbolTransformer {
    old_symbol_table: SymbolTable,
    new_symbol_table: SymbolTable,
}

impl ParameterSymbolTransformer {
    /// Perform an identity transformation on the given symbol table.
    pub fn transform(original_symbol_table: &SymbolTable) -> SymbolTable {
        dbg!("Running parameter symbol transformer");
        let new_symbol_table = SymbolTable::new(original_symbol_table.machine_model().clone());
        let old_symbol_table = original_symbol_table.clone();

        ParameterSymbolTransformer { old_symbol_table, new_symbol_table }
            .transform_symbol_table(original_symbol_table)
    }
}

impl Transformer for ParameterSymbolTransformer {
    /// Get reference to symbol table.
    fn symbol_table(&self) -> &SymbolTable {
        &self.new_symbol_table
    }

    /// Get mutable reference to symbol table.
    fn mut_symbol_table(&mut self) -> &mut SymbolTable {
        &mut self.new_symbol_table
    }

    /// Get owned symbol table.
    fn extract_symbol_table(self) -> SymbolTable {
        self.new_symbol_table
    }

    /// Transforms a function type (`return_type x(parameters)`)
    fn transform_type_code(&self, parameters: &[Parameter], return_type: &Box<Type>) -> Type {
        assert!(false);
        // Identity
        let transformed_parameters =
            parameters.iter().map(|parameter| self.transform_type_parameter(parameter)).collect();
        let transformed_return_type = self.transform_type(return_type);

        for param in parameters {
            assert!(
                self.old_symbol_table.contains(param.identifier().unwrap()),
                "Parameter {:?} for type {:?} -> {:?} is not present in the symbol table",
                param.identifier().unwrap(),
                parameters,
                return_type
            )
        }

        // Return the same function
        Type::code(transformed_parameters, transformed_return_type)
    }

    fn transform_type_variadic_code(
        &self,
        parameters: &[Parameter],
        return_type: &Box<Type>,
    ) -> Type {
        // Same implementation
        self.transform_type_code(parameters, return_type)
    }
}
