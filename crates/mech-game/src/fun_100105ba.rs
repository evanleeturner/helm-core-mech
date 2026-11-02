// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001060a(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100105ba(param_1: i32) {
    mech_trace::enter!("100105ba", param_1);
    if param_1 < 0 {
        let mut i: i32 = 0;
        while i < 7 {
            // SAFETY: the callee is the game's own function; it is called with each index in order.
            unsafe {
                mech_trace::cr!("100105ba", "1001060a", FUN_1001060a(i));
            }
            i = i.wrapping_add(1);
        }
    } else {
        // SAFETY: the callee is the game's own function.
        unsafe {
            mech_trace::cr!("100105ba", "1001060a", FUN_1001060a(param_1));
        }
    }
}
