// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f6518: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005dadc(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d301(param_1: i32) {
    mech_trace::enter!("1000d301", param_1);
    // SAFETY: the global and the callee are the game's own; the address is
    // computed from the global's address plus a byte offset, as the original does.
    unsafe {
        let base = (&raw mut DAT_101f6518) as i32;
        FUN_1005dadc(base.wrapping_add(param_1.wrapping_mul(0x7c)) as *mut i32);
    }
}
