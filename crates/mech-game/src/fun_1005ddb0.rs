// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab238: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(a: i32, b: i32) -> i32;
    fn FUN_10003d60(a: i32, b: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005ddb0() {
    mech_trace::enter!("1005ddb0");
    // SAFETY: the callees and the global are the game's own; the calls keep
    // the original's order and arguments.
    unsafe {
        let i_var1 = mech_trace::cr!("1005ddb0", "10002570", FUN_10002570(1, 0xf));
        FUN_10003d60(
            ((mech_trace::gr!("1005ddb0", "100ab238", DAT_100ab238) as u32).wrapping_shl(0x10))
                as i32,
            i_var1,
        );
    }
}
