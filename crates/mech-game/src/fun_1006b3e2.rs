// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f783(param_1: i32) -> u8;
    fn FUN_1007be21(param_1: *mut i32, param_2: u32) -> i32;
    fn FUN_1007bf28(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b3e2(param_1: *mut i32) {
    mech_trace::enter!("1006b3e2", param_1);
    // SAFETY: param_1 points at the game's object; the callees are the
    // game's own and the fields are read in the original's order.
    unsafe {
        let base = mech_trace::rd!("1006b3e2", param_1, read_unaligned);
        let arg = mech_trace::rd!(
            "1006b3e2",
            (base.wrapping_add(0x40) as *const i32),
            read_unaligned
        );
        mech_trace::cr!("1006b3e2", "1002f783", FUN_1002f783(arg));
        let base = mech_trace::rd!("1006b3e2", param_1, read_unaligned);
        let pi_var1 = mech_trace::rd!(
            "1006b3e2",
            (base.wrapping_add(0x40) as *const i32),
            read_unaligned
        ) as *mut i32;
        let pi_var2 = mech_trace::rd!("1006b3e2", param_1.add(0xe4), read_unaligned) as *mut i32;
        let p5 = mech_trace::rd!("1006b3e2", param_1.add(0xe5), read_unaligned) as *mut i32;
        let mut i_var3 = mech_trace::cr!("1006b3e2", "1007be21", FUN_1007be21(p5, 1));
        mech_trace::cr!("1006b3e2", "1007bf28", FUN_1007bf28(i_var3));
        i_var3 = mech_trace::cr!("1006b3e2", "1007be21", FUN_1007be21(pi_var2, 1));
        mech_trace::cr!("1006b3e2", "1007bf28", FUN_1007bf28(i_var3));
        i_var3 = mech_trace::cr!("1006b3e2", "1007be21", FUN_1007be21(pi_var1, 1));
        mech_trace::cr!("1006b3e2", "1007bf28", FUN_1007bf28(i_var3));
    }
}
