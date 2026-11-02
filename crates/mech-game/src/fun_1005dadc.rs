// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005dab8(param_1: u32, param_2: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005dadc(param_1: *mut u32) {
    mech_trace::enter!("1005dadc", param_1);
    // SAFETY: param_1 points at the caller's list head; nodes are linked
    // through the word at offset 0x14, and the callee is the game's own.
    unsafe {
        let mut local_8 = mech_trace::rd!("1005dadc", param_1, read_unaligned) as *mut u32;
        while !local_8.is_null() {
            mech_trace::cr!(
                "1005dadc",
                "1005dab8",
                FUN_1005dab8(param_1 as u32, local_8)
            );
            local_8 =
                mech_trace::rd!("1005dadc", local_8.byte_add(0x14), read_unaligned) as *mut u32;
        }
    }
}
