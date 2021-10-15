// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

/// CBMC uses a very loose heuristic to reason about function pointers: it
/// assumes any function of the right type could be a target. For virtual
/// function pointer through a vtable, we can often do much better than that.
/// This file build a map of virtual call sites to all of the possible trait
/// implementations that match that method and trait. We then can write out
/// this information to CBMC as function restrictions, improving verification
/// performance. CBMC asserts false if the restrictions are not met, so this
/// optimization is sound even if we get the possible implementations set wrong.

/// CBMC function restriction information:
///     http://cprover.diffblue.com/md__home_travis_build_diffblue_cbmc_doc_architectural_restrict-function-pointer.html
use crate::GotocCtx;
use cbmc::btree_map;
use cbmc::goto_program::{Expr, Location, Stmt, Symbol, Type};
use rustc_data_structures::stable_map::FxHashMap;
use tracing::debug;

use rustc_serialize::json::*;

/// This structure represents data about the vtable that we construct
/// Per trait, per method, which functions could virtual call sites
/// possibly refer to?
pub struct VtableCtx {
    // Option to actually enable restrictions
    pub restrict_vtable_fn_ptrs: bool,

    // Map: (normalized trait name, method index) -> possible implementations
    possible_methods: FxHashMap<TraitDefinedMethod, Vec<String>>,

    // All sites where a virtual call takes place
    call_sites: Vec<CallSite>,

    // Internal tracing of index needed for call site wrappers
    call_site_global_idx: usize,
}

/// Trait-defined method: the trait type and the vtable index of the method.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TraitDefinedMethod {
    // Needs to be a string to handle both the MIR and Gotoc types
    trait_name: String,
    vtable_idx: usize,
}

impl ToJson for TraitDefinedMethod {
    fn to_json(&self) -> Json {
        let mut output = btree_map![
            ("trait_name".to_string(), self.trait_name.to_json()),
            ("vtable_idx".to_string(), self.vtable_idx.to_json()),
        ];
        Json::Object(output)
    }
}

/// CBMC refers to call sites by index of use of function pointer in the
/// surrounding function
#[derive(Debug, Clone, PartialEq)]
struct CallSite {
    trait_method: TraitDefinedMethod,
    function_location: String,
}

impl ToJson for CallSite {
    fn to_json(&self) -> Json {
        let mut output = btree_map![
            ("trait_method".to_string(), self.trait_method.to_json()),
            ("function_location".to_string(), self.function_location.to_json()),
        ];
        Json::Object(output)
    }
}

/// Constructor
impl VtableCtx {
    pub fn new(restrict_ptrs: bool) -> Self {
        debug!("Restricting vtable function pointers? {:?}", restrict_ptrs);
        Self {
            restrict_vtable_fn_ptrs: restrict_ptrs,
            possible_methods: FxHashMap::default(),
            call_sites: Vec::new(),
            call_site_global_idx: 0,
        }
    }
}

/// Interface for codegen to add possible methods
impl VtableCtx {
    /// Add a possible implementation for a virtual method call.
    pub fn add_possible_method(&mut self, trait_name: String, method: usize, imp: String) {
        assert!(self.restrict_vtable_fn_ptrs);
        let key = TraitDefinedMethod { trait_name: trait_name.clone(), vtable_idx: method };

        if method == 2 {
            let drop = format!("{} {}", trait_name.clone(), imp.clone());
            dbg!(drop);
        }

        if method == 7 {
            let seven = format!("{} {}", trait_name.clone(), imp.clone());
            dbg!(seven);
        }

        if let Some(possibilities) = self.possible_methods.get_mut(&key) {
            possibilities.push(imp);
        } else {
            self.possible_methods.insert(key, vec![imp]);
        }
    }

    /// The vtable index for drop is 2
    pub fn drop_index() -> usize {
        2
    }
}

/// Internal tracking helpers
impl VtableCtx {
    fn get_call_site_global_idx(&mut self) -> usize {
        assert!(self.restrict_vtable_fn_ptrs);
        self.call_site_global_idx += 1;
        self.call_site_global_idx
    }

    /// Add a given call site for a virtual function, incremementing the call
    /// site index.
    fn add_call_site(&mut self, trait_name: String, method: usize, function_location: String) {
        assert!(self.restrict_vtable_fn_ptrs);
        let site = CallSite {
            trait_method: TraitDefinedMethod { trait_name: trait_name, vtable_idx: method },
            function_location: function_location,
        };
        self.call_sites.push(site);
    }
}

