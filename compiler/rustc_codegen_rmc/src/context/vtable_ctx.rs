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
use rustc_serialize::json::*;

/// Index into a vtable
type VtableIdx = usize;

/// CBMC refers to call sites by index of use of function pointer in the
/// surrounding function
#[derive(Debug, Clone)]
struct CallSite {
    trait_name: String,
    vtable_idx: VtableIdx,
    function_location: String,
    call_idx: usize,
}

/// This structure represents data about the vtable that we construct
/// Per trait, per method, which functions could it map to?
pub struct VtableCtx {
    // Map: (normalized trait name, method index) -> possible implementations
    possible_methods: FxHashMap<(String, VtableIdx), Vec<String>>,
    call_sites_map: FxHashMap<String, usize>,
    call_sites: Vec<CallSite>,
}

/// Constructor
impl VtableCtx {
    pub fn new() -> Self {
        Self {
            possible_methods: FxHashMap::default(),
            call_sites_map: FxHashMap::default(),
            call_sites: Vec::new(),
        }
    }
}

/// Add data
impl VtableCtx {
    /// Add a possible implementation for a virtual method call.
    pub fn add_possible_method(&mut self, trait_ty: String, method: usize, imp: String) {
        let key = (dbg!(trait_ty), method as VtableIdx);
        if let Some(possibilities) = self.possible_methods.get_mut(&key) {
            possibilities.push(imp);
        } else {
            self.possible_methods.insert(key, vec![imp]);
        }
    }

    /// Add a given call site for a virtual function, incremementing the call
    /// site index.
    pub fn add_call_site(&mut self, trait_ty: String, method: usize, function_location: String) {
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
            vtable_idx: method as VtableIdx,
            function_location: function_location,
            call_idx: call_idx,
        };
        self.call_sites.push(site);
    }
}

/// Final data processing to write out for CBMC consumption
impl VtableCtx {
    /// Write out the restrictions to JSON, like so:
    // {
    //     "function_call_site_name": ["function1", "function2", ...],
    //      ...
    // }

    pub fn write_out_function_restrictions(&self) {
        use std::io::Write;

        let mut output = btree_map![];

        for call_site in &self.call_sites {
            // dbg!(call_site);
            let key = (call_site.trait_name.clone(), call_site.vtable_idx);
            let possibilities = self.possible_methods.get(&key).unwrap();
            assert!(possibilities.len() > 1);
            let cbmc_call_site_name = format!(
                "{}.function_pointer_call.{}",
                call_site.function_location, call_site.call_idx
            );
            output.insert(
                cbmc_call_site_name,
                possibilities
                    .iter()
                    .map(|x| format!("(int (*)(void *)){}", x))
                    .collect::<Vec<String>>()
                    .to_json(),
            );
            // dbg!(possibilities);
        }
        let json_data = Json::Object(output);
        let pretty_json = json_data.pretty();
        // dbg!(json_data);
        // dbg!(encode(&json_data));
        let mut out_file = ::std::fs::File::create("restrictions").unwrap();
        write!(out_file, "{}", pretty_json.to_string()).unwrap();
    }
}
