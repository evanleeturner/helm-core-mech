// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3a9c: i32;
    static mut DAT_100b3aa0: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006a240() {
    mech_trace::enter!("1006a240");
    // SAFETY: the globals are the game's own memory, read and written in the original's order.
    unsafe {
        let i_var1 = mech_trace::gr!("1006a240", "100b3aa0", DAT_100b3aa0);
        mech_trace::gw!(
            "1006a240",
            "100b3a9c",
            DAT_100b3a9c,
            mech_trace::gr!("1006a240", "100b3a9c", DAT_100b3a9c).wrapping_sub(1)
        );
        if mech_trace::gr!("1006a240", "100b3a9c", DAT_100b3a9c) == 0 {
            mech_trace::gw!("1006a240", "100b3a9c", DAT_100b3a9c, 4);
            mech_trace::gw!(
                "1006a240",
                "100b3aa0",
                DAT_100b3aa0,
                mech_trace::gr!("1006a240", "100b3aa0", DAT_100b3aa0).wrapping_sub(1)
            );
            if i_var1 == 0 {
                mech_trace::gw!("1006a240", "100b3aa0", DAT_100b3aa0, 3);
            }
        }
    }
}
