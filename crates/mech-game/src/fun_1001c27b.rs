// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001c299(param_1: i32, param_2: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c27b(param_1: i32) {
    mech_trace::enter!("1001c27b", param_1);
    // SAFETY: param_1 is an address of a game object; the field at offset
    // 0x1c is read as the original does, and the callee is the game's own.
    unsafe {
        let v = mech_trace::rd!(
            "1001c27b",
            ((param_1 as u32).wrapping_add(0x1c) as *const u32),
            read_unaligned
        );
        FUN_1001c299(param_1, v);
    }
}
