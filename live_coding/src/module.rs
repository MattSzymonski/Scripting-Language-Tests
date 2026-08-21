//! Self-contained patch-module source generation.
//!
//! For one changed hot function, the host generates a cdylib that contains:
//! the project's lib.rs consts/types/statics + the changed function's REAL
//! new body, a dependency-address table (filled by the host at load), and
//! forwarding wrappers (each jumping through the dependency table back into
//! the base DLL) for only the hot functions the changed function transitively
//! reaches - SLICED from the full graph, so a leaf edit compiles a handful of
//! functions instead of the whole 100+ wrapper skeleton.  The base prologue
//! of the changed function is then patched to this module.

use crate::analysis::{
    extract_extern_function_definitions, extract_top_level_consts, extract_top_level_functions,
    extract_top_level_imports, extract_top_level_statics, extract_top_level_type_definitions,
};
use crate::symbols::{parse_params, symbol_path, HotSymbol};

/// A plain `pub fn` declaration (NOT exported - the host finds it through
/// `project_resolve_symbol`): either the real new body of the changed
/// function, or a forwarding wrapper that jumps through the dependency table
/// to the base DLL's copy of the function.
pub fn build_hot_definition(
    key: &str,
    name: &str,
    params: &str,
    ret: &str,
    body: &str,
    index: usize,
    changed_key: &str,
) -> String {
    let ret_suffix = if ret.is_empty() {
        String::new()
    } else {
        format!(" -> {ret}")
    };
    // Real body only for the exact changed function (compared by qualified
    // key, so a lib/module name twin never both gets a real body).
    if key == changed_key {
        // The changed function gets its real new body.
        format!("pub fn {name}({params}){ret_suffix} {{\n{body}\n}}\n")
    } else {
        // Everyone else becomes a thin wrapper that calls back into the base
        // DLL through the dependency table, so the changed function's call
        // sites (unchanged text) still resolve, but execute the base code.
        let (types, names) = parse_params(params);
        let type_list = types.join(", ");
        let fn_type = if ret.is_empty() {
            format!("fn({type_list})")
        } else {
            format!("fn({type_list}) -> {ret}")
        };
        let call_args = names.join(", ");
        format!(
            "pub fn {name}({params}){ret_suffix} {{\n    let function: {fn_type} = unsafe {{ std::mem::transmute::<usize, {fn_type}>(crate::dependency_address({index})) }};\n    function({call_args})\n}}\n"
        )
    }
}

/// Build the `mod modules { ... }` skeleton for the patch module: every hot
/// symbol that lives inside the module graph becomes a forwarding wrapper
/// (or the real body if it is the changed function), preserving the exact
/// `crate::modules::module_NNN::...` paths the function bodies use.
pub fn build_modules_skeleton(symbols: &[HotSymbol], changed_key: &str) -> String {
    // Group the graph symbols by module, preserving first-seen order.
    let mut module_order: Vec<String> = Vec::new();
    let mut module_defs: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();
    let mut root_defs: Vec<String> = Vec::new();
    for symbol in symbols {
        let definition = build_hot_definition(
            &symbol_path(symbol),
            &symbol.name,
            &symbol.params,
            &symbol.ret,
            &symbol.body,
            symbol.index,
            changed_key,
        );
        if symbol.scope == "modules" {
            root_defs.push(definition);
        } else if let Some(module_name) = symbol.scope.strip_prefix("modules::") {
            if !module_defs.contains_key(module_name) {
                module_order.push(module_name.to_string());
            }
            module_defs
                .entry(module_name.to_string())
                .or_default()
                .push(definition);
        }
        // scope == "top" is handled by the caller.
    }

    // Sliced patch modules may not need any module-graph function at all.
    if root_defs.is_empty() && module_order.is_empty() {
        return String::new();
    }
    let mut block = String::from("mod modules {\n");
    for definition in &root_defs {
        block.push_str(definition);
        block.push('\n');
    }
    for module_name in &module_order {
        block.push_str(&format!("    pub mod {module_name} {{\n"));
        for definition in &module_defs[module_name] {
            block.push_str("    ");
            block.push_str(definition);
        }
        block.push_str("    }\n");
    }
    block.push_str("}\n");
    block
}

