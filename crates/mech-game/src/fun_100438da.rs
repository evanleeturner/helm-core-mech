// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16) -> bool;
    fn FUN_1002a132(param_1: i32) -> i32;
    fn FUN_10022b70(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100438da(param_1: i32, param_2: i16) -> bool {
    mech_trace::enter!("100438da", param_1, param_2);
    // SAFETY: the callees are the game's own; param_1 is an address of a game
    // object whose fields at 0x54 and 0xcc are read as the original does.
    unsafe {
        mech_trace::cr!("100438da", "1002950f", FUN_1002950f(param_1, param_2));
        let i_var1 = mech_trace::cr!("100438da", "1002a132", FUN_1002a132(param_1));
        mech_trace::cr!("100438da", "10022b70", FUN_10022b70(param_1, i_var1));
        let a = mech_trace::rd!(
            "100438da",
            ((param_1 as u32).wrapping_add(0xcc) as *const i32),
            read_unaligned
        );
        let b = mech_trace::rd!(
            "100438da",
            ((param_1 as u32).wrapping_add(0x54) as *const i32),
            read_unaligned
        );
        mech_trace::rt!("100438da", a.wrapping_add(2000) <= b)
    }
}
