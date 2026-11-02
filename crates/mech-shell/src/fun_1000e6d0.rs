// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008e3e0: u32;
    static mut DAT_100a6780: u8;
    static mut DAT_100b0568: [i8; 0];
    static mut DAT_1008e410: [i8; 0];
    static mut DAT_1008e420: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e6d0(param_1: u32) -> *mut u8 {
    // SAFETY: the globals, format strings and sprintf are the game's own;
    // the buffer is the game's path buffer, written where the original does.
    unsafe {
        if DAT_1008e3e0 == 0 || DAT_100a6780 == 0 {
            sprintf(
                (&raw mut DAT_100b0568).cast::<i8>(),
                (&raw const DAT_1008e420).cast::<i8>(),
                param_1,
            );
        } else {
            sprintf(
                (&raw mut DAT_100b0568).cast::<i8>(),
                (&raw const DAT_1008e410).cast::<i8>(),
                &raw mut DAT_100a6780,
                param_1,
            );
        }
        DAT_1008e3e0 = 0;
        (&raw mut DAT_100b0568).cast::<u8>()
    }
}