/// The exported entry-point names the generated patch module must provide
/// (filled from the consumer's [`crate::session::LiveCodingContract`] at call
/// time).  The module always exports a dependency-table setter and a
/// hot-symbol resolver; its `set_api` comes from the project's own definition
/// (injected verbatim).
pub struct PatchModuleNames<'a> {
    /// The resolver export name (e.g. `project_resolve_symbol`).
    pub resolver: &'a str,
    /// The API-table export name (e.g. `project_set_api`) - always injected
    /// so the host can hand the module the API pointer.
    pub set_api: &'a str,
    /// The dependency-table export name (e.g. `project_set_dependencies`).
    pub set_dependencies: &'a str,
}

/// Transitive call closure of the changed function (for patch-module
/// slicing): every hot symbol whose bare name appears in the changed body (or
/// in any transitively reached helper body) gets a forwarding wrapper, and
/// every non-hot helper that appears gets its real body injected and its body
/// scanned too (fixed point).  Returns `(hot_names, helper_names)`.
///
/// Deliberately name-based and conservative: a name that shows up as a local
/// variable / field / method name just pulls in one extra harmless wrapper.
/// A MISSED edge (e.g. a `use ... as` alias) is a compile error in the patch
/// module, which the host already handles by falling back to a full rebuild -
/// never wrong code.
fn compute_closure(
    changed: &HotSymbol,
    symbols: &[HotSymbol],
    helper_definitions: &[(String, String)],
) -> (
    std::collections::HashSet<String>,
    std::collections::HashSet<String>,
) {
    let mut hot_needed: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut helper_needed: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut queue: Vec<String> = vec![changed.body.clone()];
    let mut scanned_bodies: std::collections::HashSet<String> = std::collections::HashSet::new();

    while let Some(body) = queue.pop() {
        if !scanned_bodies.insert(body.clone()) {
            continue;
        }
        let mask = crate::analysis::code_mask(&body);
        let identifiers = identifiers_in(&body, &mask);
        // Hot symbols reachable from this body become forwarding wrappers.
        for symbol in symbols {
            if identifiers.contains(&symbol.name) {
                hot_needed.insert(symbol.name.clone());
            }
        }
        // Non-hot helpers reachable from this body get real bodies and are
        // scanned in turn (their bodies may reach further hot symbols).
        for (name, definition) in helper_definitions {
            if !helper_needed.contains(name) && identifiers.contains(name) {
                helper_needed.insert(name.clone());
                queue.push(definition.clone());
            }
        }
    }
    (hot_needed, helper_needed)
}

/// Every whole identifier appearing in `body` (strings/comments masked out).
/// Word boundaries are non-alphanumeric / non-underscore, so
/// `modules::run_update(` contributes `run_update` and `get_aaa()` contributes
/// `get_aaa`.
fn identifiers_in(body: &str, mask: &[bool]) -> std::collections::HashSet<String> {
    let bytes = body.as_bytes();
    let n = bytes.len();
    let mut identifiers = std::collections::HashSet::new();
    let mut i = 0usize;
    while i < n {
        if mask[i] && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
            let start = i;
            while i < n && mask[i] && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            identifiers.insert(body[start..i].to_string());
        } else {
            i += 1;
        }
    }
    identifiers
}

/// The function name from a top-level function definition (which may start
/// with an attribute block, e.g. `#[inline]\nfn foo(...)`).
fn function_name(definition: &str) -> String {
    let Some(marker) = definition.find("fn ") else {
        return String::new();
    };
    let rest = &definition[marker + 3..];
    rest.split(['(', ' ', '\n', '\t'])
        .next()
        .unwrap_or("")
        .to_string()
}

