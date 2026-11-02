// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008e3e0: u32;
    static mut DAT_100a6780: u8;
    static mut DAT_100b0590: [i8; 0];
    static mut DAT_1008e42c: [i8; 0];
    static mut DAT_1008e43c: [i8; 0];
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e730(param_1: u32) -> *mut u8 {
    mech_trace::enter!("1000e730", param_1);
    // SAFETY: the globals and sprintf belong to the game; the buffer and
    // format strings are its own memory, used as the original does.
    unsafe {
        if mech_trace::gr!("1000e730", "1008e3e0", DAT_1008e3e0) == 0
            || mech_trace::gr!("1000e730", "100a6780", DAT_100a6780) == 0
        {
            mech_trace::cr!(
                "1000e730",
                "sprintf",
                sprintf(
                    (&raw mut DAT_100b0590).cast::<i8>(),
                    (&raw mut DAT_1008e43c).cast::<i8>(),
                    param_1,
                )
            );
        } else {
            mech_trace::cr!(
                "1000e730",
                "sprintf",
                sprintf(
                    (&raw mut DAT_100b0590).cast::<i8>(),
                    (&raw mut DAT_1008e42c).cast::<i8>(),
                    &raw mut DAT_100a6780,
                    param_1,
                )
            );
        }
        mech_trace::gw!("1000e730", "1008e3e0", DAT_1008e3e0, 0);
        mech_trace::rt!("1000e730", (&raw mut DAT_100b0590).cast::<u8>())
    }
}
