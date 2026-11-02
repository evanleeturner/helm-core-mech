// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10086951(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003eb60(param_1: u32) {
    // SAFETY: the callee is the game's own function; it is forwarded the
    // same argument unchanged.
    unsafe {
        FUN_10086951(param_1);
    }
}
