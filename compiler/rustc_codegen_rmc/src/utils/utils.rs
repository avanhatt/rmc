// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
use super::super::codegen::TypeExt;
use crate::GotocCtx;
use cbmc::btree_string_map;
use cbmc::goto_program::{Expr, Location, Stmt, SymbolTable, Type};
use tracing::debug;

// Should move into rvalue
//make this a member function
pub fn slice_fat_ptr(typ: Type, data: Expr, len: Expr, symbol_table: &SymbolTable) -> Expr {
    Expr::struct_expr(typ, btree_string_map![("data", data), ("len", len)], symbol_table)
}

pub fn dynamic_fat_ptr(typ: Type, data: Expr, vtable: Expr, symbol_table: &SymbolTable) -> Expr {
    Expr::struct_expr(typ, btree_string_map![("data", data), ("vtable", vtable)], symbol_table)
}

impl<'tcx> GotocCtx<'tcx> {
    /// RMC does not currently support all MIR constructs.
    /// When we hit a construct we don't handle, we have two choices:
    /// We can use the `unimplemented!()` macro, which causes a compile time failure.
    /// Or, we can use this function, which inserts an `assert(false, "FOO is not currently supported by RMC")` into the generated code.
    /// This means that if the unimplemented feature is dynamically used by the code being verified, we will see an assertion failure.
    /// If it is not used, we the assertion will pass.
    /// This allows us to continue to make progress parsing rust code, while remaining sound (thanks to the `assert(false)`)
    ///
    /// TODO: https://github.com/model-checking/rmc/issues/8 assume the required validity constraints for the nondet return
    /// TODO: https://github.com/model-checking/rmc/issues/9 Have a parameter that decides whether to `assume(0)` to block further traces or not
    pub fn codegen_unimplemented(
        &mut self,
        operation_name: &str,
        t: Type,
        loc: Location,
        url: &str,
    ) -> Expr {
        // We should possibly upgrade this to a warning in the future, but for now emit at least something
        debug!("codegen_unimplemented: {} at {}", operation_name, loc.short_string());

        let body = vec![
            // Assert false to alert the user that there is a path that uses an unimplemented feature.
            Stmt::assert_false(
                &format!(
                    "{} is not currently supported by RMC. Please post your example at {} ",
                    operation_name, url
                ),
                loc.clone(),
            ),
            // Assume false to block any further exploration of this path.
            Stmt::assume(Expr::bool_false(), loc.clone()),
            t.nondet().as_stmt(loc.clone()).with_location(loc.clone()), //TODO assume rust validity contraints
        ];

        Expr::statement_expression(body, t).with_location(loc)
    }
}

impl<'tcx> GotocCtx<'tcx> {
    /// Dereference a boxed type `std::boxed::Box<T>` to get a `*T`.
    ///
    /// WARNING: This is based on a manual inspection of how boxed types are currently
    /// a) implemented by the rust standard library
    /// b) codegenned by RMC.
    /// If either of those change, this will almost certainly stop working.
    pub fn deref_box(&self, e: Expr) -> Expr {
        // Internally, a Boxed type is stored as a chain of structs.
        // In particular:
        // `Box<T>` is an owning reference to an allocation of type T on the heap.
        // It has a pointer of type `ptr::Unique<T>` and an allocator of type `alloc::Global`
        // Unique<T> is an owning raw pointer to a location in memory.
        // So given a Box<T>, we can follow the chain to get the desired pointer.
        // If either rustc or RMC changes how boxed types are represented, this will need to be
        // updated.
        //
        // The following C code is the result of running `rmc --gen-c` on rust with boxed types:
        // Given a boxed type (note that Rust can reorder fields to improve struct packing):
        // ```
        // struct std::boxed::Box<[u8]>
        // {
        //   struct std::alloc::Global 1;
        //   struct std::ptr::Unique<[u8]> 0;
        // };
        // ```
        // We follow the Unique pointer:
        // ```
        // struct std::ptr::Unique<[u8]>
        // {
        //   struct std::marker::PhantomData<[u8]> _marker;
        //   struct &[u8] pointer;
        // };
        // ```
        // And notice that its `.pointer` field is exactly what we want.
        self.assert_is_rust_box_like(e.typ());
        e.member("0", &self.symbol_table).member("pointer", &self.symbol_table)
    }

