// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0f18: i32;
    static mut DAT_100b0f1c: u32;
    static mut DAT_1011515c: u32;
    static mut DAT_10115170: u32;
    static mut DAT_10167e70: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10035f13();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10054cc0(param_1: i32);
    fn FUN_100547e0(param_1: *mut i32, param_2: *mut i32);
    fn FUN_10054e09(param_1: *mut i16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10054c14(param_1: i32) {
    mech_trace::enter!("10054c14", param_1);
    // SAFETY: the globals and callees are the game's own; the array at
    // DAT_10167e70 holds 8-int entries, indexed as the original does.
    unsafe {
        FUN_10035f13();
        mech_trace::gw!("10054c14", "100b0f18", DAT_100b0f18, 0);
        mech_trace::gw!("10054c14", "100b0f1c", DAT_100b0f1c, 0);
        mech_trace::gw!("10054c14", "1011515c", DAT_1011515c, 1);
        mech_trace::gw!(
            "10054c14",
            "10115170",
            DAT_10115170,
            mech_trace::gr!("10054c14", "10167e70", DAT_10167e70) as u32
        );
        FUN_10054cc0(param_1);
        if 1 < mech_trace::gr!("10054c14", "100b0f18", DAT_100b0f18) {
            FUN_100547e0(
                mech_trace::gr!("10054c14", "10167e70", DAT_10167e70),
                mech_trace::gr!("10054c14", "10167e70", DAT_10167e70).offset(
                    mech_trace::gr!("10054c14", "100b0f18", DAT_100b0f18)
                        .wrapping_sub(1)
                        .wrapping_mul(8) as isize,
                ),
            );
        }
        let mut local_8: i32 = 0;
        while local_8 < mech_trace::gr!("10054c14", "100b0f18", DAT_100b0f18) {
            let p = mech_trace::rd!(
                "10054c14",
                mech_trace::gr!("10054c14", "10167e70", DAT_10167e70)
                    .offset(local_8.wrapping_mul(8) as isize),
                read_unaligned
            );
            FUN_10054e09(p as usize as *mut i16);
            local_8 = local_8.wrapping_add(1);
        }
    }
}
