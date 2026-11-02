// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9d88: i32;
    static mut DAT_100a6798: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003cc10() {
    mech_trace::enter!("1003cc10");
    // SAFETY: the globals are the game's own memory; DAT_100a6798 points at
    // a structure whose field at offset 4 is read as an i32.
    unsafe {
        mech_trace::gw!(
            "1003cc10",
            "100b9d88",
            DAT_100b9d88,
            mech_trace::gr!("1003cc10", "100b9d88", DAT_100b9d88).wrapping_add(1)
        );
        let limit = mech_trace::rd!(
            "1003cc10",
            mech_trace::gr!("1003cc10", "100a6798", DAT_100a6798)
                .byte_add(4)
                .cast::<i32>(),
            read_unaligned
        );
        if limit <= mech_trace::gr!("1003cc10", "100b9d88", DAT_100b9d88) {
            mech_trace::gw!("1003cc10", "100b9d88", DAT_100b9d88, 0);
        }
    }
}