    /// Box<T> initializer
    /// `boxed_type the_box = >>> { .0=nondet(), .1={ ._marker=nondet(), .pointer=boxed_value } } <<<`
    /// `boxed_type` is the type of the resulting expression
    pub fn box_value(&self, boxed_value: Expr, boxed_type: Type) -> Expr {
        self.assert_is_rust_box_like(&boxed_type);
        let get_field_type = |struct_typ, field| {
            self.symbol_table.lookup_field_type_in_type(struct_typ, field).unwrap().clone()
        };
        let unique_ptr_typ = get_field_type(&boxed_type, "0");
        self.assert_is_rust_unique_pointer_like(&unique_ptr_typ);
        let unique_ptr_pointer_typ = get_field_type(&unique_ptr_typ, "pointer");
        assert_eq!(&unique_ptr_pointer_typ, boxed_value.typ());
        let unique_ptr_val = Expr::struct_expr_with_nondet_fields(
            unique_ptr_typ,
            btree_string_map![("pointer", boxed_value),],
            &self.symbol_table,
        );
        let boxed_val = Expr::struct_expr_with_nondet_fields(
            boxed_type,
            btree_string_map![("0", unique_ptr_val),],
            &self.symbol_table,
        );
        boxed_val
    }

    /// Best effort check if the struct represents a rust "std::alloc::Global".
    pub fn assert_is_rust_global_alloc_like(&self, t: &Type) {
        // TODO: A std::alloc::Global appears to be an empty struct, in the cases we've seen.
        // Is there something smarter we can do here?
        assert!(t.is_struct_like());
        let components = self.symbol_table.lookup_components_in_type(t).unwrap();
        assert_eq!(components.len(), 0);
    }

    /// Best effort check if the struct represents a rust "std::marker::PhantomData".
    pub fn assert_is_rust_phantom_data_like(&self, t: &Type) {
        // TODO: A std::marker::PhantomData appears to be an empty struct, in the cases we've seen.
        // Is there something smarter we can do here?
        assert!(t.is_struct_like());
        let components = self.symbol_table.lookup_components_in_type(t).unwrap();
        assert_eq!(components.len(), 0);
    }

    /// Best effort check if the struct represents a Rust "Box". May return false positives.
    pub fn assert_is_rust_box_like(&self, t: &Type) {
        // struct std::boxed::Box<[u8; 8]>::15334369982748499855
        // {
        //   // 1
        //   struct std::alloc::Global::13633191317886109837 1;
        //   // 0
        //   struct std::ptr::Unique<[u8; 8]>::14713681870393313245 0;
        // };
        assert!(t.is_struct_like());
        let components = self.symbol_table.lookup_components_in_type(t).unwrap();
        assert_eq!(components.len(), 2);
        for c in components {
            match c.name().to_string().as_str() {
                "0" => self.assert_is_rust_unique_pointer_like(&c.typ()),
                "1" => self.assert_is_rust_global_alloc_like(&c.typ()),
                _ => panic!("Unexpected component {} in {:?}", c.name(), t),
            }
        }
    }

    /// Checks if the struct represents a Rust "std::ptr::Unique"
    pub fn assert_is_rust_unique_pointer_like(&self, t: &Type) {
        // struct std::ptr::Unique<[u8; 8]>::14713681870393313245
        // {
        //   // _marker
        //   struct std::marker::PhantomData<[u8; 8]>::18073278521438838603 _marker;
        //   // pointer
        //   struct [u8::16712579856250238426; 8] *pointer;
        // };
        assert!(t.is_struct_like());
        let components = self.symbol_table.lookup_components_in_type(t).unwrap();
        assert_eq!(components.len(), 2);
        for c in components {
            match c.name().to_string().as_str() {
                "_marker" => self.assert_is_rust_phantom_data_like(&c.typ()),
                "pointer" => {
                    assert!(c.typ().is_pointer() || c.typ().is_rust_fat_ptr(&self.symbol_table))
                }
                _ => panic!("Unexpected component {} in {:?}", c.name(), t),
            }
        }
    }
}
