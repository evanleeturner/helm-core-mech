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
    mech_trace::enter!(
        "1000b940",
        param_1,
        param_2,
        found_index_out,
        callee_value_out
    );
    let mut probe_index: i32 = 0;
    let mut callee_value: i32 = 0;
    loop {
        // SAFETY: the callee is the game's own; callee_value is a live local
        // whose address is passed as the out parameter.
        let hit = unsafe {
            mech_trace::cr!(
                "1000b940",
                "1000b8f0",
                FUN_1000b8f0(param_1, param_2, probe_index, &raw mut callee_value)
            )
        };
        if hit != 0 {
            if !found_index_out.is_null() {
                // SAFETY: caller-provided non-null out pointer.
                unsafe {
                    mech_trace::wr!("1000b940", found_index_out, write_unaligned, probe_index)
                };
            }
            if !callee_value_out.is_null() {
                // SAFETY: caller-provided non-null out pointer.
                unsafe {
                    mech_trace::wr!("1000b940", callee_value_out, write_unaligned, callee_value)
                };
            }
            return mech_trace::rt!("1000b940", 1);
        }
        probe_index = probe_index.wrapping_add(1);
        if probe_index >= 8 {
            break;
        }
    }
    mech_trace::rt!("1000b940", 0)
}
