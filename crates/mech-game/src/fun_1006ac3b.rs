// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c40: u32;
    static mut DAT_100b3c44: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100467ab();
    fn FUN_1003c260();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006ac3b() {
    mech_trace::enter!("1006ac3b");
    // SAFETY: the globals and callees are the game's own; they are read and
    // called in the original's order.
    unsafe {
        if mech_trace::gr!("1006ac3b", "100b3c40", DAT_100b3c40) != u32::MAX {
            FUN_100467ab();
        }
        if mech_trace::gr!("1006ac3b", "100b3c44", DAT_100b3c44) != u32::MAX {
            FUN_1003c260();
        }
    }
}
