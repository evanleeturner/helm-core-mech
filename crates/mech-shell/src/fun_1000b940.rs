// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000b8f0(
        param_1: u32,
        param_2: u32,
        found_index: i32,
        callee_value_out: *mut i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000b940(
    param_1: u32,
    param_2: u32,
    found_index_out: *mut i32,
    callee_value_out: *mut i32,
) -> u32 {
    let mut probe_index: i32 = 0;
    let mut callee_value: i32 = 0;
    loop {
        // SAFETY: the callee is the game's own; callee_value is a live local
        // whose address is passed as the out parameter.
        let hit = unsafe { FUN_1000b8f0(param_1, param_2, probe_index, &raw mut callee_value) };
        if hit != 0 {
            if !found_index_out.is_null() {
                // SAFETY: caller-provided non-null out pointer.
                unsafe { found_index_out.write_unaligned(probe_index) };
            }
            if !callee_value_out.is_null() {
                // SAFETY: caller-provided non-null out pointer.
                unsafe { callee_value_out.write_unaligned(callee_value) };
            }
            return 1;
        }
        probe_index = probe_index.wrapping_add(1);
        if probe_index >= 8 {
            break;
        }
    }
    0
}
