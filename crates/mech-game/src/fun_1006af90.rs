// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _MEM_free_lock_4(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006af90(param_1: u32) {
    mech_trace::enter!("1006af90", param_1);
    // SAFETY: the callee is the game's own function; it is called with the
    // caller's argument unchanged.
    unsafe {
        _MEM_free_lock_4(param_1);
    }
}
