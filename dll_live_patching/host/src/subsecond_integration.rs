// subsecond_integration.rs — Subsecond runtime compatibility (Step 3)
//
// Demonstrates that our DLL-aware patching is compatible with Subsecond's
// types and call()-based dispatch.

use crate::jump_table;
use crate::types::LocalJumpTable;
use std::path::PathBuf;

/// Call the hot-reloadable function through Subsecond's dispatch.
/// Routes through subsecond::call() → HotFn::try_call() → call_hot_add()
/// → APP_JUMP_TABLE lookup → patched function.
pub fn call_via_subsecond(left: i32, right: i32) -> i32 {
    subsecond::call(|| unsafe { jump_table::call_hot_add(left, right) })
}

/// Convert our local JumpTable to a subsecond_types::JumpTable for
/// compatibility demonstration.
pub fn to_subsecond_jump_table(table: &LocalJumpTable) -> subsecond_types::JumpTable {
    subsecond_types::JumpTable {
        lib: PathBuf::new(),
        map: table.map.clone().into_iter().collect(),
        aslr_reference: 0,
        new_base_address: 0,
        ifunc_count: 0,
    }
}
