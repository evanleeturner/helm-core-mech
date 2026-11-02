// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab37c: *mut u32;
    static mut DAT_100ab380: *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10023920(param_1: *mut u32) {
    mech_trace::enter!("10023920", param_1);
    if param_1.is_null() {
        return;
    }
    // SAFETY: the globals are the game's own list head and tail; param_1 points
    // at a node of at least two u32 fields, and the tail node is valid when set.
    unsafe {
        if mech_trace::gr!("10023920", "100ab37c", DAT_100ab37c).is_null()
            && mech_trace::gr!("10023920", "100ab380", DAT_100ab380).is_null()
        {
            mech_trace::wr!("10023920", param_1, write_unaligned, 0);
            mech_trace::wr!("10023920", param_1.add(1), write_unaligned, 0);
            mech_trace::gw!("10023920", "100ab37c", DAT_100ab37c, param_1);
            mech_trace::gw!("10023920", "100ab380", DAT_100ab380, param_1);
        } else {
            mech_trace::wr!(
                "10023920",
                param_1,
                write_unaligned,
                mech_trace::gr!("10023920", "100ab380", DAT_100ab380) as usize as u32
            );
            mech_trace::wr!("10023920", param_1.add(1), write_unaligned, 0);
            mech_trace::wr!(
                "10023920",
                mech_trace::gr!("10023920", "100ab380", DAT_100ab380).add(1),
                write_unaligned,
                param_1 as usize as u32
            );
            mech_trace::gw!("10023920", "100ab380", DAT_100ab380, param_1);
        }
    }
}
