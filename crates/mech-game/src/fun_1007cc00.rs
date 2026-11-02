// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b681c: *mut u32;
    static mut DAT_100b6820: *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007cc00(param_1: *mut u32) {
    mech_trace::enter!("1007cc00", param_1);
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 is a caller-provided list node and the globals are the
    // game's own list head/tail pointers; accesses mirror the original.
    unsafe {
        if mech_trace::gr!("1007cc00", "100b681c", DAT_100b681c).is_null()
            && mech_trace::gr!("1007cc00", "100b6820", DAT_100b6820).is_null()
        {
            mech_trace::wr!("1007cc00", param_1.add(1), write_unaligned, 0);
            mech_trace::wr!("1007cc00", param_1, write_unaligned, 0);
            mech_trace::gw!("1007cc00", "100b681c", DAT_100b681c, param_1);
            mech_trace::gw!("1007cc00", "100b6820", DAT_100b6820, param_1);
        } else {
            mech_trace::wr!(
                "1007cc00",
                param_1.add(1),
                write_unaligned,
                mech_trace::gr!("1007cc00", "100b6820", DAT_100b6820) as u32
            );
            mech_trace::wr!("1007cc00", param_1, write_unaligned, 0);
            mech_trace::wr!(
                "1007cc00",
                mech_trace::gr!("1007cc00", "100b6820", DAT_100b6820),
                write_unaligned,
                param_1 as u32
            );
            mech_trace::gw!("1007cc00", "100b6820", DAT_100b6820, param_1);
        }
    }
}