/// Build the self-contained module for one changed hot function: the
/// project's OWN lib.rs imports/consts/types/statics (injected verbatim, so
/// the module is a faithful copy of the project's layout), a dependency-
/// address table (filled by the host at load) and - SLICED to the changed
/// function's transitive call closure - the changed function's real body plus
/// forwarding wrappers for only the hot functions it (transitively) reaches.
///
/// This keeps a leaf edit (e.g. `get_aaa`) compiling a handful of functions
/// instead of the whole 100+ wrapper skeleton, so rustc typechecks far less.
/// Unreachable helpers/modules/plumbing are left out; a missed edge degrades
/// to a compile error -> full-rebuild fallback, never wrong code.
pub fn build_patch_module_source(
    lib_source: &str,
    symbols: &[HotSymbol],
    changed_key: &str,
    names: &PatchModuleNames,
) -> String {
    let imports = extract_top_level_imports(lib_source).join("\n");
    let consts = extract_top_level_consts(lib_source).join("\n");
    let type_definitions = extract_top_level_type_definitions(lib_source).join("\n");
    let statics = extract_top_level_statics(lib_source).join("\n");

    // All non-extern helper definitions (name -> full definition) and the
    // project's plumbing exports (name -> full definition incl. attributes).
    let helper_definitions: Vec<(String, String)> = extract_top_level_functions(lib_source)
        .into_iter()
        .map(|definition| (function_name(&definition), definition))
        .collect();
    let extern_definitions = extract_extern_function_definitions(lib_source);

    // Compute the closure.  If the changed symbol is unknown (shouldn't
    // happen), fall back to everything - exactly like the unsliced behaviour.
    let (hot_needed, helper_needed) = match symbols
        .iter()
        .find(|symbol| symbol_path(symbol) == changed_key)
    {
        Some(changed) => compute_closure(changed, symbols, &helper_definitions),
        None => (
            symbols.iter().map(|symbol| symbol.name.clone()).collect(),
            helper_definitions
                .iter()
                .map(|(name, _)| name.clone())
                .collect(),
        ),
    };

    // The changed function (real body) + every other hot symbol the closure
    // reaches (forwarding wrappers through the dependency table).
    let needed_symbols: Vec<HotSymbol> = symbols
        .iter()
        .filter(|symbol| symbol_path(symbol) == changed_key || hot_needed.contains(&symbol.name))
        .cloned()
        .collect();

    // The project's plumbing exports: `set_api` is always injected (the host
    // calls it on every patch module); any other non-hot extern the closure
    // reaches is injected too.  The resolver is excluded (generated below)
    // and hot exports (e.g. `project_update`) are emitted as wrappers/real
    // bodies in `top_definitions` / the module skeleton instead.
    let plumbing_externs = extern_definitions
        .into_iter()
        .filter(|(name, _)| {
            name == names.set_api || hot_needed.contains(name) || helper_needed.contains(name)
        })
        .filter(|(name, _)| {
            name != names.resolver
                && !symbols
                    .iter()
                    .any(|symbol| symbol.scope == "top" && symbol.name == *name)
        })
        .map(|(_, definition)| definition)
        .collect::<Vec<_>>()
        .join("\n");

    // Non-hot helpers the closure reaches (e.g. `state` when `project_update`
    // is the changed function), injected verbatim.  Hot helpers are emitted
    // as wrappers/real-bodies in `top_definitions` / the module skeleton.
    let helper_functions = helper_definitions
        .iter()
        .filter(|(name, _)| {
            helper_needed.contains(name)
                && !symbols
                    .iter()
                    .any(|symbol| symbol.scope == "top" && symbol.name == *name)
        })
        .map(|(_, definition)| definition.clone())
        .collect::<Vec<_>>()
        .join("\n");

    let modules_skeleton = build_modules_skeleton(&needed_symbols, changed_key);

    // Top-level hot symbols in the closure: real body if it is the one being
    // patched, otherwise a forwarding wrapper.
    let top_definitions = needed_symbols
        .iter()
        .filter(|symbol| symbol.scope == "top")
        .map(|symbol| {
            build_hot_definition(
                &symbol_path(symbol),
                &symbol.name,
                &symbol.params,
                &symbol.ret,
                &symbol.body,
                symbol.index,
                changed_key,
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    // The patch module needs its own resolver so the host can resolve the
    // (non-exported) hot functions by name in the patch DLL too.
    let resolver_arms = needed_symbols
        .iter()
        .map(|symbol| {
            let path = symbol_path(symbol);
            format!("        {path:?} => {path} as usize,")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let resolver = format!(
        "#[unsafe(no_mangle)]\npub extern \"C\" fn {}(name: *const std::os::raw::c_char) -> usize {{\n\
         \x20   if name.is_null() {{\n\
         \x20       return 0;\n\
         \x20   }}\n\
         \x20   let name = unsafe {{ std::ffi::CStr::from_ptr(name) }};\n\
         \x20   let Ok(name) = name.to_str() else {{ return 0; }};\n\
         \x20   match name {{\n\
         {resolver_arms}\n\
         \x20       _ => 0,\n\
         \x20   }}\n\
         }}\n",
        names.resolver
    );

    format!(
        "#![allow(dead_code)]\n\
         \n\
         {imports}\n\
         \n\
         {consts}\n\
         \n\
         {type_definitions}\n\
         \n\
         {statics}\n\
         \n\
         {plumbing_externs}\n\
         \n\
         {helper_functions}\n\
         \n\
         static mut DEPENDENCY_ADDRESSES: *const usize = std::ptr::null();\n\
         \n\
         #[no_mangle]\n\
         #[allow(private_interfaces)]\n\
         pub extern \"C\" fn {}(addresses: *const usize) {{\n\
             unsafe {{ DEPENDENCY_ADDRESSES = addresses; }}\n\
         }}\n\
         \n\
         fn dependency_address(index: usize) -> usize {{\n\
             unsafe {{ *DEPENDENCY_ADDRESSES.add(index) }}\n\
         }}\n\
         \n\
         {top_definitions}\n\
         \n\
         {modules_skeleton}\n\
         \n\
         {resolver}\n",
        names.set_dependencies
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbols::HotSymbol;

    fn symbol(name: &str, scope: &str, index: usize, body: &str) -> HotSymbol {
        HotSymbol {
            name: name.to_string(),
            params: "input: i32".to_string(),
            body: body.to_string(),
            ret: "i32".to_string(),
            scope: scope.to_string(),
            index,
        }
    }

    #[test]
    fn changed_function_gets_real_body() {
        let definition = build_hot_definition("get_aaa", "get_aaa", "", "i32", "42", 0, "get_aaa");
        assert!(definition.contains("pub fn get_aaa() -> i32 {"));
        assert!(definition.contains("42"));
        assert!(!definition.contains("dependency_address"));
    }

    #[test]
    fn unchanged_function_gets_forwarding_wrapper() {
        let definition =
            build_hot_definition("get_aaa", "get_aaa", "input: i32", "i32", "42", 3, "other");
        assert!(definition.contains("crate::dependency_address(3)"));
        assert!(definition.contains("std::mem::transmute::<usize, fn(i32) -> i32>"));
    }

    #[test]
    fn name_twins_get_independent_real_body_decisions() {
        // The lib helper is the changed one; the module twin (same bare name)
        // must still become a wrapper (keys differ).
        let lib = build_hot_definition(
            "scale_value_003",
            "scale_value_003",
            "input: i32",
            "i32",
            "new body",
            0,
            "scale_value_003",
        );
        let module = build_hot_definition(
            "modules::module_003::scale_value_003",
            "scale_value_003",
            "value: i32",
            "i32",
            "old body",
            1,
            "scale_value_003",
        );
        assert!(lib.contains("new body"));
        assert!(!module.contains("old body"));
        assert!(module.contains("dependency_address(1)"));
    }

    #[test]
    fn modules_skeleton_preserves_module_paths() {
        let symbols = vec![
            symbol("run_update", "modules", 0, "old"),
            symbol("tick_003", "modules::module_003", 1, "old"),
        ];
        let skeleton = build_modules_skeleton(&symbols, "modules::module_003::tick_003");
        assert!(skeleton.contains("mod modules {"));
        assert!(skeleton.contains("pub mod module_003 {"));
        // The changed module fn keeps its body; run_update is a wrapper.
        assert!(skeleton.contains("old"));
    }

    #[test]
    fn patch_module_source_has_contract_and_resolver() {
        let lib_source = r#"
            use std::ffi::c_void;
            #[repr(C)]
            struct ProjectApi {
                get_state: extern "C" fn() -> *mut c_void,
            }
            const QUAD_SIZE: f32 = 46.0;
            static mut API: *const ProjectApi = std::ptr::null();
            #[unsafe(no_mangle)]
            #[allow(private_interfaces)]
            pub extern "C" fn project_set_api(api: *const ProjectApi) {
                unsafe { API = api; }
            }
            fn state() -> &'static mut ProjectApi {
                unsafe { &mut *(((*API).get_state)() as *mut ProjectApi) }
            }
            fn get_aaa() -> i32 { 3 }
        "#;
        let symbols = vec![symbol("get_aaa", "top", 0, "3")];
        let names = PatchModuleNames {
            resolver: "project_resolve_symbol",
            set_api: "project_set_api",
            set_dependencies: "project_set_dependencies",
        };
        // A DIFFERENT function is the changed one, so get_aaa becomes a
        // forwarding wrapper through dependency index 0.
        let source = build_patch_module_source(lib_source, &symbols, "project_update", &names);
        // The project's own definitions are injected verbatim...
        assert!(source.contains("pub extern \"C\" fn project_set_api"));
        assert!(source.contains("static mut API"));
        assert!(source.contains("fn state()"));
        assert!(source.contains("struct ProjectApi"));
        assert!(source.contains("use std::ffi::c_void;"));
        // ...keeping the ABI-relevant attributes (a mirror without #[repr(C)]
        // would get Rust's unspecified default layout and misread state).
        assert!(source.contains("#[repr(C)]"));
        // ...and the module still provides its own dependency table, resolver
        // and the wrapper for get_aaa.
        assert!(source.contains("project_set_dependencies"));
        assert!(source.contains("fn dependency_address(index: usize)"));
        assert!(source.contains("crate::dependency_address(0)"));
        assert!(source.contains("\"get_aaa\" => get_aaa as usize,"));
        assert!(source.contains("QUAD_SIZE"));
    }

    #[test]
    fn resolver_and_set_dependencies_use_contract_names() {
        let lib_source = "fn get_aaa() -> i32 { 3 }\n";
        let symbols = vec![symbol("get_aaa", "top", 0, "3")];
        let names = PatchModuleNames {
            resolver: "resolve_my_hot_symbol",
            set_api: "project_set_api",
            set_dependencies: "hand_me_dependencies",
        };
        let source = build_patch_module_source(lib_source, &symbols, "project_update", &names);
        assert!(source.contains("fn resolve_my_hot_symbol(name: *const std::os::raw::c_char)"));
        assert!(
            source.contains("pub extern \"C\" fn hand_me_dependencies(addresses: *const usize)")
        );
        // The mini-engine default names must not leak in.
        assert!(!source.contains("project_resolve_symbol"));
        assert!(!source.contains("project_set_dependencies"));
    }

    #[test]
    fn leaf_patch_module_is_sliced_to_reachable_symbols() {
        let lib_source = r#"
            fn state() -> i32 { 1 }
            fn spawn_default_quads() {}
            fn get_aaa() -> i32 { 3 }
            fn scale_value_0032(input: i32, factor: i32) -> i32 { input * factor * get_aaa() }
        "#;
        let symbols = vec![
            symbol("get_aaa", "top", 0, "3"),
            symbol("scale_value_0032", "top", 1, "input * factor * get_aaa()"),
            symbol("run_update", "modules", 2, "1"),
            symbol("tick_000", "modules::module_000", 3, "1"),
        ];
        let names = PatchModuleNames {
            resolver: "project_resolve_symbol",
            set_api: "project_set_api",
            set_dependencies: "project_set_dependencies",
        };
        // The changed function is the leaf `get_aaa`, whose body reaches
        // nothing - so only it (plus the always-present plumbing) is emitted.
        let source = build_patch_module_source(lib_source, &symbols, "get_aaa", &names);
        assert!(source.contains("pub fn get_aaa(input: i32) -> i32 {"));
        // Unreached hot symbols, helpers and the whole module graph are sliced
        // out - no wrappers at all.
        assert!(!source.contains("scale_value_0032"));
        assert!(!source.contains("run_update"));
        assert!(!source.contains("mod modules"));
        assert!(!source.contains("fn state()"));
        assert!(!source.contains("spawn_default_quads"));
        assert!(!source.contains("crate::dependency_address"));
        // The resolver only knows the symbols actually present.
        assert!(source.contains("\"get_aaa\" => get_aaa as usize,"));
        assert!(!source.contains("\"scale_value_0032\" =>"));
    }

    #[test]
    fn closure_includes_reached_hot_wrappers_and_helpers() {
        let lib_source = r#"
            fn state() -> i32 { 1 }
            fn get_aaa() -> i32 { 3 }
            fn scale_value_0032(input: i32, factor: i32) -> i32 { input * factor * get_aaa() }
        "#;
        let symbols = vec![
            symbol("get_aaa", "top", 0, "3"),
            symbol("scale_value_0032", "top", 1, "input * factor * get_aaa()"),
            symbol("run_update", "modules", 2, "1"),
        ];
        let names = PatchModuleNames {
            resolver: "project_resolve_symbol",
            set_api: "project_set_api",
            set_dependencies: "project_set_dependencies",
        };
        // scale_value_0032's body calls get_aaa, so get_aaa becomes a wrapper
        // (dependency index 0) while the unreached module fn is sliced out.
        let source = build_patch_module_source(lib_source, &symbols, "scale_value_0032", &names);
        assert!(source.contains("pub fn scale_value_0032(input: i32) -> i32 {"));
        assert!(source.contains("pub fn get_aaa(input: i32) -> i32 {"));
        assert!(source.contains("crate::dependency_address(0)"));
        assert!(!source.contains("run_update"));
        assert!(!source.contains("mod modules"));
        assert!(!source.contains("fn state()"));
    }

    #[test]
    fn module_leaf_keeps_its_module_path_when_sliced() {
        let lib_source = "fn get_aaa() -> i32 { 3 }\n";
        let symbols = vec![
            symbol("get_aaa", "top", 0, "3"),
            symbol("tick_000", "modules::module_000", 1, "input + 1"),
        ];
        let names = PatchModuleNames {
            resolver: "project_resolve_symbol",
            set_api: "project_set_api",
            set_dependencies: "project_set_dependencies",
        };
        // Changing a module leaf keeps the containing `mod modules` path but
        // slices out the unreached lib helper.
        let source = build_patch_module_source(
            lib_source,
            &symbols,
            "modules::module_000::tick_000",
            &names,
        );
        assert!(source.contains("pub mod module_000 {"));
        assert!(source.contains("pub fn tick_000(input: i32) -> i32 {"));
        assert!(source.contains("input + 1"));
        assert!(!source.contains("get_aaa"));
    }
}
