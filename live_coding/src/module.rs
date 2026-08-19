//! Self-contained patch-module source generation.
//!
//! For one changed hot function, the host generates a complete cdylib that
//! contains: lib.rs consts/types/helpers + ABI mirrors + `state()` +
//! `project_set_api`, a dependency-address table (filled by the host at
//! load), the changed function's REAL new body, and a forwarding wrapper for
//! every OTHER hot function (each jumping through the dependency table back
//! into the base DLL).  The base prologue of the changed function is then
//! patched to this module.

use crate::analysis::{
    extract_top_level_consts, extract_top_level_functions, extract_top_level_type_definitions,
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

/// Build the self-contained module for one changed hot function: lib.rs
/// consts/types/helpers + ABI mirrors + state() + project_set_api, a
/// dependency-address table (filled by the host at load) and forwarding
/// wrappers for every OTHER hot function, so the changed function's call
/// sites (unchanged text) resolve to stubs that jump back into the base DLL.
/// This is what makes patching a function "in the middle" of the call graph
/// take effect: the base prologue of the changed function is patched to this
/// module, and its calls hit the base (whose own prologues can be patched
/// later too - patches compose).
pub fn build_patch_module_source(
    lib_source: &str,
    symbols: &[HotSymbol],
    changed_key: &str,
) -> String {
    let consts = extract_top_level_consts(lib_source).join("\n");
    let type_definitions = extract_top_level_type_definitions(lib_source).join("\n");
    // Inject non-hot lib helpers only - hot helpers (get_aaa, scale_value_003,
    // project_update) are emitted as wrappers/real-bodies in `top_definitions`
    // instead, so injecting them again here would be a duplicate definition.
    // Non-hot helpers (infrastructure like `state`/`spawn_default_quads`) are
    // kept as-is.
    let helper_functions = extract_top_level_functions(lib_source)
        .into_iter()
        .filter(|definition| {
            let name = definition
                .strip_prefix("fn ")
                .and_then(|rest| rest.split(['(', ' ', '\n', '\t']).next())
                .unwrap_or("");
            !symbols
                .iter()
                .any(|symbol| symbol.name == name && symbol.scope == "top")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let modules_skeleton = build_modules_skeleton(symbols, changed_key);

    // The top-level hot symbol (project_update): real body if it is the one
    // being patched, otherwise a forwarding wrapper.
    let top_definitions = symbols
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

    // The patch module needs its own `project_resolve_symbol` so the host can
    // resolve the (non-exported) hot functions by name in the patch DLL too.
    let resolver_arms = symbols
        .iter()
        .map(|symbol| {
            let path = symbol_path(symbol);
            format!("        {path:?} => {path} as usize,")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let resolver = format!(
        "#[unsafe(no_mangle)]\npub extern \"C\" fn project_resolve_symbol(name: *const std::os::raw::c_char) -> usize {{\n\
         \x20   if name.is_null() {{\n\
         \x20       return 0;\n\
         \x20   }}\n\
         \x20   let name = unsafe {{ std::ffi::CStr::from_ptr(name) }};\n\
         \x20   let Ok(name) = name.to_str() else {{ return 0; }};\n\
         \x20   match name {{\n\
         {resolver_arms}\n\
         \x20       _ => 0,\n\
         \x20   }}\n\
         }}\n"
    );

    format!(
        "#![allow(dead_code)]\n\
         use std::ffi::c_void;\n\
         \n\
         {consts}\n\
         \n\
         {type_definitions}\n\
         \n\
         {helper_functions}\n\
         \n\
         #[repr(C)]\n\
         struct ProjectApi {{\n\
             get_state: extern \"C\" fn() -> *mut std::ffi::c_void,\n\
             screen_width: extern \"C\" fn() -> f32,\n\
             screen_height: extern \"C\" fn() -> f32,\n\
         }}\n\
         \n\
         #[repr(C)]\n\
         struct Quad {{\n\
             x: f32,\n\
             y: f32,\n\
             base_y: f32,\n\
             w: f32,\n\
             h: f32,\n\
             vx: f32,\n\
             jump_phase: f32,\n\
             jump_speed: f32,\n\
             jump_height: f32,\n\
             color: u32,\n\
         }}\n\
         \n\
         const MAX_QUADS: usize = 8;\n\
         \n\
         #[repr(C)]\n\
         struct GameState {{\n\
             tick: f32,\n\
             quad_count: usize,\n\
             quads: [Quad; MAX_QUADS],\n\
         }}\n\
         \n\
         static mut API: *const ProjectApi = std::ptr::null();\n\
         \n\
         #[no_mangle]\n\
         #[allow(private_interfaces)]\n\
         pub extern \"C\" fn project_set_api(api: *const ProjectApi) {{\n\
             unsafe {{ API = api; }}\n\
         }}\n\
         \n\
         fn state() -> &'static mut GameState {{\n\
             unsafe {{\n\
                 let api = API;\n\
                 assert!(!api.is_null(), \"project_set_api must be called before update\");\n\
                 let state_pointer = ((*api).get_state)() as *mut GameState;\n\
                 &mut *state_pointer\n\
             }}\n\
         }}\n\
         \n\
         static mut DEPENDENCY_ADDRESSES: *const usize = std::ptr::null();\n\
         \n\
         #[no_mangle]\n\
         #[allow(private_interfaces)]\n\
         pub extern \"C\" fn project_set_dependencies(addresses: *const usize) {{\n\
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
         {resolver}\n"
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
        let definition = build_hot_definition(
            "get_aaa",
            "get_aaa",
            "",
            "i32",
            "42",
            0,
            "get_aaa",
        );
        assert!(definition.contains("pub fn get_aaa() -> i32 {"));
        assert!(definition.contains("42"));
        assert!(!definition.contains("dependency_address"));
    }

    #[test]
    fn unchanged_function_gets_forwarding_wrapper() {
        let definition = build_hot_definition(
            "get_aaa",
            "get_aaa",
            "input: i32",
            "i32",
            "42",
            3,
            "other",
        );
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
        let lib_source = "const QUAD_SIZE: f32 = 46.0;\n\nfn get_aaa() -> i32 { 3 }\n";
        let symbols = vec![symbol("get_aaa", "top", 0, "3")];
        // A DIFFERENT function is the changed one, so get_aaa becomes a
        // forwarding wrapper through dependency index 0.
        let source = build_patch_module_source(lib_source, &symbols, "project_update");
        assert!(source.contains("pub extern \"C\" fn project_set_api"));
        assert!(source.contains("project_set_dependencies"));
        assert!(source.contains("fn dependency_address(index: usize)"));
        assert!(source.contains("crate::dependency_address(0)"));
        assert!(source.contains("\"get_aaa\" => get_aaa as usize,"));
        assert!(source.contains("QUAD_SIZE"));
    }
}
