// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use cbmc::btree_map;
/// CBMC uses a very loose heuristic to reason about function pointers: it
/// assumes any function of the right type could be a target. For virtual
/// function pointer through a vtable, we can do much better than that. This
/// file build a map of virtual call sites to all of the possible trait
/// implementations that match that method and trait. We then can write out
/// this information to CBMC as function restrictions, improving verification
/// performance.

/// CBMC function restriction information:
///     http://cprover.diffblue.com/md__home_travis_build_diffblue_cbmc_doc_architectural_restrict-function-pointer.html
use rustc_data_structures::stable_map::FxHashMap;
use rustc_middle::ty::Ty;
use rustc_serialize::json::*;

/// This structure represents data about the vtable that we construct
/// Per trait, per method, which functions could it map to?
pub struct VtableCtx {
    pub restrict_vtable_fn_ptrs: bool,
    // Map: (normalized trait name, method index) -> possible implementations
    // TODO: make trait + method a type
    // TODO: value could be a ref to the symbol, Strings not great
    // TODO: file a bug so that the restriction API for CBMC also takes labels
    possible_methods: FxHashMap<(String, usize), Vec<String>>,
    call_sites: Vec<CallSite>,
    call_sites_map: FxHashMap<String, usize>, // this can die
    call_site_global_idx: usize,
}

/// Trait-defined method: the trait type and the vtable index of the method.
#[derive(Debug, Clone, PartialEq)]
struct TraitDefinedMethod<'tcx> {
    trait_type: Ty<'tcx>,
    vtable_idx: usize,
}

/// CBMC refers to call sites by index of use of function pointer in the
/// surrounding function
#[derive(Debug, Clone, PartialEq)]
struct CallSite {
    trait_name: String,
    vtable_idx: usize,
    function_location: String,
    call_idx: usize,
}

/// Constructor
impl VtableCtx {
    pub fn new(restrict_ptrs: bool) -> Self {
        Self {
            restrict_vtable_fn_ptrs: restrict_ptrs,
            possible_methods: FxHashMap::default(),
            call_sites_map: FxHashMap::default(),
            call_sites: Vec::new(),
            call_site_global_idx: 0,
        }
    }
}

// dyn Error + Sync + Send

/// Add and get data
impl VtableCtx {
    /// Add a possible implementation for a virtual method call.
    pub fn add_possible_method(&mut self, trait_ty: String, method: usize, imp: String) {
        assert!(self.restrict_vtable_fn_ptrs);
        let key = (trait_ty, method);
        if let Some(possibilities) = self.possible_methods.get_mut(&key) {
            possibilities.push(imp);
        } else {
            self.possible_methods.insert(key, vec![imp]);
        }
    }

    /// Add a given call site for a virtual function, incremementing the call
    /// site index.
    pub fn add_call_site(&mut self, trait_ty: String, method: usize, function_location: String) {
        assert!(self.restrict_vtable_fn_ptrs);
        let call_idx = if let Some(call_idx) = self.call_sites_map.get(&function_location) {
            *call_idx
        } else {
            // CBMC index is 1-indexed:
            // http://cprover.diffblue.com/md__home_travis_build_diffblue_cbmc_doc_architectural_restrict-function-pointer.html
            1
        };
        self.call_sites_map.insert(function_location.clone(), call_idx + 1);
        let site = CallSite {
            trait_name: trait_ty,
            vtable_idx: method,
            function_location: function_location,
            call_idx: call_idx,
        };
        self.call_sites.push(site);
    }

    pub fn get_call_site_global_idx(&mut self) -> usize {
        assert!(self.restrict_vtable_fn_ptrs);
        self.call_site_global_idx += 1;
        self.call_site_global_idx
    }

    // pub fn copy_drop_possibilities(&mut self, original_trait_ty: String, new_trait_ty: String) {
    //     let original_key = (original_trait_ty.clone(), 2 as VtableIdx);
    //     let new_key = (new_trait_ty.clone(), 2 as VtableIdx);
    //     let possibilities = self.possible_methods.get_mut(&original_key).map(|x| x.clone());
    //     if let Some(possibilities) = possibilities {
    //         let copy_drop = format!("good drop copy for {} {}", original_trait_ty, new_trait_ty);
    //         dbg!(copy_drop);
    //         dbg!(&possibilities);
    //         self.possible_methods.insert(new_key, possibilities);
    //     } else {
    //         let copy_drop_bad = format!("no drop copy for {} {}", original_trait_ty, new_trait_ty);
    //         dbg!(copy_drop_bad);
    //     }
    // }
}

/// Final data processing to write out for CBMC consumption
impl VtableCtx {
    /// Write out the restrictions to JSON, like so:
    // {
    //     "function_call_site_name": ["function1", "function2", ...],
    //      ...
    // }
    // TODO: return json to be used by link
    pub fn write_out_function_restrictions(&self, crate_name: String) {
        assert!(self.restrict_vtable_fn_ptrs);
        use std::io::Write;

        let mut output = btree_map![];
        for call_site in &self.call_sites {
            let key = (call_site.trait_name.clone(), call_site.vtable_idx);
            let cbmc_call_site_name = format!(
                "{}.function_pointer_call.{}",
                call_site.function_location, call_site.call_idx
            );
            if let Some(possibilities) = self.possible_methods.get(&key) {
                output.insert(cbmc_call_site_name, possibilities.to_json());
            } else {
                output.insert(cbmc_call_site_name, Vec::<String>::new().to_json());
            }
        }

        for ((trait_ref, idx), _) in &self.possible_methods {
            let key = (trait_ref, idx);
            if !self.call_sites.iter().any(|call_site| {
                (call_site.trait_name == *trait_ref) && (call_site.vtable_idx == *idx)
            }) {
                // AVH TODO debug
            }
        }

        // TODO: condition on whether output is there
        let json_data = Json::Object(output);
        let pretty_json = json_data.pretty();
        let filename = format!("/tmp/{}_restrictions", crate_name).replace("::", "_");
        let mut out_file = ::std::fs::File::create(filename).unwrap();
        write!(out_file, "{}", pretty_json.to_string()).unwrap();
    }
}
