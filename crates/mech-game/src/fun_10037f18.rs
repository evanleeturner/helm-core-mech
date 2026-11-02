// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf78: u32;
    static mut DAT_100adf7c: u32;
    static DAT_100ae410: [i8; 0];
    static DAT_100ae41c: [i8; 0];
    static DAT_100ae428: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(str1: *const i8, str2: *const i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10037f18(str1: *const i8) {
    mech_trace::enter!("10037f18", str1);
    // SAFETY: the callee, strings and globals are the game's own; str1 is a
    // NUL-terminated string supplied by the caller.
    unsafe {
        if mech_trace::cr!(
            "10037f18",
            "__strcmpi",
            __strcmpi(str1, (&raw const DAT_100ae410).cast::<i8>())
        ) == 0
        {
            mech_trace::gw!("10037f18", "100adf78", DAT_100adf78, 0x140);
            mech_trace::gw!("10037f18", "100adf7c", DAT_100adf7c, 200);
        } else if mech_trace::cr!(
            "10037f18",
            "__strcmpi",
            __strcmpi(str1, (&raw const DAT_100ae41c).cast::<i8>())
        ) == 0
        {
            mech_trace::gw!("10037f18", "100adf78", DAT_100adf78, 0x280);
            mech_trace::gw!("10037f18", "100adf7c", DAT_100adf7c, 0x1e0);
        } else if mech_trace::cr!(
            "10037f18",
            "__strcmpi",
            __strcmpi(str1, (&raw const DAT_100ae428).cast::<i8>())
        ) == 0
        {
            mech_trace::gw!("10037f18", "100adf78", DAT_100adf78, 0x400);
            mech_trace::gw!("10037f18", "100adf7c", DAT_100adf7c, 0x300);
        } else {
            mech_trace::gw!("10037f18", "100adf78", DAT_100adf78, 0x140);
            mech_trace::gw!("10037f18", "100adf7c", DAT_100adf7c, 200);
        }
    }
}
