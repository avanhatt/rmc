// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use rustc_data_structures::stable_map::FxHashMap;

/// Index into a vtable
type VtableIdx = usize;

/// CBMC refers to call sites by index of use of function pointer in the
/// surrounding function
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

// Add data
impl VtableCtx {
    pub fn add_possible_method(&mut self, trait_ty: String, method: usize, imp: String) {
        let key = (trait_ty, method as VtableIdx);
        if let Some(possibilities) = self.possible_methods.get_mut(&key) {
            possibilities.push(imp);
        } else {
            self.possible_methods.insert(key, vec![imp]);
        }
    }

    pub fn add_call_site(&mut self, trait_ty: String, method: usize, function_location: String) {
        let call_idx = if let Some(call_idx) = self.call_sites_map.get(&function_location) {
            *call_idx
        } else {
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

// Final data processing to write out for CBMC consumption
impl VtableCtx {
    pub fn write_out_function_restrictions(&self) {
        // TODO
    }
}