impl<'tcx> GotocCtx<'tcx> {
    /// Wrap a virtual call through a function pointer and restrict the
    /// possible targets.
    ///
    /// We need to wrap because CBMC employs a hard-to-get-right naming scheme
    /// for restrictions: the call site is named for its index in  a given
    /// function. We don't have a good way to track _all_ function pointers
    /// within the function, so wrapping the call to a function that makes a
    /// single virtual function pointer call makes the naming unambiguous.
    ///
    /// This can be simplified if CBMC implemented label-based restrictions.
    pub fn virtual_call_with_restricted_fn_ptr(
        &mut self,
        trait_ty: Type,
        vtable_idx: usize,
        fn_ptr: Expr,
        args: Vec<Expr>,
    ) -> Expr {
        assert!(self.vtable_ctx.restrict_vtable_fn_ptrs);

        // Crate-based naming scheme for wrappers
        let full_crate_name = self.full_crate_name().to_string();
        let wrapper_name = format!(
            "restricted_call_{}_{}",
            full_crate_name,
            self.vtable_ctx.get_call_site_global_idx()
        );

        // We only have the Gotoc type, we need to normalize to match the MIR type.
        assert!(trait_ty.is_struct_tag());
        let normalized_trait_name = trait_ty.type_name().unwrap().replace("tag-", "");
        self.vtable_ctx.add_call_site(
            normalized_trait_name.clone(),
            vtable_idx,
            wrapper_name.clone(),
        );

        let call =
            format!("{} {} {}", wrapper_name.clone(), normalized_trait_name.clone(), vtable_idx);
        dbg!(call);

        // Declare the wrapper's parameters
        let func_exp: Expr = fn_ptr.dereference();
        let fn_type = func_exp.typ().clone();
        let parameters: Vec<Symbol> = fn_type
            .parameters()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, parameter)| {
                let name = format!("{}_{}", wrapper_name.clone(), i);
                let param = Symbol::variable(
                    name.to_string(),
                    name.to_string(),
                    parameter.typ().clone(),
                    Location::none(),
                );
                self.symbol_table.insert(param.clone());
                param
            })
            .collect();

        // Finish constructing the wrapper type
        let ret_typ = fn_type.return_type().unwrap().clone();
        let param_typs = parameters.clone().iter().map(|p| p.to_function_parameter()).collect();
        let new_typ = if fn_type.is_code() {
            Type::code(param_typs, ret_typ.clone())
        } else {
            Type::variadic_code(param_typs, ret_typ.clone())
        };

        // Build the body: call the original function pointer
        let body = func_exp
            .clone()
            .call(parameters.iter().map(|p| p.to_expr()).collect())
            .ret(Location::none());

        // Build and insert the wrapper function itself
        let sym = Symbol::function(
            &wrapper_name,
            new_typ,
            Some(Stmt::block(vec![body], Location::none())),
            None,
            Location::none(),
        );
        self.symbol_table.insert(sym.clone());
        sym.to_expr().call(args.to_vec())
    }
}

/// Final data processing to write out for CBMC consumption
impl VtableCtx {
    /// Write out the restrictions to JSON, like so:
    /// {
    ///     "foo.function_pointer_call.1": ["function1", "function2", ...],
    ///      ...
    /// }
    pub fn get_virtual_function_restrictions(&self) -> rustc_serialize::json::Json {
        assert!(self.restrict_vtable_fn_ptrs);

        let mut output = btree_map![];

        output.insert("call_sites".to_string(), self.call_sites.to_json());

        let mut possible_methods = vec![];
        for (trait_def, methods) in &self.possible_methods {
            let value = btree_map![
                ("trait_def".to_string(), trait_def.to_json()),
                ("methods".to_string(), methods.to_json()),
            ];
            possible_methods.push(value);
        }

        output.insert("possible_methods".to_string(), possible_methods.to_json());

        let mut single_crate = btree_map![];
        for call_site in &self.call_sites {
            // CBMC index is 1-indexed:
            // http://cprover.diffblue.com/md__home_travis_build_diffblue_cbmc_doc_architectural_restrict-function-pointer.html
            let cbmc_call_site_name =
                format!("{}.function_pointer_call.1", call_site.function_location);

            // Look up all possibilities, defaulting to the empty set
            if let Some(possibilities) = self.possible_methods.get(&call_site.trait_method) {
                single_crate.insert(cbmc_call_site_name, possibilities.to_json());
            } else {
                single_crate.insert(cbmc_call_site_name, Vec::<String>::new().to_json());
            };
        }
        output.insert("single_crate".to_string(), single_crate.to_json());
        Json::Object(output)
    }
}
