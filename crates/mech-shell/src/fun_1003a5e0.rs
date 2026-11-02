// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_1008ee18: u32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10064a90(this: *mut core::ffi::c_void, param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003a5e0(param_1: *mut core::ffi::c_void, param_2: i32) {
    mech_trace::enter!("1003a5e0", param_1, param_2);
    // SAFETY: the globals and the callee are the game's own; DAT_10083508
    // holds the address of a game structure that is read at the original's offsets.
    unsafe {
        if (mech_trace::rd!(
            "1003a5e0",
            (mech_trace::gr!("1003a5e0", "10083508", DAT_10083508).wrapping_add(0x239a)
                as *const i8),
            read
        ) == 0)
            && (mech_trace::gr!("1003a5e0", "1008ee18", DAT_1008ee18) == 0)
        {
            mech_trace::cr!("1003a5e0", "10064a90", FUN_10064a90(param_1, 10));
        }
        if (param_2 != 0xc02b)
            && (param_2 != 0xc031)
            && (param_2 != 0xc029)
            && (param_2 != 0xc02f
                || mech_trace::rd!(
                    "1003a5e0",
                    (mech_trace::gr!("1003a5e0", "10083508", DAT_10083508).wrapping_add(0x23bb)
                        as *const i32),
                    read_unaligned
                ) != 2)
        {
            mech_trace::cr!("1003a5e0", "10064a90", FUN_10064a90(param_1, 0xb));
        }
    }
}
