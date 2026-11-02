// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067a90(param_1: *mut u32, param_2: u32, param_3: u32) -> bool {
    mech_trace::enter!("10067a90", param_1, param_2, param_3);
    // SAFETY: param_1 points at the caller's array of at least four u32 values;
    // the global and __ftol are the game's own.
    unsafe {
        mech_trace::wr!("10067a90", param_1.add(1), write_unaligned, param_2);
        mech_trace::wr!("10067a90", param_1.add(2), write_unaligned, param_3);
        let l_var1 = mech_trace::cr!("10067a90", "__ftol", __ftol());
        mech_trace::wr!("10067a90", param_1.add(3), write_unaligned, l_var1 as u32);
        mech_trace::wr!(
            "10067a90",
            param_1,
            write_unaligned,
            mech_trace::gr!("10067a90", "100b38fc", DAT_100b38fc)
        );
        mech_trace::rt!(
            "10067a90",
            0 < mech_trace::rd!("10067a90", param_1.add(3), read_unaligned)
        )
    }
}
