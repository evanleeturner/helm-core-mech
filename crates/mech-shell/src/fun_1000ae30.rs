// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d010: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1006f780(param_1: *const i8);
    fn FUN_1006e790(param_1: *mut u8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000ae30(param_1: *mut u8) {
    mech_trace::enter!("1000ae30", param_1);
    let mut local_20: [i8; 32] = [0; 32];
    // SAFETY: param_1 points at the caller's object, read at the original's
    // offsets; the callees and the format string global are the game's own,
    // and local_20 is the local buffer the original formats into.
    unsafe {
        if mech_trace::rd!("1000ae30", param_1.byte_add(0x84b), read) == 1 {
            mech_trace::cr!(
                "1000ae30",
                "sprintf",
                sprintf(
                    local_20.as_mut_ptr(),
                    &raw mut DAT_1008d010 as *const i8,
                    param_1.byte_add(0x823),
                )
            );
            FUN_1006f780(local_20.as_ptr());
        }
        FUN_1006e790(param_1);
    }
}
