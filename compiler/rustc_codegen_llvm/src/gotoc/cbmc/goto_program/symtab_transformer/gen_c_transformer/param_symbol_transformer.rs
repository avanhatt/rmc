// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use super::super::Transformer;
use crate::gotoc::cbmc::goto_program::{
    Expr, Location, Parameter, Stmt, Symbol, SymbolTable, SymbolValues, Type,
};

/// Struct for performing the identity transformation on a symbol table.
/// Mainly used as a demo/for testing.
pub struct ParameterSymbolTransformer {
    new_symbol_table: SymbolTable,
}

impl ParameterSymbolTransformer {
    /// Perform an identity transformation on the given symbol table.
    pub fn transform(original_symbol_table: &SymbolTable) -> SymbolTable {
        let new_symbol_table = SymbolTable::new(original_symbol_table.machine_model().clone());

        ParameterSymbolTransformer { new_symbol_table }
            .transform_symbol_table(original_symbol_table)
    }

    /// Add identifier to a transformed parameter if it's missing;
    /// necessary when function wasn't originally a definition, e.g. extern functions,
    /// so that we can give them a function body.
    pub fn add_parameter_identifier(&mut self, parameter: &Parameter) -> Parameter {
        if parameter.identifier().is_some() {
            parameter.clone()
        } else {
            let name = format!("__{}", parameter.typ().to_identifier());
            let parameter_sym = self.mut_symbol_table().ensure(&name, |_symtab, name| {
                Symbol::variable(
                    name.to_string(),
                    name.to_string(),
                    parameter.typ().clone(),
                    Location::none(),
                )
            });
            parameter_sym.to_function_parameter()
        }
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

    // /// Transforms a function type (`return_type x(parameters)`)
    // fn transform_type_code(&mut self, parameters: &[Parameter], return_type: &Box<Type>) -> Type {
    //     // Identity
    //     let transformed_parameters =
    //         parameters.iter().map(|parameter| self.transform_type_parameter(parameter)).collect();
    //     let transformed_return_type = self.transform_type(return_type);

    // for param in parameters {
    //     if let Some(id) = param.identifier() {
    //         assert!(
    //             self.old_symbol_table.contains(id),
    //             "Parameter {:?} for type {:?} -> {:?} is not present in the symbol table",
    //             id,
    //             parameters,
    //             return_type
    //         )
    //     } else {
    //         println!("No parameter identifier for {:?}", parameters);
    //     }
    // }

    // // Return the same function
    // Type::code(transformed_parameters, transformed_return_type)
    // }

    // fn transform_type_variadic_code(
    //     &mut self,
    //     parameters: &[Parameter],
    //     return_type: &Box<Type>,
    // ) -> Type {
    //     // Same implementation
    //     self.transform_type_code(parameters, return_type)
    // }

    /// When indexing into a SIMD vector, cast to a pointer first to make legal indexing in C.
    /// `typ __attribute__((vector_size (size * sizeof(typ)))) var;`
    /// `((typ*) &var)[index]`
    /// Tracking issue: https://github.com/model-checking/rmc/issues/444
    fn transform_expr_index(&mut self, _typ: &Type, array: &Expr, index: &Expr) -> Expr {
        let transformed_array = self.transform_expr(array);
        let transformed_index = self.transform_expr(index);
        if transformed_array.typ().is_vector() {
            let base_type = transformed_array.typ().base_type().unwrap().clone();
            transformed_array.address_of().cast_to(base_type.to_pointer()).index(transformed_index)
        } else {
            transformed_array.index(transformed_index)
        }
    }

    /// Replace `extern` functions and values with `nondet` so linker doesn't break.
    fn transform_symbol(&mut self, symbol: &Symbol) -> Symbol {
        let mut new_symbol = symbol.clone();

        if symbol.is_extern {
            if symbol.typ.is_code() || symbol.typ.is_variadic_code() {
                // Replace `extern` function with one which returns `nondet`
                assert!(symbol.value.is_none(), "Extern function should have no body.");

                let transformed_typ = self.transform_type(&symbol.typ);

                // Fill missing parameter names with dummy name
                let parameters = transformed_typ
                    .parameters()
                    .unwrap()
                    .iter()
                    .map(|parameter| self.add_parameter_identifier(parameter))
                    .collect();
                let ret_typ = transformed_typ.return_type().unwrap().clone();
                let new_typ = if transformed_typ.is_code() {
                    Type::code(parameters, ret_typ.clone())
                } else {
                    Type::variadic_code(parameters, ret_typ.clone())
                };

                // Create body, which returns nondet
                let ret_e = if ret_typ.is_empty() { None } else { Some(Expr::nondet(ret_typ)) };
                let body = Stmt::ret(ret_e, Location::none());
                let new_value = SymbolValues::Stmt(body);

                new_symbol.is_extern = false;
                new_symbol.typ = new_typ;
                new_symbol.value = new_value;
            } else {
                // Replace `extern static`s and initialize in `main`
                assert!(
                    symbol.is_static_lifetime,
                    "Extern objects that aren't functions should be static variables."
                );
                let new_typ = self.transform_type(&symbol.typ);

                // Symbol is no longer extern
                new_symbol.is_extern = false;

                // Set location to none so that it is a global static
                new_symbol.location = Location::none();

                new_symbol.typ = new_typ;
                new_symbol.value = SymbolValues::None;
            }
        } else {
            // Handle all other symbols normally
            let new_typ = self.transform_type(&symbol.typ);
            let new_value = self.transform_value(&symbol.value);
            new_symbol.typ = new_typ;
            new_symbol.value = new_value;
        }

        new_symbol
    }
}
